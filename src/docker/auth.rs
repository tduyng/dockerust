use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::Deserialize;

use crate::DOCKER_AUTH;

#[derive(Debug, Deserialize)]
pub struct AuthToken {
    token: String,
}

pub fn get_auth_token(image: &str) -> Result<String> {
    let client = Client::new();
    let token: AuthToken = client
        .get(DOCKER_AUTH)
        .query(&[
            ("service", "registry.docker.io"),
            ("scope", &format!("repository:library/{}:pull", image)),
        ])
        .send()
        .context("Failed to send auth request")?
        .json()
        .context("Failed to deserialize auth token")?;

    Ok(token.token)
}
