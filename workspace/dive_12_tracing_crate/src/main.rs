use tracing::{Level, event, instrument, span};
use tracing_subscriber;

fn main() {
    let subscriber = tracing_subscriber::fmt::Subscriber::new();
    tracing::subscriber::set_global_default(subscriber)
        .expect("failed to set global default subscriber");

    event!(Level::INFO, "hello, tracing!");
    let span = span!(Level::INFO, "main");
    let _enter = span.enter();
    event!(Level::INFO, "inside span");
    sub();
    event!(Level::INFO, "back in main span");
}

#[instrument]
fn sub() {
    // let span = span!(Level::INFO, "sub");
    // let _enter = span.enter();
    event!(Level::WARN, "inside sub function");
    event!(Level::ERROR, "inside sub span");
}
