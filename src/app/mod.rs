use std::env;

pub fn new() -> Box<dyn AppBuilder> {
    return Box::new(CliAppBuilder {
        name: "".to_string(),
        description: "".to_string(),
        parameter: Vec::new(),
        subcommands: Vec::new(),
    });
}

pub trait AppBuilder {
    fn name(&self, name: &str) -> Box<dyn AppBuilder>;
    fn description(&self, description: &str) -> Box<dyn AppBuilder>;
    fn add_paramater(&self, parameter: Parameter) -> Box<dyn AppBuilder>;
    fn add_subcommand(&self, subcommand: SubCommandAppBuilder) -> Box<dyn AppBuilder>;
    fn parse(&self);
}

struct CliAppBuilder {
    name: String,
    description: String,
    parameter: Vec<Parameter>,
    subcommands: Vec<SubCommandAppBuilder>,
}

impl AppBuilder for CliAppBuilder {
    fn name(&self, name: &str) -> Box<dyn AppBuilder> {
        let mut parameters: Vec<Parameter> = Vec::new();
        for x in &self.parameter {
            parameters.push(x.clone());
        }

        let mut subcommands: Vec<SubCommandAppBuilder> = Vec::new();
        for x in &self.subcommands {
            subcommands.push(x.clone());
        }
        return Box::new(CliAppBuilder {
            name: name.to_string(),
            description: self.description.to_string(),
            parameter: parameters,
            subcommands: subcommands,
        });
    }

    fn description(&self, description: &str) -> Box<dyn AppBuilder> {
        let mut parameters: Vec<Parameter> = Vec::new();
        for x in &self.parameter {
            parameters.push(x.clone());
        }

        let mut subcommands: Vec<SubCommandAppBuilder> = Vec::new();
        for x in &self.subcommands {
            subcommands.push(x.clone());
        }

        return Box::new(CliAppBuilder {
            name: self.name.to_string(),
            description: description.to_string(),
            parameter: parameters,
            subcommands: subcommands,
        });
    }

    fn add_paramater(&self, parameter: Parameter) -> Box<dyn AppBuilder> {
        let mut parameters: Vec<Parameter> = Vec::new();
        for x in &self.parameter {
            parameters.push(x.clone());
        }

        parameters.push(parameter);

        let mut subcommands: Vec<SubCommandAppBuilder> = Vec::new();
        for x in &self.subcommands {
            subcommands.push(x.clone());
        }

        return Box::new(CliAppBuilder {
            name: self.name.to_string(),
            description: self.description.to_string(),
            parameter: parameters,
            subcommands: subcommands,
        });
    }

    fn add_subcommand(&self, subcommand: SubCommandAppBuilder) -> Box<dyn AppBuilder> {
        let mut parameters: Vec<Parameter> = Vec::new();
        for x in &self.parameter {
            parameters.push(x.clone());
        }

        let mut subcommands: Vec<SubCommandAppBuilder> = Vec::new();
        for x in &self.subcommands {
            subcommands.push(x.clone());
        }

        subcommands.push(subcommand);

        return Box::new(CliAppBuilder {
            name: self.name.to_string(),
            description: self.description.to_string(),
            parameter: parameters,
            subcommands: subcommands,
        });
    }

    fn parse(&self) {
        let args: Vec<String> = env::args().collect();

        println!("Starting app {}", self.name);
        println!("{:?}", args);

        for x in &args[1..] {
            for p in &self.parameter {

                let s = x;
                if 
                // *s[0..1] == "--" && 
                p.name == s[2..] {
                    println!("found {}", p.name);
                    (p.function)(&p.name);
                }

                // if s.chars().count() == 2 && s.as_bytes()[0] == '-' && p.short == s.as_bytes()[1] {
                //     println!("found {}", p.name);
                // }
            }
        }
    }
}

pub fn new_parameter(name: &str, short: &str, description: &str, f: fn(&str)) -> Parameter {
    Parameter {
        name: name.to_string(),
        short: short.to_string(),
        description: description.to_string(),
        function: f,
    }
}

#[derive(Clone)]
pub struct Parameter {
    name: String,
    short: String,
    description: String,
    function: fn(&str),
}

#[derive(Clone)]
pub struct SubCommandAppBuilder {
    name: String,
}
