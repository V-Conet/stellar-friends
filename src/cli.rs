use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "stellar-friends")]
#[command(about = "友链数据生成工具")]
pub struct Cli {
    /// 标签文件路径
    #[arg(short = 'l', long)]
    pub labels: Option<String>,

    /// 好友文件路径
    #[arg(short = 'f', long, required = true)]
    pub friends: String,

    /// 输出文件路径
    #[arg(short = 'o', long, required = true)]
    pub output: String,
}
