use tracing::Level;
use zapp::{zapp, Config};

fn main() {
    let config = Config {
        rust_format: true,
        filesystem: None,
        log_level: Some(Level::TRACE),
    };

    zapp(config);
}
