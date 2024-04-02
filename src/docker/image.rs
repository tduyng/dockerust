use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::path::Path;
use tar::Archive;

use crate::{get_auth_token, DOCKER_HUB};

#[derive(Deserialize, Debug)]
struct ImageManifest {
    layers: Vec<ImageConfig>,
}

#[derive(Deserialize, Debug)]
struct ImageConfig {
    digest: String,
}

fn extract_image_info(image_and_version: &str) -> (String, String) {
    match image_and_version.split_once(':') {
        Some((image, version)) => (image.to_string(), version.to_string()),
        None => (image_and_version.to_string(), "latest".to_string()),
    }
}

pub fn download_image(image_and_version: &str, path: impl AsRef<Path>) -> Result<()> {
    let client = Client::new();
    let (image_name, image_version) = extract_image_info(image_and_version);
    let auth_token = get_auth_token(&image_name)?;

    let manifest: ImageManifest = client
        .get(format!(
            "{}/library/{}/manifests/{}",
            DOCKER_HUB, image_name, image_version
        ))
        .header(
            reqwest::header::ACCEPT,
            "application/vnd.docker.distribution.manifest.v2+json",
        )
        .bearer_auth(auth_token.clone())
        .send()
        .context("Failed to get image manifest")?
        .json()
        .context("Failed to parse image manifest")?;

    for layer in manifest.layers {
        let layer_bytes = client
            .get(&format!(
                "{}/library/{}/blobs/{}",
                DOCKER_HUB, image_name, layer.digest
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
