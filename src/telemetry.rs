use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

// combine multiple layers into tracing subscriber
pub fn get_tracing_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
// this is a higher-ranked trait bound (HRTB)
// It basically means that Sink implements the `MakeWriter`
// trait for all choices of the lifetime parameter `'a`
// Check out https://doc.rust-lang.org/nomicon/hrtb.html
// for more details.”
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    // set trace span level using RUST_LOG env var and  default to `info` if not set
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(
        name, // output formatted spans to STDOUT
        sink,
    );

    //setup subscriber
    // `.with` provided layer::SubscriberExt, an extension trait
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

// register Subscriber as a global default and init LogTracer
// should only be called once
pub fn init_tracing(subscriber: impl Subscriber + Send + Sync) {
    // Redirect all `log` calls to tracing subscriber below
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set global tracing subscriber");
}
