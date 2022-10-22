mod page;
pub use page::{Page, PageEvent, PageMessage};

mod pages;
pub use pages::main_page::{MainPage, MainPageEvent};

mod client;
pub use client::{Client, ClientMessage};

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use cursive::views::{Dialog, TextView};

pub struct Manager {
    client: Arc<Mutex<Client>>,
    tx: mpsc::Sender<ClientMessage>,
    rx: Arc<Mutex<mpsc::Receiver<ClientMessage>>>,
    tx_page: mpsc::Sender<PageMessage>,
    rx_page: Arc<Mutex<mpsc::Receiver<PageMessage>>>,
    siv: cursive::CursiveRunnable,
    channel_handler: Option<thread::JoinHandle<()>>,
    page_handler: Option<thread::JoinHandle<()>>,
    cb_sink: cursive::CbSink,
}

impl Manager {
    pub fn new(siv: cursive::CursiveRunnable) -> Self {
        let (tx_manager, rx_client) = mpsc::channel();
        let (tx_client, rx_manager) = mpsc::channel();
        let (tx_page, rx_page) = mpsc::channel();

        let client = Client::new(tx_client, rx_client);

        Self {
            client,
            tx: tx_manager,
            rx: Arc::new(Mutex::new(rx_manager)),
            tx_page,
            rx_page: Arc::new(Mutex::new(rx_page)),
            cb_sink: siv.cb_sink().clone(),
            siv,
            channel_handler: None,
            page_handler: None,
        }
    }

    pub fn run(&mut self) {
        let rx = Arc::clone(&self.rx);
        let rx_page = Arc::clone(&self.rx_page);

        let tx = self.tx.clone();

        let cb_sink = self.cb_sink.clone();

        let channel_handler = thread::spawn(move || Self::manage_client(cb_sink, rx));

        self.channel_handler = Some(channel_handler);

        let page_handler = thread::spawn(move || Self::manage_page(tx, rx_page));

        self.page_handler = Some(page_handler);

        self.render_main_page();

        self.siv.run();
    }

    fn manage_client(cb_sink: cursive::CbSink, rx: Arc<Mutex<mpsc::Receiver<ClientMessage>>>) {
        loop {
            let message = rx.lock().unwrap().recv().unwrap();

            match message {
                ClientMessage::Terminate => break,
                ClientMessage::Err(err) => {
                    cb_sink
                        .send(Box::new(|s| s.add_layer(Self::render_error(err))))
                        .unwrap();
                }
                _ => {}
            }
        }
    }

    fn manage_page(tx: mpsc::Sender<ClientMessage>, rx: Arc<Mutex<mpsc::Receiver<PageMessage>>>) {
        loop {
            let message = rx.lock().unwrap().recv().unwrap();

            match message.downcast_ref::<PageEvent>() {
                Some(PageEvent::Terminate) => break,
                _ => {}
            }

            match message.downcast_ref::<MainPageEvent>() {
                Some(MainPageEvent::Quit) => {
                    break;
                }
                Some(MainPageEvent::ConnectToServer(addr, port)) => {
                    tx.send(ClientMessage::ConnectToServer(
                        String::from(addr),
                        String::from(port),
                    ))
                    .unwrap();
                }
                _ => {}
            }
        }
    }

    fn render_main_page(&mut self) {
        let main_page = MainPage::new(self.tx_page.clone());
        self.siv.add_layer(main_page.body());
    }

    fn render_error(message: String) -> Dialog {
        Dialog::around(TextView::new(message))
            .title("Error")
            .button("Ok", |s| {
                s.pop_layer();
            })
    }
}

impl Drop for Manager {
    fn drop(&mut self) {
        println!("[!] Terminating Client");

        self.tx.send(ClientMessage::Terminate).unwrap();
        self.tx_page.send(Box::new(PageEvent::Terminate)).unwrap();

        if let Some(handler) = self.client.lock().unwrap().channel_handler.take() {
            handler.join().unwrap();
        }

        println!("[!] Client Terminated");

        if let Some(handler) = self.channel_handler.take() {
            handler.join().unwrap();
        }

        if let Some(handler) = self.page_handler.take() {
            handler.join().unwrap();
        }

        println!("[!] Server Terminated");
    }
}
