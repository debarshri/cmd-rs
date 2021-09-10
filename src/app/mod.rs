pub fn new() -> Box<dyn AppBuilder> {
    return Box::new(CliAppBuilder{shorts: "".to_string()})
}

pub trait AppBuilder {
    fn add(&self, data: &str)  -> Box<dyn AppBuilder> ;
    fn execute(&self);
}

struct CliAppBuilder {
    shorts: String,
}

impl AppBuilder for CliAppBuilder {
    fn add(&self, data: &str) -> Box<dyn AppBuilder>  {
        return Box::new(CliAppBuilder{shorts: data.to_string()})
    }

    fn execute(&self)  {
        println!("{}", &self.shorts.to_string());
    }
}

