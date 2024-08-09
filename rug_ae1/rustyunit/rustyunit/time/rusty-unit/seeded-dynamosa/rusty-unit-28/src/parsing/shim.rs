//! Extension traits for things either not implemented or not yet stable in the MSRV.

/// Equivalent of `foo.parse()` for slices.
pub(crate) trait IntegerParseBytes<T> {
    #[allow(clippy::missing_docs_in_private_items)]
    fn parse_bytes(&self) -> Option<T>;
}

impl<T: Integer> IntegerParseBytes<T> for [u8] {
    fn parse_bytes(&self) -> Option<T> {
        T::parse_bytes(self)
    }
}

/// Marker trait for all integer types, including `NonZero*`
pub(crate) trait Integer: Sized {
    #[allow(clippy::missing_docs_in_private_items)]
    fn parse_bytes(src: &[u8]) -> Option<Self>;
}

/// Parse the given types from bytes.
macro_rules! impl_parse_bytes {
    ($($t:ty)*) => ($(
        impl Integer for $t {
            #[allow(trivial_numeric_casts)]
            fn parse_bytes(src: &[u8]) -> Option<Self> {
                src.iter().try_fold::<Self, _, _>(0, |result, c| {
                    result.checked_mul(10)?.checked_add((c - b'0') as Self)
                })
            }
        }
    )*)
}
impl_parse_bytes! { u8 u16 u32 }

/// Parse the given types from bytes.
macro_rules! impl_parse_bytes_nonzero {
    ($($t:ty)*) => {$(
        impl Integer for $t {
            fn parse_bytes(src: &[u8]) -> Option<Self> {
                Self::new(src.parse_bytes()?)
            }
        }
    )*}
}

impl_parse_bytes_nonzero! {
    core::num::NonZeroU8
    core::num::NonZeroU16
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_509() {
//    rusty_monitor::set_test_id(509);
    let mut i32_0: i32 = 15i32;
    let mut i32_1: i32 = 376i32;
    let mut i32_2: i32 = 235i32;
    let mut i32_3: i32 = 280i32;
    let mut i32_4: i32 = 212i32;
    let mut i32_5: i32 = 22i32;
    let mut i32_6: i32 = 36525i32;
    let mut i32_7: i32 = 364i32;
    let mut i32_8: i32 = 1000000i32;
    let mut i32_9: i32 = 1i32;
    let mut i32_10: i32 = 336i32;
    let mut i32_11: i32 = 342i32;
    let mut i32_12: i32 = 144i32;
    let mut i32_13: i32 = 2147483647i32;
    let mut i32_14: i32 = 364i32;
    let mut i32_15: i32 = 1000i32;
    let mut u8_0: u8 = crate::util::weeks_in_year(i32_15);
    let mut u8_1: u8 = crate::util::weeks_in_year(i32_14);
    let mut u8_2: u8 = crate::util::weeks_in_year(i32_13);
    let mut u8_3: u8 = crate::util::weeks_in_year(i32_12);
    let mut u8_4: u8 = crate::util::weeks_in_year(i32_11);
    let mut u8_5: u8 = crate::util::weeks_in_year(i32_10);
    let mut u8_6: u8 = crate::util::weeks_in_year(i32_9);
    let mut u8_7: u8 = crate::util::weeks_in_year(i32_8);
    let mut u8_8: u8 = crate::util::weeks_in_year(i32_7);
    let mut u8_9: u8 = crate::util::weeks_in_year(i32_6);
    let mut u8_10: u8 = crate::util::weeks_in_year(i32_5);
    let mut u8_11: u8 = crate::util::weeks_in_year(i32_4);
    let mut u8_12: u8 = crate::util::weeks_in_year(i32_3);
    let mut u8_13: u8 = crate::util::weeks_in_year(i32_2);
    let mut u8_14: u8 = crate::util::weeks_in_year(i32_1);
    let mut u8_15: u8 = crate::util::weeks_in_year(i32_0);
//    panic!("From RustyUnit with love");
}
}