use api_generator::api_generator_visitor::ApiGeneratorVisitor;
use python_file_system::{interface::IPythonEntityVisitor, recurse::walk};

pub mod api_generator;
pub mod ruff_formatter;

pub mod python_file_system;
use ruff_formatter::visitor::RuffFormatVisitor;
use which::which;
#[cfg(test)]
pub mod test_helpers;
mod tests; // Include the test module conditionally for tests

pub struct Config {
    pub rust_format: bool,
}

const RUFF: &str = "ruff"; // Change this to the program you want to check

pub fn zapp(config: Config) {
    tracing_subscriber::fmt::init();

    let mut visitors: Vec<Box<dyn IPythonEntityVisitor>> = Vec::new();

    visitors.push(Box::new(ApiGeneratorVisitor::new()));

    if config.rust_format {
        match which(RUFF) {
            Ok(path) => {
                tracing::info!("{} is available at: {}", RUFF, path.display());
                visitors.push(Box::new(RuffFormatVisitor {}));
            }
            Err(_) => {
                tracing::error!("'{}' is not found in $PATH", RUFF);
                std::process::exit(1);
            }
        }
    }

    match walk(visitors, None) {
        Ok(_) => {
            tracing::info!("Operation completed successfully.");
        }
        Err(e) => {
            tracing::error!("Error: {:?}", e);
            std::process::exit(1);
        }
    }
}
