use std::{env, net::TcpListener, sync::Arc};

use threadpool::ThreadPool;

mod client;
use client::Client;

mod database;
use crate::database::Database;

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

pub fn run(config: Config) -> Result<(), String> {
    let listener = match TcpListener::bind(config.addr) {
        Ok(listener) => listener,
        Err(err) => return Err(err.to_string()),
    };

    let pool = ThreadPool::new(config.max_workers);

    println!("[!] Server is running");

    let database = match Database::new() {
        Ok(db) => db,
        Err(err) => return Err(err.to_string()),
    };

    let database = Arc::new(database);

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            _ => continue,
        };

        let mut client = Client::new(stream, Arc::clone(&database));

        pool.execute(move || client.run());
    }

    Ok(())
}
