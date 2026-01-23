use glam::Vec2;

pub(crate) fn angle_to_vector(angle: f32) -> Vec2 {
    Vec2::new(angle.cos(), angle.sin())
}
