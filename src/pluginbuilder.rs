pub trait PluginWithOptions: Plugin {
    type Options;
    fn init(&mut self, Self::Options);
}

pub trait Plugin {
    fn run(&mut self);
    fn stop(&mut self);
}


// #[cfg(test)]
// mod tests {
//     #[test]
//     fn create_plugin_builder() {
//         let app = MyApp {};
//         let b = PluginBuilder::new(app);
//     }
// }
