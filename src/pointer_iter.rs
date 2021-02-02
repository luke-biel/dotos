pub struct PointerIter<T> {
    end: *mut T,
    ptr: *mut T,
}

impl<T> Iterator for PointerIter<T> {
    type Item = *mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr < self.end {
            let ptr = self.ptr;
            self.ptr = unsafe { self.ptr.offset(1) };
            Some(ptr)
        } else {
            None
        }
    }
}

impl<T> PointerIter<T> {
    pub fn new(start: *mut T, end: *mut T) -> Self {
        Self {
            end,
            ptr: start,
        }
    }
}
