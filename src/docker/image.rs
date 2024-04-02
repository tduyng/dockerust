use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::path::Path;
use tar::Archive;

use crate::{get_auth_token, DOCKER_HUB, DOCKER_REGISTRY};

#[derive(Deserialize, Debug)]
pub struct ImageManifest {
    pub layers: Vec<ImageConfig>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ImageConfig {
    pub digest: String,
    #[serde(rename = "mediaType")]
    pub media_type: String,
    pub size: usize,
}

pub fn download_image(image: &str, path: impl AsRef<Path>) -> Result<()> {
    let client = Client::new();
    let auth_token = get_auth_token(image)?;

    let manifest: ImageManifest = client
        .get(format!("{}/library/{}/manifests/latest", DOCKER_HUB, image))
        .header(
            reqwest::header::ACCEPT,
            "application/vnd.docker.distribution.manifest.v2+json",
        )
        .bearer_auth(auth_token.clone())
        .send()?
        .json()
        .context("Failed to parse image manifest")?;

    for layer in manifest.layers {
        let layer_bytes = client
            .get(&format!(
                "{}/library/{}/blobs/{}",
                DOCKER_REGISTRY, image, layer.digest
            ))
            .bearer_auth(auth_token.clone())
            .send()
            .context("Failed to get image layer")?
            .bytes()
            .context("Failed to get image layer bytes")?;

        let reader = Box::new(std::io::Cursor::new(layer_bytes));
        let tar = GzDecoder::new(reader);
        let mut archive = Archive::new(tar);
        archive
            .unpack(&path)
            .context("Failed to extract image layer")?;
    }

    Ok(())
}
