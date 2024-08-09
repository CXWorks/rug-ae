//! A Builder type for [`Uuid`]s.
//!
//! [`Uuid`]: ../struct.Uuid.html
use crate::{error::*, timestamp, Bytes, Uuid, Variant, Version};
/// A builder for creating a UUID.
///
/// This type is useful if you need to mutate individual fields of a [`Uuid`]
/// while constructing it. Since the [`Uuid`] type is `Copy`, it doesn't offer
/// any methods to mutate in place. They live on the `Builder` instead.
///
/// The `Builder` type also always exposes APIs to construct [`Uuid`]s for any
/// version without needing crate features or additional dependencies. It's a
/// lower-level API than the methods on [`Uuid`].
///
/// # Examples
///
/// Creating a version 4 UUID from externally generated random bytes:
///
/// ```
/// # use uuid::{Builder, Version, Variant};
/// # let rng = || [
/// #     70, 235, 208, 238, 14, 109, 67, 201, 185, 13, 204, 195, 90,
/// # 145, 63, 62,
/// # ];
/// let random_bytes = rng();
///
/// let uuid = Builder::from_random_bytes(random_bytes).into_uuid();
///
/// assert_eq!(Some(Version::Random), uuid.get_version());
/// assert_eq!(Variant::RFC4122, uuid.get_variant());
/// ```
#[allow(missing_copy_implementations)]
#[derive(Debug)]
pub struct Builder(Uuid);
impl Uuid {
    /// The 'nil UUID' (all zeros).
    ///
    /// The nil UUID is a special form of UUID that is specified to have all
    /// 128 bits set to zero.
    ///
    /// # References
    ///
    /// * [Nil UUID in RFC4122](https://tools.ietf.org/html/rfc4122.html#section-4.1.7)
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use uuid::Uuid;
    /// let uuid = Uuid::nil();
    ///
    /// assert_eq!(
    ///     "00000000-0000-0000-0000-000000000000",
    ///     uuid.hyphenated().to_string(),
    /// );
    /// ```
    pub const fn nil() -> Self {
        Uuid::from_bytes([0; 16])
    }
    /// The 'max UUID' (all ones).
    ///
    /// The max UUID is a special form of UUID that is specified to have all
    /// 128 bits set to one.
    ///
    /// # References
    ///
    /// * [Max UUID in Draft RFC: New UUID Formats, Version 4](https://datatracker.ietf.org/doc/html/draft-peabody-dispatch-new-uuid-format-04#section-5.4)
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use uuid::Uuid;
    /// let uuid = Uuid::max();
    ///
    /// assert_eq!(
    ///     "ffffffff-ffff-ffff-ffff-ffffffffffff",
    ///     uuid.hyphenated().to_string(),
    /// );
    /// ```
    #[cfg(uuid_unstable)]
    pub const fn max() -> Self {
        Uuid::from_bytes([0xFF; 16])
    }
    /// Creates a UUID from four field values.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use uuid::Uuid;
    /// let d1 = 0xa1a2a3a4;
    /// let d2 = 0xb1b2;
    /// let d3 = 0xc1c2;
    /// let d4 = [0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8];
    ///
    /// let uuid = Uuid::from_fields(d1, d2, d3, &d4);
    ///
    /// assert_eq!(
    ///     "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8",
    ///     uuid.hyphenated().to_string(),
    /// );
    /// ```
    pub const fn from_fields(d1: u32, d2: u16, d3: u16, d4: &[u8; 8]) -> Uuid {
        Uuid::from_bytes([
            (d1 >> 24) as u8,
            (d1 >> 16) as u8,
            (d1 >> 8) as u8,
            d1 as u8,
            (d2 >> 8) as u8,
            d2 as u8,
            (d3 >> 8) as u8,
            d3 as u8,
            d4[0],
            d4[1],
            d4[2],
            d4[3],
            d4[4],
            d4[5],
            d4[6],
            d4[7],
        ])
    }
    /// Creates a UUID from four field values in little-endian order.
    ///
    /// The bytes in the `d1`, `d2` and `d3` fields will be flipped to convert
    /// into big-endian order. This is based on the endianness of the UUID,
    /// rather than the target environment so bytes will be flipped on both
    /// big and little endian machines.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use uuid::Uuid;
    /// let d1 = 0xa1a2a3a4;
    /// let d2 = 0xb1b2;
    /// let d3 = 0xc1c2;
    /// let d4 = [0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8];
    ///
    /// let uuid = Uuid::from_fields_le(d1, d2, d3, &d4);
    ///
    /// assert_eq!(
    ///     "a4a3a2a1-b2b1-c2c1-d1d2-d3d4d5d6d7d8",
    ///     uuid.hyphenated().to_string(),
    /// );
    /// ```
    pub const fn from_fields_le(d1: u32, d2: u16, d3: u16, d4: &[u8; 8]) -> Uuid {
        Uuid::from_bytes([
            d1 as u8,
            (d1 >> 8) as u8,
            (d1 >> 16) as u8,
            (d1 >> 24) as u8,
            (d2) as u8,
            (d2 >> 8) as u8,
            d3 as u8,
            (d3 >> 8) as u8,
            d4[0],
            d4[1],
            d4[2],
            d4[3],
            d4[4],
            d4[5],
            d4[6],
            d4[7],
        ])
    }
    /// Creates a UUID from a 128bit value.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use uuid::Uuid;
    /// let v = 0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8u128;
    ///
    /// let uuid = Uuid::from_u128(v);
    ///
    /// assert_eq!(
    ///     "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8",
    ///     uuid.hyphenated().to_string(),
    /// );
    /// ```
    pub const fn from_u128(v: u128) -> Self {
        Uuid::from_bytes([
            (v >> 120) as u8,
            (v >> 112) as u8,
            (v >> 104) as u8,
            (v >> 96) as u8,
            (v >> 88) as u8,
            (v >> 80) as u8,
            (v >> 72) as u8,
            (v >> 64) as u8,
            (v >> 56) as u8,
            (v >> 48) as u8,
            (v >> 40) as u8,
            (v >> 32) as u8,
            (v >> 24) as u8,
            (v >> 16) as u8,
            (v >> 8) as u8,
            v as u8,
        ])
    }
    /// Creates a UUID from a 128bit value in little-endian order.
    ///
    /// The entire value will be flipped to convert into big-endian order.
    /// This is based on the endianness of the UUID, rather than the target
    /// environment so bytes will be flipped on both big and little endian
    /// machines.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use uuid::Uuid;
    /// let v = 0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8u128;
    ///
    /// let uuid = Uuid::from_u128_le(v);
    ///
    /// assert_eq!(
    ///     "d8d7d6d5-d4d3-d2d1-c2c1-b2b1a4a3a2a1",
    ///     uuid.hyphenated().to_string(),
    /// );
    /// ```
    pub const fn from_u128_le(v: u128) -> Self {
        Uuid::from_bytes([
            v as u8,
            (v >> 8) as u8,
            (v >> 16) as u8,
            (v >> 24) as u8,
            (v >> 32) as u8,
            (v >> 40) as u8,
            (v >> 48) as u8,
            (v >> 56) as u8,
            (v >> 64) as u8,
            (v >> 72) as u8,
            (v >> 80) as u8,
            (v >> 88) as u8,
            (v >> 96) as u8,
            (v >> 104) as u8,
            (v >> 112) as u8,
            (v >> 120) as u8,
        ])
    }
    /// Creates a UUID from two 64bit values.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use uuid::Uuid;
    /// let hi = 0xa1a2a3a4b1b2c1c2u64;
    /// let lo = 0xd1d2d3d4d5d6d7d8u64;
    ///
    /// let uuid = Uuid::from_u64_pair(hi, lo);
    ///
    /// assert_eq!(
    ///     "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8",
    ///     uuid.hyphenated().to_string(),
    /// );
    /// ```
    pub const fn from_u64_pair(high_bits: u64, low_bits: u64) -> Self {
        Uuid::from_bytes([
            (high_bits >> 56) as u8,
            (high_bits >> 48) as u8,
            (high_bits >> 40) as u8,
            (high_bits >> 32) as u8,
            (high_bits >> 24) as u8,
            (high_bits >> 16) as u8,
            (high_bits >> 8) as u8,
            high_bits as u8,
            (low_bits >> 56) as u8,
            (low_bits >> 48) as u8,
            (low_bits >> 40) as u8,
            (low_bits >> 32) as u8,
            (low_bits >> 24) as u8,
            (low_bits >> 16) as u8,
            (low_bits >> 8) as u8,
            low_bits as u8,
        ])
    }
    /// Creates a UUID using the supplied bytes.
    ///
    /// # Errors
    ///
    /// This function will return an error if `b` has any length other than 16.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # fn main() -> Result<(), uuid::Error> {
    /// # use uuid::Uuid;
    /// let bytes = [
    ///     0xa1, 0xa2, 0xa3, 0xa4,
    ///     0xb1, 0xb2,
    ///     0xc1, 0xc2,
    ///     0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8,
    /// ];
    ///
    /// let uuid = Uuid::from_slice(&bytes)?;
    ///
    /// assert_eq!(
    ///     "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8",
    ///     uuid.hyphenated().to_string(),
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_slice(b: &[u8]) -> Result<Uuid, Error> {
        if b.len() != 16 {
            return Err(
                Error(ErrorKind::ByteLength {
                    len: b.len(),
                }),
            );
        }
        let mut bytes: Bytes = [0; 16];
        bytes.copy_from_slice(b);
        Ok(Uuid::from_bytes(bytes))
    }
    /// Creates a UUID using the supplied bytes in little endian order.
    ///
    /// The individual fields encoded in the buffer will be flipped.
    ///
    /// # Errors
    ///
    /// This function will return an error if `b` has any length other than 16.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # fn main() -> Result<(), uuid::Error> {
    /// # use uuid::Uuid;
    /// let bytes = [
    ///     0xa1, 0xa2, 0xa3, 0xa4,
    ///     0xb1, 0xb2,
    ///     0xc1, 0xc2,
    ///     0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8,
    /// ];
    ///
    /// let uuid = Uuid::from_slice_le(&bytes)?;
    ///
    /// assert_eq!(
    ///     uuid.hyphenated().to_string(),
    ///     "a4a3a2a1-b2b1-c2c1-d1d2-d3d4d5d6d7d8"
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_slice_le(b: &[u8]) -> Result<Uuid, Error> {
        if b.len() != 16 {
            return Err(
                Error(ErrorKind::ByteLength {
                    len: b.len(),
                }),
            );
        }
        let mut bytes: Bytes = [0; 16];
        bytes.copy_from_slice(b);
        Ok(Uuid::from_bytes_le(bytes))
    }
    /// Creates a UUID using the supplied bytes.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # fn main() -> Result<(), uuid::Error> {
    /// # use uuid::Uuid;
    /// let bytes = [
    ///     0xa1, 0xa2, 0xa3, 0xa4,
    ///     0xb1, 0xb2,
    ///     0xc1, 0xc2,
    ///     0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8,
    /// ];
    ///
    /// let uuid = Uuid::from_bytes(bytes);
    ///
    /// assert_eq!(
    ///     uuid.hyphenated().to_string(),
    ///     "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8"
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub const fn from_bytes(bytes: Bytes) -> Uuid {
        Uuid(bytes)
    }
    /// Creates a UUID using the supplied bytes in little endian order.
    ///
    /// The individual fields encoded in the buffer will be flipped.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # fn main() -> Result<(), uuid::Error> {
    /// # use uuid::Uuid;
    /// let bytes = [
    ///     0xa1, 0xa2, 0xa3, 0xa4,
    ///     0xb1, 0xb2,
    ///     0xc1, 0xc2,
    ///     0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8,
    /// ];
    ///
    /// let uuid = Uuid::from_bytes_le(bytes);
    ///
    /// assert_eq!(
    ///     "a4a3a2a1-b2b1-c2c1-d1d2-d3d4d5d6d7d8",
    ///     uuid.hyphenated().to_string(),
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub const fn from_bytes_le(b: Bytes) -> Uuid {
        Uuid([
            b[3],
            b[2],
            b[1],
            b[0],
            b[5],
            b[4],
            b[7],
            b[6],
            b[8],
            b[9],
            b[10],
            b[11],
            b[12],
            b[13],
            b[14],
            b[15],
        ])
    }
    /// Creates a reference to a UUID from a reference to the supplied bytes.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # fn main() -> Result<(), uuid::Error> {
    /// # use uuid::Uuid;
    /// let bytes = [
    ///     0xa1, 0xa2, 0xa3, 0xa4,
    ///     0xb1, 0xb2,
    ///     0xc1, 0xc2,
    ///     0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8,
    /// ];
    ///
    /// let uuid = Uuid::from_bytes_ref(&bytes);
    ///
    /// assert_eq!(
    ///     uuid.hyphenated().to_string(),
    ///     "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8"
    /// );
    ///
    /// assert!(std::ptr::eq(
    ///     uuid as *const Uuid as *const u8,
    ///     &bytes as *const [u8; 16] as *const u8,
    /// ));
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_bytes_ref(bytes: &Bytes) -> &Uuid {
        unsafe { &*(bytes as *const Bytes as *const Uuid) }
    }
}
impl Builder {
    /// Creates a `Builder` using the supplied bytes.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use uuid::Builder;
    /// let bytes = [
    ///     0xa1, 0xa2, 0xa3, 0xa4,
    ///     0xb1, 0xb2,
    ///     0xc1, 0xc2,
    ///     0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8,
    /// ];
    ///
    /// let uuid = Builder::from_bytes(bytes).into_uuid();
    ///
    /// assert_eq!(
    ///     "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8",
    ///     uuid.hyphenated().to_string(),
    /// );
    /// ```
    pub const fn from_bytes(b: Bytes) -> Self {
        Builder(Uuid::from_bytes(b))
    }
    /// Creates a `Builder` using the supplied bytes in little endian order.
    ///
    /// The individual fields encoded in the buffer will be flipped.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # fn main() -> Result<(), uuid::Error> {
    /// # use uuid::{Builder, Uuid};
    /// let bytes = [
    ///     0xa1, 0xa2, 0xa3, 0xa4,
    ///     0xb1, 0xb2,
    ///     0xc1, 0xc2,
    ///     0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8,
    /// ];
    ///
    /// let uuid = Builder::from_bytes_le(bytes).into_uuid();
    ///
    /// assert_eq!(
    ///     "a4a3a2a1-b2b1-c2c1-d1d2-d3d4d5d6d7d8",
    ///     uuid.hyphenated().to_string(),
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub const fn from_bytes_le(b: Bytes) -> Self {
        Builder(Uuid::from_bytes_le(b))
    }
    /// Creates a `Builder` for a version 1 UUID using the supplied timestamp and node ID.
    pub const fn from_rfc4122_timestamp(
        ticks: u64,
        counter: u16,
        node_id: &[u8; 6],
    ) -> Self {
        Builder(timestamp::encode_rfc4122_timestamp(ticks, counter, node_id))
    }
    /// Creates a `Builder` for a version 3 UUID using the supplied MD5 hashed bytes.
    pub const fn from_md5_bytes(md5_bytes: Bytes) -> Self {
        Builder(Uuid::from_bytes(md5_bytes))
            .with_variant(Variant::RFC4122)
            .with_version(Version::Md5)
    }
    /// Creates a `Builder` for a version 4 UUID using the supplied random bytes.
    ///
    /// This method assumes the bytes are already sufficiently random, it will only
    /// set the appropriate bits for the UUID version and variant.
    ///
    /// # Examples
    ///
    /// ```
    /// # use uuid::{Builder, Variant, Version};
    /// # let rng = || [
    /// #     70, 235, 208, 238, 14, 109, 67, 201, 185, 13, 204, 195, 90,
    /// # 145, 63, 62,
    /// # ];
    /// let random_bytes = rng();
    /// let uuid = Builder::from_random_bytes(random_bytes).into_uuid();
    ///
    /// assert_eq!(Some(Version::Random), uuid.get_version());
    /// assert_eq!(Variant::RFC4122, uuid.get_variant());
    /// ```
    pub const fn from_random_bytes(random_bytes: Bytes) -> Self {
        Builder(Uuid::from_bytes(random_bytes))
            .with_variant(Variant::RFC4122)
            .with_version(Version::Random)
    }
    /// Creates a `Builder` for a version 5 UUID using the supplied SHA-1 hashed bytes.
    ///
    /// This method assumes the bytes are already a SHA-1 hash, it will only set the appropriate
    /// bits for the UUID version and variant.
    pub const fn from_sha1_bytes(sha1_bytes: Bytes) -> Self {
        Builder(Uuid::from_bytes(sha1_bytes))
            .with_variant(Variant::RFC4122)
            .with_version(Version::Sha1)
    }
    /// Creates a `Builder` for a version 6 UUID using the supplied timestamp and node ID.
    ///
    /// This method will encode the ticks, counter, and node ID in a sortable UUID.
    #[cfg(uuid_unstable)]
    pub const fn from_sorted_rfc4122_timestamp(
        ticks: u64,
        counter: u16,
        node_id: &[u8; 6],
    ) -> Self {
        Builder(timestamp::encode_sorted_rfc4122_timestamp(ticks, counter, node_id))
    }
    /// Creates a `Builder` for a version 7 UUID using the supplied Unix timestamp and random bytes.
    ///
    /// This method assumes the bytes are already sufficiently random.
    ///
    /// # Examples
    ///
    /// Creating a UUID using the current system timestamp:
    ///
    /// ```
    /// # use std::convert::TryInto;
    /// use std::time::{Duration, SystemTime};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use uuid::{Builder, Uuid, Variant, Version, Timestamp, NoContext};
    /// # let rng = || [
    /// #     70, 235, 208, 238, 14, 109, 67, 201, 185, 13
    /// # ];
    /// let ts = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    ///
    /// let random_bytes = rng();
    ///
    /// let uuid = Builder::from_unix_timestamp_millis(ts.as_millis().try_into()?, &random_bytes).into_uuid();
    ///
    /// assert_eq!(Some(Version::SortRand), uuid.get_version());
    /// assert_eq!(Variant::RFC4122, uuid.get_variant());
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(uuid_unstable)]
    pub const fn from_unix_timestamp_millis(
        millis: u64,
        random_bytes: &[u8; 10],
    ) -> Self {
        Builder(timestamp::encode_unix_timestamp_millis(millis, random_bytes))
    }
    /// Creates a `Builder` for a version 8 UUID using the supplied user-defined bytes.
    ///
    /// This method won't interpret the given bytes in any way, except to set the appropriate
    /// bits for the UUID version and variant.
    #[cfg(uuid_unstable)]
    pub const fn from_custom_bytes(custom_bytes: Bytes) -> Self {
        Builder::from_bytes(custom_bytes)
            .with_variant(Variant::RFC4122)
            .with_version(Version::Custom)
    }
    /// Creates a `Builder` using the supplied bytes.
    ///
    /// # Errors
    ///
    /// This function will return an error if `b` has any length other than 16.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use uuid::Builder;
    /// # fn main() -> Result<(), uuid::Error> {
    /// let bytes = [
    ///     0xa1, 0xa2, 0xa3, 0xa4,
    ///     0xb1, 0xb2,
    ///     0xc1, 0xc2,
    ///     0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8,
    /// ];
    ///
    /// let uuid = Builder::from_slice(&bytes)?.into_uuid();
    ///
    /// assert_eq!(
    ///     "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8",
    ///     uuid.hyphenated().to_string(),
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_slice(b: &[u8]) -> Result<Self, Error> {
        Ok(Builder(Uuid::from_slice(b)?))
    }
    /// Creates a `Builder` using the supplied bytes in little endian order.
    ///
    /// The individual fields encoded in the buffer will be flipped.
    ///
    /// # Errors
    ///
    /// This function will return an error if `b` has any length other than 16.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use uuid::Builder;
    /// # fn main() -> Result<(), uuid::Error> {
    /// let bytes = [
    ///     0xa1, 0xa2, 0xa3, 0xa4,
    ///     0xb1, 0xb2,
    ///     0xc1, 0xc2,
    ///     0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8,
    /// ];
    ///
    /// let uuid = Builder::from_slice_le(&bytes)?.into_uuid();
    ///
    /// assert_eq!(
    ///     "a4a3a2a1-b2b1-c2c1-d1d2-d3d4d5d6d7d8",
    ///     uuid.hyphenated().to_string(),
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_slice_le(b: &[u8]) -> Result<Self, Error> {
        Ok(Builder(Uuid::from_slice_le(b)?))
    }
    /// Creates a `Builder` from four field values.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use uuid::Builder;
    /// let d1 = 0xa1a2a3a4;
    /// let d2 = 0xb1b2;
    /// let d3 = 0xc1c2;
    /// let d4 = [0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8];
    ///
    /// let uuid = Builder::from_fields(d1, d2, d3, &d4).into_uuid();
    ///
    /// assert_eq!(
    ///     uuid.hyphenated().to_string(),
    ///     "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8"
    /// );
    /// ```
    pub const fn from_fields(d1: u32, d2: u16, d3: u16, d4: &[u8; 8]) -> Self {
        Builder(Uuid::from_fields(d1, d2, d3, d4))
    }
    /// Creates a `Builder` from four field values.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use uuid::Builder;
    /// let d1 = 0xa1a2a3a4;
    /// let d2 = 0xb1b2;
    /// let d3 = 0xc1c2;
    /// let d4 = [0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8];
    ///
    /// let uuid = Builder::from_fields_le(d1, d2, d3, &d4).into_uuid();
    ///
    /// assert_eq!(
    ///     uuid.hyphenated().to_string(),
    ///     "a4a3a2a1-b2b1-c2c1-d1d2-d3d4d5d6d7d8"
    /// );
    /// ```
    pub const fn from_fields_le(d1: u32, d2: u16, d3: u16, d4: &[u8; 8]) -> Self {
        Builder(Uuid::from_fields_le(d1, d2, d3, d4))
    }
    /// Creates a `Builder` from a 128bit value.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use uuid::Builder;
    /// let v = 0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8u128;
    ///
    /// let uuid = Builder::from_u128(v).into_uuid();
    ///
    /// assert_eq!(
    ///     "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8",
    ///     uuid.hyphenated().to_string(),
    /// );
    /// ```
    pub const fn from_u128(v: u128) -> Self {
        Builder(Uuid::from_u128(v))
    }
    /// Creates a UUID from a 128bit value in little-endian order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use uuid::Builder;
    /// let v = 0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8u128;
    ///
    /// let uuid = Builder::from_u128_le(v).into_uuid();
    ///
    /// assert_eq!(
    ///     "d8d7d6d5-d4d3-d2d1-c2c1-b2b1a4a3a2a1",
    ///     uuid.hyphenated().to_string(),
    /// );
    /// ```
    pub const fn from_u128_le(v: u128) -> Self {
        Builder(Uuid::from_u128_le(v))
    }
    /// Creates a `Builder` with an initial [`Uuid::nil`].
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use uuid::Builder;
    /// let uuid = Builder::nil().into_uuid();
    ///
    /// assert_eq!(
    ///     "00000000-0000-0000-0000-000000000000",
    ///     uuid.hyphenated().to_string(),
    /// );
    /// ```
    pub const fn nil() -> Self {
        Builder(Uuid::nil())
    }
    /// Specifies the variant of the UUID.
    pub fn set_variant(&mut self, v: Variant) -> &mut Self {
        *self = Builder(self.0).with_variant(v);
        self
    }
    /// Specifies the variant of the UUID.
    pub const fn with_variant(mut self, v: Variant) -> Self {
        let byte = (self.0).0[8];
        (self.0)
            .0[8] = match v {
            Variant::NCS => byte & 0x7f,
            Variant::RFC4122 => (byte & 0x3f) | 0x80,
            Variant::Microsoft => (byte & 0x1f) | 0xc0,
            Variant::Future => byte | 0xe0,
        };
        self
    }
    /// Specifies the version number of the UUID.
    pub fn set_version(&mut self, v: Version) -> &mut Self {
        *self = Builder(self.0).with_version(v);
        self
    }
    /// Specifies the version number of the UUID.
    pub const fn with_version(mut self, v: Version) -> Self {
        (self.0).0[6] = ((self.0).0[6] & 0x0f) | ((v as u8) << 4);
        self
    }
    /// Get a reference to the underlying [`Uuid`].
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use uuid::Builder;
    /// let builder = Builder::nil();
    ///
    /// let uuid1 = builder.as_uuid();
    /// let uuid2 = builder.as_uuid();
    ///
    /// assert_eq!(uuid1, uuid2);
    /// ```
    pub const fn as_uuid(&self) -> &Uuid {
        &self.0
    }
    /// Convert the builder into a [`Uuid`].
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use uuid::Builder;
    /// let uuid = Builder::nil().into_uuid();
    ///
    /// assert_eq!(
    ///     uuid.hyphenated().to_string(),
    ///     "00000000-0000-0000-0000-000000000000"
    /// );
    /// ```
    pub const fn into_uuid(self) -> Uuid {
        self.0
    }
}
#[cfg(test)]
mod tests_llm_16_31 {
    use crate::Uuid;
    #[test]
    fn test_from_bytes() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
        let uuid_string = uuid.hyphenated().to_string();
        debug_assert_eq!(rug_fuzz_16, uuid_string);
             }
});    }
    #[test]
    fn test_from_bytes_round_trip() {

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
        let uuid_bytes = uuid.into_bytes();
        debug_assert_eq!(bytes, uuid_bytes);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_32 {
    use crate::Uuid;
    #[test]
    fn test_from_bytes_le() {

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
        let uuid = Uuid::from_bytes_le(bytes);
        debug_assert_eq!(uuid.to_string(), "a4a3a2a1-b2b1-c2c1-d1d2-d3d4d5d6d7d8");
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_33 {
    use crate::{builder::Bytes, Uuid};
    #[test]
    fn test_from_bytes_ref() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bytes: Bytes = [
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
        let uuid_ref = Uuid::from_bytes_ref(&bytes);
        let uuid = Uuid::from_bytes(bytes);
        let uuid_expected_str = rug_fuzz_16;
        debug_assert_eq!(
            * uuid_ref, uuid,
            "The reference UUID does not match the one created from bytes directly"
        );
        debug_assert_eq!(
            uuid_ref.as_hyphenated().to_string(), uuid_expected_str,
            "The UUID string does not match the expected hyphenated format"
        );
        debug_assert_eq!(
            uuid_ref as * const Uuid as * const u8, & bytes as * const Bytes as * const
            u8, "The UUID and bytes do not point to the same address"
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_34 {
    use super::*;
    use crate::*;
    #[test]
    fn test_from_fields() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(u32, u16, u16, u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let d1 = rug_fuzz_0;
        let d2 = rug_fuzz_1;
        let d3 = rug_fuzz_2;
        let d4 = [
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
        ];
        let uuid = Uuid::from_fields(d1, d2, d3, &d4);
        debug_assert_eq!(
            uuid.hyphenated().to_string(), "12345678-1234-1234-1234-567890abcdef"
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_35 {
    use crate::Uuid;
    #[test]
    fn test_from_fields_le() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(u32, u16, u16, u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let d1 = rug_fuzz_0;
        let d2 = rug_fuzz_1;
        let d3 = rug_fuzz_2;
        let d4 = [
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
        ];
        let uuid = Uuid::from_fields_le(d1, d2, d3, &d4);
        debug_assert_eq!(uuid.as_fields(), (0x78563412, 0x3412, 0x7856, & d4,));
        debug_assert_eq!(uuid.to_string(), "78563412-3412-7856-90ab-cdef01234567");
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_37_llm_16_37 {
    use super::*;
    use crate::*;
    use crate::error::ErrorKind::ByteLength;
    #[test]
    fn test_from_slice_le_correct_length() {

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
        let result = Uuid::from_slice_le(&bytes);
        debug_assert!(result.is_ok());
        let uuid = result.unwrap();
        debug_assert_eq!(
            uuid.hyphenated().to_string(), "a4a3a2a1-b2b1-c2c1-d1d2-d3d4d5d6d7d8"
        );
             }
});    }
    #[test]
    fn test_from_slice_le_incorrect_length() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
        ];
        let result = Uuid::from_slice_le(&bytes);
        debug_assert!(result.is_err());
        let error = result.unwrap_err();
        debug_assert!(matches!(error, Error(ByteLength { len : 14 })));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_38_llm_16_38 {
    use crate::Uuid;
    #[test]
    fn test_from_u128() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u128, u128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid_val: u128 = rug_fuzz_0;
        let uuid = Uuid::from_u128(uuid_val);
        debug_assert_eq!(
            uuid.hyphenated().to_string(), "12345678-90ab-cdef-1234-567890abcdef"
        );
        let uuid_val: u128 = rug_fuzz_1;
        let uuid = Uuid::from_u128(uuid_val);
        debug_assert!(uuid.is_nil());
        #[cfg(uuid_unstable)]
        {
            let uuid_val: u128 = u128::MAX;
            let uuid = Uuid::from_u128(uuid_val);
            debug_assert!(uuid.is_max());
        }
             }
});    }
    #[test]
    fn test_from_u128_le() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u128, u128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid_val: u128 = rug_fuzz_0;
        let uuid = Uuid::from_u128_le(uuid_val);
        debug_assert_eq!(
            uuid.hyphenated().to_string(), "efcdab9078563412efcdab9078563412"
        );
        let uuid_val: u128 = rug_fuzz_1;
        let uuid = Uuid::from_u128_le(uuid_val);
        debug_assert!(uuid.is_nil());
        #[cfg(uuid_unstable)]
        {
            let uuid_val: u128 = u128::MAX;
            let uuid = Uuid::from_u128_le(uuid_val);
            debug_assert!(uuid.is_max());
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_39 {
    use super::*;
    use crate::*;
    #[test]
    fn test_from_u128_le() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u128, u128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let expected = Uuid::from_u128(rug_fuzz_1);
        let result = Uuid::from_u128_le(input);
        debug_assert_eq!(
            result, expected,
            "from_u128_le should convert a 128-bit u128 integer in little-endian to a Uuid by flipping its bytes"
        );
             }
});    }
    #[test]
    fn test_from_u128_le_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input = rug_fuzz_0;
        let expected = Uuid::nil();
        let result = Uuid::from_u128_le(input);
        debug_assert_eq!(
            result, expected,
            "from_u128_le should convert a 128-bit u128 zero integer to a nil Uuid"
        );
             }
});    }
    #[test]
    fn test_from_u128_le_max() {
        let _rug_st_tests_llm_16_39_rrrruuuugggg_test_from_u128_le_max = 0;
        let input = u128::MAX;
        let expected = Uuid::from_u128(u128::MAX);
        let result = Uuid::from_u128_le(input);
        debug_assert_eq!(
            result, expected,
            "from_u128_le should convert a 128-bit u128 max integer to max Uuid by flipping its bytes"
        );
        let _rug_ed_tests_llm_16_39_rrrruuuugggg_test_from_u128_le_max = 0;
    }
    #[test]
    fn test_from_u128_le_endianness() {

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
        let input = u128::from_le_bytes(bytes);
        let expected = Uuid::from_bytes(bytes);
        let result = Uuid::from_u128_le(input);
        debug_assert_eq!(
            result, expected, "from_u128_le should properly handle endianness"
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_40 {
    use super::*;
    use crate::*;
    #[test]
    fn test_from_u64_pair() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u64, u64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let high = rug_fuzz_0;
        let low = rug_fuzz_1;
        let expected = rug_fuzz_2;
        let uuid = Uuid::from_u64_pair(high, low);
        debug_assert_eq!(expected, uuid.to_string());
             }
});    }
    #[test]
    fn test_from_u64_pair_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u64, u64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let high = rug_fuzz_0;
        let low = rug_fuzz_1;
        let expected = rug_fuzz_2;
        let uuid = Uuid::from_u64_pair(high, low);
        debug_assert_eq!(expected, uuid.to_string());
             }
});    }
    #[test]
    fn test_from_u64_pair_max() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let high = u64::MAX;
        let low = u64::MAX;
        let expected = rug_fuzz_0;
        let uuid = Uuid::from_u64_pair(high, low);
        debug_assert_eq!(expected, uuid.to_string());
             }
});    }
    #[test]
    fn test_from_u64_pair_endianess() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u64, u64, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let high = rug_fuzz_0;
        let low = rug_fuzz_1;
        let expected = rug_fuzz_2;
        let uuid = Uuid::from_u64_pair(high, low);
        debug_assert_eq!(expected, uuid.to_string());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_41 {
    use super::*;
    use crate::*;
    #[test]
    fn test_nil_uuid() {
        let _rug_st_tests_llm_16_41_rrrruuuugggg_test_nil_uuid = 0;
        let nil_uuid = Uuid::nil();
        debug_assert_eq!(nil_uuid.to_string(), "00000000-0000-0000-0000-000000000000");
        debug_assert!(nil_uuid.is_nil());
        debug_assert_eq!(nil_uuid.as_bytes(), & [0; 16]);
        debug_assert_eq!(nil_uuid.as_u128(), 0);
        debug_assert_eq!(nil_uuid.as_u64_pair(), (0, 0));
        debug_assert_eq!(nil_uuid.get_version(), Some(Version::Nil));
        debug_assert_eq!(nil_uuid.get_variant(), Variant::RFC4122);
        let _rug_ed_tests_llm_16_41_rrrruuuugggg_test_nil_uuid = 0;
    }
    #[test]
    fn test_nil_uuid_is_default() {
        let _rug_st_tests_llm_16_41_rrrruuuugggg_test_nil_uuid_is_default = 0;
        let default_uuid = Uuid::default();
        debug_assert_eq!(
            default_uuid.to_string(), "00000000-0000-0000-0000-000000000000"
        );
        debug_assert!(default_uuid.is_nil());
        let _rug_ed_tests_llm_16_41_rrrruuuugggg_test_nil_uuid_is_default = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_42 {
    use super::*;
    use crate::*;
    use crate::builder::Builder;
    use crate::Uuid;
    #[test]
    fn test_as_uuid_returns_correct_uuid() {

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
        let builder = Builder::from_bytes(bytes);
        let uuid = builder.as_uuid();
        let expected_uuid = Uuid::from_bytes(bytes);
        debug_assert_eq!(uuid, & expected_uuid);
             }
});    }
    #[test]
    fn test_as_uuid_returns_consistent_references() {
        let _rug_st_tests_llm_16_42_rrrruuuugggg_test_as_uuid_returns_consistent_references = 0;
        let builder = Builder::nil();
        let uuid1 = builder.as_uuid();
        let uuid2 = builder.as_uuid();
        debug_assert_eq!(uuid1, uuid2);
        debug_assert!(
            std::ptr::eq(uuid1, uuid2), "References should point to the same Uuid"
        );
        let _rug_ed_tests_llm_16_42_rrrruuuugggg_test_as_uuid_returns_consistent_references = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_43 {
    use crate::Builder;
    #[test]
    fn test_from_bytes() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

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
        let expected = rug_fuzz_16;
        let builder = Builder::from_bytes(bytes);
        let uuid = builder.into_uuid();
        debug_assert_eq!(expected, uuid.hyphenated().to_string());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_44 {
    use crate::Builder;
    use crate::Bytes;
    #[test]
    fn test_builder_from_bytes_le() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bytes: Bytes = [
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
        let builder = Builder::from_bytes_le(bytes);
        let uuid = builder.into_uuid();
        debug_assert_eq!(
            uuid.hyphenated().to_string(), "2a019410-ed0b-5f11-8523-27fc156b5c5f"
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_45 {
    use super::*;
    use crate::*;
    #[test]
    fn test_from_fields() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(u32, u16, u16, u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let d1 = rug_fuzz_0;
        let d2 = rug_fuzz_1;
        let d3 = rug_fuzz_2;
        let d4 = [
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
        ];
        let builder = Builder::from_fields(d1, d2, d3, &d4);
        let uuid = builder.into_uuid();
        debug_assert_eq!(uuid.as_fields(), (d1, d2, d3, & d4));
        debug_assert_eq!(
            uuid.hyphenated().to_string(), "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8"
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_46 {
    use crate::builder::Builder;
    use crate::Uuid;
    #[test]
    fn test_from_fields_le() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(u32, u16, u16, u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let d1 = rug_fuzz_0;
        let d2 = rug_fuzz_1;
        let d3 = rug_fuzz_2;
        let d4 = [
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
        ];
        let builder = Builder::from_fields_le(d1, d2, d3, &d4);
        let uuid = builder.into_uuid();
        debug_assert_eq!(
            uuid.as_fields(), (0x78563412, 0x3412, 0x3412, & [0x12, 0x34, 0x56, 0x78,
            0x90, 0xAB, 0xCD, 0xEF])
        );
        debug_assert_eq!(uuid.to_string(), "78563412-3412-3412-1234-567890abcdef");
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_47 {
    use super::*;
    use crate::*;
    #[test]
    fn test_from_md5_bytes() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let md5_bytes: crate::Bytes = [
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
        let md5_builder = Builder::from_md5_bytes(md5_bytes);
        let uuid = md5_builder.into_uuid();
        debug_assert_eq!(uuid.get_version(), Some(crate ::Version::Md5));
        debug_assert_eq!(uuid.get_variant(), crate ::Variant::RFC4122);
        debug_assert_eq!(
            uuid.as_bytes(), & [0xd4, 0x1d, 0x8c, 0xd9, 0x8f, 0x00, 0xb2, 0x04, 0xe9,
            0x80, 0x09, 0x98, 0xec, 0xf8, 0x42, 0x7e,]
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_48 {
    use crate::Builder;
    use crate::{Bytes, Variant, Version};
    #[test]
    fn test_builder_from_random_bytes() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let random_bytes: Bytes = [
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
        let builder = Builder::from_random_bytes(random_bytes);
        let uuid = builder.into_uuid();
        debug_assert_eq!(uuid.get_version(), Some(Version::Random));
        debug_assert_eq!(uuid.get_variant(), Variant::RFC4122);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_49 {
    use super::*;
    use crate::*;
    use crate::builder::Builder;
    #[test]
    fn test_from_rfc4122_timestamp() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18, mut rug_fuzz_19, mut rug_fuzz_20, mut rug_fuzz_21, mut rug_fuzz_22, mut rug_fuzz_23)) = <(u64, u16, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let ticks = rug_fuzz_0;
        let counter = rug_fuzz_1;
        let node_id = [
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
        ];
        let builder = Builder::from_rfc4122_timestamp(ticks, counter, &node_id);
        let uuid = builder.into_uuid();
        let expected_bytes = [
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
            rug_fuzz_11,
            rug_fuzz_12,
            rug_fuzz_13,
            rug_fuzz_14,
            rug_fuzz_15,
            rug_fuzz_16,
            rug_fuzz_17,
            rug_fuzz_18,
            rug_fuzz_19,
            rug_fuzz_20,
            rug_fuzz_21,
            rug_fuzz_22,
            rug_fuzz_23,
        ];
        debug_assert_eq!(
            uuid.as_bytes(), & expected_bytes,
            "UUID bytes do not match expected RFC 4122 encoding"
        );
        let uuid_variant = uuid.get_variant();
        debug_assert_eq!(uuid_variant, Variant::RFC4122, "UUID variant is not RFC4122");
        let uuid_version = uuid.get_version();
        debug_assert_eq!(
            uuid_version, Some(Version::Mac), "UUID version is not MAC (Version 1)"
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_50_llm_16_50 {
    use crate::{Builder, Bytes, Variant, Version};
    #[test]
    fn test_from_sha1_bytes() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let sha1_bytes: Bytes = [
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
        let builder = Builder::from_sha1_bytes(sha1_bytes);
        let uuid = builder.into_uuid();
        debug_assert_eq!(uuid.get_version(), Some(Version::Sha1));
        debug_assert_eq!(uuid.get_variant(), Variant::RFC4122);
        debug_assert_eq!(uuid.as_bytes(), & sha1_bytes);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_52_llm_16_52 {
    use crate::builder::Builder;
    use crate::Error;
    #[test]
    fn test_builder_from_slice_le_correct_length() -> Result<(), Error> {
        let bytes = [
            0xa1,
            0xa2,
            0xa3,
            0xa4,
            0xb1,
            0xb2,
            0xc1,
            0xc2,
            0xd1,
            0xd2,
            0xd3,
            0xd4,
            0xd5,
            0xd6,
            0xd7,
            0xd8,
        ];
        let builder = Builder::from_slice_le(&bytes)?;
        let uuid = builder.into_uuid();
        assert_eq!(
            "a4a3a2a1-b2b1-c2c1-d1d2-d3d4d5d6d7d8", uuid.hyphenated().to_string(),
        );
        Ok(())
    }
    #[test]
    fn test_builder_from_slice_le_incorrect_length() {
        let bytes = [
            0xa1,
            0xa2,
            0xa3,
            0xa4,
            0xb1,
            0xb2,
            0xc1,
            0xc2,
            0xd1,
            0xd2,
            0xd3,
            0xd4,
            0xd5,
            0xd6,
            0xd7,
        ];
        assert!(Builder::from_slice_le(& bytes).is_err());
    }
}
#[cfg(test)]
mod tests_llm_16_53 {
    use super::*;
    use crate::*;
    use crate::builder::Builder;
    use crate::Uuid;
    #[test]
    fn test_builder_from_u128() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u128) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let input: u128 = rug_fuzz_0;
        let builder = Builder::from_u128(input);
        let uuid = builder.into_uuid();
        debug_assert_eq!(
            uuid.hyphenated().to_string(), "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8"
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_54 {
    use crate::builder::Builder;
    use crate::Uuid;
    #[test]
    fn test_from_u128_le() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(u128, &str, u128, &str, u128, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let test_cast_values: [(u128, &str); 3] = [
            (rug_fuzz_0, rug_fuzz_1),
            (rug_fuzz_2, rug_fuzz_3),
            (rug_fuzz_4, rug_fuzz_5),
        ];
        for (input, expected) in test_cast_values {
            let uuid = Builder::from_u128_le(input).into_uuid();
            let uuid_string = uuid.hyphenated().to_string();
            debug_assert_eq!(uuid_string, expected);
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_55 {
    use super::*;
    use crate::*;
    #[test]
    fn test_into_uuid_nil() {
        let _rug_st_tests_llm_16_55_rrrruuuugggg_test_into_uuid_nil = 0;
        let builder = Builder::nil();
        let uuid = builder.into_uuid();
        debug_assert_eq!(uuid, Uuid::nil());
        let _rug_ed_tests_llm_16_55_rrrruuuugggg_test_into_uuid_nil = 0;
    }
    #[test]
    fn test_into_uuid_from_bytes() {

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
        let builder = Builder::from_bytes(bytes);
        let uuid = builder.into_uuid();
        debug_assert_eq!(uuid, Uuid::from_bytes(bytes));
             }
});    }
    #[test]
    fn test_into_uuid_with_variant() {
        let _rug_st_tests_llm_16_55_rrrruuuugggg_test_into_uuid_with_variant = 0;
        let builder = Builder::nil().with_variant(Variant::Microsoft);
        let uuid = builder.into_uuid();
        debug_assert_eq!(uuid.get_variant(), Variant::Microsoft);
        let _rug_ed_tests_llm_16_55_rrrruuuugggg_test_into_uuid_with_variant = 0;
    }
    #[test]
    fn test_into_uuid_with_version() {
        let _rug_st_tests_llm_16_55_rrrruuuugggg_test_into_uuid_with_version = 0;
        let builder = Builder::nil().with_version(Version::Md5);
        let uuid = builder.into_uuid();
        debug_assert_eq!(uuid.get_version(), Some(Version::Md5));
        let _rug_ed_tests_llm_16_55_rrrruuuugggg_test_into_uuid_with_version = 0;
    }
    #[test]
    fn test_into_uuid_from_fields() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(u32, u16, u16, u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let d1 = rug_fuzz_0;
        let d2 = rug_fuzz_1;
        let d3 = rug_fuzz_2;
        let d4 = [
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
            rug_fuzz_9,
            rug_fuzz_10,
        ];
        let builder = Builder::from_fields(d1, d2, d3, &d4);
        let uuid = builder.into_uuid();
        debug_assert_eq!(uuid, Uuid::from_fields(d1, d2, d3, & d4));
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_56 {
    use crate::Builder;
    #[test]
    fn test_builder_nil() {
        let _rug_st_tests_llm_16_56_rrrruuuugggg_test_builder_nil = 0;
        let builder = Builder::nil();
        let uuid = builder.into_uuid();
        debug_assert_eq!(uuid.to_string(), "00000000-0000-0000-0000-000000000000");
        let _rug_ed_tests_llm_16_56_rrrruuuugggg_test_builder_nil = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_57 {
    use super::*;
    use crate::*;
    use crate::{builder::Builder, Variant, Version};
    #[test]
    fn test_builder_set_variant() {
        let _rug_st_tests_llm_16_57_rrrruuuugggg_test_builder_set_variant = 0;
        let mut builder = Builder::nil();
        builder.set_variant(Variant::RFC4122);
        let uuid = builder.into_uuid();
        debug_assert_eq!(uuid.get_variant(), Variant::RFC4122);
        let _rug_ed_tests_llm_16_57_rrrruuuugggg_test_builder_set_variant = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_58_llm_16_58 {
    use super::*;
    use crate::*;
    use crate::{Builder, Version};
    #[test]
    fn test_set_version() {
        let _rug_st_tests_llm_16_58_llm_16_58_rrrruuuugggg_test_set_version = 0;
        let mut builder = Builder::nil();
        builder.set_version(Version::Random);
        let uuid = builder.into_uuid();
        debug_assert_eq!(uuid.get_version(), Some(Version::Random));
        let _rug_ed_tests_llm_16_58_llm_16_58_rrrruuuugggg_test_set_version = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_59 {
    use crate::{Builder, Variant, Uuid};
    #[test]
    fn test_with_variant() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10)) = <(u8, usize, u8, usize, u8, usize, u8, usize, u8, usize, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut bytes: [u8; 16] = [rug_fuzz_0; 16];
        bytes[rug_fuzz_1] = rug_fuzz_2;
        let mut builder = Builder::from_bytes(bytes);
        let uuid = builder.with_variant(Variant::NCS).into_uuid();
        debug_assert_eq!(
            uuid.as_bytes() [rug_fuzz_3] & rug_fuzz_4, 0b0000_0000,
            "NCS variant did not match"
        );
        builder = Builder::from_bytes(bytes);
        let uuid = builder.with_variant(Variant::RFC4122).into_uuid();
        debug_assert_eq!(
            uuid.as_bytes() [rug_fuzz_5] & rug_fuzz_6, 0b1000_0000,
            "RFC4122 variant did not match"
        );
        builder = Builder::from_bytes(bytes);
        let uuid = builder.with_variant(Variant::Microsoft).into_uuid();
        debug_assert_eq!(
            uuid.as_bytes() [rug_fuzz_7] & rug_fuzz_8, 0b1100_0000,
            "Microsoft variant did not match"
        );
        builder = Builder::from_bytes(bytes);
        let uuid = builder.with_variant(Variant::Future).into_uuid();
        debug_assert_eq!(
            uuid.as_bytes() [rug_fuzz_9] & rug_fuzz_10, 0b1110_0000,
            "Future variant did not match"
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_60 {
    use crate::{builder::Builder, Version, Variant};
    #[test]
    fn test_with_version() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let uuid_bytes = [
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
        let builder_v1 = Builder::from_bytes(uuid_bytes)
            .with_version(Version::Mac)
            .with_variant(Variant::RFC4122);
        let uuid_v1 = builder_v1.into_uuid();
        let builder_v3 = Builder::from_bytes(uuid_bytes)
            .with_version(Version::Md5)
            .with_variant(Variant::RFC4122);
        let uuid_v3 = builder_v3.into_uuid();
        let builder_v4 = Builder::from_bytes(uuid_bytes)
            .with_version(Version::Random)
            .with_variant(Variant::RFC4122);
        let uuid_v4 = builder_v4.into_uuid();
        let builder_v5 = Builder::from_bytes(uuid_bytes)
            .with_version(Version::Sha1)
            .with_variant(Variant::RFC4122);
        let uuid_v5 = builder_v5.into_uuid();
        debug_assert!(matches!(uuid_v1.get_version(), Some(Version::Mac)));
        debug_assert!(matches!(uuid_v3.get_version(), Some(Version::Md5)));
        debug_assert!(matches!(uuid_v4.get_version(), Some(Version::Random)));
        debug_assert!(matches!(uuid_v5.get_version(), Some(Version::Sha1)));
             }
});    }
}
