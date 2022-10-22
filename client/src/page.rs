use cursive::View;
use std::{any::Any, sync::mpsc};

pub trait Page {
    fn body(&self) -> Box<dyn View>;
    fn new(tx: mpsc::Sender<PageMessage>) -> Self;
}

pub type PageMessage = Box<dyn Any + Send + 'static>;

pub enum PageEvent {
    Terminate,
}
