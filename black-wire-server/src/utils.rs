use tokio_postgres::connect;
use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::MakeTlsConnector;
use crate::definitions::*;
use rocket::serde::json::Json;
pub struct Database {
    pub url: String
}

impl Database {
    pub async fn add_device<'a>(&self, device: &'a Json<Device>) -> Result<&'a Json<Device>, &'a str> {
        let builder = SslConnector::builder(SslMethod::tls()).unwrap();
        let connector = MakeTlsConnector::new(builder.build());
        
        let client = connect(&self.url, connector).await;

        let client_result = match client {
            Ok(client) => {
                let (client, connection) = client;
                tokio::spawn(async move {
                    if let Err(e) = connection.await {
                        eprintln!("Connection error: {}", e);
                    }
                });
                
                if let Err(some_err) = client
                .execute("INSERT INTO devices (id, os, os_version) VALUES ($1, $2, $3)",
                &[&device.id, &device.os, &device.os_version])
                .await{
                    println!("{some_err}") 
                };
                Ok(device)
            }
            Err(_) => {
                Err("Error")
            }
        };
        client_result
    }
}