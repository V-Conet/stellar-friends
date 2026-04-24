use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::models::{Friend, Label};

/// 加载并解析标签文件
pub fn load_labels<P: AsRef<Path>>(path: P) -> Result<HashMap<String, Label>> {
    let content = fs::read_to_string(&path)
        .with_context(|| format!("无法读取标签文件: {}", path.as_ref().display()))?;

    let labels: Vec<Label> = serde_saphyr::from_str(&content)
        .with_context(|| format!("解析标签文件失败: {}", path.as_ref().display()))?;

    let map: HashMap<String, Label> = labels
        .into_iter()
        .map(|l| (l.name.clone(), l))
        .collect();

    Ok(map)
}

/// 加载并解析好友文件
pub fn load_friends<P: AsRef<Path>>(path: P) -> Result<Vec<Friend>> {
    let content = fs::read_to_string(&path)
        .with_context(|| format!("无法读取好友文件: {}", path.as_ref().display()))?;

    let friends: Vec<Friend> = serde_saphyr::from_str(&content)
        .with_context(|| format!("解析好友文件失败: {}", path.as_ref().display()))?;

    Ok(friends)
}
