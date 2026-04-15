use rust_todo::config::Appconfig;
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn production_requires_db_url() {
        let cfg = AppConfig {
            app_name: "demo".to_string(),
            environment: "production".to_string(),
            port: 3000,
            db_url: None,
        };
        let result = validate(&cfg);
        assert!(result.is_err());
    }
}
