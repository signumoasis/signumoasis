#![cfg(feature = "server")]
use std::{future::Future, pin::Pin};

use crate::configuration::Settings;
use surrealdb::{engine::any::Any, Surreal};

pub fn load_plugins() -> Vec<PluginData> {
    Vec::new()
}

pub struct PluginData {
    /// The plugin's chosen ID string.
    pub plugin_id: String,
    /// Routes the plugin would like registered into the main web server.
    //pub route_definitions: Vec<axum::Router>,
    /// A list of tasks that the plugin would like to those to run on its behalf.
    pub spawnable_tasks:
        Vec<fn(Surreal<Any>, Settings) -> Pin<Box<dyn Future<Output = anyhow::Result<()>>>>>,
}
