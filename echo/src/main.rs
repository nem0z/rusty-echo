use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

mod server;
use server::server::Server;

mod client;
use client::client::Client;

fn main() {
    let server_thread = thread::spawn(|| {
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

    let reader_thread = thread::spawn({
        let client = Arc::clone(&client);
        move || {
            loop {
                let msg = match client.lock().unwrap().read_message() {
                    Ok(msg) => msg,
                    Err(err) => {
                        eprintln!("Error starting client: {}", err);
                        continue;
                    }
                };
        
                println!("Received message : {}", msg);
            }
        }
    });

    for i in 1..=5 {
        if let Err(err) = client.lock().unwrap().send_message(&format!("Hello {} time(s)!", i)) {
            eprintln!("Error sending message {}", err);
        };
        thread::sleep(Duration::from_secs(1));
    }


    reader_thread.join().expect("Reader thread panicked!");
    server_thread.join().expect("Server thread panicked!");
}
