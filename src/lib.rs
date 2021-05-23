//! `BytesStr` is an immutable reference counted UTF8-String
//! useful for storing views into UTF8-encoded parts of data.

use bytes::Bytes;
use std::fmt;
use std::ops::Deref;
use std::str::{from_utf8, from_utf8_unchecked, Utf8Error};

#[cfg(feature = "serde")]
mod serde;

/// BytesStr is an immutable UTF8-String using [Bytes] as its buffer.
#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BytesStr {
    // This must always be valid UTF8
    bytes: Bytes,
}

impl BytesStr {
    /// Returns an empty BytesStr
    #[inline]
    pub const fn empty() -> Self {
        BytesStr {
            bytes: Bytes::new(),
        }
    }

    /// Returns a new BytesStr built from an static &str
    #[inline]
    pub const fn from_static(str: &'static str) -> Self {
        Self {
            bytes: Bytes::from_static(str.as_bytes()),
        }
    }

    /// Returns a BytesStr using the buffer `src` containing the `subset`.
    ///
    /// # Example
    ///
    /// ```
    /// use bytes::Bytes;
    /// use bytesstr::BytesStr;
    /// use std::str::from_utf8;
    ///
    /// let buffer = Bytes::from_static(b"Test!");
    ///
    /// let subset = &buffer[1..];
    /// let subset = from_utf8(subset).unwrap();
    ///
    /// let bytes_str = BytesStr::from_parse(&buffer, subset);
    ///
    /// assert_eq!(bytes_str, "est!");
    /// ```
    ///
    /// # Panics
    ///
    /// The given `subset` must point into `src` buffer
    #[inline]
    pub fn from_parse(src: &Bytes, subset: &str) -> Self {
        Self {
            bytes: src.slice_ref(subset.as_bytes()),
        }
    }

    /// Try to create a BytesStr from an Bytes buffer.
    ///
    /// # Example
    ///
    /// ```
    /// use bytes::Bytes;
    /// use bytesstr::BytesStr;
    ///
    /// let buffer = Bytes::from_static(b"Test!");
    ///
    /// let bytes_str = BytesStr::from_utf8_bytes(buffer).unwrap();
    ///
    /// assert_eq!(bytes_str, "Test!");
    /// ```
    #[inline]
    pub fn from_utf8_bytes(bytes: Bytes) -> Result<Self, Utf8Error> {
        from_utf8(&bytes)?;
        Ok(Self { bytes })
    }

    /// Create a `BytesStr` from a `Bytes` which contains valid UTF8 and doesn't need to be checked.
    ///
    /// # Example
    ///
    /// ```
    /// use bytes::Bytes;
    /// use bytesstr::BytesStr;
    ///
    /// let buffer = Bytes::from_static(b"Test!");
    ///
    /// let bytes_str = unsafe { BytesStr::from_utf8_bytes_unchecked(buffer) };
    ///
    /// assert_eq!(bytes_str, "Test!");
    /// ```
    /// # Safety
    ///
    /// Passed `Bytes` parameter must be valid UTF8
    ///
    /// # Panics
    ///
    /// If `debug-assertions` are enabled an UTF8 check is performed, which panics on error.
    #[inline]
    pub unsafe fn from_utf8_bytes_unchecked(bytes: Bytes) -> Self {
        debug_assert!(from_utf8(&bytes).is_ok());

        Self { bytes }
    }

    /// Returns a str slice into the internal buffer
    #[inline]
    pub fn as_str(&self) -> &str {
        // Safety:
        // There is no safe way to construct a BytesStr from an invalid UTF8 string
        unsafe { from_utf8_unchecked(&self.bytes) }
    }

    /// Pass an subset of the BytesStr to create a new BytesStr containing the `subset` slice
    ///
    /// # Example
    ///
    /// ```
    /// use bytesstr::BytesStr;
    ///
    /// let bytes_str1 = BytesStr::from_static("Test!");
    ///
    /// let bytes_str2 = bytes_str1.slice_ref(&bytes_str1[1..]);
    ///
    /// assert_eq!(bytes_str2, "est!");
    /// ```
    ///
    /// # Panics
    ///
    /// The given `subset` must point into the buffer of `self`
    #[inline]
    pub fn slice_ref(&self, subset: &str) -> Self {
        Self::from_parse(&self.bytes, subset)
    }

    /// Creates a new BytesStr containing the same bytes but in a new seperate buffer
    ///
    /// # Example
    ///
    /// ```
    /// use bytesstr::BytesStr;
    ///
    /// let bytes_str1 = BytesStr::from_static("Test!");
    ///
    /// let bytes_str2 = bytes_str1.clone_detach();
    ///
    /// assert_eq!(bytes_str1, bytes_str2);
    /// assert_ne!(bytes_str1.as_ptr(), bytes_str2.as_ptr());
    /// ```
    #[inline]
    pub fn clone_detach(&self) -> Self {
        Self {
            bytes: Bytes::copy_from_slice(&self.bytes),
        }
    }
}

impl PartialEq<[u8]> for BytesStr {
    fn eq(&self, other: &[u8]) -> bool {
        self.bytes.eq(other)
    }
}

impl PartialEq<str> for BytesStr {
    fn eq(&self, other: &str) -> bool {
        self.bytes.eq(other.as_bytes())
    }
}

impl PartialEq<&str> for BytesStr {
    fn eq(&self, other: &&str) -> bool {
        self.bytes.eq(other.as_bytes())
    }
}

impl Deref for BytesStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl AsRef<str> for BytesStr {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<[u8]> for BytesStr {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl AsRef<Bytes> for BytesStr {
    fn as_ref(&self) -> &Bytes {
        &self.bytes
    }
}

impl From<&str> for BytesStr {
    fn from(s: &str) -> Self {
        BytesStr {
            bytes: Bytes::copy_from_slice(s.as_bytes()),
        }
    }
}

impl From<String> for BytesStr {
    fn from(s: String) -> Self {
        Self {
            bytes: Bytes::from(s.into_bytes()),
        }
    }
}

impl fmt::Display for BytesStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl fmt::Debug for BytesStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}
