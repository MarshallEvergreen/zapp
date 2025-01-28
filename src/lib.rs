use python_file_system::{interface::IPythonEntityVisitor, recurse::walk};

pub mod api_generator;
pub use api_generator::api_generator_visitor::ApiGeneratorVisitor;

pub mod python_file_system;

#[cfg(test)]
mod tests; // Include the test module conditionally for tests

pub fn zapp() {
    tracing_subscriber::fmt::init();

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
