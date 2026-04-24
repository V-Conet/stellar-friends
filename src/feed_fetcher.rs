use std::time::Duration;

use anyhow::{anyhow, bail, Context, Result};
use feed_rs::model::Entry;
use feed_rs::parser;
use reqwest::Client;

use crate::models::Post;

const MAX_RETRIES: u32 = 3;
const REQUEST_TIMEOUT: Duration = Duration::from_secs(15);
const RETRY_DELAY: Duration = Duration::from_secs(1);

/// 从 RSS/Atom feed URL 获取最近的文章
pub async fn fetch_posts_from_feed(feed_url: &str, max_posts: usize) -> Result<Vec<Post>> {
    let client = Client::builder()
        .timeout(REQUEST_TIMEOUT)
        .user_agent("stellar-friends/0.1.0")
        .build()?;

    let mut last_error = None;

    for attempt in 1..=MAX_RETRIES {
        match fetch_and_parse(&client, feed_url, max_posts).await {
            Ok(posts) => return Ok(posts),
            Err(error) => {
                last_error = Some(error);
                if attempt < MAX_RETRIES {
                    tokio::time::sleep(RETRY_DELAY).await;
                }
            }
        }
    }

    Err(last_error.unwrap_or_else(|| anyhow!("获取 feed 失败: {}", feed_url)))
}

async fn fetch_and_parse(client: &Client, feed_url: &str, max_posts: usize) -> Result<Vec<Post>> {
    let response = client
        .get(feed_url)
        .send()
        .await
        .with_context(|| format!("请求 feed 失败: {}", feed_url))?;

    if !response.status().is_success() {
        bail!(
            "HTTP 错误: {} {}",
            response.status().as_u16(),
            response.status()
        );
    }

    let content = response
        .text()
        .await
        .with_context(|| format!("读取 feed 内容失败: {}", feed_url))?;

    let feed = parser::parse(content.as_bytes())
        .map_err(|e| anyhow!("解析 feed 失败: {}", e))?;

    Ok(feed
        .entries
        .into_iter()
        .filter_map(entry_to_post)
        .take(max_posts)
        .collect())
}

/// 将 feed entry 转换为 Post
fn entry_to_post(entry: Entry) -> Option<Post> {
    let title = entry.title.map(|t| t.content).unwrap_or_default();

    // 获取链接
    let link = entry
        .links
        .into_iter()
        .next()
        .map(|l| l.href)
        .unwrap_or_default();

    // 获取发布时间
    let published = entry
        .published
        .or_else(|| entry.updated)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_else(|| "1970-01-01 00:00".to_string());

    if title.is_empty() || link.is_empty() {
        return None;
    }

    Some(Post {
        title,
        link,
        published,
    })
}
