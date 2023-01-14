use std::collections::HashMap;

use reqwest::blocking::{Client, RequestBuilder};
use serde::Deserialize;

pub fn authenticate() -> Result<AuthResponse, AuthApiError> {
    let client = Client::new();
    let mut params = HashMap::new();

    params.insert("grant_type", "client_credentials");

    let req: RequestBuilder = client
        .post("https://accounts.spotify.com/api/token")
        .basic_auth("", Some("")) //TODO: Get client data in here
        .form(&params);

    let res = req.send();

    let token_data = match res {
        Ok(response) => {  
            println!("{:?}", response);
            response.json::<R>() //WTF
                .map_err(|_| AuthApiError::Unauthorized)
        },
        Err(e) => { eprintln!("{:?}", e); Err(AuthApiError::Unauthorized) },
    }?;

    println!("Auth response: {:?}", token_data);

    Ok(AuthResponse { access_token: token_data.access_token })
}

// TODO: Fix this _please_
#[derive(Debug, Deserialize)]
struct R {
    access_token: String,
    token_type: String,
    expires_in: i32
}

#[derive(Debug, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    //pub refresh_token: String,
    //pub expiration_time: String,
}

#[derive(Debug)]
pub enum AuthApiError {
    Unauthorized,
}
