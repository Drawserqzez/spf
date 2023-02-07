use clap::Args;

use crate::spotify::auth::authenticator::get_auth_token;
use crate::spotify::error::SpotifyError;

pub fn handle_play_command(input: &Song) -> Result<String, SpotifyError> {
    let _auth = get_auth_token()?;

    let msg = if let Some(track) = &input.title {
        if let Some(artist) = &input.artist {
            format!("Playing '{}' by '{}'", track, artist)
        } else {
            format!("Playing '{}'", track)
        }
    } else if let Some(id) = &input.id {
        format!("Playing song with id '{}'", id)
    } else {
        format!("Toggling playback!")
    };

    Ok(msg)
}

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
