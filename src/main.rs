mod config;

use clap::{Parser, Subcommand, Args};
use config::config_loader::load_config;


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
    Config(Configure)
}

#[derive(Args)]
struct Configure {
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
        Commands::Config(cfg) => {
            let existing = load_config().expect("Error loading config");

            println!("New client id: '{}' is replacing old client id: '{}'", 
                     cfg.client_id, existing.app.client_id);
        }
    }
}

