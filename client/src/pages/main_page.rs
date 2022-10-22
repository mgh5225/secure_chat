use crate::{Page, PageMessage};
use cursive::{
    view::Nameable,
    views::{Dialog, EditView, LinearLayout, TextView},
};

use std::sync::mpsc;

pub struct MainPage {
    tx: mpsc::Sender<PageMessage>,
}

impl Page for MainPage {
    fn body(&self) -> Box<dyn cursive::View> {
        let q_tx = self.tx.clone();
        let c_tx = self.tx.clone();

        Box::new(
            Dialog::around(
                LinearLayout::vertical()
                    .child(TextView::new("Server Address"))
                    .child(EditView::new().with_name("addr"))
                    .child(TextView::new("Server Port"))
                    .child(EditView::new().with_name("port")),
            )
            .title("Secure Chat App")
            .button("Quit", move |s| {
                q_tx.send(Box::new(MainPageEvent::Quit)).unwrap();
                s.quit();
            })
            .button("Connect", move |s| {
                let addr = s
                    .call_on_name("addr", |view: &mut EditView| view.get_content())
                    .unwrap();
                let port = s
                    .call_on_name("port", |view: &mut EditView| view.get_content())
                    .unwrap();

                c_tx.send(Box::new(MainPageEvent::ConnectToServer(
                    String::from(addr.as_str()),
                    String::from(port.as_str()),
                )))
                .unwrap();
            }),
        )
    }

    fn new(tx: mpsc::Sender<PageMessage>) -> Self {
        Self { tx }
    }
}

pub enum MainPageEvent {
    Retry,
    ConnectToServer(String, String),
    Quit,
}
