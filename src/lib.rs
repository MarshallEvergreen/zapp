use api_generator::visitor::ApiGeneratorVisitor;
use python_file_system::{interface::IPythonEntityVisitor, recurse::walk};

pub mod api_generator;
pub mod ruff_formatter;

pub mod python_file_system;
use ruff_formatter::visitor::RuffFormatVisitor;
use tracing::{error, info, trace, Level};
use tracing_subscriber::util::SubscriberInitExt;
use vfs::VfsPath;
use which::which;
#[cfg(test)]
pub mod test_helpers;
mod tests; // Include the test module conditionally for tests

pub struct Config {
    pub ruff_format: bool,
    pub filesystem: Option<VfsPath>,
    pub log_level: Option<Level>,
}

const RUFF: &str = "ruff"; // Change this to the program you want to check

pub fn zapp(config: Config) {
    if let Some(level) = config.log_level {
        tracing_subscriber::fmt()
            // filter spans/events with level TRACE or higher.
            .with_max_level(level)
            // build but do not install the subscriber.
            .finish()
            .init();
    }

    info!("Zapping Python files 🐍⚡️");

    let mut visitors: Vec<Box<dyn IPythonEntityVisitor>> = Vec::new();

    visitors.push(Box::new(ApiGeneratorVisitor::new()));

    if config.ruff_format {
        trace!("Checking for the presence of '{}'", RUFF);
        match which(RUFF) {
            Ok(path) => {
                trace!("{} is available at: {}", RUFF, path.display());
                visitors.push(Box::new(RuffFormatVisitor {}));
            }
            Err(_) => {
                error!("'{}' is not found in $PATH", RUFF);
                std::process::exit(1);
            }
        }
    }

    match walk(visitors, config.filesystem.as_ref()) {
        Ok(_) => {
            info!("Files zapped 🐍⚡️");
        }
        Err(e) => {
            error!("Error: {:?}", e);
            std::process::exit(1);
        }
    }
}
