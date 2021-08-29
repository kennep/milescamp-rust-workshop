fn main() {
    let name = String::from("Ferris");
    say_hello(name.clone());

    println!("I said hello to {}!", name);
}

fn say_hello(name: String)
{
    println!("Hello, {}!", name);
}
