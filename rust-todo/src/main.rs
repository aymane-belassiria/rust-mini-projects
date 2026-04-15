use rust_todo::config::AppConfig;

fn main() -> anyhow::Result<()> {
    let cfg = AppConfig::load().expect("Failed to load config");
    println!("Application {}", cfg.app_name);
    println!("Environment {}", cfg.env);
    println!("Port {}", cfg.port);
    println!(
        "Database: {}",
        if cfg.db_url.is_some() {
            "configured"
        } else {
            "not configured"
        }
    );
    Ok(())
}
