use std::net::TcpStream;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct Client {
    server: Option<TcpStream>,
    tx: mpsc::Sender<ClientMessage>,
    pub channel_handler: Option<thread::JoinHandle<()>>,
}

impl Client {
    pub fn new(
        tx: mpsc::Sender<ClientMessage>,
        rx: mpsc::Receiver<ClientMessage>,
    ) -> Arc<Mutex<Self>> {
        let rx = Arc::new(Mutex::new(rx));

        let tx_2 = tx.clone();

        let client = Arc::new(Mutex::new(Client {
            server: None,
            tx,
            channel_handler: None,
        }));

        let me = Arc::clone(&client);

        let channel_handler =
            thread::spawn(move || Self::manage_channel(me, tx_2, Arc::clone(&rx)));

        client.lock().unwrap().channel_handler = Some(channel_handler);

        client
    }

    pub fn connect(&mut self, addr: String, port: String) {
        let stream = TcpStream::connect(format!("{addr}:{port}", addr = addr, port = port));

        match stream {
            Ok(server) => {
                self.server = Some(server);
            }
            Err(err) => self.tx.send(ClientMessage::Err(err.to_string())).unwrap(),
        }
    }

    fn manage_channel(
        client: Arc<Mutex<Client>>,
        tx: mpsc::Sender<ClientMessage>,
        rx: Arc<Mutex<mpsc::Receiver<ClientMessage>>>,
    ) {
        loop {
            let message = rx.lock().unwrap().recv().unwrap();

            match message {
                ClientMessage::Terminate => {
                    tx.send(ClientMessage::Terminate).unwrap();
                    break;
                }
                ClientMessage::ConnectToServer(addr, port) => {
                    client.lock().unwrap().connect(addr, port);
                }
                _ => {}
            }
        }
    }
}

pub enum ClientMessage {
    Terminate,
    ConnectToServer(String, String),
    Err(String),
}
