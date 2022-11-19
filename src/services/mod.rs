use mongodb::Client;
use mongodb::options::ClientOptions;

pub mod todo;
pub mod data;
pub mod aggregate;
pub mod event_store;

pub async fn create_mongo_client() -> Client {
    let client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .expect("Could not parse client options");
    
    Client::with_options(client_options).unwrap()
}