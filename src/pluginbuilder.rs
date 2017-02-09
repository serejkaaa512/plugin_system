pub trait PluginWithOptions: Plugin {
    type Options;
    fn init(&mut self, Self::Options);
}

pub trait Plugin: Send + Sync {
    fn run(&mut self);
    fn stop(&mut self);
}
