use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg(short, long)]
    pub path: String,

    #[arg(short = 'P', long)]
    pub parallel: bool,

    #[arg(short, long)]
    pub write: bool,
}
