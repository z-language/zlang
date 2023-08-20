#[derive(Debug, clap::Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Path to the input file
    pub file: String,

    /// Run without compiling
    #[arg(short, long)]
    pub parse_only: bool,

    #[arg(long)]
    pub dry_run: bool,

    /// Output generated asm
    #[arg(long)]
    pub asm: Option<String>,

    /// Path to the output file
    #[arg(short, long, default_value_t = String::from("main.o"))]
    pub out: String,
}
