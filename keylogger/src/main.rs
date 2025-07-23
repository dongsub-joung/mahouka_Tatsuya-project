use rdev::{listen, Event};

// This will block.
if let Err(error) = listen(callback) {
    println!("Error: {:?}", error)
}

fn callback(event: Event) {
    println!("My callback {:?}", event);
    match event.name {
        Some(string) => println!("User wrote {:?}", string),
        None => (),
    }
}

fn main() {
    println!("Hello, world!");
}
