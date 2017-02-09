extern crate plugin_system;
use plugin_system::{M, Pluggable, Plugin1Options, Plugins};
use std::thread;
use std::time::Duration;

fn main() {
    let m = M::new();
    let mut pl_app = Pluggable::new(m);
    pl_app.load_plugin(Plugins::Plugin1(Plugin1Options::new(1)));
    pl_app.load_plugin(Plugins::Plugin2);
    pl_app.run_app();
    pl_app.run_plugins();
    thread::sleep(Duration::from_millis(5));
    pl_app.stop();
}