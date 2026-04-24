# Stellar 博客动态友链
使用 Rust 编写的命令行工具，用于从 YAML 文件中提取博客动态友链信息，并生成 JSON 格式的输出，以便在 Stellar 博客中使用。

## Usage

```shell
$ stellar-friends --help
友链数据处理工具

Usage: stellar-friends [OPTIONS] --friends <FRIENDS> --output <OUTPUT>

Options:
  -l, --labels <LABELS>    标签文件路径
  -f, --friends <FRIENDS>  好友文件路径
  -o, --output <OUTPUT>    输出文件路径
  -h, --help               Print help
```

无标签：
```shell
$ stellar-friends -f path/to/friends.yaml -o path/to/output.json
```

有标签：
```shell
$ stellar-friends -l path/to/labels.yaml -f path/to/friends.yaml -o path/to/output.json
```
## 友链及标签格式

均采用 YAML 格式，具体格式如下：

```yaml
- title: 友链标题
  url: 友链链接
  description: 友链描述
  keywords: 友链关键词，逗号分隔（可选）
  icon: 友链图标链接
  snapshot: 友链快照链接（可选）
  feed: 友链 RSS/Atom 链接（可选，若需要显示最近文章，则必须）
  issue_number: 原为 GitHub Issue 编号，已废弃，但仍需提供以保持排序
  labels: 友链标签列表（可选）
```

可复用标签格式：

```yaml
- name: 标签名称
  color: 标签颜色（十六进制，无 \#）
  hue: 标签色相（0-360）
  saturation: 标签饱和度（0-100）
  lightness: 标签亮度（0-100）
```