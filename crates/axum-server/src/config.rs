#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn new() -> Config {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found");

        Config { database_url, }
    }
}