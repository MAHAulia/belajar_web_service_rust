use mongodb::{Client, options::ClientOptions};
use std::{sync::Arc,env};
pub struct AppConfig {
    pub server_address: String,
    pub arc_client:Arc<Client>,
    // ... other configuration parameters ...
}

pub async fn load_config() -> AppConfig {
    // Set up the MongoDB connection options
    // Read the value of the DB_URL environment variable
    let db_url = env::var("DB_URL").expect("DB_URL environment variable not set");
    let client_options = ClientOptions::parse(&db_url).await.unwrap();
    // Connect to the MongoDB server
    let client = Client::with_options(client_options).unwrap();
    let arc_client = Arc::new(client);
    // Logic to load configuration from a file, environment variables, etc.
    AppConfig {
        server_address: env::var("HOST").unwrap_or_default().to_string(),
        arc_client:arc_client,
        // ... initialize other configuration parameters ...
    }
}
