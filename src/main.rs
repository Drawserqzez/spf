mod cfg;

use clap::{Parser, Subcommand, Args};
use cfg::config_handler::load_config;

use crate::cfg::config_handler::update_client_info;


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
    UserConfig(UserConfigure)
}

#[derive(Args, Debug)]
struct UserConfigure {
    /// Sets client id, taken from Spotify Dev Dashboard
    #[arg(long)]
    client_id: String, 
    /// Sets client secret, taken from Spofity Dev Dashboard
    #[arg(long)]
    client_secret: String,
    /// Sets the port that spf will listen on
    #[arg(long, default_value_t = 1337)]
    redirect_port: u32
}

#[derive(Debug)]
struct SpfError(String);

fn main() {
    let cli = Cli::parse();

    match &cli.cmd {
        Commands::Play { song } => {
            if let Some(songname) = song {
                println!("Playing '{}'", songname);
            } else {
                println!("Toggling playback");
            }
        },
        Commands::UserConfig(cfg) => {
            // TODO: Check input to see if we have new fields, else print info about config path
            let mut domain_config:cfg::config::Config = Default::default();

            domain_config.app.client_secret = cfg.client_secret.to_owned();
            domain_config.app.client_id = cfg.client_secret.to_owned();
            domain_config.app.redirect_port = cfg.redirect_port;

            match update_client_info(&domain_config) {
                Ok(config) => println!("Replacing config with new: {:?}", config),
                Err(e) => eprintln!("Error when trying to replace config: {:?}", e)
            };

        }
    }
}

