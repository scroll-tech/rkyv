use super::{ErasedPtr, Pooling};

/// A shared pointer strategy that duplicates deserializations of the same
/// shared pointer.
#[derive(Debug, Default)]
pub struct Duplicate;

impl<E> Pooling<E> for Duplicate {
    fn get_shared_ptr(&mut self, _: usize) -> Option<ErasedPtr> {
        None
    }

    unsafe fn add_shared_ptr(
        &mut self,
        _: usize,
        _: ErasedPtr,
        _: unsafe fn(ErasedPtr),
    ) -> Result<(), E> {
        Ok(())
    }
}
