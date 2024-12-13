use std::collections::TryReserveError;

pub trait TryPush<T> {
    fn try_push(&mut self, elem: T) -> Result<(), TryReserveError>;
}

impl<T> TryPush<T> for Vec<T> {
    fn try_push(&mut self, elem: T) -> Result<(), TryReserveError> {
        self.try_reserve(1)?;
        self.push(elem);
        Ok(())
    }
}
