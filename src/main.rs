fn main() {
    let app = cmd_rs::app::new();

    app.name("test")
        .add_paramater(cmd_rs::app::new_parameter("chart", "c", "descr", list))
        .add_paramater(cmd_rs::app::new_parameter("chart2", "a", "descr", list))
        .parse();
}

fn list(x: &str) {
    println!("parameter {}", x);
}
