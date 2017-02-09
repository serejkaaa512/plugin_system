extern crate plugin_system;
use plugin_system::{M, Pluggable, Plugin1Options, Plugins};
use std::thread;
use std::sync::{Arc, RwLock};
use std::time::Duration;



fn main() {
    let m = M::new();
    let mut pl_app = Pluggable::new(m);
    let p1o = Plugin1Options::new(1);
    pl_app.load_plugin(Plugins::Plugin1(p1o));
    pl_app.load_plugin(Plugins::Plugin2);

    let a = Arc::new(RwLock::new(pl_app));

    let aa = a.clone();
    aa.write()
        .expect("can not run app")
        .is_running = true;
    let guard_app = thread::spawn(move || {
        while aa.read().unwrap().is_running {
            aa.write()
                .expect("can not run app")
                .run_app();
            thread::yield_now();
        }
    });

    let aa = a.clone();
    let guard_pl = thread::spawn(move || {
        while aa.read().unwrap().is_running {
            aa.write()
                .expect("can not run plugins")
                .run_plugins();
            thread::yield_now();
        }
    });


    thread::sleep(Duration::from_millis(5));

    let aa = a.clone();
    aa.write()
        .expect("can not stop")
        .stop();

    guard_pl.join()
        .unwrap();
    guard_app.join()
        .unwrap();
}