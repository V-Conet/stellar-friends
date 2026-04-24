use std::collections::HashMap;
use std::fs;

use anyhow::{bail, Context, Result};
use clap::Parser;

mod cli;
mod feed_fetcher;
mod models;
mod parser;
mod transformer;

use cli::Cli;
use feed_fetcher::fetch_posts_from_feed;
use models::{Label, Output};
use parser::{load_friends, load_labels};
use transformer::{transform, validate_labels};

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let cli = Cli::parse();

    // 加载好友数据
    let friends = load_friends(&cli.friends)?;

    // 处理标签
    let labels: HashMap<String, Label> = if let Some(ref labels_path) = cli.labels {
        // 提供了标签文件
        load_labels(labels_path)?
    } else {
        // 未提供标签文件，检查好友是否有标签
        let has_labels = friends.iter().any(|f| {
            f.labels.as_ref().map(|l| !l.is_empty()).unwrap_or(false)
        });

        if has_labels {
            eprintln!("Warning: 未提供标签文件，{} 中的标签将被忽略", cli.friends);
        }

        HashMap::new()
    };

    // 验证标签（如果提供了标签文件）
    if cli.labels.is_some() {
        let (valid, missing) = validate_labels(&friends, &labels);
        if !valid {
            eprintln!("Error: 以下标签未在标签文件中定义:");
            for (friend, label) in missing {
                eprintln!("  - 好友 '{}' 使用了未定义的标签 '{}'", friend, label);
            }
            bail!("标签验证失败，无法生成输出");
        }
    }

    // 转换数据
    let output: Output = transform(friends, &labels);

    let mut output = output;
    for blog in &mut output.content {
        if let Some(feed_url) = blog.feed.as_deref() {
            match fetch_posts_from_feed(feed_url, 3).await {
                Ok(posts) => {
                    blog.posts = posts;
                }
                Err(error) => {
                    eprintln!("warning: 获取 feed 失败，已跳过 '{}': {}", blog.title, error);
                }
            }
        }
    }

    // 写入输出文件
    let json = serde_json::to_string_pretty(&output)
        .context("序列化输出数据失败")?;

    fs::write(&cli.output, json)
        .with_context(|| format!("写入输出文件失败: {}", cli.output))?;

    println!("成功生成: {}", cli.output);

    Ok(())
}
