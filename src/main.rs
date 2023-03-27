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
    #[command(arg_required_else_help = true)]
    Unmark { index: Option<usize> },

    /// Check the availability of marked directories
    Check,

    /// Clears marked directory list
    Clear,

    /// Display marked directory list
    List,
}

fn main() {
    let app = App::new();
    let cmd = Cli::parse();
    match &cmd.command {
        Commands::Mark => app.mark(),
        Commands::Unmark { index } => app.unmark(index.unwrap()),
        Commands::Check => app.check(),
        Commands::Clear => app.clear(),
        Commands::List => app.list(),
    }
}
