use pluginbuilder::Plugin;

pub struct Plugin2;

impl Plugin2 {
    pub fn new() -> Self {
        Plugin2
    }
}

impl Plugin for Plugin2 {
    fn run(&mut self) {
        println!("Plugin2 run!");
    }
    fn stop(&mut self) {
        println!("Plugin2 stop!");
    }
}
