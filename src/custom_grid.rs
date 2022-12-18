/// This is implemented as per the chapter in The Rustonomicon...
/// Some features that maybe are stabilised now:
/// - std::ptr::Unique;
/// - std::alloc::Global;
use std::ptr::NonNull;
use std::marker::PhantomData;
use std::mem;
use std::alloc::{self, Layout};
use std::ops::{Deref, DerefMut, Add};
use num_traits::Num;

pub struct GridArray<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
    _marker: PhantomData<T>,
}

/// As I understand it, Send and Sync are implemented for concurrency features,
/// something I'm not too worried about currently.
unsafe impl<T: Send> Send for GridArray<T> {}
unsafe impl<T: Sync> Sync for GridArray<T> {}

impl<T> GridArray<T> {
    pub fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "We're not ready to handle Zero-Sized Types");
        GridArray {
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0,
            _marker: PhantomData,
        }
    }

    /// A Vec is inherently a growable data-structure
    /// When implementing an actual grid array, we
    /// don't want to grow the array.
    fn grow(&mut self) {
        // -- HANDLING CHANGES IN CAPACITY AND MEMORY LAYOUTS --
        // If it's a brand new GridArray:
        let (new_cap, new_layout) = if self.cap == 0 {
            // New cap is 1, the layout is the size of 1 T
            (1, Layout::array::<T>(1).unwrap())
        } else {
            // Otherwise, the new cap and layout is 2 times the current cap
            let new_cap = 2 * self.cap;

            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        // Check the new allocation isn't greater than isize::MAX
        assert!(new_layout.size() <= isize::MAX as usize, "Allocation too large");

        // -- ALLOCATING/REALLOCATING THE NonNull POINTER --
        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout  = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        // Setting the stored ptr to the new_ptr and
        // handling if the allocation above failed
        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }

    pub fn push(&mut self, elem: T) {
        if self.len == self.cap { self.grow(); }

        // ptr::write just overwrites a memory address
        // without evaluating the memory (meaning
        // dereferencing perhaps?)
        // .add(self.len) computes the offset from a pointer
        // by value self.len * size_of::<T>()
        // write then puts a new elem into that offset
        unsafe {
            std::ptr::write(self.ptr.as_ptr().add(self.len), elem);
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            // Removes the element at offset self.len
            // and returns it
            unsafe {
                Some(std::ptr::read(self.ptr.as_ptr().add(self.len)))
            }
        }
    }

    pub fn insert(&mut self, index: usize, elem: T) {
        assert!(index <= self.len, "index out of bounds");
        if self.cap == self.len { self.grow(); }

        unsafe {
            std::ptr::copy(self.ptr.as_ptr().add(index),
                            self.ptr.as_ptr().add(index + 1),
                            self.len - index);
            std::ptr::write(self.ptr.as_ptr().add(index), elem);
            self.len += 1;
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len, "index out of bounds");
        unsafe {
            self.len -= 1;
            let result = std::ptr::read(self.ptr.as_ptr().add(index));
            std::ptr::copy(self.ptr.as_ptr().add(index + 1),
                           self.ptr.as_ptr().add(index),
                           self.len - index);
            result
        }
    }
}

impl<T> Drop for GridArray<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            while let Some(_) = self.pop() { }
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout)
            }
        }
    }
}

// Implementing slicing
// This implements a lot of functions for free
// len, first, last, indexing, slicing, sorting, iter, iter_mut
impl<T> Deref for GridArray<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts(self.ptr.as_ptr(), self.len)
        }
    }
}

impl<T> DerefMut for GridArray<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len)
        }
    }
}

impl<T: Num + Copy>Add for GridArray<T> {
    type Output = GridArray<T>;

    fn add(self, _rhs: GridArray<T>) -> GridArray<T> {
        assert!(self.len == _rhs.len, "arrays not the same length");
        _rhs.iter().zip(self.iter()).map(|(&b, &v)| b + v).collect()
    }
}

pub struct IntoIter<T> {
    buf: NonNull<T>,
    cap: usize,
    start: *const T,
    end: *const T,
    _marker: PhantomData<T>,
}

impl<T> IntoIterator for GridArray<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        let ptr = self.ptr;
        let cap = self.cap;
        let len = self.len;

        mem::forget(self);

        unsafe {
            IntoIter {
                buf: ptr,
                cap: cap,
                start: ptr.as_ptr(),
                end: if cap == 0 {
                    ptr.as_ptr()
                } else {
                    ptr.as_ptr().add(len)
                },
                _marker: PhantomData,
            }
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                let result = std::ptr::read(self.start);
                self.start = self.start.offset(1);
                Some(result)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.end as usize  - self.start as usize) / mem::size_of::<T>();
        (len, Some(len))
    }
}

impl<T> FromIterator<T> for GridArray<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut g = GridArray::new();

        for i in iter {
            g.push(i);
        }

        g
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for GridArray<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        std::fmt::Debug::fmt(&**self, f)
    }
}
