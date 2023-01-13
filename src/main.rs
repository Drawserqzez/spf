mod cfg;
mod spotify;
mod handlers;

use clap::{Parser, Subcommand, Args};

use crate::spotify::authenticator::authenticate;
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
struct UserConfigure {
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
            if authenticate().is_err() {
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
            let mut app_cfg:cfg::models::App = Default::default();

            if let Some(secret) = &cfg.client_secret {
                app_cfg.client_secret = secret.to_owned();
            }

            if let Some(id) = &cfg.client_id {
                app_cfg.client_id = id.to_owned();
            }

            if let Some(port) = cfg.redirect_port {
                app_cfg.redirect_port = port.to_owned();
            }

            match config_handler::update_config(&app_cfg) {
                Ok(msg) => println!("{}", msg),
                Err(e) => eprintln!("Error when trying to replace config: {:?}", e)
            };

        }
    }
}

