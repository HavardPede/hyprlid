mod config;
mod lid;
mod lid_closed_event;

fn main() {
    // Start listening to zbus events
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(lid_closed_event::watch())
        .unwrap();
}
