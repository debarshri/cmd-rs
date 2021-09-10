use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let app = cmd_rs::app::new();

    app.
    add(&args[1]).
    execute();
}