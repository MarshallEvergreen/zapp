use python_file_system::{interface::IPythonEntityVisitor, recurse::walk};

pub mod api_generator;
pub use api_generator::api_generator_visitor::ApiGeneratorVisitor;

pub mod python_file_system;

#[cfg(test)]
pub mod test_helpers;
mod tests; // Include the test module conditionally for tests

pub fn zapp() {
    tracing_subscriber::fmt::init();

    // TODO Add rust formatting visitor based on passed command args
    let visitors: Vec<Box<dyn IPythonEntityVisitor>> = vec![Box::new(ApiGeneratorVisitor::new())];

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
