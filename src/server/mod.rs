use crate::msg::handle_msg;
use std::collections::HashMap;
use std::error::Error;
use std::io::Write;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone)]
pub struct Server {
    pub streams: Arc<Mutex<HashMap<String, TcpStream>>>,
    pub ip: SocketAddr,
}

impl Server {
    pub fn new(port: u16) -> Result<Self, Box<dyn Error>> {
        let streams: Arc<Mutex<HashMap<String, TcpStream>>> = Arc::new(Mutex::new(HashMap::new()));
        let ip = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
        Ok(Server { streams, ip })
    }

    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let receiver_listener =
            TcpListener::bind(self.ip).expect("Failed and bind with the sender");
        // Getting a handle of the underlying thread.
        let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
        // listen to incoming connections messages and bind them to a sever socket address.
        for stream in receiver_listener.incoming() {
            // TODO: Improve Error Handling
            let stream = stream.expect("failed");
            let stream_clone = stream.try_clone().unwrap();
            self.streams
                .lock()
                .unwrap()
                .insert(stream.peer_addr().unwrap().to_string(), stream);
            // let the receiver connect with the sender
            let handle = thread::spawn(move || {
                //receiver failed to read from the stream
                handle_msg(stream_clone).unwrap_or_else(|error| eprintln!("{:?}", error))
            });

            // Push messages in the order they are sent
            thread_vec.push(handle);
        }

        for handle in thread_vec {
            // return each single value Output contained in the heap
            handle.join().unwrap();
        }
        Ok(())
        // success value
    }

    pub fn add_stream(&mut self, stream: TcpStream) {
        let stream_clone = stream.try_clone().unwrap();
        self.streams
            .lock()
            .unwrap()
            .insert(stream.peer_addr().unwrap().to_string(), stream);
        thread::spawn(move || {
            handle_msg(stream_clone).unwrap_or_else(|error| eprintln!("{:?}", error))
        });
    }

    pub fn connect_to_peers(
        &mut self,
        clients: Vec<SocketAddr>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for client in clients {
            let stream = TcpStream::connect(client)?;
            self.add_stream(stream);
        }
        Ok(())
    }

    pub fn broadcast(&mut self, msg: &[u8]) {
        for mut stream in self.streams.lock().unwrap().values() {
            stream.write_all(msg).unwrap();
        }
    }
}
