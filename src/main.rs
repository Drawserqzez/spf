mod cfg;
mod spotify;
mod handlers;
mod commands;

use clap::{Parser, Subcommand};

use crate::spotify::auth::authenticator::get_auth_token;
use crate::handlers::config_handler;
use crate::commands::user_configure::UserConfigure;
use crate::commands::song_interaction::Song;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand)]
enum Commands {
    /// Toggles playback
    Play(Song),
    /// Configures spf 
    Config(UserConfigure)
}

fn main() {
    let cli = Cli::parse();

    match &cli.cmd {
        Commands::Play(song) => {
            if get_auth_token().is_err() {
                eprintln!("User is not authenticated!");
            } else {
                if let Some(track) = &song.title {
                    if let Some(artist) = &song.artist {
                        println!("Playing '{}' by '{}'", track, artist);
                    } else {
                        println!("Playing '{}'", track);
                    }
                } else if let Some(id) = &song.id {
                    println!("Playing song with id '{}'", id);
                } else {
                    println!("Toggling playback");
                }
            }
        },
        Commands::Config(cfg) => {
            match config_handler::update_config(&cfg) {
                Ok(msg) => println!("{}", msg),
                Err(e) => eprintln!("Error when trying to update config: {:?}", e)
            };

        }
    }
}

