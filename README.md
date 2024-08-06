<div align="center">
  <h1><code>Bumpalo Try</code></h1>
  <p><strong>Allocates a fallible iterator into a bumpalo.</strong></p>
</div>

## About

Provides the following functions on a [bumpalo](https://crates.io/crates/bumpalo):
* ```rs
  fn alloc_slice_fill_with_result<T, E>(&self, len: usize, f: impl FnMut(usize) -> Result<T, E>) -> Result<&mut [T], E>
  ```
* ```rs
  fn alloc_slice_fill_with_option<T>(&self, len: usize, mut f: impl FnMut(usize) -> Option<T>) -> Option<&mut [T]>
  ```
* ```rs
  fn alloc_slice_fill_iter_result<T, E, I>(&self, iter: I) -> Result<&mut [T], E>
    where
        I: IntoIterator<Item = Result<T, E>>,
        I::IntoIter: ExactSizeIterator
  ```
* ```rs
  fn alloc_slice_fill_iter_option<T, I>(&self, iter: I) -> Option<&mut [T]>
    where
        I: IntoIterator<Item = Option<T>>,
        I::IntoIter: ExactSizeIterator
  ```

These functions will early-exit if the stream of values indicates a failure.
