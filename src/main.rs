use tracing::Level;
use zapp::{zapp, Config};

fn main() {
    let config = Config {
        ruff_format: true,
        filesystem: None,
        log_level: Some(Level::TRACE),
    };

    zapp(config);
}
