#[derive(Copy, Clone)]
pub struct M {
    pub m: u32,
}

impl M {
    pub fn new() -> M {
        M { m: 0 }
    }

    pub fn run(&mut self) {
        self.m = self.m + 10;
        println!("App is running. M = {}", self.m);
    }
}