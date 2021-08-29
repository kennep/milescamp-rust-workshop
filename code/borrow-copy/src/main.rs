fn main() {
    let robot_name = 9000;
    say_hello(robot_name);

    println!("I said hello to HAL {}!", robot_name);
}

fn say_hello(robot_name: u32)
{
    println!("Hello, HAL {}!", robot_name);
}
