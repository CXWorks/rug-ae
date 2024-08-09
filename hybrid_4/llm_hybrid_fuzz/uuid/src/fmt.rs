//! Adapters for alternative string formats.
use crate::{
    std::{borrow::Borrow, fmt, ptr, str},
    Uuid, Variant,
};
impl std::fmt::Debug for Uuid {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(self, f)
    }
}
impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(self, f)
    }
}
impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Variant::NCS => write!(f, "NCS"),
            Variant::RFC4122 => write!(f, "RFC4122"),
            Variant::Microsoft => write!(f, "Microsoft"),
            Variant::Future => write!(f, "Future"),
        }
    }
}
impl fmt::LowerHex for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(self.as_hyphenated(), f)
    }
}
impl fmt::UpperHex for Uuid {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(self.as_hyphenated(), f)
    }
}
/// Format a [`Uuid`] as a hyphenated string, like
/// `67e55044-10b1-426f-9247-bb680e5fe0c8`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Hyphenated(Uuid);
/// Format a [`Uuid`] as a simple string, like
/// `67e5504410b1426f9247bb680e5fe0c8`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Simple(Uuid);
/// Format a [`Uuid`] as a URN string, like
/// `urn:uuid:67e55044-10b1-426f-9247-bb680e5fe0c8`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Urn(Uuid);
/// Format a [`Uuid`] as a braced hyphenated string, like
/// `{67e55044-10b1-426f-9247-bb680e5fe0c8}`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Braced(Uuid);
impl Uuid {
    /// Get a [`Hyphenated`] formatter.
    #[inline]
    pub const fn hyphenated(self) -> Hyphenated {
        Hyphenated(self)
    }
    /// Get a borrowed [`Hyphenated`] formatter.
    #[inline]
    pub fn as_hyphenated(&self) -> &Hyphenated {
        unsafe { &*(self as *const Uuid as *const Hyphenated) }
    }
    /// Get a [`Simple`] formatter.
    #[inline]
    pub const fn simple(self) -> Simple {
        Simple(self)
    }
    /// Get a borrowed [`Simple`] formatter.
    #[inline]
    pub fn as_simple(&self) -> &Simple {
        unsafe { &*(self as *const Uuid as *const Simple) }
    }
    /// Get a [`Urn`] formatter.
    #[inline]
    pub const fn urn(self) -> Urn {
        Urn(self)
    }
    /// Get a borrowed [`Urn`] formatter.
    #[inline]
    pub fn as_urn(&self) -> &Urn {
        unsafe { &*(self as *const Uuid as *const Urn) }
    }
    /// Get a [`Braced`] formatter.
    #[inline]
    pub const fn braced(self) -> Braced {
        Braced(self)
    }
    /// Get a borrowed [`Braced`] formatter.
    #[inline]
    pub fn as_braced(&self) -> &Braced {
        unsafe { &*(self as *const Uuid as *const Braced) }
    }
}
const UPPER: [u8; 16] = [
    b'0',
    b'1',
    b'2',
    b'3',
    b'4',
    b'5',
    b'6',
    b'7',
    b'8',
    b'9',
    b'A',
    b'B',
    b'C',
    b'D',
    b'E',
    b'F',
];
const LOWER: [u8; 16] = [
    b'0',
    b'1',
    b'2',
    b'3',
    b'4',
    b'5',
    b'6',
    b'7',
    b'8',
    b'9',
    b'a',
    b'b',
    b'c',
    b'd',
    b'e',
    b'f',
];
#[inline]
const fn format_simple(src: &[u8; 16], upper: bool) -> [u8; 32] {
    let lut = if upper { &UPPER } else { &LOWER };
    let mut dst = [0; 32];
    let mut i = 0;
    while i < 16 {
        let x = src[i];
        dst[i * 2] = lut[(x >> 4) as usize];
        dst[i * 2 + 1] = lut[(x & 0x0f) as usize];
        i += 1;
    }
    dst
}
#[inline]
const fn format_hyphenated(src: &[u8; 16], upper: bool) -> [u8; 36] {
    let lut = if upper { &UPPER } else { &LOWER };
    let groups = [(0, 8), (9, 13), (14, 18), (19, 23), (24, 36)];
    let mut dst = [0; 36];
    let mut group_idx = 0;
    let mut i = 0;
    while group_idx < 5 {
        let (start, end) = groups[group_idx];
        let mut j = start;
        while j < end {
            let x = src[i];
            i += 1;
            dst[j] = lut[(x >> 4) as usize];
            dst[j + 1] = lut[(x & 0x0f) as usize];
            j += 2;
        }
        if group_idx < 4 {
            dst[end] = b'-';
        }
        group_idx += 1;
    }
    dst
}
#[inline]
fn encode_simple<'b>(src: &[u8; 16], buffer: &'b mut [u8], upper: bool) -> &'b mut str {
    let buf = &mut buffer[..Simple::LENGTH];
    let dst = buf.as_mut_ptr();
    unsafe {
        ptr::write(dst.cast(), format_simple(src, upper));
        str::from_utf8_unchecked_mut(buf)
    }
}
#[inline]
fn encode_hyphenated<'b>(
    src: &[u8; 16],
    buffer: &'b mut [u8],
    upper: bool,
) -> &'b mut str {
    let buf = &mut buffer[..Hyphenated::LENGTH];
    let dst = buf.as_mut_ptr();
    unsafe {
        ptr::write(dst.cast(), format_hyphenated(src, upper));
        str::from_utf8_unchecked_mut(buf)
    }
}
#[inline]
fn encode_braced<'b>(src: &[u8; 16], buffer: &'b mut [u8], upper: bool) -> &'b mut str {
    let buf = &mut buffer[..Braced::LENGTH];
    buf[0] = b'{';
    buf[Braced::LENGTH - 1] = b'}';
    unsafe {
        let dst = buf.as_mut_ptr().add(1);
        ptr::write(dst.cast(), format_hyphenated(src, upper));
        str::from_utf8_unchecked_mut(buf)
    }
}
#[inline]
fn encode_urn<'b>(src: &[u8; 16], buffer: &'b mut [u8], upper: bool) -> &'b mut str {
    let buf = &mut buffer[..Urn::LENGTH];
    buf[..9].copy_from_slice(b"urn:uuid:");
    unsafe {
        let dst = buf.as_mut_ptr().add(9);
        ptr::write(dst.cast(), format_hyphenated(src, upper));
        str::from_utf8_unchecked_mut(buf)
    }
}
impl Hyphenated {
    /// The length of a hyphenated [`Uuid`] string.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    pub const LENGTH: usize = 36;
    /// Creates a [`Hyphenated`] from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`Hyphenated`]: struct.Hyphenated.html
    pub const fn from_uuid(uuid: Uuid) -> Self {
        Hyphenated(uuid)
    }
    /// Writes the [`Uuid`] as a lower-case hyphenated string to
    /// `buffer`, and returns the subslice of the buffer that contains the
    /// encoded UUID.
    ///
    /// This is slightly more efficient than using the formatting
    /// infrastructure as it avoids virtual calls, and may avoid
    /// double buffering.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    ///
    /// # Panics
    ///
    /// Panics if the buffer is not large enough: it must have length at least
    /// [`LENGTH`]. [`Uuid::encode_buffer`] can be used to get a
    /// sufficiently-large temporary buffer.
    ///
    /// [`LENGTH`]: #associatedconstant.LENGTH
    /// [`Uuid::encode_buffer`]: ../struct.Uuid.html#method.encode_buffer
    ///
    /// # Examples
    ///
    /// ```rust
    /// use uuid::Uuid;
    ///
    /// fn main() -> Result<(), uuid::Error> {
    ///     let uuid = Uuid::parse_str("936DA01f9abd4d9d80c702af85c822a8")?;
    ///
    ///     // the encoded portion is returned
    ///     assert_eq!(
    ///         uuid.hyphenated()
    ///             .encode_lower(&mut Uuid::encode_buffer()),
    ///         "936da01f-9abd-4d9d-80c7-02af85c822a8"
    ///     );
    ///
    ///     // the buffer is mutated directly, and trailing contents remains
    ///     let mut buf = [b'!'; 40];
    ///     uuid.hyphenated().encode_lower(&mut buf);
    ///     assert_eq!(
    ///         &buf as &[_],
    ///         b"936da01f-9abd-4d9d-80c7-02af85c822a8!!!!" as &[_]
    ///     );
    ///
    ///     Ok(())
    /// }
    /// ```
    /// */
    #[inline]
    pub fn encode_lower<'buf>(&self, buffer: &'buf mut [u8]) -> &'buf mut str {
        encode_hyphenated(self.0.as_bytes(), buffer, false)
    }
    /// Writes the [`Uuid`] as an upper-case hyphenated string to
    /// `buffer`, and returns the subslice of the buffer that contains the
    /// encoded UUID.
    ///
    /// This is slightly more efficient than using the formatting
    /// infrastructure as it avoids virtual calls, and may avoid
    /// double buffering.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    ///
    /// # Panics
    ///
    /// Panics if the buffer is not large enough: it must have length at least
    /// [`LENGTH`]. [`Uuid::encode_buffer`] can be used to get a
    /// sufficiently-large temporary buffer.
    ///
    /// [`LENGTH`]: #associatedconstant.LENGTH
    /// [`Uuid::encode_buffer`]: ../struct.Uuid.html#method.encode_buffer
    ///
    /// # Examples
    ///
    /// ```rust
    /// use uuid::Uuid;
    ///
    /// fn main() -> Result<(), uuid::Error> {
    ///     let uuid = Uuid::parse_str("936da01f9abd4d9d80c702af85c822a8")?;
    ///
    ///     // the encoded portion is returned
    ///     assert_eq!(
    ///         uuid.hyphenated()
    ///             .encode_upper(&mut Uuid::encode_buffer()),
    ///         "936DA01F-9ABD-4D9D-80C7-02AF85C822A8"
    ///     );
    ///
    ///     // the buffer is mutated directly, and trailing contents remains
    ///     let mut buf = [b'!'; 40];
    ///     uuid.hyphenated().encode_upper(&mut buf);
    ///     assert_eq!(
    ///         &buf as &[_],
    ///         b"936DA01F-9ABD-4D9D-80C7-02AF85C822A8!!!!" as &[_]
    ///     );
    ///
    ///     Ok(())
    /// }
    /// ```
    /// */
    #[inline]
    pub fn encode_upper<'buf>(&self, buffer: &'buf mut [u8]) -> &'buf mut str {
        encode_hyphenated(self.0.as_bytes(), buffer, true)
    }
    /// Get a reference to the underlying [`Uuid`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use uuid::Uuid;
    ///
    /// let hyphenated = Uuid::nil().hyphenated();
    /// assert_eq!(*hyphenated.as_uuid(), Uuid::nil());
    /// ```
    pub const fn as_uuid(&self) -> &Uuid {
        &self.0
    }
    /// Consumes the [`Hyphenated`], returning the underlying [`Uuid`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use uuid::Uuid;
    ///
    /// let hyphenated = Uuid::nil().hyphenated();
    /// assert_eq!(hyphenated.into_uuid(), Uuid::nil());
    /// ```
    pub const fn into_uuid(self) -> Uuid {
        self.0
    }
}
impl Braced {
    /// The length of a braced [`Uuid`] string.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    pub const LENGTH: usize = 38;
    /// Creates a [`Braced`] from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`Braced`]: struct.Braced.html
    pub const fn from_uuid(uuid: Uuid) -> Self {
        Braced(uuid)
    }
    /// Writes the [`Uuid`] as a lower-case hyphenated string surrounded by
    /// braces to `buffer`, and returns the subslice of the buffer that contains
    /// the encoded UUID.
    ///
    /// This is slightly more efficient than using the formatting
    /// infrastructure as it avoids virtual calls, and may avoid
    /// double buffering.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    ///
    /// # Panics
    ///
    /// Panics if the buffer is not large enough: it must have length at least
    /// [`LENGTH`]. [`Uuid::encode_buffer`] can be used to get a
    /// sufficiently-large temporary buffer.
    ///
    /// [`LENGTH`]: #associatedconstant.LENGTH
    /// [`Uuid::encode_buffer`]: ../struct.Uuid.html#method.encode_buffer
    ///
    /// # Examples
    ///
    /// ```rust
    /// use uuid::Uuid;
    ///
    /// fn main() -> Result<(), uuid::Error> {
    ///     let uuid = Uuid::parse_str("936DA01f9abd4d9d80c702af85c822a8")?;
    ///
    ///     // the encoded portion is returned
    ///     assert_eq!(
    ///         uuid.braced()
    ///             .encode_lower(&mut Uuid::encode_buffer()),
    ///         "{936da01f-9abd-4d9d-80c7-02af85c822a8}"
    ///     );
    ///
    ///     // the buffer is mutated directly, and trailing contents remains
    ///     let mut buf = [b'!'; 40];
    ///     uuid.braced().encode_lower(&mut buf);
    ///     assert_eq!(
    ///         &buf as &[_],
    ///         b"{936da01f-9abd-4d9d-80c7-02af85c822a8}!!" as &[_]
    ///     );
    ///
    ///     Ok(())
    /// }
    /// ```
    /// */
    #[inline]
    pub fn encode_lower<'buf>(&self, buffer: &'buf mut [u8]) -> &'buf mut str {
        encode_braced(self.0.as_bytes(), buffer, false)
    }
    /// Writes the [`Uuid`] as an upper-case hyphenated string surrounded by
    /// braces to `buffer`, and returns the subslice of the buffer that contains
    /// the encoded UUID.
    ///
    /// This is slightly more efficient than using the formatting
    /// infrastructure as it avoids virtual calls, and may avoid
    /// double buffering.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    ///
    /// # Panics
    ///
    /// Panics if the buffer is not large enough: it must have length at least
    /// [`LENGTH`]. [`Uuid::encode_buffer`] can be used to get a
    /// sufficiently-large temporary buffer.
    ///
    /// [`LENGTH`]: #associatedconstant.LENGTH
    /// [`Uuid::encode_buffer`]: ../struct.Uuid.html#method.encode_buffer
    ///
    /// # Examples
    ///
    /// ```rust
    /// use uuid::Uuid;
    ///
    /// fn main() -> Result<(), uuid::Error> {
    ///     let uuid = Uuid::parse_str("936da01f9abd4d9d80c702af85c822a8")?;
    ///
    ///     // the encoded portion is returned
    ///     assert_eq!(
    ///         uuid.braced()
    ///             .encode_upper(&mut Uuid::encode_buffer()),
    ///         "{936DA01F-9ABD-4D9D-80C7-02AF85C822A8}"
    ///     );
    ///
    ///     // the buffer is mutated directly, and trailing contents remains
    ///     let mut buf = [b'!'; 40];
    ///     uuid.braced().encode_upper(&mut buf);
    ///     assert_eq!(
    ///         &buf as &[_],
    ///         b"{936DA01F-9ABD-4D9D-80C7-02AF85C822A8}!!" as &[_]
    ///     );
    ///
    ///     Ok(())
    /// }
    /// ```
    /// */
    #[inline]
    pub fn encode_upper<'buf>(&self, buffer: &'buf mut [u8]) -> &'buf mut str {
        encode_braced(self.0.as_bytes(), buffer, true)
    }
    /// Get a reference to the underlying [`Uuid`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use uuid::Uuid;
    ///
    /// let braced = Uuid::nil().braced();
    /// assert_eq!(*braced.as_uuid(), Uuid::nil());
    /// ```
    pub const fn as_uuid(&self) -> &Uuid {
        &self.0
    }
    /// Consumes the [`Braced`], returning the underlying [`Uuid`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use uuid::Uuid;
    ///
    /// let braced = Uuid::nil().braced();
    /// assert_eq!(braced.into_uuid(), Uuid::nil());
    /// ```
    pub const fn into_uuid(self) -> Uuid {
        self.0
    }
}
impl Simple {
    /// The length of a simple [`Uuid`] string.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    pub const LENGTH: usize = 32;
    /// Creates a [`Simple`] from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`Simple`]: struct.Simple.html
    pub const fn from_uuid(uuid: Uuid) -> Self {
        Simple(uuid)
    }
    /// Writes the [`Uuid`] as a lower-case simple string to `buffer`,
    /// and returns the subslice of the buffer that contains the encoded UUID.
    ///
    /// This is slightly more efficient than using the formatting
    /// infrastructure as it avoids virtual calls, and may avoid
    /// double buffering.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    ///
    /// # Panics
    ///
    /// Panics if the buffer is not large enough: it must have length at least
    /// [`LENGTH`]. [`Uuid::encode_buffer`] can be used to get a
    /// sufficiently-large temporary buffer.
    ///
    /// [`LENGTH`]: #associatedconstant.LENGTH
    /// [`Uuid::encode_buffer`]: ../struct.Uuid.html#method.encode_buffer
    ///
    /// # Examples
    ///
    /// ```rust
    /// use uuid::Uuid;
    ///
    /// fn main() -> Result<(), uuid::Error> {
    ///     let uuid = Uuid::parse_str("936DA01f9abd4d9d80c702af85c822a8")?;
    ///
    ///     // the encoded portion is returned
    ///     assert_eq!(
    ///         uuid.simple().encode_lower(&mut Uuid::encode_buffer()),
    ///         "936da01f9abd4d9d80c702af85c822a8"
    ///     );
    ///
    ///     // the buffer is mutated directly, and trailing contents remains
    ///     let mut buf = [b'!'; 36];
    ///     assert_eq!(
    ///         uuid.simple().encode_lower(&mut buf),
    ///         "936da01f9abd4d9d80c702af85c822a8"
    ///     );
    ///     assert_eq!(
    ///         &buf as &[_],
    ///         b"936da01f9abd4d9d80c702af85c822a8!!!!" as &[_]
    ///     );
    ///
    ///     Ok(())
    /// }
    /// ```
    /// */
    #[inline]
    pub fn encode_lower<'buf>(&self, buffer: &'buf mut [u8]) -> &'buf mut str {
        encode_simple(self.0.as_bytes(), buffer, false)
    }
    /// Writes the [`Uuid`] as an upper-case simple string to `buffer`,
    /// and returns the subslice of the buffer that contains the encoded UUID.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    ///
    /// # Panics
    ///
    /// Panics if the buffer is not large enough: it must have length at least
    /// [`LENGTH`]. [`Uuid::encode_buffer`] can be used to get a
    /// sufficiently-large temporary buffer.
    ///
    /// [`LENGTH`]: #associatedconstant.LENGTH
    /// [`Uuid::encode_buffer`]: ../struct.Uuid.html#method.encode_buffer
    ///
    /// # Examples
    ///
    /// ```rust
    /// use uuid::Uuid;
    ///
    /// fn main() -> Result<(), uuid::Error> {
    ///     let uuid = Uuid::parse_str("936da01f9abd4d9d80c702af85c822a8")?;
    ///
    ///     // the encoded portion is returned
    ///     assert_eq!(
    ///         uuid.simple().encode_upper(&mut Uuid::encode_buffer()),
    ///         "936DA01F9ABD4D9D80C702AF85C822A8"
    ///     );
    ///
    ///     // the buffer is mutated directly, and trailing contents remains
    ///     let mut buf = [b'!'; 36];
    ///     assert_eq!(
    ///         uuid.simple().encode_upper(&mut buf),
    ///         "936DA01F9ABD4D9D80C702AF85C822A8"
    ///     );
    ///     assert_eq!(
    ///         &buf as &[_],
    ///         b"936DA01F9ABD4D9D80C702AF85C822A8!!!!" as &[_]
    ///     );
    ///
    ///     Ok(())
    /// }
    /// ```
    /// */
    #[inline]
    pub fn encode_upper<'buf>(&self, buffer: &'buf mut [u8]) -> &'buf mut str {
        encode_simple(self.0.as_bytes(), buffer, true)
    }
    /// Get a reference to the underlying [`Uuid`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use uuid::Uuid;
    ///
    /// let simple = Uuid::nil().simple();
    /// assert_eq!(*simple.as_uuid(), Uuid::nil());
    /// ```
    pub const fn as_uuid(&self) -> &Uuid {
        &self.0
    }
    /// Consumes the [`Simple`], returning the underlying [`Uuid`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use uuid::Uuid;
    ///
    /// let simple = Uuid::nil().simple();
    /// assert_eq!(simple.into_uuid(), Uuid::nil());
    /// ```
    pub const fn into_uuid(self) -> Uuid {
        self.0
    }
}
impl Urn {
    /// The length of a URN [`Uuid`] string.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    pub const LENGTH: usize = 45;
    /// Creates a [`Urn`] from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`Urn`]: struct.Urn.html
    pub const fn from_uuid(uuid: Uuid) -> Self {
        Urn(uuid)
    }
    /// Writes the [`Uuid`] as a lower-case URN string to
    /// `buffer`, and returns the subslice of the buffer that contains the
    /// encoded UUID.
    ///
    /// This is slightly more efficient than using the formatting
    /// infrastructure as it avoids virtual calls, and may avoid
    /// double buffering.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    ///
    /// # Panics
    ///
    /// Panics if the buffer is not large enough: it must have length at least
    /// [`LENGTH`]. [`Uuid::encode_buffer`] can be used to get a
    /// sufficiently-large temporary buffer.
    ///
    /// [`LENGTH`]: #associatedconstant.LENGTH
    /// [`Uuid::encode_buffer`]: ../struct.Uuid.html#method.encode_buffer
    ///
    /// # Examples
    ///
    /// ```rust
    /// use uuid::Uuid;
    ///
    /// fn main() -> Result<(), uuid::Error> {
    ///     let uuid = Uuid::parse_str("936DA01f9abd4d9d80c702af85c822a8")?;
    ///
    ///     // the encoded portion is returned
    ///     assert_eq!(
    ///         uuid.urn().encode_lower(&mut Uuid::encode_buffer()),
    ///         "urn:uuid:936da01f-9abd-4d9d-80c7-02af85c822a8"
    ///     );
    ///
    ///     // the buffer is mutated directly, and trailing contents remains
    ///     let mut buf = [b'!'; 49];
    ///     uuid.urn().encode_lower(&mut buf);
    ///     assert_eq!(
    ///         uuid.urn().encode_lower(&mut buf),
    ///         "urn:uuid:936da01f-9abd-4d9d-80c7-02af85c822a8"
    ///     );
    ///     assert_eq!(
    ///         &buf as &[_],
    ///         b"urn:uuid:936da01f-9abd-4d9d-80c7-02af85c822a8!!!!" as &[_]
    ///     );
    ///
    ///     Ok(())
    /// }
    /// ```
    /// */
    #[inline]
    pub fn encode_lower<'buf>(&self, buffer: &'buf mut [u8]) -> &'buf mut str {
        encode_urn(self.0.as_bytes(), buffer, false)
    }
    /// Writes the [`Uuid`] as an upper-case URN string to
    /// `buffer`, and returns the subslice of the buffer that contains the
    /// encoded UUID.
    ///
    /// This is slightly more efficient than using the formatting
    /// infrastructure as it avoids virtual calls, and may avoid
    /// double buffering.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    ///
    /// # Panics
    ///
    /// Panics if the buffer is not large enough: it must have length at least
    /// [`LENGTH`]. [`Uuid::encode_buffer`] can be used to get a
    /// sufficiently-large temporary buffer.
    ///
    /// [`LENGTH`]: #associatedconstant.LENGTH
    /// [`Uuid::encode_buffer`]: ../struct.Uuid.html#method.encode_buffer
    ///
    /// # Examples
    ///
    /// ```rust
    /// use uuid::Uuid;
    ///
    /// fn main() -> Result<(), uuid::Error> {
    ///     let uuid = Uuid::parse_str("936da01f9abd4d9d80c702af85c822a8")?;
    ///
    ///     // the encoded portion is returned
    ///     assert_eq!(
    ///         uuid.urn().encode_upper(&mut Uuid::encode_buffer()),
    ///         "urn:uuid:936DA01F-9ABD-4D9D-80C7-02AF85C822A8"
    ///     );
    ///
    ///     // the buffer is mutated directly, and trailing contents remains
    ///     let mut buf = [b'!'; 49];
    ///     assert_eq!(
    ///         uuid.urn().encode_upper(&mut buf),
    ///         "urn:uuid:936DA01F-9ABD-4D9D-80C7-02AF85C822A8"
    ///     );
    ///     assert_eq!(
    ///         &buf as &[_],
    ///         b"urn:uuid:936DA01F-9ABD-4D9D-80C7-02AF85C822A8!!!!" as &[_]
    ///     );
    ///
    ///     Ok(())
    /// }
    /// ```
    /// */
    #[inline]
    pub fn encode_upper<'buf>(&self, buffer: &'buf mut [u8]) -> &'buf mut str {
        encode_urn(self.0.as_bytes(), buffer, true)
    }
    /// Get a reference to the underlying [`Uuid`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use uuid::Uuid;
    ///
    /// let urn = Uuid::nil().urn();
    /// assert_eq!(*urn.as_uuid(), Uuid::nil());
    /// ```
    pub const fn as_uuid(&self) -> &Uuid {
        &self.0
    }
    /// Consumes the [`Urn`], returning the underlying [`Uuid`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use uuid::Uuid;
    ///
    /// let urn = Uuid::nil().urn();
    /// assert_eq!(urn.into_uuid(), Uuid::nil());
    /// ```
    pub const fn into_uuid(self) -> Uuid {
        self.0
    }
}
macro_rules! impl_fmt_traits {
    ($($T:ident <$($a:lifetime),*>),+) => {
        $(impl <$($a),*> fmt::Display for $T <$($a),*> { #[inline] fn fmt(& self, f : &
        mut fmt::Formatter <'_ >) -> fmt::Result { fmt::LowerHex::fmt(self, f) } } impl
        <$($a),*> fmt::LowerHex for $T <$($a),*> { fn fmt(& self, f : & mut
        fmt::Formatter <'_ >) -> fmt::Result { f.write_str(self.encode_lower(& mut [0;
        Self::LENGTH])) } } impl <$($a),*> fmt::UpperHex for $T <$($a),*> { fn fmt(&
        self, f : & mut fmt::Formatter <'_ >) -> fmt::Result { f.write_str(self
        .encode_upper(& mut [0; Self::LENGTH])) } } impl_fmt_from!($T <$($a),*>);)+
    };
}
macro_rules! impl_fmt_from {
    ($T:ident <>) => {
        impl From < Uuid > for $T { #[inline] fn from(f : Uuid) -> Self { $T (f) } } impl
        From <$T > for Uuid { #[inline] fn from(f : $T) -> Self { f.into_uuid() } } impl
        AsRef < Uuid > for $T { #[inline] fn as_ref(& self) -> & Uuid { & self.0 } } impl
        Borrow < Uuid > for $T { #[inline] fn borrow(& self) -> & Uuid { & self.0 } }
    };
    ($T:ident <$a:lifetime >) => {
        impl <$a > From <&$a Uuid > for $T <$a > { #[inline] fn from(f : &$a Uuid) ->
        Self { $T ::from_uuid_ref(f) } } impl <$a > From <$T <$a >> for &$a Uuid {
        #[inline] fn from(f : $T <$a >) -> &$a Uuid { f.0 } } impl <$a > AsRef < Uuid >
        for $T <$a > { #[inline] fn as_ref(& self) -> & Uuid { self.0 } } impl <$a >
        Borrow < Uuid > for $T <$a > { #[inline] fn borrow(& self) -> & Uuid { self.0 } }
    };
}
impl_fmt_traits! {
    Hyphenated <>, Simple <>, Urn <>, Braced <>
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hyphenated_trailing() {
        let mut buf = [b'x'; 100];
        let len = Uuid::nil().hyphenated().encode_lower(&mut buf).len();
        assert_eq!(len, super::Hyphenated::LENGTH);
        assert!(buf[len..].iter().all(| x | * x == b'x'));
    }
    #[test]
    fn hyphenated_ref_trailing() {
        let mut buf = [b'x'; 100];
        let len = Uuid::nil().as_hyphenated().encode_lower(&mut buf).len();
        assert_eq!(len, super::Hyphenated::LENGTH);
        assert!(buf[len..].iter().all(| x | * x == b'x'));
    }
    #[test]
    fn simple_trailing() {
        let mut buf = [b'x'; 100];
        let len = Uuid::nil().simple().encode_lower(&mut buf).len();
        assert_eq!(len, super::Simple::LENGTH);
        assert!(buf[len..].iter().all(| x | * x == b'x'));
    }
    #[test]
    fn simple_ref_trailing() {
        let mut buf = [b'x'; 100];
        let len = Uuid::nil().as_simple().encode_lower(&mut buf).len();
        assert_eq!(len, super::Simple::LENGTH);
        assert!(buf[len..].iter().all(| x | * x == b'x'));
    }
    #[test]
    fn urn_trailing() {
        let mut buf = [b'x'; 100];
        let len = Uuid::nil().urn().encode_lower(&mut buf).len();
        assert_eq!(len, super::Urn::LENGTH);
        assert!(buf[len..].iter().all(| x | * x == b'x'));
    }
    #[test]
    fn urn_ref_trailing() {
        let mut buf = [b'x'; 100];
        let len = Uuid::nil().as_urn().encode_lower(&mut buf).len();
        assert_eq!(len, super::Urn::LENGTH);
        assert!(buf[len..].iter().all(| x | * x == b'x'));
    }
    #[test]
    fn braced_trailing() {
        let mut buf = [b'x'; 100];
        let len = Uuid::nil().braced().encode_lower(&mut buf).len();
        assert_eq!(len, super::Braced::LENGTH);
        assert!(buf[len..].iter().all(| x | * x == b'x'));
    }
    #[test]
    fn braced_ref_trailing() {
        let mut buf = [b'x'; 100];
        let len = Uuid::nil().as_braced().encode_lower(&mut buf).len();
        assert_eq!(len, super::Braced::LENGTH);
        assert!(buf[len..].iter().all(| x | * x == b'x'));
    }
    #[test]
    #[should_panic]
    fn hyphenated_too_small() {
        Uuid::nil().hyphenated().encode_lower(&mut [0; 35]);
    }
    #[test]
    #[should_panic]
    fn simple_too_small() {
        Uuid::nil().simple().encode_lower(&mut [0; 31]);
    }
    #[test]
    #[should_panic]
    fn urn_too_small() {
        Uuid::nil().urn().encode_lower(&mut [0; 44]);
    }
    #[test]
    #[should_panic]
    fn braced_too_small() {
        Uuid::nil().braced().encode_lower(&mut [0; 37]);
    }
    #[test]
    fn hyphenated_to_inner() {
        let hyphenated = Uuid::nil().hyphenated();
        assert_eq!(Uuid::from(hyphenated), Uuid::nil());
    }
    #[test]
    fn simple_to_inner() {
        let simple = Uuid::nil().simple();
        assert_eq!(Uuid::from(simple), Uuid::nil());
    }
    #[test]
    fn urn_to_inner() {
        let urn = Uuid::nil().urn();
        assert_eq!(Uuid::from(urn), Uuid::nil());
    }
    #[test]
    fn braced_to_inner() {
        let braced = Uuid::nil().braced();
        assert_eq!(Uuid::from(braced), Uuid::nil());
    }
}
#[cfg(test)]
mod tests_llm_16_4_llm_16_4 {
    use crate::fmt::Braced;
    use crate::Uuid;
    use std::borrow::Borrow;
    #[test]
    fn test_braced_borrow() {
        let _rug_st_tests_llm_16_4_llm_16_4_rrrruuuugggg_test_braced_borrow = 0;
        let uuid = Uuid::nil();
        let braced = Braced::from(uuid);
        let borrowed_uuid: &Uuid = braced.borrow();
        debug_assert_eq!(borrowed_uuid, & uuid);
        let _rug_ed_tests_llm_16_4_llm_16_4_rrrruuuugggg_test_braced_borrow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_5_llm_16_5 {
    use crate::fmt::Braced;
    use crate::Uuid;
    use std::convert::AsRef;
    #[test]
    fn test_as_ref() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let braced = Braced::from_uuid(uuid);
        let uuid_ref: &Uuid = braced.as_ref();
        debug_assert_eq!(uuid_ref, & uuid);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_6 {
    use super::*;
    use crate::*;
    use crate::fmt::Braced;
    use crate::Uuid;
    use std::convert::From;
    #[test]
    fn test_braced_from_uuid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let braced: Braced = Braced::from(uuid);
        let expected = rug_fuzz_1;
        debug_assert_eq!(expected, braced.to_string());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_7 {
    use super::*;
    use crate::*;
    use std::borrow::Borrow;
    #[test]
    fn borrow_returns_correct_uuid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let hyphenated = uuid.hyphenated();
        debug_assert_eq!(Borrow:: < Uuid > ::borrow(& hyphenated), & uuid);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_8 {
    use super::*;
    use crate::*;
    use crate::{fmt::Hyphenated, Uuid};
    #[test]
    fn as_ref_returns_correct_uuid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let hyphenated = Hyphenated::from_uuid(uuid);
        debug_assert_eq!(hyphenated.as_ref(), & uuid);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_9 {
    use std::convert::From;
    use crate::fmt::Hyphenated;
    use crate::Uuid;
    #[test]
    fn test_hyphenated_from_uuid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let hyphenated: Hyphenated = Hyphenated::from(uuid);
        debug_assert_eq!(
            Hyphenated::LENGTH, hyphenated.encode_lower(& mut Uuid::encode_buffer())
            .len()
        );
        debug_assert_eq!(
            rug_fuzz_1, hyphenated.encode_lower(& mut Uuid::encode_buffer())
        );
             }
});    }
    #[test]
    fn test_hyphenated_uppercase() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let hyphenated = Hyphenated::from(uuid);
        debug_assert_eq!(
            rug_fuzz_1, hyphenated.encode_upper(& mut Uuid::encode_buffer())
        );
             }
});    }
    #[test]
    fn test_hyphenated_as_uuid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let hyphenated = Hyphenated::from(uuid);
        debug_assert_eq!(& uuid, hyphenated.as_uuid());
             }
});    }
    #[test]
    fn test_hyphenated_into_uuid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let hyphenated = Hyphenated::from(uuid);
        debug_assert_eq!(uuid, hyphenated.into_uuid());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_10_llm_16_10 {
    use crate::fmt::Simple;
    use crate::Uuid;
    use std::borrow::Borrow;
    #[test]
    fn test_borrow_simple() {
        let _rug_st_tests_llm_16_10_llm_16_10_rrrruuuugggg_test_borrow_simple = 0;
        let simple = Simple::from_uuid(Uuid::nil());
        let borrowed_uuid: &Uuid = simple.borrow();
        debug_assert_eq!(* borrowed_uuid, Uuid::nil());
        let _rug_ed_tests_llm_16_10_llm_16_10_rrrruuuugggg_test_borrow_simple = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_11 {
    use crate::fmt::Simple;
    use crate::Uuid;
    use std::convert::AsRef;
    #[test]
    fn test_as_ref_returns_correct_uuid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let simple = Simple::from_uuid(uuid);
        let uuid_ref: &Uuid = simple.as_ref();
        debug_assert_eq!(uuid, * uuid_ref);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_14 {
    use super::*;
    use crate::*;
    use std::convert::AsRef;
    use crate::fmt::Urn;
    use crate::Uuid;
    #[test]
    fn as_ref_returns_correct_uuid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let urn = Urn::from_uuid(uuid);
        debug_assert_eq!(AsRef:: < Uuid > ::as_ref(& urn), & uuid);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_15 {
    use super::*;
    use crate::*;
    use std::convert::From;
    #[test]
    fn test_from_uuid_to_urn() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let urn: Urn = Urn::from(uuid);
        let expected_urn_str = rug_fuzz_1;
        debug_assert_eq!(urn.to_string(), expected_urn_str);
             }
});    }
}
#[cfg(test)]
mod as_braced_tests {
    use super::*;
    use crate::*;
    use crate::fmt::Braced;
    use crate::Uuid;
    use std::str::FromStr;
    #[test]
    fn as_braced_returns_braced_formatter() {
        let uuid = Uuid::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let braced = uuid.as_braced();
        let expected = Braced::from_uuid(uuid);
        assert_eq!(* braced, expected);
    }
    #[test]
    fn as_braced_formatter_to_string_matches_braced_output() {
        let uuid = Uuid::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let braced = uuid.as_braced();
        let braced_string = braced.to_string();
        let expected = "{550e8400-e29b-41d4-a716-446655440000}";
        assert_eq!(braced_string, expected);
    }
    #[test]
    fn as_braced_references_same_uuid() {
        let uuid = Uuid::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let braced = uuid.as_braced();
        assert!(std::ptr::eq(braced.as_uuid(), & uuid));
    }
}
#[cfg(test)]
mod tests_llm_16_64 {
    use super::*;
    use crate::*;
    use crate::{
        fmt::{Braced, Hyphenated, Simple, Urn},
        Bytes, Uuid, Version,
    };
    #[test]
    fn test_as_simple() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let simple_formatter = uuid.as_simple();
        let uuid_from_simple = simple_formatter.as_uuid();
        debug_assert_eq!(& uuid, uuid_from_simple);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_65 {
    use super::*;
    use crate::*;
    use crate::fmt::Urn;
    use crate::Uuid;
    #[test]
    fn test_as_urn() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::nil();
        let urn = uuid.as_urn();
        debug_assert_eq!(
            urn.to_string(), "urn:uuid:00000000-0000-0000-0000-000000000000"
        );
        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let urn = uuid.as_urn();
        debug_assert_eq!(
            urn.to_string(), "urn:uuid:550e8400-e29b-41d4-a716-446655440000"
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_66_llm_16_66 {
    use super::*;
    use crate::*;
    use crate::fmt::Braced;
    use crate::Uuid;
    #[test]
    fn test_braced() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let braced = uuid.braced();
        debug_assert_eq!(* braced.as_uuid(), uuid);
        let expected_str = rug_fuzz_1;
        debug_assert_eq!(braced.to_string(), expected_str);
        let mut buffer = Uuid::encode_buffer();
        debug_assert_eq!(
            braced.encode_lower(& mut buffer).as_mut(), expected_str.to_lowercase()
            .as_str()
        );
        debug_assert_eq!(
            braced.encode_upper(& mut buffer).as_mut(), expected_str.to_uppercase()
            .as_str()
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_67 {
    use super::*;
    use crate::*;
    #[test]
    fn test_hyphenated() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let hyphenated = uuid.hyphenated();
        let hyphenated_str = hyphenated.to_string();
        debug_assert_eq!(hyphenated_str, "550e8400-e29b-41d4-a716-446655440000");
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_68 {
    use super::*;
    use crate::*;
    use crate::fmt::Simple;
    use crate::Uuid;
    #[test]
    fn test_simple_formatter() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let simple = uuid.simple();
        debug_assert_eq!(Simple::from_uuid(uuid), simple);
        debug_assert_eq!(Simple::LENGTH, 32);
        debug_assert_eq!(format!("{}", simple), "67e5504410b1426f9247bb680e5fe0c8");
        debug_assert_eq!(format!("{:X}", simple), "67E5504410B1426F9247BB680E5FE0C8");
        debug_assert_eq!(format!("{:x}", simple), "67e5504410b1426f9247bb680e5fe0c8");
        debug_assert_eq!(
            simple.encode_lower(& mut Uuid::encode_buffer()),
            "67e5504410b1426f9247bb680e5fe0c8"
        );
        debug_assert_eq!(
            simple.encode_upper(& mut Uuid::encode_buffer()),
            "67E5504410B1426F9247BB680E5FE0C8"
        );
        debug_assert_eq!(simple.as_uuid(), & uuid);
        debug_assert_eq!(simple.into_uuid(), uuid);
             }
});    }
    #[test]
    fn test_simple_formatter_length() {
        let _rug_st_tests_llm_16_68_rrrruuuugggg_test_simple_formatter_length = 0;
        debug_assert_eq!(Simple::LENGTH, 32);
        let _rug_ed_tests_llm_16_68_rrrruuuugggg_test_simple_formatter_length = 0;
    }
    #[test]
    fn test_simple_formatter_encode_lower() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let simple = uuid.simple();
        let mut buffer = Uuid::encode_buffer();
        debug_assert_eq!(
            simple.encode_lower(& mut buffer), "67e5504410b1426f9247bb680e5fe0c8"
        );
             }
});    }
    #[test]
    fn test_simple_formatter_encode_upper() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let simple = uuid.simple();
        let mut buffer = Uuid::encode_buffer();
        debug_assert_eq!(
            simple.encode_upper(& mut buffer), "67E5504410B1426F9247BB680E5FE0C8"
        );
             }
});    }
    #[test]
    fn test_simple_from_and_into_uuid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let simple = Simple::from_uuid(uuid);
        debug_assert_eq!(simple.into_uuid(), uuid);
             }
});    }
    #[test]
    fn test_simple_as_uuid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let simple = Simple::from_uuid(uuid);
        debug_assert_eq!(simple.as_uuid(), & uuid);
             }
});    }
    #[test]
    fn test_simple_display() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let simple = Simple::from_uuid(uuid);
        debug_assert_eq!(format!("{}", simple), "67e5504410b1426f9247bb680e5fe0c8");
             }
});    }
    #[test]
    fn test_simple_lower_hex() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let simple = Simple::from_uuid(uuid);
        debug_assert_eq!(format!("{:x}", simple), "67e5504410b1426f9247bb680e5fe0c8");
             }
});    }
    #[test]
    fn test_simple_upper_hex() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let simple = Simple::from_uuid(uuid);
        debug_assert_eq!(format!("{:X}", simple), "67E5504410B1426F9247BB680E5FE0C8");
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_70 {
    use super::*;
    use crate::*;
    use crate::fmt::Braced;
    use crate::Uuid;
    fn create_test_uuid() -> Uuid {
        Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8").unwrap()
    }
    #[test]
    fn test_from_braced_to_uuid() {
        let uuid = create_test_uuid();
        let braced = Braced::from_uuid(uuid);
        let uuid_from_braced = Uuid::from(braced);
        assert_eq!(uuid_from_braced, uuid);
    }
    #[test]
    fn test_braced_into_uuid() {
        let uuid = create_test_uuid();
        let braced = Braced::from_uuid(uuid);
        let uuid_from_braced = braced.into_uuid();
        assert_eq!(uuid_from_braced, uuid);
    }
}
#[cfg(test)]
mod tests_llm_16_71 {
    use super::*;
    use crate::*;
    use crate::fmt::Hyphenated;
    use crate::Uuid;
    use std::str::FromStr;
    #[test]
    fn test_from_hyphenated() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let hyphen_string = rug_fuzz_0;
        let hyphenated = Hyphenated::from_uuid(Uuid::from_str(hyphen_string).unwrap());
        let uuid: Uuid = Uuid::from(hyphenated);
        debug_assert_eq!(uuid.to_string(), hyphen_string);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_72 {
    use crate::fmt::Simple;
    use crate::Uuid;
    #[test]
    fn test_from_simple() {
        let _rug_st_tests_llm_16_72_rrrruuuugggg_test_from_simple = 0;
        let simple = Simple::from_uuid(Uuid::nil());
        let uuid_from_simple = Uuid::from(simple);
        debug_assert_eq!(uuid_from_simple, Uuid::nil());
        let _rug_ed_tests_llm_16_72_rrrruuuugggg_test_from_simple = 0;
    }
    #[test]
    fn test_from_simple_hex() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let simple = Simple::from_uuid(uuid);
        let uuid_from_simple = Uuid::from(simple);
        debug_assert_eq!(uuid_from_simple, uuid);
             }
});    }
    #[test]
    fn test_from_simple_uppercase() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let simple = Simple::from_uuid(uuid);
        let uuid_from_simple = Uuid::from(simple);
        debug_assert_eq!(uuid_from_simple, uuid);
             }
});    }
    #[test]
    fn test_from_simple_hyphenated() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let simple = Simple::from_uuid(uuid);
        let uuid_from_simple = Uuid::from(simple);
        debug_assert_eq!(uuid_from_simple, uuid);
             }
});    }
    #[test]
    fn test_from_simple_braced() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let simple = Simple::from_uuid(uuid);
        let uuid_from_simple = Uuid::from(simple);
        debug_assert_eq!(uuid_from_simple, uuid);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_73 {
    use crate::{fmt::Urn, Uuid, Error};
    #[test]
    fn test_urn_to_uuid_conversion() {
        let uuid_str = "a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8";
        let uuid = Uuid::parse_str(uuid_str).expect("Failed to parse UUID");
        let urn = Urn::from(uuid);
        let converted_uuid = Uuid::from(urn);
        assert_eq!(uuid, converted_uuid, "Converted UUID should match original");
    }
    #[test]
    fn test_uuid_from_urn() -> Result<(), Error> {
        let expected_uuid_str = "67e55044-10b1-426f-9247-bb680e5fe0c8";
        let uuid = Uuid::parse_str(expected_uuid_str)?;
        let urn = Urn::from_uuid(uuid);
        let result_uuid = Uuid::from(urn);
        assert_eq!(uuid, result_uuid);
        Ok(())
    }
    #[test]
    fn test_uuid_from_urn_default() {
        let urn = Urn::default();
        let uuid_from_urn = Uuid::from(urn);
        let default_uuid = Uuid::default();
        assert_eq!(
            uuid_from_urn, default_uuid,
            "UUID converted from default Urn should match Uuid::default()"
        );
    }
    #[test]
    fn test_uuid_from_urn_nil() {
        let urn = Urn::from_uuid(Uuid::nil());
        let uuid_from_urn = Uuid::from(urn);
        assert!(
            uuid_from_urn.is_nil(), "UUID converted from Urn with NIL UUID should be nil"
        );
    }
    #[test]
    fn test_uuid_from_custom_urn() {
        let custom_uuid = Uuid::from_bytes([
            0x12,
            0x34,
            0x56,
            0x78,
            0x90,
            0xab,
            0xcd,
            0xef,
            0x01,
            0x23,
            0x45,
            0x67,
            0x89,
            0xab,
            0xcd,
            0xef,
        ]);
        let urn = Urn::from_uuid(custom_uuid);
        let uuid_from_urn = Uuid::from(urn);
        assert_eq!(
            custom_uuid, uuid_from_urn,
            "UUID converted from custom Urn should match original custom UUID"
        );
    }
}
#[cfg(test)]
mod tests_llm_16_74_llm_16_74 {
    use crate::fmt::Braced;
    use crate::Uuid;
    #[test]
    fn as_uuid_returns_correct_uuid() {
        let _rug_st_tests_llm_16_74_llm_16_74_rrrruuuugggg_as_uuid_returns_correct_uuid = 0;
        let uuid = Uuid::nil();
        let braced = Braced::from_uuid(uuid);
        debug_assert_eq!(* braced.as_uuid(), uuid);
        let _rug_ed_tests_llm_16_74_llm_16_74_rrrruuuugggg_as_uuid_returns_correct_uuid = 0;
    }
    #[test]
    fn braced_as_uuid_returns_the_same_as_direct() {
        let _rug_st_tests_llm_16_74_llm_16_74_rrrruuuugggg_braced_as_uuid_returns_the_same_as_direct = 0;
        let uuid = Uuid::nil();
        let braced = Braced::from_uuid(uuid);
        debug_assert_eq!(* braced.as_uuid(), uuid);
        let _rug_ed_tests_llm_16_74_llm_16_74_rrrruuuugggg_braced_as_uuid_returns_the_same_as_direct = 0;
    }
    #[test]
    fn as_uuid_is_consistent_with_direct_uuid() {
        let _rug_st_tests_llm_16_74_llm_16_74_rrrruuuugggg_as_uuid_is_consistent_with_direct_uuid = 0;
        let uuid1 = Uuid::nil();
        let braced = Braced::from_uuid(uuid1);
        let uuid2 = braced.as_uuid();
        debug_assert_eq!(uuid1, * uuid2);
        let _rug_ed_tests_llm_16_74_llm_16_74_rrrruuuugggg_as_uuid_is_consistent_with_direct_uuid = 0;
    }
    #[test]
    fn as_uuid_is_consistent_with_from_str() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid_str = rug_fuzz_0;
        let uuid_direct = Uuid::parse_str(uuid_str).unwrap();
        let braced = Braced::from_uuid(uuid_direct);
        debug_assert_eq!(* braced.as_uuid(), uuid_direct);
             }
});    }
    #[test]
    fn as_uuid_is_consistent_with_from_fields() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(u32, u16, u16, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid_fields = Uuid::from_fields(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            &[rug_fuzz_3; 8],
        );
        let braced = Braced::from_uuid(uuid_fields);
        debug_assert_eq!(* braced.as_uuid(), uuid_fields);
             }
});    }
    #[test]
    fn as_uuid_returns_correct_uuid_for_non_nil() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let non_nil_uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let braced = Braced::from_uuid(non_nil_uuid);
        debug_assert_eq!(* braced.as_uuid(), non_nil_uuid);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_75_llm_16_75 {
    use crate::{*, fmt::Braced};
    use std::panic::AssertUnwindSafe;
    #[test]
    fn test_encode_lower() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let mut buffer = Uuid::encode_buffer();
        let encoded_uuid = uuid.braced().encode_lower(&mut buffer);
        debug_assert_eq!(encoded_uuid, "{550e8400-e29b-41d4-a716-446655440000}");
             }
});    }
    #[test]
    fn test_encode_lower_buffer_too_small() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let mut buffer = [rug_fuzz_1; Braced::LENGTH - 1];
        let result = std::panic::catch_unwind(
            AssertUnwindSafe(|| {
                uuid.braced().encode_lower(&mut buffer);
            }),
        );
        debug_assert!(result.is_err());
             }
});    }
    #[test]
    fn test_encode_lower_buffer_exactly_sized() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let mut buffer = [rug_fuzz_1; Braced::LENGTH];
        let encoded_uuid = uuid.braced().encode_lower(&mut buffer);
        debug_assert_eq!(encoded_uuid, "{550e8400-e29b-41d4-a716-446655440000}");
             }
});    }
    #[test]
    fn test_encode_lower_buffer_larger_than_needed() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let mut buffer = [rug_fuzz_1; Braced::LENGTH + 4];
        let encoded_uuid = uuid.braced().encode_lower(&mut buffer);
        debug_assert_eq!(encoded_uuid, "{550e8400-e29b-41d4-a716-446655440000}");
        debug_assert_eq!(& buffer[Braced::LENGTH..], & [0u8; 4]);
             }
});    }
    #[test]
    fn test_encode_lower_with_additional_content_in_buffer() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let mut buffer = [rug_fuzz_1; 42];
        let encoded_uuid = uuid.braced().encode_lower(&mut buffer);
        debug_assert_eq!(encoded_uuid, "{550e8400-e29b-41d4-a716-446655440000}");
        debug_assert_eq!(& buffer[Braced::LENGTH..], b"!!!!");
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_76 {
    use super::*;
    use crate::*;
    use crate::fmt::Braced;
    #[test]
    fn test_encode_upper() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let braced = Braced::from_uuid(uuid);
        let mut buffer = Uuid::encode_buffer();
        let result = braced.encode_upper(&mut buffer);
        debug_assert_eq!(result, "{936DA01F-9ABD-4D9D-80C7-02AF85C822A8}");
             }
});    }
    #[test]
    #[should_panic(expected = "buffer is not large enough")]
    fn test_encode_upper_panics_when_buffer_too_small() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let braced = Braced::from_uuid(uuid);
        let mut buffer = [rug_fuzz_1; Braced::LENGTH - 1];
        braced.encode_upper(&mut buffer);
             }
});    }
    #[test]
    fn test_encode_upper_with_sufficient_buffer() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let braced = Braced::from_uuid(uuid);
        let mut buffer = [rug_fuzz_1; 40];
        braced.encode_upper(&mut buffer);
        debug_assert_eq!(
            & buffer[..Braced::LENGTH], b"{936DA01F-9ABD-4D9D-80C7-02AF85C822A8}"
        );
        debug_assert_eq!(& buffer[Braced::LENGTH..], & [0u8; 40 - Braced::LENGTH]);
             }
});    }
    #[test]
    fn test_encode_upper_with_trailing_contents() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let braced = Braced::from_uuid(uuid);
        let mut buffer = [rug_fuzz_1; 40];
        braced.encode_upper(&mut buffer);
        debug_assert_eq!(
            & buffer[..Braced::LENGTH], b"{936DA01F-9ABD-4D9D-80C7-02AF85C822A8}"
        );
        debug_assert_eq!(& buffer[Braced::LENGTH..], b"!!" as & [u8]);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_77_llm_16_77 {
    use crate::fmt::Braced;
    use crate::Uuid;
    #[cfg(feature = "v4")]
    #[test]
    fn test_from_uuid_creates_proper_braced() {
        let _rug_st_tests_llm_16_77_llm_16_77_rrrruuuugggg_test_from_uuid_creates_proper_braced = 0;
        let rug_fuzz_0 = "67e55044-10b1-426f-9247-bb680e5fe0c8";
        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let braced = Braced::from_uuid(uuid);
        debug_assert_eq!(braced.to_string(), "{67e55044-10b1-426f-9247-bb680e5fe0c8}");
        let _rug_ed_tests_llm_16_77_llm_16_77_rrrruuuugggg_test_from_uuid_creates_proper_braced = 0;
    }
    #[cfg(feature = "v4")]
    #[test]
    fn test_from_uuid_creates_braced_with_same_uuid() {
        let _rug_st_tests_llm_16_77_llm_16_77_rrrruuuugggg_test_from_uuid_creates_braced_with_same_uuid = 0;
        let uuid = Uuid::new_v4();
        let braced = Braced::from_uuid(uuid);
        debug_assert_eq!(braced.as_uuid(), & uuid);
        let _rug_ed_tests_llm_16_77_llm_16_77_rrrruuuugggg_test_from_uuid_creates_braced_with_same_uuid = 0;
    }
    #[cfg(feature = "v4")]
    #[test]
    fn test_from_uuid_empty_braced() {
        let _rug_st_tests_llm_16_77_llm_16_77_rrrruuuugggg_test_from_uuid_empty_braced = 0;
        let uuid = Uuid::nil();
        let braced = Braced::from_uuid(uuid);
        debug_assert_eq!(braced.to_string(), "{00000000-0000-0000-0000-000000000000}");
        let _rug_ed_tests_llm_16_77_llm_16_77_rrrruuuugggg_test_from_uuid_empty_braced = 0;
    }
    #[cfg(feature = "v4")]
    #[test]
    fn test_from_uuid_uppercase_braced() {
        let _rug_st_tests_llm_16_77_llm_16_77_rrrruuuugggg_test_from_uuid_uppercase_braced = 0;
        let rug_fuzz_0 = "67e55044-10b1-426f-9247-bb680e5fe0c8";
        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let braced = Braced::from_uuid(uuid);
        debug_assert_eq!(
            format!("{:X}", braced), "{67E55044-10B1-426F-9247-BB680E5FE0C8}"
        );
        let _rug_ed_tests_llm_16_77_llm_16_77_rrrruuuugggg_test_from_uuid_uppercase_braced = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_78 {
    use super::*;
    use crate::*;
    use crate::fmt::Braced;
    use crate::Uuid;
    use std::str::FromStr;
    #[test]
    fn braced_into_uuid_returns_correct_uuid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let known_uuid_str = rug_fuzz_0;
        let known_uuid = Uuid::from_str(known_uuid_str).unwrap();
        let braced = Braced::from_uuid(known_uuid);
        let uuid_from_braced = braced.into_uuid();
        debug_assert_eq!(uuid_from_braced, known_uuid);
             }
});    }
    #[test]
    fn braced_into_uuid_returns_nil_uuid() {
        let _rug_st_tests_llm_16_78_rrrruuuugggg_braced_into_uuid_returns_nil_uuid = 0;
        let nil_uuid = Uuid::nil();
        let braced = Braced::from_uuid(nil_uuid);
        let uuid_from_braced = braced.into_uuid();
        debug_assert!(uuid_from_braced.is_nil());
        let _rug_ed_tests_llm_16_78_rrrruuuugggg_braced_into_uuid_returns_nil_uuid = 0;
    }
    #[test]
    fn braced_into_uuid_returns_max_uuid() {
        let _rug_st_tests_llm_16_78_rrrruuuugggg_braced_into_uuid_returns_max_uuid = 0;
        #[cfg(uuid_unstable)]
        {
            let max_uuid = Uuid::max();
            let braced = Braced::from_uuid(max_uuid);
            let uuid_from_braced = braced.into_uuid();
            debug_assert!(uuid_from_braced.is_max());
        }
        let _rug_ed_tests_llm_16_78_rrrruuuugggg_braced_into_uuid_returns_max_uuid = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_79 {
    use super::*;
    use crate::*;
    use crate::{fmt, Uuid};
    #[test]
    fn test_as_uuid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let nil_uuid = Uuid::nil();
        let hyphenated = nil_uuid.hyphenated();
        debug_assert_eq!(* hyphenated.as_uuid(), nil_uuid);
        let uuid_str = rug_fuzz_0;
        let uuid = Uuid::parse_str(uuid_str).unwrap();
        let hyphenated = uuid.hyphenated();
        debug_assert_eq!(* hyphenated.as_uuid(), uuid);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_80 {
    use super::*;
    use crate::*;
    use crate::fmt::Hyphenated;
    #[test]
    fn test_encode_lower() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let hyphenated = Hyphenated::from_uuid(uuid);
        let mut buffer = [rug_fuzz_1; Hyphenated::LENGTH];
        let slice = hyphenated.encode_lower(&mut buffer);
        debug_assert_eq!(slice, "550e8400-e29b-41d4-a716-446655440000");
             }
});    }
    #[test]
    #[should_panic(expected = "buffer is not large enough")]
    fn test_encode_lower_insufficient_buffer() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let hyphenated = Hyphenated::from_uuid(uuid);
        let mut buffer = [rug_fuzz_1; Hyphenated::LENGTH - 1];
        let _ = hyphenated.encode_lower(&mut buffer);
             }
});    }
    #[test]
    fn test_encode_lower_excess_buffer() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let hyphenated = Hyphenated::from_uuid(uuid);
        let mut buffer = [rug_fuzz_1; Hyphenated::LENGTH + 4];
        let slice = hyphenated.encode_lower(&mut buffer);
        debug_assert_eq!(slice, "550e8400-e29b-41d4-a716-446655440000");
        debug_assert_eq!(& buffer[Hyphenated::LENGTH..], b"!!!!");
             }
});    }
    #[test]
    fn test_encode_lower_with_uppercase_input() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let hyphenated = Hyphenated::from_uuid(uuid);
        let mut buffer = [rug_fuzz_1; Hyphenated::LENGTH];
        let slice = hyphenated.encode_lower(&mut buffer);
        debug_assert_eq!(slice, "550e8400-e29b-41d4-a716-446655440000");
             }
});    }
    #[test]
    fn test_encode_lower_with_hyphenated_input() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let hyphenated = Hyphenated::from_uuid(uuid);
        let mut buffer = [rug_fuzz_1; Hyphenated::LENGTH];
        let slice = hyphenated.encode_lower(&mut buffer);
        debug_assert_eq!(slice, "550e8400-e29b-41d4-a716-446655440000");
             }
});    }
    #[test]
    fn test_encode_lower_nil_uuid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::nil();
        let hyphenated = Hyphenated::from_uuid(uuid);
        let mut buffer = [rug_fuzz_0; Hyphenated::LENGTH];
        let slice = hyphenated.encode_lower(&mut buffer);
        debug_assert_eq!(slice, "00000000-0000-0000-0000-000000000000");
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_81_llm_16_81 {
    use crate::fmt::{self, Hyphenated};
    #[test]
    fn test_encode_upper() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = crate::Uuid::parse_str(rug_fuzz_0).unwrap();
        let mut buffer = crate::Uuid::encode_buffer();
        let result = uuid.hyphenated().encode_upper(&mut buffer);
        debug_assert_eq!(result, "550E8400-E29B-41D4-A716-446655440000");
             }
});    }
    #[test]
    #[should_panic(expected = "buffer is not large enough")]
    fn test_encode_upper_buffer_too_small() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = crate::Uuid::parse_str(rug_fuzz_0).unwrap();
        let mut buffer = [rug_fuzz_1; 10];
        let _ = uuid.hyphenated().encode_upper(&mut buffer);
             }
});    }
    #[test]
    fn test_encode_upper_no_panic_on_exact_size_buffer() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = crate::Uuid::parse_str(rug_fuzz_0).unwrap();
        let mut buffer = [rug_fuzz_1; Hyphenated::LENGTH];
        let result = uuid.hyphenated().encode_upper(&mut buffer);
        debug_assert_eq!(result, "550E8400-E29B-41D4-A716-446655440000");
             }
});    }
    #[test]
    fn test_encode_upper_no_panic_on_larger_buffer() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = crate::Uuid::parse_str(rug_fuzz_0).unwrap();
        let mut buffer = [rug_fuzz_1; Hyphenated::LENGTH + 10];
        let result = uuid.hyphenated().encode_upper(&mut buffer);
        debug_assert_eq!(result, "550E8400-E29B-41D4-A716-446655440000");
             }
});    }
    #[test]
    fn test_encode_upper_no_panic_on_buffer_fill() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = crate::Uuid::parse_str(rug_fuzz_0).unwrap();
        let mut buffer = [rug_fuzz_1; Hyphenated::LENGTH + 10];
        let result = uuid.hyphenated().encode_upper(&mut buffer);
        debug_assert_eq!(result, "550E8400-E29B-41D4-A716-446655440000");
        debug_assert_eq!(& buffer[Hyphenated::LENGTH..], & [b'!'; 10]);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_82_llm_16_82 {
    use crate::fmt::Hyphenated;
    use crate::Uuid;
    #[test]
    fn test_from_uuid_creates_hyphenated() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let hyphenated = Hyphenated::from_uuid(uuid);
        debug_assert_eq!(hyphenated.to_string(), "67e55044-10b1-426f-9247-bb680e5fe0c8");
             }
});    }
    #[test]
    fn test_from_uuid_creates_hyphenated_matches_direct_call() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let hyphenated_from_uuid = Hyphenated::from_uuid(uuid);
        let hyphenated_direct = uuid.hyphenated();
        debug_assert_eq!(hyphenated_from_uuid, hyphenated_direct);
             }
});    }
    #[test]
    #[should_panic(expected = "buffer is not large enough")]
    fn test_from_uuid_encoding_panic_on_small_buffer() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let hyphenated = Hyphenated::from_uuid(uuid);
        let small_buffer = &mut [rug_fuzz_1; Hyphenated::LENGTH - 1];
        hyphenated.encode_lower(small_buffer);
             }
});    }
    #[test]
    fn test_from_uuid_encoding_with_buffer() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let hyphenated = Hyphenated::from_uuid(uuid);
        let buffer = &mut [rug_fuzz_1; Hyphenated::LENGTH];
        debug_assert_eq!(
            hyphenated.encode_lower(buffer), "67e55044-10b1-426f-9247-bb680e5fe0c8"
        );
             }
});    }
    #[test]
    fn test_from_uuid_uppercase_encoding() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let hyphenated = Hyphenated::from_uuid(uuid);
        debug_assert_eq!(
            hyphenated.encode_upper(& mut Uuid::encode_buffer()),
            "67E55044-10B1-426F-9247-BB680E5FE0C8"
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_83 {
    use super::*;
    use crate::*;
    use crate::fmt::Hyphenated;
    use crate::Uuid;
    #[test]
    fn test_into_uuid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::nil();
        let hyphenated = uuid.hyphenated();
        let result = hyphenated.into_uuid();
        debug_assert_eq!(
            result, uuid, "Hyphenated::into_uuid did not return the original Uuid"
        );
        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let hyphenated = uuid.hyphenated();
        let result = hyphenated.into_uuid();
        debug_assert_eq!(
            result, uuid,
            "Hyphenated::into_uuid did not return the original Uuid for non-nil Uuid"
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_84 {
    use crate::{fmt, Uuid};
    #[test]
    fn simple_as_uuid_test() {
        let _rug_st_tests_llm_16_84_rrrruuuugggg_simple_as_uuid_test = 0;
        let uuid = Uuid::nil();
        let simple = fmt::Simple::from_uuid(uuid);
        debug_assert_eq!(* simple.as_uuid(), uuid);
        let _rug_ed_tests_llm_16_84_rrrruuuugggg_simple_as_uuid_test = 0;
    }
    #[test]
    fn simple_as_uuid_non_nil_test() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let simple = fmt::Simple::from_uuid(uuid);
        debug_assert_eq!(* simple.as_uuid(), uuid);
             }
});    }
    #[test]
    fn simple_as_uuid_equality_test() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid1 = Uuid::parse_str(rug_fuzz_0).unwrap();
        let simple = fmt::Simple::from_uuid(uuid1);
        let uuid2 = *simple.as_uuid();
        debug_assert_eq!(uuid1, uuid2);
             }
});    }
    #[cfg(feature = "v4")]
    #[test]
    fn simple_as_uuid_into_test() {
        let _rug_st_tests_llm_16_84_rrrruuuugggg_simple_as_uuid_into_test = 0;
        let uuid = Uuid::new_v4();
        let simple = fmt::Simple::from_uuid(uuid);
        debug_assert_eq!(simple.into_uuid(), uuid);
        let _rug_ed_tests_llm_16_84_rrrruuuugggg_simple_as_uuid_into_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_85 {
    use super::*;
    use crate::*;
    use crate::fmt::Simple;
    #[test]
    fn test_encode_lower() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let mut buffer = Uuid::encode_buffer();
        let result = uuid.simple().encode_lower(&mut buffer);
        debug_assert_eq!(result, "550e8400e29b41d4a716446655440000");
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_86 {
    use crate::fmt::Simple;
    use crate::Uuid;
    #[test]
    fn test_encode_upper() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let mut buffer = Uuid::encode_buffer();
        let encoded = uuid.simple().encode_upper(&mut buffer);
        debug_assert_eq!(encoded, "936DA01F9ABD4D9D80C702AF85C822A8");
             }
});    }
    #[test]
    #[should_panic(expected = "buffer is not large enough")]
    fn test_encode_upper_insufficient_buffer() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let mut buffer = [rug_fuzz_1; 16];
        let _ = uuid.simple().encode_upper(&mut buffer);
             }
});    }
    #[test]
    fn test_encode_upper_exactly_sized_buffer() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let mut buffer = [rug_fuzz_1; Simple::LENGTH];
        let encoded = uuid.simple().encode_upper(&mut buffer);
        debug_assert_eq!(encoded, "936DA01F9ABD4D9D80C702AF85C822A8");
             }
});    }
    #[test]
    fn test_encode_upper_with_trailing_space() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let mut buffer = [rug_fuzz_1; Simple::LENGTH + 4];
        let encoded = uuid.simple().encode_upper(&mut buffer);
        debug_assert_eq!(encoded, "936DA01F9ABD4D9D80C702AF85C822A8");
        debug_assert_eq!(buffer[rug_fuzz_2..], [0; 4]);
             }
});    }
    #[test]
    fn test_encode_upper_all_lower_hex() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let mut buffer = Uuid::encode_buffer();
        let encoded = uuid.simple().encode_upper(&mut buffer);
        debug_assert_eq!(encoded, "00000000000000000000000000000000");
             }
});    }
    #[test]
    fn test_encode_upper_all_upper_hex() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let mut buffer = Uuid::encode_buffer();
        let encoded = uuid.simple().encode_upper(&mut buffer);
        debug_assert_eq!(encoded, "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
             }
});    }
    #[test]
    fn test_encode_upper_mixed_case() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let mut buffer = Uuid::encode_buffer();
        let encoded = uuid.simple().encode_upper(&mut buffer);
        debug_assert_eq!(encoded, "FFFFFFFF000000007777777788888888");
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_87_llm_16_87 {
    use super::*;
    use crate::*;
    #[test]
    fn test_simple_from_uuid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bytes = [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
            rug_fuzz_11,
            rug_fuzz_12,
            rug_fuzz_13,
            rug_fuzz_14,
            rug_fuzz_15,
        ];
        let uuid = Uuid::from_bytes(bytes);
        let simple = Simple::from_uuid(uuid);
        let mut buffer = Uuid::encode_buffer();
        let simple_str = simple.encode_lower(&mut buffer);
        debug_assert_eq!(simple_str, "10941d4197ee47e6980ad9191a2a3a70");
        debug_assert_eq!(* simple.as_uuid(), uuid);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_88 {
    use super::*;
    use crate::*;
    use crate::fmt::Simple;
    #[test]
    fn test_into_uuid() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let test_uuids = [
            Uuid::nil(),
            Uuid::parse_str(rug_fuzz_0).unwrap(),
            Uuid::parse_str(rug_fuzz_1).unwrap(),
            Uuid::parse_str(rug_fuzz_2).unwrap(),
            Uuid::parse_str(rug_fuzz_3).unwrap(),
        ];
        for &uuid in test_uuids.iter() {
            let simple = Simple::from_uuid(uuid);
            debug_assert_eq!(simple.into_uuid(), uuid);
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_89 {
    use super::*;
    use crate::*;
    use crate::fmt::Urn;
    use crate::Uuid;
    #[test]
    fn test_urn_as_uuid() {
        let _rug_st_tests_llm_16_89_rrrruuuugggg_test_urn_as_uuid = 0;
        let uuid = Uuid::nil();
        let urn = Urn::from_uuid(uuid);
        debug_assert_eq!(urn.as_uuid(), & uuid);
        let _rug_ed_tests_llm_16_89_rrrruuuugggg_test_urn_as_uuid = 0;
    }
    #[test]
    fn test_as_uuid_non_nil() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let urn = Urn::from_uuid(uuid);
        debug_assert_eq!(urn.as_uuid(), & uuid);
             }
});    }
    #[test]
    fn test_as_uuid_random() {
        let _rug_st_tests_llm_16_89_rrrruuuugggg_test_as_uuid_random = 0;
        #[cfg(feature = "v4")]
        {
            let uuid = Uuid::new_v4();
            let urn = Urn::from_uuid(uuid);
            debug_assert_eq!(urn.as_uuid(), & uuid);
        }
        let _rug_ed_tests_llm_16_89_rrrruuuugggg_test_as_uuid_random = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_91 {
    use crate::fmt::Urn;
    use crate::Uuid;
    #[test]
    fn test_encode_upper() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let mut buffer = Uuid::encode_buffer();
        debug_assert_eq!(
            uuid.urn().encode_upper(& mut buffer),
            "urn:uuid:67E55044-10B1-426F-9247-BB680E5FE0C8"
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_92 {
    use super::*;
    use crate::*;
    use crate::fmt::Urn;
    use crate::Uuid;
    #[test]
    fn test_urn_from_uuid() {
        let _rug_st_tests_llm_16_92_rrrruuuugggg_test_urn_from_uuid = 0;
        let uuid = Uuid::nil();
        let urn = Urn::from_uuid(uuid);
        debug_assert_eq!(urn.as_uuid(), & uuid);
        let _rug_ed_tests_llm_16_92_rrrruuuugggg_test_urn_from_uuid = 0;
    }
    #[test]
    fn test_urn_from_uuid_v4() {
        let _rug_st_tests_llm_16_92_rrrruuuugggg_test_urn_from_uuid_v4 = 0;
        #[cfg(feature = "v4")]
        {
            let uuid = Uuid::new_v4();
            let urn = Urn::from_uuid(uuid);
            debug_assert_eq!(urn.as_uuid(), & uuid);
        }
        let _rug_ed_tests_llm_16_92_rrrruuuugggg_test_urn_from_uuid_v4 = 0;
    }
    #[test]
    fn test_urn_from_uuid_to_string() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::parse_str(rug_fuzz_0).unwrap();
        let urn_string = uuid.urn().to_string();
        debug_assert_eq!(urn_string, "urn:uuid:550e8400-e29b-41d4-a716-446655440000");
             }
});    }
    #[test]
    fn test_urn_from_fields() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(u32, u16, u16, u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid = Uuid::from_fields(
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            &[
                rug_fuzz_3,
                rug_fuzz_4,
                rug_fuzz_5,
                rug_fuzz_6,
                rug_fuzz_7,
                rug_fuzz_8,
                rug_fuzz_9,
                rug_fuzz_10,
            ],
        );
        let urn = Urn::from_uuid(uuid);
        debug_assert_eq!(urn.as_uuid(), & uuid);
             }
});    }
    #[test]
    #[cfg(uuid_unstable)]
    fn test_urn_from_uuid_max() {
        let _rug_st_tests_llm_16_92_rrrruuuugggg_test_urn_from_uuid_max = 0;
        let uuid = Uuid::max();
        let urn = Urn::from_uuid(uuid);
        debug_assert_eq!(urn.as_uuid(), & uuid);
        let _rug_ed_tests_llm_16_92_rrrruuuugggg_test_urn_from_uuid_max = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_95 {
    use super::*;
    use crate::*;
    #[test]
    fn test_encode_hyphenated_lower() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, &str, u8, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let src = [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
            rug_fuzz_11,
            rug_fuzz_12,
            rug_fuzz_13,
            rug_fuzz_14,
            rug_fuzz_15,
        ];
        let expected = rug_fuzz_16;
        let mut buffer = [rug_fuzz_17; Hyphenated::LENGTH];
        let encoded = encode_hyphenated(&src, &mut buffer, rug_fuzz_18);
        debug_assert_eq!(encoded, expected);
             }
});    }
    #[test]
    fn test_encode_hyphenated_upper() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, &str, u8, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let src = [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
            rug_fuzz_11,
            rug_fuzz_12,
            rug_fuzz_13,
            rug_fuzz_14,
            rug_fuzz_15,
        ];
        let expected = rug_fuzz_16;
        let mut buffer = [rug_fuzz_17; Hyphenated::LENGTH];
        let encoded = encode_hyphenated(&src, &mut buffer, rug_fuzz_18);
        debug_assert_eq!(encoded, expected);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_97 {
    use super::*;
    use crate::*;
    #[test]
    fn test_encode_urn_lowercase() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let src: [u8; 16] = [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
            rug_fuzz_11,
            rug_fuzz_12,
            rug_fuzz_13,
            rug_fuzz_14,
            rug_fuzz_15,
        ];
        let mut buffer = [rug_fuzz_16; Urn::LENGTH];
        let result = encode_urn(&src, &mut buffer, rug_fuzz_17);
        debug_assert_eq!(result, "urn:uuid:12345678-90ab-cdef-1122-334455667788");
             }
});    }
    #[test]
    fn test_encode_urn_uppercase() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let src: [u8; 16] = [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
            rug_fuzz_11,
            rug_fuzz_12,
            rug_fuzz_13,
            rug_fuzz_14,
            rug_fuzz_15,
        ];
        let mut buffer = [rug_fuzz_16; Urn::LENGTH];
        let result = encode_urn(&src, &mut buffer, rug_fuzz_17);
        debug_assert_eq!(result, "urn:uuid:12345678-90AB-CDEF-1122-334455667788");
             }
});    }
    #[test]
    #[should_panic(expected = "buffer is the correct length")]
    fn test_encode_urn_buffer_too_small() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let src: [u8; 16] = [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
            rug_fuzz_11,
            rug_fuzz_12,
            rug_fuzz_13,
            rug_fuzz_14,
            rug_fuzz_15,
        ];
        let mut buffer = [rug_fuzz_16; Urn::LENGTH - 1];
        let _ = encode_urn(&src, &mut buffer, rug_fuzz_17);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_98_llm_16_98 {
    use crate::fmt::format_hyphenated;
    #[test]
    fn test_format_hyphenated_lowercase() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16_ext, mut rug_fuzz_17)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, [u8; 36], bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_16 = & rug_fuzz_16_ext;
        let src = [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
            rug_fuzz_11,
            rug_fuzz_12,
            rug_fuzz_13,
            rug_fuzz_14,
            rug_fuzz_15,
        ];
        let expected = *rug_fuzz_16;
        let result = format_hyphenated(&src, rug_fuzz_17);
        debug_assert_eq!(result, expected);
             }
});    }
    #[test]
    fn test_format_hyphenated_uppercase() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16_ext, mut rug_fuzz_17)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, [u8; 36], bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_16 = & rug_fuzz_16_ext;
        let src = [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
            rug_fuzz_11,
            rug_fuzz_12,
            rug_fuzz_13,
            rug_fuzz_14,
            rug_fuzz_15,
        ];
        let expected = *rug_fuzz_16;
        let result = format_hyphenated(&src, rug_fuzz_17);
        debug_assert_eq!(result, expected);
             }
});    }
    #[test]
    fn test_format_hyphenated_zeroes() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1_ext, mut rug_fuzz_2)) = <(u8, [u8; 36], bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

let rug_fuzz_1 = & rug_fuzz_1_ext;
        let src = [rug_fuzz_0; 16];
        let expected = *rug_fuzz_1;
        let result = format_hyphenated(&src, rug_fuzz_2);
        debug_assert_eq!(result, expected);
             }
});    }
    #[test]
    fn test_format_hyphenated_mixed_case() {
        let _rug_st_tests_llm_16_98_llm_16_98_rrrruuuugggg_test_format_hyphenated_mixed_case = 0;
        let rug_fuzz_0 = 0x12;
        let rug_fuzz_1 = 0x34;
        let rug_fuzz_2 = 0x56;
        let rug_fuzz_3 = 0x78;
        let rug_fuzz_4 = 0x9a;
        let rug_fuzz_5 = 0xbc;
        let rug_fuzz_6 = 0xde;
        let rug_fuzz_7 = 0xf0;
        let rug_fuzz_8 = 0x12;
        let rug_fuzz_9 = 0x34;
        let rug_fuzz_10 = 0x56;
        let rug_fuzz_11 = 0x78;
        let rug_fuzz_12 = 0x9a;
        let rug_fuzz_13 = 0xbc;
        let rug_fuzz_14 = 0xde;
        let rug_fuzz_15 = 0xf0;
        let rug_fuzz_16 = b"12345678-9abc-def0-1234-56789abcdef0";
        let rug_fuzz_17 = b"12345678-9ABC-DEF0-1234-56789ABCDEF0";
        let rug_fuzz_18 = false;
        let rug_fuzz_19 = true;
        let src = [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
            rug_fuzz_11,
            rug_fuzz_12,
            rug_fuzz_13,
            rug_fuzz_14,
            rug_fuzz_15,
        ];
        let expected_lower = *rug_fuzz_16;
        let expected_upper = *rug_fuzz_17;
        debug_assert_eq!(format_hyphenated(& src, rug_fuzz_18), expected_lower);
        debug_assert_eq!(format_hyphenated(& src, rug_fuzz_19), expected_upper);
        let _rug_ed_tests_llm_16_98_llm_16_98_rrrruuuugggg_test_format_hyphenated_mixed_case = 0;
    }
    #[test]
    fn test_format_hyphenated_boundary_values() {
        let _rug_st_tests_llm_16_98_llm_16_98_rrrruuuugggg_test_format_hyphenated_boundary_values = 0;
        let rug_fuzz_0 = 0x00;
        let rug_fuzz_1 = 0x00;
        let rug_fuzz_2 = 0x00;
        let rug_fuzz_3 = 0x00;
        let rug_fuzz_4 = 0xff;
        let rug_fuzz_5 = 0xff;
        let rug_fuzz_6 = 0xff;
        let rug_fuzz_7 = 0xff;
        let rug_fuzz_8 = 0x00;
        let rug_fuzz_9 = 0x00;
        let rug_fuzz_10 = 0x00;
        let rug_fuzz_11 = 0x00;
        let rug_fuzz_12 = 0xff;
        let rug_fuzz_13 = 0xff;
        let rug_fuzz_14 = 0xff;
        let rug_fuzz_15 = 0xff;
        let rug_fuzz_16 = b"00000000-ffff-ffff-0000-0000ffffffff";
        let rug_fuzz_17 = b"00000000-FFFF-FFFF-0000-0000FFFFFFFF";
        let rug_fuzz_18 = false;
        let rug_fuzz_19 = true;
        let src = [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
            rug_fuzz_11,
            rug_fuzz_12,
            rug_fuzz_13,
            rug_fuzz_14,
            rug_fuzz_15,
        ];
        let expected_lower = *rug_fuzz_16;
        let expected_upper = *rug_fuzz_17;
        debug_assert_eq!(format_hyphenated(& src, rug_fuzz_18), expected_lower);
        debug_assert_eq!(format_hyphenated(& src, rug_fuzz_19), expected_upper);
        let _rug_ed_tests_llm_16_98_llm_16_98_rrrruuuugggg_test_format_hyphenated_boundary_values = 0;
    }
}
#[cfg(test)]
mod tests_rug_6 {
    use super::*;
    use std::str;
    #[test]
    fn test_encode_simple() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [u8; 16] = [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
            rug_fuzz_11,
            rug_fuzz_12,
            rug_fuzz_13,
            rug_fuzz_14,
            rug_fuzz_15,
        ];
        let mut p1: &mut [u8] = &mut [rug_fuzz_16; crate::fmt::Simple::LENGTH];
        let p2: bool = rug_fuzz_17;
        crate::fmt::encode_simple(&p0, p1, p2);
             }
});    }
}
#[cfg(test)]
mod tests_rug_7 {
    use super::*;
    #[test]
    fn test_encode_braced() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [u8; 16] = [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
            rug_fuzz_11,
            rug_fuzz_12,
            rug_fuzz_13,
            rug_fuzz_14,
            rug_fuzz_15,
        ];
        let mut p1: [u8; super::Braced::LENGTH] = [rug_fuzz_16; super::Braced::LENGTH];
        let mut p2: bool = rug_fuzz_17;
        crate::fmt::encode_braced(&mut p0, &mut p1, p2);
             }
});    }
}
#[cfg(test)]
mod tests_rug_8 {
    use crate::fmt::Hyphenated;
    use crate::Uuid;
    #[test]
    fn test_as_hyphenated() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0 = Uuid::nil();
        let hyphenated: &Hyphenated = p0.as_hyphenated();
        let expected = rug_fuzz_0;
        debug_assert_eq!(hyphenated.to_string(), expected);
             }
});    }
}
#[cfg(test)]
mod tests_rug_9 {
    use super::*;
    use crate::Uuid;
    #[test]
    fn test_urn() {
        let _rug_st_tests_rug_9_rrrruuuugggg_test_urn = 0;
        let mut p0 = Uuid::nil();
        let _ = p0.urn();
        let _rug_ed_tests_rug_9_rrrruuuugggg_test_urn = 0;
    }
}
#[cfg(test)]
mod tests_rug_10 {
    use super::*;
    #[test]
    fn test_encode_lower() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Urn::from_uuid(Uuid::nil());
        let mut p1: [u8; 49] = [rug_fuzz_0; 49];
        p0.encode_lower(&mut p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_11 {
    use crate::fmt::Urn;
    use crate::Uuid;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_11_rrrruuuugggg_test_rug = 0;
        let mut p0 = Urn::from_uuid(Uuid::nil());
        debug_assert_eq!(Urn::into_uuid(p0), Uuid::nil());
        let _rug_ed_tests_rug_11_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_12 {
    use super::*;
    use crate::fmt::Simple;
    use crate::Uuid;
    use std::convert::From;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_12_rrrruuuugggg_test_rug = 0;
        let p0 = Uuid::nil();
        let simple: Simple = Simple::from(p0);
        debug_assert_eq!(simple.to_string(), "00000000000000000000000000000000");
        let _rug_ed_tests_rug_12_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_13 {
    use crate::fmt::{Urn, Uuid};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_13_rrrruuuugggg_test_rug = 0;
        let mut p0 = Urn::from_uuid(Uuid::nil());
        debug_assert_eq!(
            < Urn as std::borrow::Borrow < Uuid > > ::borrow(& p0), & Uuid::nil()
        );
        let _rug_ed_tests_rug_13_rrrruuuugggg_test_rug = 0;
    }
}
