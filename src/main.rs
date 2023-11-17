use std::f32::consts::PI;
use std::fs::File;
use std::io::{self, Read, Write};
use std::net::{TcpListener, SocketAddr};
use std::net::TcpStream;
use std::thread;
use std::sync::{Arc, Mutex};

struct Client
{
    username: String,
    ip: SocketAddr
}

impl Client
{
    fn new(username: String, ip: SocketAddr) -> Self 
    {
        Client 
        {
            username,
            ip
        }
    }

    fn username(&self) -> &str { &self.username }
    fn ip(&self) -> &SocketAddr { &self.ip }
}

fn handle_client(mut stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>)
{
    println!("Client connected: {:?}", stream.peer_addr().unwrap());

    {
        let mut clients = clients.lock().unwrap();
        clients.push(stream.try_clone().expect("Failed to clone client stream"));
    }

    let mut buffer = [0u8; 4096]; // buffer for incoming messages

    loop
    {
        match stream.read(&mut buffer)
        {
            Ok(n) if n == 0 => {
                println!("Client disconnected");
                break;
            }

            Ok(n) => {
                // handle the received message
                let message = String::from_utf8_lossy(&buffer[0..n]);
                let Some(c1) = message.chars().next();
                match c1 // test the first character to know what the message is for
                {
                    10 => // register message structure: 10(username)127(password)
                    {
                        let Some(pos) = message.find(127 as char);
                        let username_slice = &message[1..pos];
                        let username = String::from(username_slice);
                        println!("Username: {}", username);
                        let password_slice = &message[pos..];
                        let password = String::from(password_slice);
                        println!("Password: {}", password);

                        let mut file = File::create("usr.txt").expect("Failed to create file");
                        let content = format!("{}{}{}", username, 127 as char, password);
                    },

                    11 => // login message structure: 11(username)127(password)
                    {

                    },

                    12 => // text message structure: 12(message)
                    {
                        print!("Client: {}", message);

                        // process/respond
                        let response = format!("Client {}: {}", stream.peer_addr().unwrap(), message);
                        let clients = clients.lock().unwrap();
                        for mut client in &*clients
                        {
                            if client.peer_addr().unwrap() != stream.peer_addr().unwrap()
                            {
                                if let Err(err) = client.write(response.as_bytes())
                                {
                                    eprintln!("Error sending response to client: {}", err);
                                }
                            }
                        }
                    }
                }

                
            }

            Err(err) => {
                eprintln!("Error reading from client: {}", err);
                break;
            }
        }
    }

    {
        let mut clients = clients.lock().unwrap();
        if let Some(pos) = clients.iter().position(|c| 
            c.peer_addr().unwrap() == stream.peer_addr().unwrap())
            {
                clients.remove(pos);
            }
    }
}

fn main() -> io::Result<()> 
{
    let listener = TcpListener::bind("127.0.0.1:8080")?; // bind to localhost
    let clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));
    for stream in listener.incoming()
    {
        match stream
        {
            Ok(stream) => {
                let clients = Arc::clone(&clients);
                thread::spawn(|| 
                { 
                    handle_client(stream, clients); 
                });
            }

            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
    Ok(())
}