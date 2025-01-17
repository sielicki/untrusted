// Copyright 2015-2021 Brian Smith.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

use crate::{no_panic, Reader};

/// A wrapper around `&'a [u8]` that helps in writing panic-free code.
///
/// No methods of `Input` will ever panic.
#[derive(Clone, Copy, Debug, Eq)]
pub struct Input<'a> {
    value: no_panic::Slice<'a>,
}

impl<'a> Input<'a> {
    /// Construct a new `Input` for the given input `bytes`.
    #[must_use]
    pub const fn from(bytes: &'a [u8]) -> Self {
        // This limit is important for avoiding integer overflow. In particular,
        // `Reader` assumes that an `i + 1 > i` if `input.value.get(i)` does
        // not return `None`. According to the Rust language reference, the
        // maximum object size is `core::isize::MAX`, and in practice it is
        // impossible to create an object of size `core::usize::MAX` or larger.
        Self {
            value: no_panic::Slice::new(bytes),
        }
    }

    /// Returns `true` if the input is empty and false otherwise.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    /// Returns the length of the `Input`.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.value.len()
    }

    /// Calls `read` with the given input as a `Reader`, ensuring that `read`
    /// consumed the entire input. If `read` does not consume the entire input,
    /// `incomplete_read` is returned.
    pub fn read_all<F, R, E>(&self, incomplete_read: E, read: F) -> Result<R, E>
    where
        F: FnOnce(&mut Reader<'a>) -> Result<R, E>,
    {
        let mut input = Reader::new(*self);
        let result = read(&mut input)?;
        if input.at_end() {
            Ok(result)
        } else {
            Err(incomplete_read)
        }
    }

    /// Access the input as a slice so it can be processed by functions that
    /// are not written using the Input/Reader framework.
    #[inline]
    #[must_use]
    pub fn as_slice_less_safe(&self) -> &'a [u8] {
        self.value.as_slice_less_safe()
    }

    pub(super) fn into_value(self) -> no_panic::Slice<'a> {
        self.value
    }
}

impl<'a> From<&'a [u8]> for Input<'a> {
    #[inline]
    fn from(value: &'a [u8]) -> Self {
        no_panic::Slice::new(value).into()
    }
}

impl<'a> From<no_panic::Slice<'a>> for Input<'a> {
    #[inline]
    fn from(value: no_panic::Slice<'a>) -> Self {
        Self { value }
    }
}

// #[derive(PartialEq)] would result in lifetime bounds that are
// unnecessarily restrictive; see
// https://github.com/rust-lang/rust/issues/26925.
impl PartialEq<Input<'_>> for Input<'_> {
    #[inline]
    fn eq(&self, other: &Input) -> bool {
        self.as_slice_less_safe() == other.as_slice_less_safe()
    }
}

impl PartialEq<[u8]> for Input<'_> {
    #[inline]
    fn eq(&self, other: &[u8]) -> bool {
        self.as_slice_less_safe() == other
    }
}

impl PartialEq<Input<'_>> for [u8] {
    #[inline]
    fn eq(&self, other: &Input) -> bool {
        other.as_slice_less_safe() == self
    }
}
