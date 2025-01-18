use super_duper_octo_lamp::entry::abinit;

fn main() {
    tracing_subscriber::fmt::init();

    match abinit(None) {
        Ok(_) => {
            tracing::info!("Operation completed successfully.");
        }
        Err(e) => {
            tracing::error!("Error: {:?}", e);
            std::process::exit(1);
        }
    }
}
