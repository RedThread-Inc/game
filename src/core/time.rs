pub struct RTTime {
    pub delta: f32,
    pub elapsed: f32,
}

impl RTTime {
    pub fn update(&mut self, delta: f32) {
        self.delta = delta;
        self.elapsed += delta;
    }
}
