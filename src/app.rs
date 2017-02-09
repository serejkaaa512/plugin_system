#[derive(Copy, Clone)]
pub struct M {
    pub m: i32,
}

impl M {
    pub fn new() -> M {
        M { m: 0 }
    }

    pub fn run(&mut self) {
        self.m = self.m - 1;
        println!("App is running. M = {}", self.m);
    }
}