use zapp::libs::python_file_system::recurse::walk;

fn main() {
    tracing_subscriber::fmt::init();
    match walk(None) {
        Ok(_) => {
            tracing::info!("Operation completed successfully.");
        }
        Err(e) => {
            tracing::error!("Error: {:?}", e);
            std::process::exit(1);
        }
    }
}
