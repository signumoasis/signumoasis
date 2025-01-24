#[cfg(feature = "server")]
use tokio::task::JoinHandle;
#[cfg(feature = "server")]
use tracing::instrument;

use tracing::Subscriber;
use tracing_subscriber::{fmt::MakeWriter, prelude::*, EnvFilter};

/// Sets up a tracing subscriber.
// INFO: Only used on Non-WASM32 targets with 'bunyan' feature disabled
#[cfg(all(not(feature = "bunyan"), not(target_arch = "wasm32")))]
pub fn get_subscriber<Sink>(
    _name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    use tracing_subscriber::fmt::{self, format::FmtSpan};
    // --This code uses tracing-subscriber--

    let filter_layer =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    let fmt_layer = if dioxus_cli_config::is_cli_enabled() {
        fmt::layer()
            .compact()
            .with_target(true)
            .with_line_number(true)
            .with_span_events(FmtSpan::NONE)
            .with_writer(sink)
            .boxed()
    } else {
        // INFO: This is a hack that sets up a cleaner subscriber when used within 'dx' cli tool.
        fmt::layer()
            .compact()
            .with_target(false)
            .without_time()
            .with_line_number(true)
            .with_span_events(FmtSpan::NONE)
            .with_writer(sink)
            .boxed()
    };

    let subscriber = tracing_subscriber::registry();
    //#[cfg(feature = "tokio-console")]
    //let subscriber = {
    //    // Only enable this if the feature is enabled.
    //    let tokio_console_fmt_layer =
    //        console_subscriber::spawn().with_filter(tracing_subscriber::filter::LevelFilter::TRACE);
    //    subscriber.with(tokio_console_fmt_layer)
    //};

    subscriber.with(fmt_layer.with_filter(filter_layer))
}

/// Sets up a tracing subscriber.
// INFO: Only used on Non-WASM32 targets with 'bunyan' feature enabled
#[cfg(all(feature = "bunyan", not(target_arch = "wasm32")))]
pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
    use tracing_subscriber::Registry;

    let filter_layer =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    let bunyan_format = BunyanFormattingLayer::new(name, sink);

    let bunyan_layer = JsonStorageLayer
        .and_then(bunyan_format)
        .with_filter(filter_layer);

    let subscriber = Registry::default();
    //#[cfg(feature = "tokio-console")]
    //let subscriber = {
    //    // Only enable this if the feature is enabled.
    //    let tokio_console_fmt_layer =
    //        console_subscriber::spawn().with_filter(tracing_subscriber::filter::LevelFilter::TRACE);
    //    subscriber.with(tokio_console_fmt_layer)
    //};

    subscriber.with(bunyan_layer)
}

/// Sets up a tracing subscriber.
// INFO: Only used on WASM32 targets
#[cfg(target_arch = "wasm32")]
pub fn get_subscriber<Sink>(
    _name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    use std::str::FromStr;

    use tracing::Level;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::Registry;

    let level = Level::from_str(&env_filter).unwrap();

    let layer_config = tracing_wasm::WASMLayerConfigBuilder::new()
        .set_max_level(level)
        .build();
    let layer = tracing_wasm::WASMLayer::new(layer_config);
    let reg = Registry::default().with(layer);

    console_error_panic_hook::set_once();
    reg
}

/// Sets the global default subscriber. Should only be called once.
pub fn init_subscriber<Sink>(name: String, env_filter: String, sink: Sink)
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    if tracing::dispatcher::has_been_set() {
        return;
    }

    // DEBUG level by default if not compiled with --release, or INFO if so.
    // Override with RUST_LOG
    let env_filter = if env_filter.to_lowercase() != "trace" && cfg!(debug_assertions) {
        "DEBUG".to_owned()
    } else {
        env_filter
    };

    let subscriber = get_subscriber(name, env_filter, sink);
    let _ = tracing::subscriber::set_global_default(subscriber)
        .map_err(|_err| eprintln!("Unable to set global default subscriber"));
}

#[cfg(feature = "server")]
#[instrument(skip_all)]
pub fn spawn_blocking_with_tracing<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let current_span = tracing::Span::current();
    tokio::task::spawn_blocking(move || current_span.in_scope(f))
}
