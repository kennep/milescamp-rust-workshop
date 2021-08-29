fn main() {
    let name = String::from("Ferris");
    say_hello(&name); // Note: ampersand

    // Now say_hello only borrows the value for the
    // duration of the function execution.
    println!("I said hello to {}!", name);
}

fn say_hello(name: &String) // Note: ampersand
{
    println!("Hello, {}!", name);
}
