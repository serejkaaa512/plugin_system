use std::sync::{Arc, RwLock};
use app::M;
use plugin1::{Plugin1Options, Plugin1};
use plugin2::Plugin2;
use pluginbuilder::{Plugin, PluginWithOptions};
use std::thread;

pub type AppRef = Arc<RwLock<M>>;

pub enum Plugins {
    Plugin1(Plugin1Options),
    Plugin2,
}

pub struct Pluggable {
    app: AppRef,
    plugins: Vec<Box<Plugin>>,
    pub is_running: bool,
}

impl Pluggable {
    pub fn new(app: M) -> Self {
        Pluggable {
            app: Arc::new(RwLock::new(app)),
            plugins: vec![],
            is_running: false,
        }
    }

    pub fn load_plugin(&mut self, pl_name: Plugins) {
        let plugin: Box<Plugin> = match pl_name {
            Plugins::Plugin1(opts) => {
                let mut p = Plugin1::new(self.app.clone());
                p.init(opts);
                Box::new(p)
            }

            Plugins::Plugin2 => Box::new(Plugin2::new()),
        };
        self.plugins.push(plugin);
    }


    pub fn run_app(&mut self) {
        self.app
            .write()
            .expect("Cant run app!")
            .run();
    }

    pub fn run_plugins(&mut self) {
        for pl in self.plugins.iter_mut() {
            pl.run();
        }
    }

    pub fn stop(&mut self) {
        self.is_running = false;
        for pl in self.plugins.iter_mut() {
            pl.stop();
        }
    }
}
