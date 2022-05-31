//AUTHOR: "AtomicGamer9523"@github.com
//FORMAT: "RUST"
pub trait System {
    fn init(&mut self);
    fn tick(&mut self);
    fn box_clone(&self) -> Box<dyn System>;
}
impl Clone for Box<dyn System> {
    fn clone(&self) -> Box<dyn System> {
        self.box_clone()
    }
}
pub struct Systems {
    pub systems: Vec<Box<dyn System>>
}
impl Systems {
    pub fn new() -> Self {
        let vec: Vec<Box<dyn System>> = Vec::new();
        Self {
            systems: vec
        }
    }

    pub fn add(&mut self, system: Box<dyn System>) {
        self.systems.push(system);
    }

    pub fn run(&mut self){
        let mut selfsystems = self.systems.clone();
        loop {
            for system in selfsystems.iter_mut() {
                system.tick();
            }
        }
    }
}