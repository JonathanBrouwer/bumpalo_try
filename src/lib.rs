use std::alloc::Layout;
use std::{ptr, slice};
use bumpalo::Bump;

pub trait BumpaloExtend {
    /// Allocates a new slice of size `len` into this `Bump` and returns an
    /// exclusive reference to the copy, early exiting if the function returns Err.
    ///
    /// The elements of the slice are initialized using the supplied closure.
    /// The closure argument is the position in the slice.
    ///
    /// ## Panics
    ///
    /// Panics if reserving space for the slice fails.
    ///
    /// ## Examples
    ///
    /// ```
    /// use bumpalo_try::BumpaloExtend;
    /// let bump = bumpalo::Bump::new();
    /// let x = bump.alloc_slice_fill_with_result(5, |i| Ok::<usize, ()>(5 * (i + 1)));
    /// assert_eq!(x.unwrap(), &[5, 10, 15, 20, 25]);
    /// ```
    ///
    /// ```
    /// use bumpalo_try::BumpaloExtend;
    /// let bump = bumpalo::Bump::new();
    /// let x = bump.alloc_slice_fill_with_result(5, |i| Err::<usize, ()>(()));
    /// assert_eq!(x, Err(()));
    /// ```
    fn alloc_slice_fill_with_result<T, E, F>(&self, len: usize, f: F) -> Result<&mut [T], E>
    where
        F: FnMut(usize) -> Result<T, E>;

    /// Allocates a new slice of size `len` into this `Bump` and returns an
    /// exclusive reference to the copy, early exiting if the function returns None.
    ///
    /// The elements of the slice are initialized using the supplied closure.
    /// The closure argument is the position in the slice.
    ///
    /// ## Panics
    ///
    /// Panics if reserving space for the slice fails.
    ///
    /// ## Examples
    ///
    /// ```
    /// use bumpalo_try::BumpaloExtend;
    /// let bump = bumpalo::Bump::new();
    /// let x = bump.alloc_slice_fill_with_option(5, |i| Some(5 * (i + 1)));
    /// assert_eq!(x.unwrap(), &[5, 10, 15, 20, 25]);
    /// ```
    ///
    /// ```
    /// use bumpalo_try::BumpaloExtend;
    /// let bump = bumpalo::Bump::new();
    /// let x = bump.alloc_slice_fill_with_option(5, |i| None::<usize>);
    /// assert_eq!(x, None);
    /// ```
    fn alloc_slice_fill_with_option<T, F>(&self, len: usize, mut f: F) -> Option<&mut [T]>
    where
        F: FnMut(usize) -> Option<T> {
        self.alloc_slice_fill_with_result(len, |i| {
            f(i).ok_or(())
        }).ok()
    }

    /// Allocates a new slice of size `len` slice into this `Bump` and return an
    /// exclusive reference to the copy, early exiting if the iterator returns Err.
    ///
    /// The elements are initialized using the supplied iterator.
    ///
    /// ## Panics
    ///
    /// Panics if reserving space for the slice fails, or if the supplied
    /// iterator returns fewer elements than it promised.
    ///
    /// ## Example
    ///
    /// ```
    /// use bumpalo_try::BumpaloExtend;
    /// let bump = bumpalo::Bump::new();
    /// let x: Result<&mut [i32], ()> = bump.alloc_slice_fill_iter_result([2, 3, 5].iter().cloned().map(|i| Ok(i * i)));
    /// assert_eq!(x.unwrap(), [4, 9, 25]);
    /// ```
    ///
    /// ```
    /// use bumpalo_try::BumpaloExtend;
    /// let bump = bumpalo::Bump::new();
    /// let x: Result<&mut [i32], ()> = bump.alloc_slice_fill_iter_result([Ok(2), Err(()), Ok(5)]);
    /// assert_eq!(x, Err(()));
    /// ```
    fn alloc_slice_fill_iter_result<T, E, I>(&self, iter: I) -> Result<&mut [T], E>
    where
        I: IntoIterator<Item = Result<T, E>>,
        I::IntoIter: ExactSizeIterator {
        let mut iter = iter.into_iter();
        self.alloc_slice_fill_with_result(iter.len(), |_| {
            iter.next().expect("Iterator supplied too few elements")
        })
    }

    /// Allocates a new slice of size `len` slice into this `Bump` and return an
    /// exclusive reference to the copy, early exiting if the iterator returns Err.
    ///
    /// The elements are initialized using the supplied iterator.
    ///
    /// ## Panics
    ///
    /// Panics if reserving space for the slice fails, or if the supplied
    /// iterator returns fewer elements than it promised.
    ///
    /// ## Example
    ///
    /// ```
    /// use bumpalo_try::BumpaloExtend;
    /// let bump = bumpalo::Bump::new();
    /// let x: Option<&mut [i32]> = bump.alloc_slice_fill_iter_option([2, 3, 5].iter().cloned().map(|i| i.checked_pow(2)));
    /// assert_eq!(x.unwrap(), [4, 9, 25]);
    /// ```
    ///
    /// ```
    /// use bumpalo_try::BumpaloExtend;
    /// let bump = bumpalo::Bump::new();
    /// let x: Option<&mut [i32]> = bump.alloc_slice_fill_iter_option([2, 3, i32::MAX].iter().cloned().map(|i| i.checked_pow(2)));
    /// assert_eq!(x, None);
    /// ```
    fn alloc_slice_fill_iter_option<T, I>(&self, iter: I) -> Option<&mut [T]>
    where
        I: IntoIterator<Item = Option<T>>,
        I::IntoIter: ExactSizeIterator {
        self.alloc_slice_fill_iter_result(iter.into_iter().map(|v| v.ok_or(()))).ok()
    }
}

impl BumpaloExtend for Bump {
    fn alloc_slice_fill_with_result<T, E, F>(&self, len: usize, mut f: F) -> Result<&mut [T], E>
    where
        F: FnMut(usize) -> Result<T, E>
    {
        let layout = Layout::array::<T>(len).unwrap_or_else(|_| panic!("out of memory"));
        let dst = self.alloc_layout(layout).cast::<T>();

        unsafe {
            for i in 0..len {
                let v = f(i)?;
                ptr::write(dst.as_ptr().add(i), v);
            }

            let result = slice::from_raw_parts_mut(dst.as_ptr(), len);
            debug_assert_eq!(Layout::for_value(result), layout);
            Ok(result)
        }
    }
}