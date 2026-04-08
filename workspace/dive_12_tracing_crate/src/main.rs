use tracing::{info, warn};
use tracing_subscriber;

fn main() {
    tracing_subscriber::fmt::init();

    info!("Hello tracing!");
    warn!("Something might be wrong");
}
