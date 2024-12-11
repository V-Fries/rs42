pub trait PipeLine: Sized {
    /// Does the same thing as Iterator::map(), but for any type
    fn pipe<T>(self, f: impl FnOnce(Self) -> T) -> T {
        f(self)
    }

    /// Does the same thing as Pipeline::pipe(), but with an unsafe fn instead of a closure
    ///
    /// # Safety
    /// f() will be called only once, ensure it is safe to call
    #[allow(dead_code)]
    unsafe fn pipe_unsafe<T>(self, f: unsafe fn(Self) -> T) -> T {
        f(self)
    }
}

impl<T> PipeLine for T {}
