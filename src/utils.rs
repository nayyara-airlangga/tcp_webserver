/// Call operator which takes ownership of the caller inside a box
pub trait FnBox {
    /// Calls the caller within box
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)();
    }
}
