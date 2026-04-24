use std::collections::HashMap;

use crate::models::{Blog, Friend, Label, Output, OutputLabel};

/// 验证好友的标签是否都在标签映射中存在
/// 返回 (是否全部有效, 缺失的标签列表)
pub fn validate_labels(
    friends: &[Friend],
    labels: &HashMap<String, Label>,
) -> (bool, Vec<(String, String)>) {
    let mut missing = Vec::new();

    for friend in friends {
        if let Some(ref friend_labels) = friend.labels {
            for label_name in friend_labels {
                if !labels.contains_key(label_name) {
                    missing.push((friend.title.clone(), label_name.clone()));
                }
            }
        }
    }

    (missing.is_empty(), missing)
}

/// 将好友数据转换为输出格式
pub fn transform(friends: Vec<Friend>, labels: &HashMap<String, Label>) -> Output {
    let content: Vec<Blog> = friends
        .into_iter()
        .map(|friend| transform_friend(friend, labels))
        .collect();

    Output {
        version: "v2".to_string(),
        content,
    }
}

fn transform_friend(friend: Friend, labels: &HashMap<String, Label>) -> Blog {
    let output_labels: Vec<OutputLabel> = friend
        .labels
        .as_ref()
        .map(|labels_list| {
            labels_list
                .iter()
                .filter_map(|name| labels.get(name).cloned().map(OutputLabel::from))
                .collect()
        })
        .unwrap_or_default();

    Blog {
        title: friend.title,
        url: friend.url,
        description: friend.description.unwrap_or_default(),
        keywords: friend.keywords.unwrap_or_default(),
        icon: friend.icon,
        snapshot: friend.snapshot.unwrap_or_default(),
        feed: friend.feed.unwrap_or_default(),
        posts: Vec::new(), // posts 由外部 RSS 解析填充
        issue_number: friend.issue_number,
        labels: output_labels,
    }
}
