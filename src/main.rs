use clap::{Parser, Subcommand};
mod app;
use app::App;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Mark a directory
    Mark,

    /// Unmark a directory
    Unmark { index: Option<usize> },

    /// Shows status of marked directories
    Status,

    /// Clears marked directory list
    Clear,

    /// Copy path from the list to your sys clipboard
    #[command(arg_required_else_help = true)]
    Clip { index: Option<usize> },

    /// Display marked directory list
    List,

    /// Restores recently cleared list
    Reset,
}

fn main() {
    let app = App::new();
    let cmd = Cli::parse();
    match &cmd.command {
        Commands::Mark => app.mark(),
        Commands::Unmark { index } => app.unmark(index),
        Commands::Status => app.status(),
        Commands::Clear => app.clear(),
        Commands::Clip { index } => app.clip(index.unwrap()),
        Commands::List => app.list(),
        Commands::Reset => app.restore(),
    }
}
