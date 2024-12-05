use std::env;

#[derive(Clone)]
pub struct ConfigModule {
    pub database_url: String,
    pub ollama_server_host: String,
    pub ollama_server_port: String,
}

impl ConfigModule {
    pub fn new(
        database_url: String,
        ollama_server_host: String,
        ollama_server_port: String,
    ) -> Self {
        ConfigModule {
            database_url,
            ollama_server_host,
            ollama_server_port,
        }
    }
    pub fn get_database_url(&self) -> String {
        self.database_url.clone()
    }

    pub fn get_ollama_server_url(&self) -> String {
        format!(
            "http://{}:{}/api",
            self.ollama_server_host, self.ollama_server_port
        )
    }
}

pub fn get_config_values() -> ConfigModule {
    set_environment_file();
    // DATABASE_URL=postgres://test:123456@localhost:5432/test
    let vendor = env::var("DB_VENDOR").unwrap_or("postgres".to_string());
    let user = env::var("DB_USER").expect("DB_USER não definido");
    let password = env::var("DB_PASSWORD").expect("DB_PASSWORD não definido");
    let host = env::var("DB_HOST").expect("DB_HOST não definido");
    let port = env::var("DB_PORT").unwrap_or("5432".to_string());
    let database = env::var("DB_NAME").unwrap_or("postgres".to_string());

    let database_url = format!(
        "{}://{}:{}@{}:{}/{}",
        vendor, user, password, host, port, database
    );

    // OLLAMA_SERVER_URL="http://localhost:11434/api"
    let ollama_host = env::var("OLLAMA_HOST").expect("OLLAMA_HOST não definido");
    let ollama_port = env::var("OLLAMA_PORT").unwrap_or("11434".to_string());

    ConfigModule::new(database_url, ollama_host, ollama_port)
}

pub fn set_environment_file() {
    let environment_file;

    if let Ok(e) = dotenvy::var("ENV") {
        environment_file = format!(".env.{}", e);
    } else {
        environment_file = String::from(".env");
    }

    dotenvy::from_filename(environment_file).ok();
    // println!("{}", environment_file);
    // info!("{}", environment_file);
}
