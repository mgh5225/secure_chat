mod page;
pub use page::{Page, PageEvent, PageMessage};

mod pages;
pub use pages::login_page::{LoginPage, LoginPageEvent};
pub use pages::main_page::{MainPage, MainPageEvent};
pub use pages::signup_page::{SignupPage, SignupPageEvent};

mod client;
pub use client::{Client, ClientMessage};

mod session;
pub use session::Session;

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
        let cb_sink = self.cb_sink.clone();
        let tx_page = self.tx_page.clone();
        let rx_client = Arc::clone(&self.rx);

        let channel_handler =
            thread::spawn(move || Self::manage_client(cb_sink, tx_page, rx_client));

        self.channel_handler = Some(channel_handler);

        let cb_sink = self.cb_sink.clone();
        let tx_client = self.tx.clone();
        let tx_page = self.tx_page.clone();
        let rx_page = Arc::clone(&self.rx_page);

        let page_handler =
            thread::spawn(move || Self::manage_page(cb_sink, tx_client, tx_page, rx_page));

        self.page_handler = Some(page_handler);

        self.render_main_page();

        self.siv.run();
    }

    fn manage_client(
        cb_sink: cursive::CbSink,
        tx_page: mpsc::Sender<PageMessage>,
        rx_client: Arc<Mutex<mpsc::Receiver<ClientMessage>>>,
    ) {
        loop {
            let message = rx_client.lock().unwrap().recv().unwrap();

            match message {
                ClientMessage::Terminate => break,
                ClientMessage::Err(err) => {
                    cb_sink
                        .send(Box::new(|s| s.add_layer(Self::render_error(err))))
                        .unwrap();
                }
                ClientMessage::ConnectedToServer => {
                    let login_tx = tx_page.clone();
                    cb_sink
                        .send(Box::new(|s| {
                            let login_page = Self::render_login_page(login_tx);

                            s.pop_layer();
                            s.add_layer(login_page.body());
                        }))
                        .unwrap();
                }
                _ => {}
            }
        }
    }

    fn manage_page(
        cb_sink: cursive::CbSink,
        tx_client: mpsc::Sender<ClientMessage>,
        tx_page: mpsc::Sender<PageMessage>,
        rx_page: Arc<Mutex<mpsc::Receiver<PageMessage>>>,
    ) {
        loop {
            let message = rx_page.lock().unwrap().recv().unwrap();

            match message.downcast_ref::<PageEvent>() {
                Some(PageEvent::Terminate) => break,
                _ => {}
            }

            match message.downcast_ref::<MainPageEvent>() {
                Some(MainPageEvent::Quit) => {
                    break;
                }
                Some(MainPageEvent::ConnectToServer(addr, port)) => {
                    tx_client
                        .send(ClientMessage::ConnectToServer(
                            String::from(addr),
                            String::from(port),
                        ))
                        .unwrap();
                }
                _ => {}
            }

            match message.downcast_ref::<LoginPageEvent>() {
                Some(LoginPageEvent::Quit) => {
                    break;
                }
                Some(LoginPageEvent::Login(user, pass)) => {
                    tx_client
                        .send(ClientMessage::Login(String::from(user), String::from(pass)))
                        .unwrap();
                }
                Some(LoginPageEvent::GoToSignup) => {
                    let signup_tx = tx_page.clone();
                    cb_sink
                        .send(Box::new(|s| {
                            let signup_page = Self::render_signup_page(signup_tx);

                            s.pop_layer();
                            s.add_layer(signup_page.body());
                        }))
                        .unwrap();
                }
                _ => {}
            }

            match message.downcast_ref::<SignupPageEvent>() {
                Some(SignupPageEvent::Quit) => {
                    break;
                }
                Some(SignupPageEvent::Signup(name, user, pass)) => {
                    tx_client
                        .send(ClientMessage::Signup(
                            String::from(name),
                            String::from(user),
                            String::from(pass),
                        ))
                        .unwrap();
                }
                Some(SignupPageEvent::GoToLogin) => {
                    let login_tx = tx_page.clone();
                    cb_sink
                        .send(Box::new(|s| {
                            let login_page = Self::render_login_page(login_tx);

                            s.pop_layer();
                            s.add_layer(login_page.body());
                        }))
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

    fn render_login_page(tx: mpsc::Sender<PageMessage>) -> LoginPage {
        LoginPage::new(tx)
    }

    fn render_signup_page(tx: mpsc::Sender<PageMessage>) -> SignupPage {
        SignupPage::new(tx)
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
