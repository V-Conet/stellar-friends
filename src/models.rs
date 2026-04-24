use serde::{Deserialize, Serialize};

// inputs

/// 标签定义
#[derive(Debug, Clone, Deserialize)]
pub struct Label {
    pub name: String,
    pub color: String,
    pub hue: i32,
    pub saturation: i32,
    pub lightness: i32,
}

/// 友链定义
#[derive(Debug, Clone, Deserialize)]
pub struct Friend {
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    pub keywords: Option<String>,
    pub icon: String,
    pub snapshot: Option<String>,
    pub feed: Option<String>,
    #[serde(rename = "issue_number")]
    pub issue_number: i64,
    pub labels: Option<Vec<String>>,
}

// outputs

/// JSON 结构输出
#[derive(Debug, Serialize)]
pub struct Output {
    pub version: String,
    pub content: Vec<Blog>,
}

/// 博客信息
#[derive(Debug, Serialize)]
pub struct Blog {
    pub title: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    pub icon: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub posts: Vec<Post>,
    #[serde(rename = "issue_number")]
    pub issue_number: i64,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub labels: Vec<OutputLabel>,
}

/// 文章信息
#[derive(Debug, Serialize)]
pub struct Post {
    pub title: String,
    pub link: String,
    pub published: String,
}

/// 输出标签
#[derive(Debug, Serialize)]
pub struct OutputLabel {
    pub name: String,
    pub color: String,
    pub hue: i32,
    pub saturation: i32,
    pub lightness: i32,
}

impl From<Label> for OutputLabel {
    fn from(label: Label) -> Self {
        OutputLabel {
            name: label.name,
            color: label.color,
            hue: label.hue,
            saturation: label.saturation,
            lightness: label.lightness,
        }
    }
}
