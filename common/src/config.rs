pub struct Config {
    host: String,
}

impl Config {
    pub fn new() -> Result<Self, &'static str> {
        Ok(Config {
            host: std::env::var("MPMP_HOST")
                .map_err(|_| "Failed to obtain MPMP_HOST environment var")?,
        })
    }

    pub fn get_host(&self) -> &str {
        &self.host
    }
}
