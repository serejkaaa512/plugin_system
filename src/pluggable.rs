use std::sync::{Arc, RwLock};
use app::M;
use plugin1::{Plugin1Options, Plugin1};
use plugin2::Plugin2;
use pluginbuilder::{Plugin, PluginWithOptions};
use std::thread;
use std::thread::JoinHandle;

pub type AppRef = Arc<RwLock<M>>;

pub enum Plugins {
    Plugin1(Plugin1Options),
    Plugin2,
}

pub struct Pluggable {
    app: AppRef,
    plugins: Arc<RwLock<Vec<Box<Plugin>>>>,
    is_running: Arc<RwLock<bool>>,
}


impl Clone for Pluggable {
    fn clone(&self) -> Self {
        Pluggable {
            app: self.app.clone(),
            plugins: self.plugins.clone(),
            is_running: self.is_running.clone(),
        }
    }
}

impl Pluggable {
    pub fn new(app: M) -> Self {
        Pluggable {
            app: Arc::new(RwLock::new(app)),
            plugins: Arc::new(RwLock::new(vec![])),
            is_running: Arc::new(RwLock::new(true)),
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
        self.plugins.write().unwrap().push(plugin);
    }


    pub fn run_app(&mut self) {
        let self_clone = self.clone();
        thread::spawn(move || {
            while *self_clone.is_running.read().unwrap() {
                self_clone.app
                    .write()
                    .expect("Cant run app!")
                    .run();
                thread::yield_now();
            }
        });
    }

    pub fn run_plugins(&mut self) {
        let self_clone = self.clone();
        thread::spawn(move || {
            while *self_clone.is_running.read().unwrap() {
                for pl in self_clone.plugins.write().unwrap().iter_mut() {
                    pl.run();
                }
                thread::yield_now();
            }
        });
    }

    pub fn stop(&mut self) {
        *self.is_running.write().unwrap() = false;
        for pl in self.plugins.write().unwrap().iter_mut() {
            pl.stop();
        }
    }
}
