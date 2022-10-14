use std::{
    env,
    error::Error,
    net::{TcpListener, TcpStream},
};

use threadpool::ThreadPool;

pub struct Config {
    pub addr: String,
    pub max_workers: usize,
    pub flags: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Self, &'static str> {
        args.next();

        let addr = match args.next() {
            Some(arg) => arg,
            None => return Err("Address not provided"),
        };

        let max_workers = match args.next() {
            Some(arg) => arg.parse().unwrap(),
            None => return Err("Max Workers not provided"),
        };

        let flags = match args.next() {
            Some(arg) => arg,
            None => return Err("Flags not provided"),
        };

        Ok(Self {
            addr,
            max_workers,
            flags,
        })
    }
}

struct Client {
    stream: TcpStream,
}

impl Client {
    fn new(stream: TcpStream) -> Self {
        Client { stream }
    }

    fn run(&self) {
        todo!()
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(config.addr).unwrap();

    let pool = ThreadPool::new(config.max_workers);

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            _ => continue,
        };

        let client = Client::new(stream);

        pool.execute(move || client.run());
    }

    Ok(())
}
