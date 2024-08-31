#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    #[arg(short, long, help = "Only read ZSH history")]
    pub zsh: bool,

    #[arg(short, long, help = "Only read Bash history")]
    pub bash: bool,

    #[arg(short, long, help = "# of top commands to list")]
    pub num_commands: Option<u16>,

    #[arg(short, long, help = "Export the data to CSV")]
    pub export: bool,
}
