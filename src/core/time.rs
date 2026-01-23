pub(crate) struct RTTime {
    pub(crate) delta: f32,
    pub(crate) elapsed: f32,
}

impl RTTime {
    pub(crate) fn update(&mut self, delta: f32) {
        self.delta = delta;
        self.elapsed += delta;
    }
}
