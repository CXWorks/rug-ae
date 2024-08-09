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
        let _rug_st_tests_llm_16_31_rrrruuuugggg_test_from_bytes = 0;
        let rug_fuzz_0 = 0xa1;
        let rug_fuzz_1 = 0xa2;
        let rug_fuzz_2 = 0xa3;
        let rug_fuzz_3 = 0xa4;
        let rug_fuzz_4 = 0xb1;
        let rug_fuzz_5 = 0xb2;
        let rug_fuzz_6 = 0xc1;
        let rug_fuzz_7 = 0xc2;
        let rug_fuzz_8 = 0xd1;
        let rug_fuzz_9 = 0xd2;
        let rug_fuzz_10 = 0xd3;
        let rug_fuzz_11 = 0xd4;
        let rug_fuzz_12 = 0xd5;
        let rug_fuzz_13 = 0xd6;
        let rug_fuzz_14 = 0xd7;
        let rug_fuzz_15 = 0xd8;
        let rug_fuzz_16 = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8";
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
        let _rug_ed_tests_llm_16_31_rrrruuuugggg_test_from_bytes = 0;
    }
    #[test]
    fn test_from_bytes_round_trip() {
        let _rug_st_tests_llm_16_31_rrrruuuugggg_test_from_bytes_round_trip = 0;
        let rug_fuzz_0 = 0xa1;
        let rug_fuzz_1 = 0xa2;
        let rug_fuzz_2 = 0xa3;
        let rug_fuzz_3 = 0xa4;
        let rug_fuzz_4 = 0xb1;
        let rug_fuzz_5 = 0xb2;
        let rug_fuzz_6 = 0xc1;
        let rug_fuzz_7 = 0xc2;
        let rug_fuzz_8 = 0xd1;
        let rug_fuzz_9 = 0xd2;
        let rug_fuzz_10 = 0xd3;
        let rug_fuzz_11 = 0xd4;
        let rug_fuzz_12 = 0xd5;
        let rug_fuzz_13 = 0xd6;
        let rug_fuzz_14 = 0xd7;
        let rug_fuzz_15 = 0xd8;
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
        let _rug_ed_tests_llm_16_31_rrrruuuugggg_test_from_bytes_round_trip = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_32 {
    use crate::Uuid;
    #[test]
    fn test_from_bytes_le() {
        let _rug_st_tests_llm_16_32_rrrruuuugggg_test_from_bytes_le = 0;
        let rug_fuzz_0 = 0xa1;
        let rug_fuzz_1 = 0xa2;
        let rug_fuzz_2 = 0xa3;
        let rug_fuzz_3 = 0xa4;
        let rug_fuzz_4 = 0xb1;
        let rug_fuzz_5 = 0xb2;
        let rug_fuzz_6 = 0xc1;
        let rug_fuzz_7 = 0xc2;
        let rug_fuzz_8 = 0xd1;
        let rug_fuzz_9 = 0xd2;
        let rug_fuzz_10 = 0xd3;
        let rug_fuzz_11 = 0xd4;
        let rug_fuzz_12 = 0xd5;
        let rug_fuzz_13 = 0xd6;
        let rug_fuzz_14 = 0xd7;
        let rug_fuzz_15 = 0xd8;
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
        let _rug_ed_tests_llm_16_32_rrrruuuugggg_test_from_bytes_le = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_33 {
    use crate::{builder::Bytes, Uuid};
    #[test]
    fn test_from_bytes_ref() {
        let _rug_st_tests_llm_16_33_rrrruuuugggg_test_from_bytes_ref = 0;
        let rug_fuzz_0 = 0xa1;
        let rug_fuzz_1 = 0xa2;
        let rug_fuzz_2 = 0xa3;
        let rug_fuzz_3 = 0xa4;
        let rug_fuzz_4 = 0xb1;
        let rug_fuzz_5 = 0xb2;
        let rug_fuzz_6 = 0xc1;
        let rug_fuzz_7 = 0xc2;
        let rug_fuzz_8 = 0xd1;
        let rug_fuzz_9 = 0xd2;
        let rug_fuzz_10 = 0xd3;
        let rug_fuzz_11 = 0xd4;
        let rug_fuzz_12 = 0xd5;
        let rug_fuzz_13 = 0xd6;
        let rug_fuzz_14 = 0xd7;
        let rug_fuzz_15 = 0xd8;
        let rug_fuzz_16 = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8";
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
        let _rug_ed_tests_llm_16_33_rrrruuuugggg_test_from_bytes_ref = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_34 {
    use super::*;
    use crate::*;
    #[test]
    fn test_from_fields() {
        let _rug_st_tests_llm_16_34_rrrruuuugggg_test_from_fields = 0;
        let rug_fuzz_0 = 0x12345678;
        let rug_fuzz_1 = 0x1234;
        let rug_fuzz_2 = 0x1234;
        let rug_fuzz_3 = 0x12;
        let rug_fuzz_4 = 0x34;
        let rug_fuzz_5 = 0x56;
        let rug_fuzz_6 = 0x78;
        let rug_fuzz_7 = 0x90;
        let rug_fuzz_8 = 0xAB;
        let rug_fuzz_9 = 0xCD;
        let rug_fuzz_10 = 0xEF;
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
        let _rug_ed_tests_llm_16_34_rrrruuuugggg_test_from_fields = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_35 {
    use crate::Uuid;
    #[test]
    fn test_from_fields_le() {
        let _rug_st_tests_llm_16_35_rrrruuuugggg_test_from_fields_le = 0;
        let rug_fuzz_0 = 0x12345678;
        let rug_fuzz_1 = 0x1234;
        let rug_fuzz_2 = 0x5678;
        let rug_fuzz_3 = 0x90;
        let rug_fuzz_4 = 0xab;
        let rug_fuzz_5 = 0xcd;
        let rug_fuzz_6 = 0xef;
        let rug_fuzz_7 = 0x01;
        let rug_fuzz_8 = 0x23;
        let rug_fuzz_9 = 0x45;
        let rug_fuzz_10 = 0x67;
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
        let _rug_ed_tests_llm_16_35_rrrruuuugggg_test_from_fields_le = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_37_llm_16_37 {
    use super::*;
    use crate::*;
    use crate::error::ErrorKind::ByteLength;
    #[test]
    fn test_from_slice_le_correct_length() {
        let _rug_st_tests_llm_16_37_llm_16_37_rrrruuuugggg_test_from_slice_le_correct_length = 0;
        let rug_fuzz_0 = 0xa1;
        let rug_fuzz_1 = 0xa2;
        let rug_fuzz_2 = 0xa3;
        let rug_fuzz_3 = 0xa4;
        let rug_fuzz_4 = 0xb1;
        let rug_fuzz_5 = 0xb2;
        let rug_fuzz_6 = 0xc1;
        let rug_fuzz_7 = 0xc2;
        let rug_fuzz_8 = 0xd1;
        let rug_fuzz_9 = 0xd2;
        let rug_fuzz_10 = 0xd3;
        let rug_fuzz_11 = 0xd4;
        let rug_fuzz_12 = 0xd5;
        let rug_fuzz_13 = 0xd6;
        let rug_fuzz_14 = 0xd7;
        let rug_fuzz_15 = 0xd8;
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
        let _rug_ed_tests_llm_16_37_llm_16_37_rrrruuuugggg_test_from_slice_le_correct_length = 0;
    }
    #[test]
    fn test_from_slice_le_incorrect_length() {
        let _rug_st_tests_llm_16_37_llm_16_37_rrrruuuugggg_test_from_slice_le_incorrect_length = 0;
        let rug_fuzz_0 = 0xa1;
        let rug_fuzz_1 = 0xa2;
        let rug_fuzz_2 = 0xa3;
        let rug_fuzz_3 = 0xa4;
        let rug_fuzz_4 = 0xb1;
        let rug_fuzz_5 = 0xb2;
        let rug_fuzz_6 = 0xc1;
        let rug_fuzz_7 = 0xc2;
        let rug_fuzz_8 = 0xd1;
        let rug_fuzz_9 = 0xd2;
        let rug_fuzz_10 = 0xd3;
        let rug_fuzz_11 = 0xd4;
        let rug_fuzz_12 = 0xd5;
        let rug_fuzz_13 = 0xd6;
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
        let _rug_ed_tests_llm_16_37_llm_16_37_rrrruuuugggg_test_from_slice_le_incorrect_length = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_38_llm_16_38 {
    use crate::Uuid;
    #[test]
    fn test_from_u128() {
        let _rug_st_tests_llm_16_38_llm_16_38_rrrruuuugggg_test_from_u128 = 0;
        let rug_fuzz_0 = 0x1234567890abcdef1234567890abcdef;
        let rug_fuzz_1 = 0;
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
        let _rug_ed_tests_llm_16_38_llm_16_38_rrrruuuugggg_test_from_u128 = 0;
    }
    #[test]
    fn test_from_u128_le() {
        let _rug_st_tests_llm_16_38_llm_16_38_rrrruuuugggg_test_from_u128_le = 0;
        let rug_fuzz_0 = 0x1234567890abcdef1234567890abcdef;
        let rug_fuzz_1 = 0;
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
        let _rug_ed_tests_llm_16_38_llm_16_38_rrrruuuugggg_test_from_u128_le = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_39 {
    use super::*;
    use crate::*;
    #[test]
    fn test_from_u128_le() {
        let _rug_st_tests_llm_16_39_rrrruuuugggg_test_from_u128_le = 0;
        let rug_fuzz_0 = 0x1234567890abcdef1234567890abcdefu128;
        let rug_fuzz_1 = 0xefcdab9078563412efcdab9078563412u128;
        let input = rug_fuzz_0;
        let expected = Uuid::from_u128(rug_fuzz_1);
        let result = Uuid::from_u128_le(input);
        debug_assert_eq!(
            result, expected,
            "from_u128_le should convert a 128-bit u128 integer in little-endian to a Uuid by flipping its bytes"
        );
        let _rug_ed_tests_llm_16_39_rrrruuuugggg_test_from_u128_le = 0;
    }
    #[test]
    fn test_from_u128_le_zero() {
        let _rug_st_tests_llm_16_39_rrrruuuugggg_test_from_u128_le_zero = 0;
        let rug_fuzz_0 = 0u128;
        let input = rug_fuzz_0;
        let expected = Uuid::nil();
        let result = Uuid::from_u128_le(input);
        debug_assert_eq!(
            result, expected,
            "from_u128_le should convert a 128-bit u128 zero integer to a nil Uuid"
        );
        let _rug_ed_tests_llm_16_39_rrrruuuugggg_test_from_u128_le_zero = 0;
    }
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
        let _rug_st_tests_llm_16_39_rrrruuuugggg_test_from_u128_le_endianness = 0;
        let rug_fuzz_0 = 0x01;
        let rug_fuzz_1 = 0x23;
        let rug_fuzz_2 = 0x45;
        let rug_fuzz_3 = 0x67;
        let rug_fuzz_4 = 0x89;
        let rug_fuzz_5 = 0xAB;
        let rug_fuzz_6 = 0xCD;
        let rug_fuzz_7 = 0xEF;
        let rug_fuzz_8 = 0x01;
        let rug_fuzz_9 = 0x23;
        let rug_fuzz_10 = 0x45;
        let rug_fuzz_11 = 0x67;
        let rug_fuzz_12 = 0x89;
        let rug_fuzz_13 = 0xAB;
        let rug_fuzz_14 = 0xCD;
        let rug_fuzz_15 = 0xEF;
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
        let _rug_ed_tests_llm_16_39_rrrruuuugggg_test_from_u128_le_endianness = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_40 {
    use super::*;
    use crate::*;
    #[test]
    fn test_from_u64_pair() {
        let _rug_st_tests_llm_16_40_rrrruuuugggg_test_from_u64_pair = 0;
        let rug_fuzz_0 = 0xa1a2a3a4b1b2c1c2u64;
        let rug_fuzz_1 = 0xd1d2d3d4d5d6d7d8u64;
        let rug_fuzz_2 = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8";
        let high = rug_fuzz_0;
        let low = rug_fuzz_1;
        let expected = rug_fuzz_2;
        let uuid = Uuid::from_u64_pair(high, low);
        debug_assert_eq!(expected, uuid.to_string());
        let _rug_ed_tests_llm_16_40_rrrruuuugggg_test_from_u64_pair = 0;
    }
    #[test]
    fn test_from_u64_pair_zero() {
        let _rug_st_tests_llm_16_40_rrrruuuugggg_test_from_u64_pair_zero = 0;
        let rug_fuzz_0 = 0u64;
        let rug_fuzz_1 = 0u64;
        let rug_fuzz_2 = "00000000-0000-0000-0000-000000000000";
        let high = rug_fuzz_0;
        let low = rug_fuzz_1;
        let expected = rug_fuzz_2;
        let uuid = Uuid::from_u64_pair(high, low);
        debug_assert_eq!(expected, uuid.to_string());
        let _rug_ed_tests_llm_16_40_rrrruuuugggg_test_from_u64_pair_zero = 0;
    }
    #[test]
    fn test_from_u64_pair_max() {
        let _rug_st_tests_llm_16_40_rrrruuuugggg_test_from_u64_pair_max = 0;
        let rug_fuzz_0 = "ffffffff-ffff-ffff-ffff-ffffffffffff";
        let high = u64::MAX;
        let low = u64::MAX;
        let expected = rug_fuzz_0;
        let uuid = Uuid::from_u64_pair(high, low);
        debug_assert_eq!(expected, uuid.to_string());
        let _rug_ed_tests_llm_16_40_rrrruuuugggg_test_from_u64_pair_max = 0;
    }
    #[test]
    fn test_from_u64_pair_endianess() {
        let _rug_st_tests_llm_16_40_rrrruuuugggg_test_from_u64_pair_endianess = 0;
        let rug_fuzz_0 = 0x0011223344556677u64;
        let rug_fuzz_1 = 0x8899aabbccddeeffu64;
        let rug_fuzz_2 = "00112233-4455-6677-8899-aabbccddeeff";
        let high = rug_fuzz_0;
        let low = rug_fuzz_1;
        let expected = rug_fuzz_2;
        let uuid = Uuid::from_u64_pair(high, low);
        debug_assert_eq!(expected, uuid.to_string());
        let _rug_ed_tests_llm_16_40_rrrruuuugggg_test_from_u64_pair_endianess = 0;
    }
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
        let _rug_st_tests_llm_16_42_rrrruuuugggg_test_as_uuid_returns_correct_uuid = 0;
        let rug_fuzz_0 = 0x12;
        let rug_fuzz_1 = 0x23;
        let rug_fuzz_2 = 0x34;
        let rug_fuzz_3 = 0x45;
        let rug_fuzz_4 = 0x56;
        let rug_fuzz_5 = 0x67;
        let rug_fuzz_6 = 0x78;
        let rug_fuzz_7 = 0x89;
        let rug_fuzz_8 = 0x9a;
        let rug_fuzz_9 = 0xab;
        let rug_fuzz_10 = 0xbc;
        let rug_fuzz_11 = 0xcd;
        let rug_fuzz_12 = 0xde;
        let rug_fuzz_13 = 0xef;
        let rug_fuzz_14 = 0xf0;
        let rug_fuzz_15 = 0x01;
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
        let _rug_ed_tests_llm_16_42_rrrruuuugggg_test_as_uuid_returns_correct_uuid = 0;
    }
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
        let _rug_st_tests_llm_16_43_rrrruuuugggg_test_from_bytes = 0;
        let rug_fuzz_0 = 0xa1;
        let rug_fuzz_1 = 0xa2;
        let rug_fuzz_2 = 0xa3;
        let rug_fuzz_3 = 0xa4;
        let rug_fuzz_4 = 0xb1;
        let rug_fuzz_5 = 0xb2;
        let rug_fuzz_6 = 0xc1;
        let rug_fuzz_7 = 0xc2;
        let rug_fuzz_8 = 0xd1;
        let rug_fuzz_9 = 0xd2;
        let rug_fuzz_10 = 0xd3;
        let rug_fuzz_11 = 0xd4;
        let rug_fuzz_12 = 0xd5;
        let rug_fuzz_13 = 0xd6;
        let rug_fuzz_14 = 0xd7;
        let rug_fuzz_15 = 0xd8;
        let rug_fuzz_16 = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8";
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
        let _rug_ed_tests_llm_16_43_rrrruuuugggg_test_from_bytes = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_44 {
    use crate::Builder;
    use crate::Bytes;
    #[test]
    fn test_builder_from_bytes_le() {
        let _rug_st_tests_llm_16_44_rrrruuuugggg_test_builder_from_bytes_le = 0;
        let rug_fuzz_0 = 0x10;
        let rug_fuzz_1 = 0x94;
        let rug_fuzz_2 = 0x01;
        let rug_fuzz_3 = 0x2A;
        let rug_fuzz_4 = 0x0B;
        let rug_fuzz_5 = 0xED;
        let rug_fuzz_6 = 0x11;
        let rug_fuzz_7 = 0x5F;
        let rug_fuzz_8 = 0x85;
        let rug_fuzz_9 = 0x23;
        let rug_fuzz_10 = 0x27;
        let rug_fuzz_11 = 0xFC;
        let rug_fuzz_12 = 0x15;
        let rug_fuzz_13 = 0x6B;
        let rug_fuzz_14 = 0x5C;
        let rug_fuzz_15 = 0x5F;
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
        let _rug_ed_tests_llm_16_44_rrrruuuugggg_test_builder_from_bytes_le = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_45 {
    use super::*;
    use crate::*;
    #[test]
    fn test_from_fields() {
        let _rug_st_tests_llm_16_45_rrrruuuugggg_test_from_fields = 0;
        let rug_fuzz_0 = 0xa1a2a3a4;
        let rug_fuzz_1 = 0xb1b2;
        let rug_fuzz_2 = 0xc1c2;
        let rug_fuzz_3 = 0xd1;
        let rug_fuzz_4 = 0xd2;
        let rug_fuzz_5 = 0xd3;
        let rug_fuzz_6 = 0xd4;
        let rug_fuzz_7 = 0xd5;
        let rug_fuzz_8 = 0xd6;
        let rug_fuzz_9 = 0xd7;
        let rug_fuzz_10 = 0xd8;
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
        let _rug_ed_tests_llm_16_45_rrrruuuugggg_test_from_fields = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_46 {
    use crate::builder::Builder;
    use crate::Uuid;
    #[test]
    fn test_from_fields_le() {
        let _rug_st_tests_llm_16_46_rrrruuuugggg_test_from_fields_le = 0;
        let rug_fuzz_0 = 0x12345678;
        let rug_fuzz_1 = 0x1234;
        let rug_fuzz_2 = 0x1234;
        let rug_fuzz_3 = 0x12;
        let rug_fuzz_4 = 0x34;
        let rug_fuzz_5 = 0x56;
        let rug_fuzz_6 = 0x78;
        let rug_fuzz_7 = 0x90;
        let rug_fuzz_8 = 0xAB;
        let rug_fuzz_9 = 0xCD;
        let rug_fuzz_10 = 0xEF;
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
        let _rug_ed_tests_llm_16_46_rrrruuuugggg_test_from_fields_le = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_47 {
    use super::*;
    use crate::*;
    #[test]
    fn test_from_md5_bytes() {
        let _rug_st_tests_llm_16_47_rrrruuuugggg_test_from_md5_bytes = 0;
        let rug_fuzz_0 = 0xd4;
        let rug_fuzz_1 = 0x1d;
        let rug_fuzz_2 = 0x8c;
        let rug_fuzz_3 = 0xd9;
        let rug_fuzz_4 = 0x8f;
        let rug_fuzz_5 = 0x00;
        let rug_fuzz_6 = 0xb2;
        let rug_fuzz_7 = 0x04;
        let rug_fuzz_8 = 0xe9;
        let rug_fuzz_9 = 0x80;
        let rug_fuzz_10 = 0x09;
        let rug_fuzz_11 = 0x98;
        let rug_fuzz_12 = 0xec;
        let rug_fuzz_13 = 0xf8;
        let rug_fuzz_14 = 0x42;
        let rug_fuzz_15 = 0x7e;
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
        let _rug_ed_tests_llm_16_47_rrrruuuugggg_test_from_md5_bytes = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_48 {
    use crate::Builder;
    use crate::{Bytes, Variant, Version};
    #[test]
    fn test_builder_from_random_bytes() {
        let _rug_st_tests_llm_16_48_rrrruuuugggg_test_builder_from_random_bytes = 0;
        let rug_fuzz_0 = 70;
        let rug_fuzz_1 = 235;
        let rug_fuzz_2 = 208;
        let rug_fuzz_3 = 238;
        let rug_fuzz_4 = 14;
        let rug_fuzz_5 = 109;
        let rug_fuzz_6 = 67;
        let rug_fuzz_7 = 201;
        let rug_fuzz_8 = 185;
        let rug_fuzz_9 = 13;
        let rug_fuzz_10 = 204;
        let rug_fuzz_11 = 195;
        let rug_fuzz_12 = 90;
        let rug_fuzz_13 = 145;
        let rug_fuzz_14 = 63;
        let rug_fuzz_15 = 62;
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
        let _rug_ed_tests_llm_16_48_rrrruuuugggg_test_builder_from_random_bytes = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_49 {
    use super::*;
    use crate::*;
    use crate::builder::Builder;
    #[test]
    fn test_from_rfc4122_timestamp() {
        let _rug_st_tests_llm_16_49_rrrruuuugggg_test_from_rfc4122_timestamp = 0;
        let rug_fuzz_0 = 0x1234_5678_9ABC_DEF0;
        let rug_fuzz_1 = 0x1234;
        let rug_fuzz_2 = 0xAA;
        let rug_fuzz_3 = 0xBB;
        let rug_fuzz_4 = 0xCC;
        let rug_fuzz_5 = 0xDD;
        let rug_fuzz_6 = 0xEE;
        let rug_fuzz_7 = 0xFF;
        let rug_fuzz_8 = 0x67;
        let rug_fuzz_9 = 0x89;
        let rug_fuzz_10 = 0xAB;
        let rug_fuzz_11 = 0xCD;
        let rug_fuzz_12 = 0xEF;
        let rug_fuzz_13 = 0xF0;
        let rug_fuzz_14 = 0x11;
        let rug_fuzz_15 = 0x34;
        let rug_fuzz_16 = 0x92;
        let rug_fuzz_17 = 0x34;
        let rug_fuzz_18 = 0xAA;
        let rug_fuzz_19 = 0xBB;
        let rug_fuzz_20 = 0xCC;
        let rug_fuzz_21 = 0xDD;
        let rug_fuzz_22 = 0xEE;
        let rug_fuzz_23 = 0xFF;
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
        let _rug_ed_tests_llm_16_49_rrrruuuugggg_test_from_rfc4122_timestamp = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_50_llm_16_50 {
    use crate::{Builder, Bytes, Variant, Version};
    #[test]
    fn test_from_sha1_bytes() {
        let _rug_st_tests_llm_16_50_llm_16_50_rrrruuuugggg_test_from_sha1_bytes = 0;
        let rug_fuzz_0 = 0x5a;
        let rug_fuzz_1 = 0xfa;
        let rug_fuzz_2 = 0x7a;
        let rug_fuzz_3 = 0xb9;
        let rug_fuzz_4 = 0x5b;
        let rug_fuzz_5 = 0xef;
        let rug_fuzz_6 = 0x9a;
        let rug_fuzz_7 = 0xa2;
        let rug_fuzz_8 = 0x1d;
        let rug_fuzz_9 = 0xf5;
        let rug_fuzz_10 = 0x9a;
        let rug_fuzz_11 = 0xf7;
        let rug_fuzz_12 = 0xca;
        let rug_fuzz_13 = 0xfa;
        let rug_fuzz_14 = 0x78;
        let rug_fuzz_15 = 0xf5;
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
        let _rug_ed_tests_llm_16_50_llm_16_50_rrrruuuugggg_test_from_sha1_bytes = 0;
    }
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
        let _rug_st_tests_llm_16_53_rrrruuuugggg_test_builder_from_u128 = 0;
        let rug_fuzz_0 = 0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8;
        let input: u128 = rug_fuzz_0;
        let builder = Builder::from_u128(input);
        let uuid = builder.into_uuid();
        debug_assert_eq!(
            uuid.hyphenated().to_string(), "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8"
        );
        let _rug_ed_tests_llm_16_53_rrrruuuugggg_test_builder_from_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_54 {
    use crate::builder::Builder;
    use crate::Uuid;
    #[test]
    fn test_from_u128_le() {
        let _rug_st_tests_llm_16_54_rrrruuuugggg_test_from_u128_le = 0;
        let rug_fuzz_0 = 0x0102030405060708090a0b0c0d0e0f10u128;
        let rug_fuzz_1 = "100f0e0d-0c0b-0a09-0807-060504030201";
        let rug_fuzz_2 = 0x112233445566778899aabbccddeeffffu128;
        let rug_fuzz_3 = "ffeeddcb-ccba-a998-8877-665544332211";
        let rug_fuzz_4 = 0xffffffffffffffffffffffffffffffffu128;
        let rug_fuzz_5 = "ffffffff-ffff-ffff-ffff-ffffffffffff";
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
        let _rug_ed_tests_llm_16_54_rrrruuuugggg_test_from_u128_le = 0;
    }
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
        let _rug_st_tests_llm_16_55_rrrruuuugggg_test_into_uuid_from_bytes = 0;
        let rug_fuzz_0 = 0x12;
        let rug_fuzz_1 = 0x34;
        let rug_fuzz_2 = 0x56;
        let rug_fuzz_3 = 0x78;
        let rug_fuzz_4 = 0x90;
        let rug_fuzz_5 = 0xab;
        let rug_fuzz_6 = 0xcd;
        let rug_fuzz_7 = 0xef;
        let rug_fuzz_8 = 0x12;
        let rug_fuzz_9 = 0x34;
        let rug_fuzz_10 = 0x56;
        let rug_fuzz_11 = 0x78;
        let rug_fuzz_12 = 0x90;
        let rug_fuzz_13 = 0xab;
        let rug_fuzz_14 = 0xcd;
        let rug_fuzz_15 = 0xef;
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
        let _rug_ed_tests_llm_16_55_rrrruuuugggg_test_into_uuid_from_bytes = 0;
    }
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
        let _rug_st_tests_llm_16_55_rrrruuuugggg_test_into_uuid_from_fields = 0;
        let rug_fuzz_0 = 0x12345678;
        let rug_fuzz_1 = 0x90ab;
        let rug_fuzz_2 = 0xcdef;
        let rug_fuzz_3 = 0x12;
        let rug_fuzz_4 = 0x34;
        let rug_fuzz_5 = 0x56;
        let rug_fuzz_6 = 0x78;
        let rug_fuzz_7 = 0x90;
        let rug_fuzz_8 = 0xab;
        let rug_fuzz_9 = 0xcd;
        let rug_fuzz_10 = 0xef;
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
        let _rug_ed_tests_llm_16_55_rrrruuuugggg_test_into_uuid_from_fields = 0;
    }
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
        let _rug_st_tests_llm_16_59_rrrruuuugggg_test_with_variant = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 8;
        let rug_fuzz_2 = 0b1011_1010;
        let rug_fuzz_3 = 8;
        let rug_fuzz_4 = 0b1000_0000;
        let rug_fuzz_5 = 8;
        let rug_fuzz_6 = 0b1100_0000;
        let rug_fuzz_7 = 8;
        let rug_fuzz_8 = 0b1110_0000;
        let rug_fuzz_9 = 8;
        let rug_fuzz_10 = 0b1110_0000;
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
        let _rug_ed_tests_llm_16_59_rrrruuuugggg_test_with_variant = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_60 {
    use crate::{builder::Builder, Version, Variant};
    #[test]
    fn test_with_version() {
        let _rug_st_tests_llm_16_60_rrrruuuugggg_test_with_version = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 0;
        let rug_fuzz_10 = 0;
        let rug_fuzz_11 = 0;
        let rug_fuzz_12 = 0;
        let rug_fuzz_13 = 0;
        let rug_fuzz_14 = 0;
        let rug_fuzz_15 = 0;
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
        let _rug_ed_tests_llm_16_60_rrrruuuugggg_test_with_version = 0;
    }
}
#[cfg(test)]
mod tests_rug_17 {
    use super::*;
    #[test]
    fn test_from_slice_valid() {
        let _rug_st_tests_rug_17_rrrruuuugggg_test_from_slice_valid = 0;
        let rug_fuzz_0 = 0xa1;
        let rug_fuzz_1 = 0xa2;
        let rug_fuzz_2 = 0xa3;
        let rug_fuzz_3 = 0xa4;
        let rug_fuzz_4 = 0xb1;
        let rug_fuzz_5 = 0xb2;
        let rug_fuzz_6 = 0xc1;
        let rug_fuzz_7 = 0xc2;
        let rug_fuzz_8 = 0xd1;
        let rug_fuzz_9 = 0xd2;
        let rug_fuzz_10 = 0xd3;
        let rug_fuzz_11 = 0xd4;
        let rug_fuzz_12 = 0xd5;
        let rug_fuzz_13 = 0xd6;
        let rug_fuzz_14 = 0xd7;
        let rug_fuzz_15 = 0xd8;
        let rug_fuzz_16 = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8";
        let p0: &[u8] = &[
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
        let uuid = Uuid::from_slice(p0).unwrap();
        debug_assert_eq!(rug_fuzz_16, uuid.hyphenated().to_string());
        let _rug_ed_tests_rug_17_rrrruuuugggg_test_from_slice_valid = 0;
    }
    #[test]
    fn test_from_slice_invalid_length() {
        let _rug_st_tests_rug_17_rrrruuuugggg_test_from_slice_invalid_length = 0;
        let rug_fuzz_0 = 0x00;
        let p0: &[u8] = &[rug_fuzz_0];
        debug_assert!(
            matches!(Uuid::from_slice(p0), Err(Error(ErrorKind::ByteLength { len : _ })))
        );
        let _rug_ed_tests_rug_17_rrrruuuugggg_test_from_slice_invalid_length = 0;
    }
}
#[cfg(test)]
mod tests_rug_18 {
    use crate::builder::Builder;
    use crate::Error;
    use crate::Uuid;
    #[test]
    fn test_rug() -> Result<(), Error> {
        let p0: &[u8] = &[
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
        let builder = Builder::from_slice(p0)?;
        let uuid = builder.into_uuid();
        assert_eq!(
            uuid.hyphenated().to_string(), "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8"
        );
        Ok(())
    }
}
