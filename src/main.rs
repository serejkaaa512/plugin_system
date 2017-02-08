extern crate plugin_system;
use plugin_system::{M, Pluggable, Plugin1Options, Plugins};


fn main() {
    let m = M::new();
    let mut pl_app = Pluggable::new(m);
    let p1o = Plugin1Options { val1: 123 };
    pl_app.load_plugin(Plugins::Plugin1(p1o));
    pl_app.load_plugin(Plugins::Plugin2);
    pl_app.start_plugins();
    pl_app.stop_plugins();
}