pub trait PipeLine: Sized {
    /// Calls the given function passing self as argument and returning the output
    /// # Examples
    /// ```
    /// use rs42::extensions::PipeLine;
    ///
    /// let a = 40_i32.pipe(|nb| nb + 2);
    /// assert_eq!(a, 42);
    ///
    /// let b = 42_i32.pipe(Some);
    /// assert_eq!(b, Some(42));
    /// ```
    fn pipe<T>(self, f: impl FnOnce(Self) -> T) -> T {
        f(self)
    }

    /// Calls the given unsafe function passing self as argument and returning the output
    ///
    /// # Safety
    /// f() will be called only once, ensure it is safe to call
    ///
    /// # Examples
    /// ```
    /// use rs42::extensions::PipeLine;
    ///
    /// unsafe fn foo(nb: i32) -> i32 {
    ///     // Unsafe code...
    ///     return nb + 2;
    /// }
    ///
    /// unsafe {
    ///     assert_eq!(40.pipe_unsafe(foo), 42);
    /// }
    /// ```
    #[allow(dead_code)]
    unsafe fn pipe_unsafe<T>(self, f: unsafe fn(Self) -> T) -> T {
        f(self)
    }

    /// Calls the given c function passing self as argument and returning the output
    ///
    /// # Safety
    /// f() will be called only once, ensure it is safe to call
    ///
    /// # Examples
    /// ```
    /// use rs42::extensions::PipeLine;
    /// use std::ffi::c_char;
    ///
    /// extern "C" { fn strlen(s: *const c_char) -> usize; }
    ///
    /// unsafe {
    ///     assert_eq!(c"42".as_ptr().pipe_c_fn(strlen), 2);
    /// }
    /// ```
    #[allow(dead_code)]
    unsafe fn pipe_c_fn<T>(self, f: unsafe extern "C" fn(Self) -> T) -> T {
        f(self)
    }
}

impl<T> PipeLine for T {}
