use tracing::{Level, event, span};
use tracing_subscriber;

fn main() {
    let subscriber = tracing_subscriber::fmt::Subscriber::new();
    tracing::subscriber::set_global_default(subscriber)
        .expect("failed to set global default subscriber");

    event!(Level::INFO, "hello, tracing!");

    let span = span!(Level::INFO, "main_span");
    let _enter = span.enter();

    event!(Level::INFO, "inside span");

    sub();

    event!(Level::INFO, "back in main span");
}

fn sub() {
    event!(Level::WARN, "inside sub function");

    let span = span!(Level::INFO, "sub_span");
    let _enter = span.enter();

    event!(Level::ERROR, "inside sub span");
}
