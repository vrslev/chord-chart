use crate::note::Scale;

pub trait Transpose {
    fn transpose(&self, semitone_incr: &i32, scale: &Scale) -> Self;
}
