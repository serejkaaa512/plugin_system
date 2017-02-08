use std::thread;
use std::cell::RefCell;
use std::rc::Rc;
use app::M;
use plugin1::{Plugin1Options, Plugin1};
use plugin2::Plugin2;
use pluginbuilder::{Plugin, PluginWithOptions};

pub type AppRef = Rc<RefCell<M>>;

pub enum Plugins {
    Plugin1(Plugin1Options),
    Plugin2,
}

pub struct Pluggable {
    app: AppRef,
    plugins: Vec<Box<Plugin>>,
    is_running: bool,
}

impl Pluggable {
    pub fn new(app: M) -> Self {
        Pluggable {
            app: Rc::new(RefCell::new(app)),
            plugins: vec![],
            is_running: false,
        }
    }

    pub fn start_app(&mut self) {
        self.app
            .borrow_mut()
            .run();
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

    pub fn start_plugins(&mut self) {
        self.is_running = true;
        thread::spawn(|| {
            while self.is_running {
                for pl in self.plugins.iter_mut() {
                    pl.run();
                }
            }
        });

    }

    pub fn stop_plugins(&mut self) {
        self.is_running = false;
        for pl in self.plugins.iter_mut() {
            pl.stop();
        }
    }
}
