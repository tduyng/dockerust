use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::Deserialize;

use crate::{DOCKER_AUTH, DOCKER_REGISTRY};

#[derive(Debug, Deserialize)]
struct AuthToken {
    token: String,
}

pub fn get_auth_token(image_name: &str) -> Result<String> {
    let client = Client::new();
    let token: AuthToken = client
        .get(DOCKER_AUTH)
        .query(&[
            ("service", DOCKER_REGISTRY),
            ("scope", &format!("repository:library/{}:pull", image_name)),
        ])
        .send()
        .context("Failed to send auth request")?
        .json()
        .context("Failed to deserialize auth token")?;

    Ok(token.token)
}
