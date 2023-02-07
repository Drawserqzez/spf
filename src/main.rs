mod cfg;
mod spotify;
mod handlers;

use clap::{Parser, Subcommand};
use handlers::{config_handler, play_handler};



#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand)]
enum Commands {
    /// Toggles playback
    Play(play_handler::Song),
    /// Configures spf 
    Config(config_handler::UserConfigure)
}

fn main() {
    let cli = Cli::parse();

    match &cli.cmd {
        Commands::Play(song) => {
            match play_handler::handle_play_command(&song) {
                Ok(msg) => println!("{}", msg),
                Err(e) => eprintln!("Error when trying to play: {:?}", e),
            } 
        },
        Commands::Config(cfg) => {
            match config_handler::update_config(&cfg) {
                Ok(msg) => println!("{}", msg),
                Err(e) => eprintln!("Error when trying to update config: {:?}", e),
            };

        }
    }
}

