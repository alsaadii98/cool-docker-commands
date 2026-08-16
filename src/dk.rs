//! Docker daemon access and the small view structs the commands render.

use anyhow::{Context, Result};
use bollard::Docker;
use bollard::models::{ContainerSummary, ImageSummary, Network, Volume};
use bollard::query_parameters::{
    ListContainersOptionsBuilder, ListImagesOptionsBuilder, ListNetworksOptions, ListVolumesOptions,
};

pub const COMPOSE_PROJECT: &str = "com.docker.compose.project";
pub const COMPOSE_SERVICE: &str = "com.docker.compose.service";

pub fn connect() -> Result<Docker> {
    // Demo mode never talks to a daemon, but the commands still expect a
    // client value; an unconnected one is fine since no request is made.
    if crate::demo::enabled() {
        return Ok(Docker::connect_with_defaults()
            .unwrap_or_else(|_| Docker::connect_with_local_defaults().expect("demo client")));
    }
    Docker::connect_with_defaults()
        .context("cannot reach the docker daemon (is it running? check DOCKER_HOST)")
}

pub async fn containers(docker: &Docker, all: bool) -> Result<Vec<ContainerSummary>> {
    if crate::demo::enabled() {
        let mut list = crate::demo::containers();
        if !all {
            list.retain(|c| state_of(c) == "running");
        }
        return Ok(list);
    }
    let opts = ListContainersOptionsBuilder::default().all(all).build();
    Ok(docker.list_containers(Some(opts)).await?)
}

pub async fn images(docker: &Docker, all: bool) -> Result<Vec<ImageSummary>> {
    if crate::demo::enabled() {
        return Ok(crate::demo::images());
    }
    let opts = ListImagesOptionsBuilder::default().all(all).build();
    Ok(docker.list_images(Some(opts)).await?)
}

pub async fn networks(docker: &Docker) -> Result<Vec<Network>> {
    if crate::demo::enabled() {
        return Ok(crate::demo::networks());
    }
    Ok(docker.list_networks(None::<ListNetworksOptions>).await?)
}

pub async fn volumes(docker: &Docker) -> Result<Vec<Volume>> {
    if crate::demo::enabled() {
        return Ok(crate::demo::volumes());
    }
    Ok(docker.list_volumes(None::<ListVolumesOptions>).await?.volumes.unwrap_or_default())
}

pub async fn df(docker: &Docker) -> Result<bollard::models::SystemDataUsageResponse> {
    if crate::demo::enabled() {
        return Ok(crate::demo::df());
    }
    Ok(docker.df(None::<bollard::query_parameters::DataUsageOptions>).await?)
}

/// Resolve a user-typed name/id prefix to a concrete container name.
pub async fn resolve(docker: &Docker, needle: &str) -> Result<String> {
    let list = containers(docker, true).await?;
    // Exact name wins over a substring match, so `db` never picks `db-backup`.
    let exact = list.iter().find(|c| name_of(c) == needle);
    let fuzzy = list.iter().find(|c| {
        name_of(c).contains(needle)
            || c.id.as_deref().is_some_and(|id| id.starts_with(needle))
            || label(c, COMPOSE_SERVICE) == Some(needle)
    });
    exact.or(fuzzy).map(name_of).with_context(|| format!("no container matches `{needle}`"))
}

/// Container name without docker's leading `/`.
pub fn name_of(c: &ContainerSummary) -> String {
    c.names
        .as_ref()
        .and_then(|n| n.first())
        .map(|n| n.trim_start_matches('/').to_string())
        .unwrap_or_else(|| c.id.clone().unwrap_or_default().chars().take(12).collect())
}

pub fn label<'a>(c: &'a ContainerSummary, key: &str) -> Option<&'a str> {
    c.labels.as_ref()?.get(key).map(String::as_str)
}

pub fn state_of(c: &ContainerSummary) -> String {
    c.state.map(|s| s.to_string()).unwrap_or_default()
}

pub fn health_of(c: &ContainerSummary) -> Option<String> {
    let h = c.health.as_ref()?.status?;
    let s = h.to_string();
    if s.is_empty() || s == "none" { None } else { Some(s) }
}

/// Seconds since the container/image was created.
pub fn age_secs(created_unix: i64) -> i64 {
    chrono::Utc::now().timestamp() - created_unix
}
