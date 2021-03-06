use pluggable::AppRef;
use pluginbuilder::{PluginWithOptions, Plugin};

pub struct Plugin1Options {
    val1: i32,
}

impl Plugin1Options {
    pub fn new(v: i32) -> Self {
        Plugin1Options { val1: v }
    }
}

pub struct Plugin1 {
    val1: i32,
    app: AppRef,
}

impl Plugin1 {
    pub fn new(app: AppRef) -> Self {
        Plugin1 {
            val1: 0,
            app: app,
        }
    }
}

impl PluginWithOptions for Plugin1 {
    type Options = Plugin1Options;

    fn init(&mut self, opts: Plugin1Options) {
        self.val1 = opts.val1;
    }
}

impl Plugin for Plugin1 {
    fn run(&mut self) {
        let mut a = self.app
            .write()
            .expect("Failed to acquire a write lock on app!");
        a.m = a.m + self.val1;
        println!("Plugin1 run! Changed 'm' from App to {}", a.m);
    }

    fn stop(&mut self) {
        println!("Plugin1 stop!");
    }
}
