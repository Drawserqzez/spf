use clap::Args;

#[derive(Args, Debug)]
pub struct Song {
    /// An optional song title
    #[arg(short, long)]
    pub title: Option<String>,
    /// The song id, if you prefer
    #[arg(short, long)]
    pub id: Option<String>,
    /// Optional artist, if you want to have better search results
    #[arg(long)]
    pub artist: Option<String>,
}
