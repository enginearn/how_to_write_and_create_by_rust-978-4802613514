use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead, Write};
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let server_address = "127.0.0.1:8888";
    let (tx, rx) = mpsc::channel::<String>();
    let mut clients: Vec<TcpStream> = Vec::new();

    let server = TcpListener::bind(server_address).expect("Failed to bind");
    server.set_nonblocking(true).expect("Failed to set non-blocking");
    println!("Chat server started on {}", server_address);

    loop {
        if let Ok((client, address)) = server.accept() {
            println!("Client {} connected", address);
            clients.push(client.try_clone().expect("Failed to clone client"));
            start_thread(client, tx.clone());
        }
        // waiting for threads to send messages
        if let Ok(msg) = rx.try_recv() {
            println!("Received: {}", msg);
            clients = send_all(clients, &msg);
        }
        thread::sleep(Duration::from_millis(100));
    }
}

// thread for each client
fn start_thread(client: TcpStream, tx: mpsc::Sender<String>) {
    let mut reader = BufReader::new(client);
    thread::spawn(move || loop {
        let mut msg = String::new();
        if let Ok(n) = reader.read_line(&mut msg) {
            if n == 0 {
                break;
            }
            tx.send(msg).expect("Failed to send message");
        }
        thread::sleep(Duration::from_millis(100));
    });
}

// send message to all clients
fn send_all(clients: Vec<TcpStream>, msg: &str) -> Vec<TcpStream> {
    let mut collector = Vec::new();
    for mut socket in clients.into_iter() {
        let bytes = String::from(msg).into_bytes();
        if let Err(e) = socket.write_all(&bytes) {
            println!("Failed to send message: {}", e);
            continue;
        }
        collector.push(socket);
    }
    collector // return
}
