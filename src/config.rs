use tokio::sync::OnceCell;

#[derive(Debug)]
pub struct Config {
    pub jwt_secret: &'static [u8]
}

// Initialize and access the configuration
pub static CONFIG: OnceCell<Config> = OnceCell::const_new();
