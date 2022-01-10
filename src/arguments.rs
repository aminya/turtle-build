use clap::Parser;

#[derive(Parser)]
#[clap(about = "The Ninja build system clone written in Rust", version)]
pub struct Arguments {
    #[clap(short, help = "Set a root build file")]
    pub file: Option<String>,
}
