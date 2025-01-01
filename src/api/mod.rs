pub mod recommendations;

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    recommendations::config(cfg);
} 