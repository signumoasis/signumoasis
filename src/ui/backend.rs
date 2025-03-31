#[cfg(feature = "server")]
use std::sync::atomic::{AtomicU32, Ordering};

use dioxus::prelude::*;

#[cfg(feature = "server")]
static GLOBAL_COUNTER: AtomicU32 = AtomicU32::new(0);

#[server(endpoint = "get_counter")]
#[tracing::instrument]
pub async fn serverside_counter_get() -> Result<u32, ServerFnError> {
    let counter = GLOBAL_COUNTER.load(Ordering::Relaxed);
    Ok(counter)
}

#[server(endpoint = "increment_counter")]
#[tracing::instrument]
pub async fn serverside_counter_increment() -> Result<(), ServerFnError> {
    let _ = GLOBAL_COUNTER.fetch_add(1, Ordering::Relaxed);
    let counter = GLOBAL_COUNTER.load(Ordering::Relaxed);
    tracing::trace!("Global Counter: {}", counter);
    Ok(())
}
