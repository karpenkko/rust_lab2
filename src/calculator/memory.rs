pub struct StandardMemory {
    value: f64,
}

impl StandardMemory {
    pub fn new() -> Self {
        Self { value: 0.0 }
    }

    pub fn get(&self) -> f64 {
        self.value
    }

    pub fn set(&mut self, value: f64) {
        self.value = value;
    }

    pub fn reset(&mut self) {
        self.value = 0.0;
    }
}