#[cfg(test)]
mod tests {

    pub mod App;

    #[test]
    fn it_works() {
        let app = App::new();
    }
}

pub mod app;