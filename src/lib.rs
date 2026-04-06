pub mod error;
pub mod receipt;

pub(crate) fn assert_close(lhs: f32, rhs: f32) {
    let delta = (lhs - rhs).abs();
    assert!(delta < f32::EPSILON);
}
