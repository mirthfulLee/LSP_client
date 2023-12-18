//! # The Rust Standard Library
//!
//! The Rust Standard Library is the foundation of portable Rust software, a
//! set of minimal and battle-tested shared abstractions for the [broader Rust
//! ecosystem][crates.io]. It offers core types, like [`Vec<T>`] and
//! [`Option<T>`], library-defined [operations on language
//! primitives](#primitives), [standard macros](#macros), [I/O] and
//! [multithreading], among [many other things][other].

use std::num::NonZeroUsize;

/// Get the default number of threads to use, if not explicitly specified.
#[inline]
fn default_num_threads() -> NonZeroUsize {
    // If we can't get the amount of parallelism for some reason, then
    // default to a single thread, because that is safe.
    // Note that the minimum value for a NonZeroUsize is 1.
    // Unfortunately, we can't do `NonZeroUsize::new(1).unwrap()`
    // in a const context.
    const FALLBACK_PARALLELISM: NonZeroUsize = NonZeroUsize::MIN;
    // As the number of threads increases, the startup time suffers from
    // initializing the threads, and we get diminishing returns from additional
    // parallelism. So set a maximum number of threads to use by default.
    //
    // This value is based on some empirical observations, but the ideal value
    // probably depends on the exact hardware in use.
    //
    // Safety: The literal "20" is known not to be zero.
    const MAX_DEFAULT_THREADS: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(20) };

    std::cmp::min(
        std::thread::available_parallelism().unwrap_or(FALLBACK_PARALLELISM),
        MAX_DEFAULT_THREADS,
    )
}
#[derive(Debug)]
struct Numbered<T> {
    id: usize,
    field: T,
}

pub trait Effective: Sized {
    unsafe fn modify_raw(list: &[Self], index: usize, value: Self);
    fn replace(list: &[Self], value: Self);
}

impl<T> Effective for Numbered<T> {
    /// modify the item in the slice even the reference is immutable
    ///
    /// # Safety
    /// the index must in the valid range
    unsafe fn modify_raw(list: &[Self], index: usize, value: Self) {
        let ptr = list.as_ptr();
        let p = ptr.add(index) as usize as *mut Numbered<T>;
        *p = value;
    }

    /// replace the first item in the list that have lower id than value;
    /// if all of the items in the list have bigger id than value, value will be discarded.
    fn replace(list: &[Self], value: Self) {
        let mut p = list.len();
        for i in 0..list.len() {
            if list[i].id < value.id {
                p = i;
                break;
            }
        }
        if p < list.len() {
            // Safety: index(p) have already been check in the above if
            unsafe {
                Self::modify_raw(list, p, value);
            }
        }
    }
}

fn main() {
    // let num = Numbered { id: 1, field: 10 };
    let slc = [
        Numbered { id: 234, field: 10 },
        Numbered { id: 125, field: 20 },
        Numbered { id: 325, field: 30 },
        Numbered { id: 14, field: 40 },
        Numbered { id: 255, field: 50 },
    ];
    let value = Numbered { id: 20, field: 100 };
    println!("{:?}", slc);
    Numbered::replace(&slc, value);
    println!("{:?}", slc);
}
