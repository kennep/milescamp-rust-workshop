fn main() {
    let name = String::from("Ferris");
    say_hello(name);
    // The 'name' variable is not accessible 
    // after it has been moved into say_hello
    println!("I said hello to {}!", name);
}

fn say_hello(name: String)
{
    println!("Hello, {}!", name);
}
