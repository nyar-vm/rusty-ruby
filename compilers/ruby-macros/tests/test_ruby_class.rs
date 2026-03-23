//! Test for the RubyClass derive macro

use ruby::{Ruby, ToRubyValue};
use ruby_macros::RubyClass;

/// A test struct to derive RubyClass for
#[derive(RubyClass)]
struct Person {
    /// The person's name
    name: String,
    /// The person's age
    age: i32,
    /// Whether the person is active
    active: bool,
}

fn main() {
    // Create a new Ruby runtime
    let mut ruby = Ruby::new().unwrap();

    // Define the Person class in Ruby
    Person::define_ruby_class(&mut ruby).unwrap();

    // Create a Person instance
    let person = Person { name: "John Doe".to_string(), age: 30, active: true };

    // Convert the Person instance to a Ruby value
    let ruby_value = person.to_ruby_value();

    // Print the Ruby value
    println!("Ruby value: {:?}", ruby_value);

    println!("Test passed!");
}
