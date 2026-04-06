use tracing::Level;

pub fn init_logging() {
    tracing_subscriber::fmt::Subscriber::builder()
        .with_max_level(Level::INFO)
        .init();
}
