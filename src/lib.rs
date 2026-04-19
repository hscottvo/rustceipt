pub mod error;
pub mod receipt;

pub(crate) fn close(lhs: f32, rhs: f32) -> bool {
    let delta = (lhs - rhs).abs();
    delta < f32::EPSILON
}
