use std::io::{stdin, BufReader, BufRead, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let server_address = "127.0.0.1:8888";
    let mut socket = TcpStream::connect(server_address).expect("Failed to connect");
    socket.set_nonblocking(true).expect("Failed to set non-blocking");
    println!("Connected to {}", server_address);

    start_thread(socket.try_clone().expect("Failed to clone socket"));
    let username = input("Enter your name: ");
    println!("Welcome, {}!", username);

    loop {
        let msg = input("");
        let msg = format!("{}> {}\n", username, msg);
        let buf = msg.as_bytes();
        socket.write_all(buf).expect("Failed to write");
    }
}

fn start_thread(socket: TcpStream) {
    let mut reader = BufReader::new(socket);
    thread::spawn(move || loop {
        let mut buf = String::new();
        if let Ok(n) = reader.read_line(&mut buf) {
            if n == 0 {
                break;
            }
            print!("{}", buf);
        }
        thread::sleep(Duration::from_millis(100));
    });
}

fn input(msg: &str) -> String {
    if msg != "" {
        println!("{}", msg);
    }
    let _ = std::io::stdout().flush();
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("Failed to read");
    String::from(buf.trim())
}
