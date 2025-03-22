use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Hypraland keyboard utils", about = "Helper program to control hypraland keyboard stuff.")]
pub struct CommandLineArgs {
    /// Config file path
    #[arg(short, long)]
    pub config_file: Option<String>,

    /// Print the current keyboard layout for all keyboards
    #[arg(short, long)]
    pub print_layouts: Option<bool>,

    /// Switch to the next layout
    #[arg(short, long)]
    pub switch_layout: Option<bool>,

    /// Map layouts to short string i.e. "English (US)=us;German=de"
    #[arg(short, long)]
    pub map: Option<String>

}

impl CommandLineArgs {
    pub fn new() -> Self {
        CommandLineArgs::parse()
    }
}
