use crate::{Page, PageMessage};
use cursive::{
    view::Nameable,
    views::{Dialog, EditView, LinearLayout, TextView},
};

use std::sync::mpsc;

pub struct SignupPage {
    tx: mpsc::Sender<PageMessage>,
}

impl Page for SignupPage {
    fn body(&self) -> Box<dyn cursive::View> {
        let q_tx = self.tx.clone();
        let c_tx = self.tx.clone();
        let l_tx = self.tx.clone();

        Box::new(
            Dialog::around(
                LinearLayout::vertical()
                    .child(TextView::new("Name"))
                    .child(EditView::new().with_name("name"))
                    .child(TextView::new("Username"))
                    .child(EditView::new().with_name("user"))
                    .child(TextView::new("Password"))
                    .child(EditView::new().secret().with_name("pass")),
            )
            .title("Welcome to The Secure Chat App")
            .button("Quit", move |s| {
                q_tx.send(Box::new(SignupPageEvent::Quit)).unwrap();
                s.quit();
            })
            .button("Signup", move |s| {
                let name = s
                    .call_on_name("name", |view: &mut EditView| view.get_content())
                    .unwrap();
                let user = s
                    .call_on_name("user", |view: &mut EditView| view.get_content())
                    .unwrap();
                let pass = s
                    .call_on_name("pass", |view: &mut EditView| view.get_content())
                    .unwrap();

                c_tx.send(Box::new(SignupPageEvent::Signup(
                    String::from(name.as_str()),
                    String::from(user.as_str()),
                    String::from(pass.as_str()),
                )))
                .unwrap();
            })
            .button("Go to Login", move |_| {
                l_tx.send(Box::new(SignupPageEvent::GoToLogin)).unwrap();
            }),
        )
    }

    fn new(tx: mpsc::Sender<PageMessage>) -> Self {
        Self { tx }
    }
}

pub enum SignupPageEvent {
    Signup(String, String, String),
    Quit,
    GoToLogin,
}
