use clap::Args;

#[derive(Args, Debug)]
pub struct UserConfigure {
    /// Sets client id, taken from Spotify Dev Dashboard
    #[arg(long)]
    pub client_id: Option<String>, 
    /// Sets client secret, taken from Spofity Dev Dashboard
    #[arg(long)]
    pub client_secret: Option<String>,
    /// Sets the port that spf will listen on
    #[arg(long)]
    pub redirect_port: Option<u32>,
}

