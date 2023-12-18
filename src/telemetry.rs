use tracing::{Subscriber, subscriber::set_global_default};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt, fmt::MakeWriter};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};

/// Compose multiple layers into a `tracing`'s subscriber.
///
/// # Implementation notes
/// We are using `impl Subscriber` as a return type to avoid having to
/// spell out the actual type of the returned subscriber, which is indeed quite complex.
/// We need to explicitly call out that the returned subscriber is `Send` and `Sync` to make it
/// positive to pass it to `init_subscriber` function later on.
pub fn get_subscriber<Sink>(name: String, env_filter: String, sink: Sink) -> impl Subscriber + Sync + Send
    where Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name, sink);
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

///Register a subscriber as a global default to process span data.
///
/// It should only be called once!
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}
