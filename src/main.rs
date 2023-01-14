mod cfg;
mod spotify;
mod handlers;

use clap::{Parser, Subcommand, Args};

use crate::spotify::auth::authenticator::get_auth_token;
use crate::handlers::config_handler;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand)]
enum Commands {
    /// Toggles playback. If song is given, it will try to play that specific song
    Play { song: Option<String> },
    /// Configures spf 
    Config(UserConfigure)
}

#[derive(Args, Debug)]
pub struct UserConfigure {
    /// Sets client id, taken from Spotify Dev Dashboard
    #[arg(long)]
    client_id: Option<String>, 
    /// Sets client secret, taken from Spofity Dev Dashboard
    #[arg(long)]
    client_secret: Option<String>,
    /// Sets the port that spf will listen on
    #[arg(long)]
    redirect_port: Option<u32>,
}

fn main() {
    let cli = Cli::parse();

    match &cli.cmd {
        Commands::Play { song } => {
            if get_auth_token().is_err() {
                eprintln!("User is not authenticated!");
            } else {
                if let Some(songname) = song {
                    println!("Playing '{}'", songname);
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

