fn main() {
    let siv = cursive::default();

    let mut manager = client::Manager::new(siv);

    manager.run();
}
