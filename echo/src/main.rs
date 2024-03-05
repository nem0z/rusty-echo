use std::sync::{Arc, Mutex};
use tokio::time::sleep;
use std::time::Duration;

mod server;
use server::server::Server;

mod client;
use client::client::Client;

#[tokio::main]
async fn main() {
    let server_thread = tokio::spawn(async {
        let mut server = match Server::new("localhost:8080") {
            Ok(server) => server,
            Err(err) => {
                eprintln!("Error starting server: {}", err);
                return;
            }
        };

        server.start();
    });

    let client = match Client::new("localhost:8080") {
        Ok(client) => Arc::new(Mutex::new(client)),
        Err(err) => {
            eprintln!("Error starting client: {}", err);
            return;
        }
    };

    let reader_thread = {
        let client = Arc::clone(&client);
        tokio::spawn(async move {
            loop {
                let msg = match client.lock() {
                    Ok(mut client) => client.read_message(),
                    Err(err) => {
                        eprintln!("Error acquiring client lock: {}", err);
                        continue;
                    }
                };

                match msg {
                    Ok(msg) => println!("Received message: {}", msg),
                    Err(err) => eprintln!("Error reading message: {}", err),
                }

                // Optionally, release the lock here if not needed anymore
            }
        })
    };

    for i in 1..=5 {
        let client_clone = Arc::clone(&client);
        tokio::spawn(async move {
            if let Ok(mut client) = client_clone.lock() {
                if let Err(err) = client.send_message(&format!("Hello {} time(s)!", i)) {
                    eprintln!("Error sending message: {}", err);
                };
            } else {
                eprintln!("Error acquiring client lock for sending message");
            }
        });

        sleep(Duration::from_secs(1)).await;
    }

    reader_thread.await.expect("Reader thread panicked!");
    server_thread.await.expect("Server thread panicked!");
}
