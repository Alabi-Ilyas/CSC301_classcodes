mod greetings;
extern crate hello_world_lib;
use greetings::{english,french,spanish};

fn main() {
    println!("Hello, world!");
    println!("{}", english:: default_greeting());
    println!("{}", spanish::default_greeting());
    println!("{}", french::default_greeting());
    println!("{}", hello_world_lib::greeting_from_lib());
}