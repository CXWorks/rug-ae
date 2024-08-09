use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
#[doc(hidden)]
#[cfg(feature = "serde")]
pub const NAME: &str = "$__serde_spanned_private_Spanned";
#[doc(hidden)]
#[cfg(feature = "serde")]
pub const START_FIELD: &str = "$__serde_spanned_private_start";
#[doc(hidden)]
#[cfg(feature = "serde")]
pub const END_FIELD: &str = "$__serde_spanned_private_end";
#[doc(hidden)]
#[cfg(feature = "serde")]
pub const VALUE_FIELD: &str = "$__serde_spanned_private_value";
#[doc(hidden)]
#[cfg(feature = "serde")]
pub fn is_spanned(name: &'static str, fields: &'static [&'static str]) -> bool {
    name == NAME && fields == [START_FIELD, END_FIELD, VALUE_FIELD]
}
/// A spanned value, indicating the range at which it is defined in the source.
#[derive(Clone, Debug)]
pub struct Spanned<T> {
    /// Byte range
    span: std::ops::Range<usize>,
    /// The spanned value.
    value: T,
}
impl<T> Spanned<T> {
    /// Byte range
    pub fn span(&self) -> std::ops::Range<usize> {
        self.span.clone()
    }
    /// Consumes the spanned value and returns the contained value.
    pub fn into_inner(self) -> T {
        self.value
    }
    /// Returns a reference to the contained value.
    pub fn get_ref(&self) -> &T {
        &self.value
    }
    /// Returns a mutable reference to the contained value.
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }
}
impl std::borrow::Borrow<str> for Spanned<String> {
    fn borrow(&self) -> &str {
        self.get_ref()
    }
}
impl<T> AsRef<T> for Spanned<T> {
    fn as_ref(&self) -> &T {
        self.get_ref()
    }
}
impl<T> AsMut<T> for Spanned<T> {
    fn as_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}
impl<T: PartialEq> PartialEq for Spanned<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}
impl<T: Eq> Eq for Spanned<T> {}
impl<T: Hash> Hash for Spanned<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}
impl<T: PartialOrd> PartialOrd for Spanned<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}
impl<T: Ord> Ord for Spanned<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}
#[cfg(feature = "serde")]
impl<'de, T> serde::de::Deserialize<'de> for Spanned<T>
where
    T: serde::de::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Spanned<T>, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct SpannedVisitor<T>(::std::marker::PhantomData<T>);
        impl<'de, T> serde::de::Visitor<'de> for SpannedVisitor<T>
        where
            T: serde::de::Deserialize<'de>,
        {
            type Value = Spanned<T>;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("a spanned value")
            }
            fn visit_map<V>(self, mut visitor: V) -> Result<Spanned<T>, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                if visitor.next_key()? != Some(START_FIELD) {
                    return Err(serde::de::Error::custom("spanned start key not found"));
                }
                let start: usize = visitor.next_value()?;
                if visitor.next_key()? != Some(END_FIELD) {
                    return Err(serde::de::Error::custom("spanned end key not found"));
                }
                let end: usize = visitor.next_value()?;
                if visitor.next_key()? != Some(VALUE_FIELD) {
                    return Err(serde::de::Error::custom("spanned value key not found"));
                }
                let value: T = visitor.next_value()?;
                Ok(Spanned { span: start..end, value })
            }
        }
        let visitor = SpannedVisitor(::std::marker::PhantomData);
        static FIELDS: [&str; 3] = [START_FIELD, END_FIELD, VALUE_FIELD];
        deserializer.deserialize_struct(NAME, &FIELDS, visitor)
    }
}
#[cfg(feature = "serde")]
impl<T: serde::ser::Serialize> serde::ser::Serialize for Spanned<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.value.serialize(serializer)
    }
}
#[cfg(test)]
mod tests_llm_16_1_llm_16_1 {
    use crate::spanned::Spanned;
    use std::cmp::Ordering;
    #[test]
    fn test_cmp() {
        let _rug_st_tests_llm_16_1_llm_16_1_rrrruuuugggg_test_cmp = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 5;
        let rug_fuzz_2 = 5;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 10;
        let rug_fuzz_5 = 20;
        let rug_fuzz_6 = 10;
        let span_a = std::ops::Range {
            start: rug_fuzz_0,
            end: rug_fuzz_1,
        };
        let span_b = std::ops::Range {
            start: rug_fuzz_2,
            end: rug_fuzz_3,
        };
        let spanned_value_a = Spanned {
            span: span_a.clone(),
            value: rug_fuzz_4,
        };
        let spanned_value_b = Spanned {
            span: span_b,
            value: rug_fuzz_5,
        };
        let spanned_value_c = Spanned {
            span: span_a,
            value: rug_fuzz_6,
        };
        debug_assert_eq!(spanned_value_a.cmp(& spanned_value_b), Ordering::Less);
        debug_assert_eq!(spanned_value_b.cmp(& spanned_value_a), Ordering::Greater);
        debug_assert_eq!(spanned_value_a.cmp(& spanned_value_c), Ordering::Equal);
        let _rug_ed_tests_llm_16_1_llm_16_1_rrrruuuugggg_test_cmp = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2 {
    use crate::Spanned;
    use std::cmp::PartialEq;
    #[test]
    fn test_eq() {
        let _rug_st_tests_llm_16_2_rrrruuuugggg_test_eq = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 5;
        let rug_fuzz_2 = "hello";
        let rug_fuzz_3 = 5;
        let rug_fuzz_4 = 10;
        let rug_fuzz_5 = "hello";
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 5;
        let rug_fuzz_8 = "world";
        let rug_fuzz_9 = 5;
        let rug_fuzz_10 = 10;
        let rug_fuzz_11 = "world";
        let span_a = Spanned {
            span: rug_fuzz_0..rug_fuzz_1,
            value: rug_fuzz_2.to_string(),
        };
        let span_b = Spanned {
            span: rug_fuzz_3..rug_fuzz_4,
            value: rug_fuzz_5.to_string(),
        };
        let span_c = Spanned {
            span: rug_fuzz_6..rug_fuzz_7,
            value: rug_fuzz_8.to_string(),
        };
        let span_d = Spanned {
            span: rug_fuzz_9..rug_fuzz_10,
            value: rug_fuzz_11.to_string(),
        };
        debug_assert!(span_a.eq(& span_b), "Values are equal but spans are different");
        debug_assert!(! span_a.eq(& span_c), "Values are different and spans are equal");
        debug_assert!(! span_c.eq(& span_d), "Values are equal but spans are different");
        debug_assert!(span_b.eq(& span_d), "Values are equal and spans are different");
        let _rug_ed_tests_llm_16_2_rrrruuuugggg_test_eq = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_3 {
    use crate::Spanned;
    use std::cmp::Ordering;
    #[test]
    fn test_partial_cmp_equal() {
        let _rug_st_tests_llm_16_3_rrrruuuugggg_test_partial_cmp_equal = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 5;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 8;
        let rug_fuzz_5 = 10;
        let span_a = Spanned {
            span: rug_fuzz_0..rug_fuzz_1,
            value: rug_fuzz_2,
        };
        let span_b = Spanned {
            span: rug_fuzz_3..rug_fuzz_4,
            value: rug_fuzz_5,
        };
        debug_assert_eq!(span_a.partial_cmp(& span_b), Some(Ordering::Equal));
        let _rug_ed_tests_llm_16_3_rrrruuuugggg_test_partial_cmp_equal = 0;
    }
    #[test]
    fn test_partial_cmp_less() {
        let _rug_st_tests_llm_16_3_rrrruuuugggg_test_partial_cmp_less = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 5;
        let rug_fuzz_2 = 5;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 8;
        let rug_fuzz_5 = 10;
        let span_a = Spanned {
            span: rug_fuzz_0..rug_fuzz_1,
            value: rug_fuzz_2,
        };
        let span_b = Spanned {
            span: rug_fuzz_3..rug_fuzz_4,
            value: rug_fuzz_5,
        };
        debug_assert_eq!(span_a.partial_cmp(& span_b), Some(Ordering::Less));
        let _rug_ed_tests_llm_16_3_rrrruuuugggg_test_partial_cmp_less = 0;
    }
    #[test]
    fn test_partial_cmp_greater() {
        let _rug_st_tests_llm_16_3_rrrruuuugggg_test_partial_cmp_greater = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 5;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 8;
        let rug_fuzz_5 = 5;
        let span_a = Spanned {
            span: rug_fuzz_0..rug_fuzz_1,
            value: rug_fuzz_2,
        };
        let span_b = Spanned {
            span: rug_fuzz_3..rug_fuzz_4,
            value: rug_fuzz_5,
        };
        debug_assert_eq!(span_a.partial_cmp(& span_b), Some(Ordering::Greater));
        let _rug_ed_tests_llm_16_3_rrrruuuugggg_test_partial_cmp_greater = 0;
    }
    #[test]
    fn test_partial_cmp_none() {
        let _rug_st_tests_llm_16_3_rrrruuuugggg_test_partial_cmp_none = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 5;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 8;
        let rug_fuzz_4 = 5.0;
        let span_a = Spanned {
            span: rug_fuzz_0..rug_fuzz_1,
            value: std::f64::NAN,
        };
        let span_b = Spanned {
            span: rug_fuzz_2..rug_fuzz_3,
            value: rug_fuzz_4,
        };
        debug_assert_eq!(span_a.partial_cmp(& span_b), None);
        let _rug_ed_tests_llm_16_3_rrrruuuugggg_test_partial_cmp_none = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_4 {
    use crate::Spanned;
    use std::convert::AsMut;
    #[test]
    fn test_as_mut() {
        let _rug_st_tests_llm_16_4_rrrruuuugggg_test_as_mut = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = 42;
        let rug_fuzz_3 = 10;
        let mut spanned_value = Spanned {
            span: rug_fuzz_0..rug_fuzz_1,
            value: rug_fuzz_2,
        };
        let value_mut: &mut i32 = spanned_value.as_mut();
        *value_mut = rug_fuzz_3;
        debug_assert_eq!(* spanned_value.get_ref(), 10);
        let _rug_ed_tests_llm_16_4_rrrruuuugggg_test_as_mut = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_5 {
    use crate::Spanned;
    use std::convert::AsRef;
    #[test]
    fn test_as_ref() {
        let _rug_st_tests_llm_16_5_rrrruuuugggg_test_as_ref = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = 42;
        let spanned_value = Spanned {
            span: rug_fuzz_0..rug_fuzz_1,
            value: rug_fuzz_2,
        };
        let value_ref: &i32 = spanned_value.as_ref();
        debug_assert_eq!(value_ref, & spanned_value.value);
        let _rug_ed_tests_llm_16_5_rrrruuuugggg_test_as_ref = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_6 {
    use crate::Spanned;
    use std::hash::{Hash, Hasher};
    struct DummyHasher(u64);
    impl Hasher for DummyHasher {
        fn finish(&self) -> u64 {
            self.0
        }
        fn write(&mut self, bytes: &[u8]) {
            for byte in bytes {
                self.0 = self.0.wrapping_add(u64::from(*byte));
            }
        }
    }
    #[test]
    fn hash_spanned_value() {
        let _rug_st_tests_llm_16_6_rrrruuuugggg_hash_spanned_value = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = "Test Value A";
        let rug_fuzz_3 = 100;
        let rug_fuzz_4 = 100;
        let rug_fuzz_5 = "Test Value A";
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 0;
        let spanned_a = Spanned {
            span: rug_fuzz_0..rug_fuzz_1,
            value: rug_fuzz_2,
        };
        let spanned_b = Spanned {
            span: rug_fuzz_3..rug_fuzz_4,
            value: rug_fuzz_5,
        };
        let mut hasher_a = DummyHasher(rug_fuzz_6);
        let mut hasher_b = DummyHasher(rug_fuzz_7);
        spanned_a.hash(&mut hasher_a);
        spanned_b.hash(&mut hasher_b);
        debug_assert_eq!(hasher_a.finish(), hasher_b.finish());
        let _rug_ed_tests_llm_16_6_rrrruuuugggg_hash_spanned_value = 0;
    }
    #[test]
    fn hash_different_spanned_values() {
        let _rug_st_tests_llm_16_6_rrrruuuugggg_hash_different_spanned_values = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = "Test Value A";
        let rug_fuzz_3 = 100;
        let rug_fuzz_4 = 100;
        let rug_fuzz_5 = "Test Value B";
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 0;
        let spanned_a = Spanned {
            span: rug_fuzz_0..rug_fuzz_1,
            value: rug_fuzz_2,
        };
        let spanned_b = Spanned {
            span: rug_fuzz_3..rug_fuzz_4,
            value: rug_fuzz_5,
        };
        let mut hasher_a = DummyHasher(rug_fuzz_6);
        let mut hasher_b = DummyHasher(rug_fuzz_7);
        spanned_a.hash(&mut hasher_a);
        spanned_b.hash(&mut hasher_b);
        debug_assert_ne!(hasher_a.finish(), hasher_b.finish());
        let _rug_ed_tests_llm_16_6_rrrruuuugggg_hash_different_spanned_values = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_7 {
    use super::*;
    use crate::*;
    use std::borrow::Borrow;
    #[test]
    fn spanned_borrow_returns_correct_str_slice() {
        let _rug_st_tests_llm_16_7_rrrruuuugggg_spanned_borrow_returns_correct_str_slice = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = "Hello, World!";
        let spanned = Spanned {
            span: rug_fuzz_0..rug_fuzz_1,
            value: rug_fuzz_2.to_string(),
        };
        let borrowed: &str = spanned.borrow();
        debug_assert_eq!(borrowed, "Hello, World!");
        let _rug_ed_tests_llm_16_7_rrrruuuugggg_spanned_borrow_returns_correct_str_slice = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_8 {
    use super::*;
    use crate::*;
    #[test]
    fn test_get_mut() {
        let _rug_st_tests_llm_16_8_rrrruuuugggg_test_get_mut = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = 42;
        let rug_fuzz_3 = 100;
        let mut spanned_value = Spanned {
            span: rug_fuzz_0..rug_fuzz_1,
            value: rug_fuzz_2,
        };
        *spanned_value.get_mut() = rug_fuzz_3;
        debug_assert_eq!(spanned_value.value, 100);
        let _rug_ed_tests_llm_16_8_rrrruuuugggg_test_get_mut = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_9 {
    use crate::Spanned;
    #[test]
    fn test_get_ref() {
        let _rug_st_tests_llm_16_9_rrrruuuugggg_test_get_ref = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = 42;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 20;
        let rug_fuzz_5 = "Hello, World!";
        let rug_fuzz_6 = 20;
        let rug_fuzz_7 = 30;
        let rug_fuzz_8 = 1;
        let spanned_i32 = Spanned {
            span: rug_fuzz_0..rug_fuzz_1,
            value: rug_fuzz_2,
        };
        debug_assert_eq!(spanned_i32.get_ref(), & 42);
        let spanned_string = Spanned {
            span: rug_fuzz_3..rug_fuzz_4,
            value: rug_fuzz_5.to_string(),
        };
        debug_assert_eq!(spanned_string.get_ref(), "Hello, World!");
        let spanned_vec = Spanned {
            span: rug_fuzz_6..rug_fuzz_7,
            value: vec![rug_fuzz_8, 2, 3],
        };
        debug_assert_eq!(spanned_vec.get_ref(), & vec![1, 2, 3]);
        let _rug_ed_tests_llm_16_9_rrrruuuugggg_test_get_ref = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_10 {
    use super::*;
    use crate::*;
    #[test]
    fn test_into_inner() {
        let _rug_st_tests_llm_16_10_rrrruuuugggg_test_into_inner = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = 42;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 20;
        let rug_fuzz_5 = "Hello, World!";
        let rug_fuzz_6 = 20;
        let rug_fuzz_7 = 30;
        let rug_fuzz_8 = 1;
        let spanned_int = Spanned {
            span: rug_fuzz_0..rug_fuzz_1,
            value: rug_fuzz_2,
        };
        let value_int = spanned_int.into_inner();
        debug_assert_eq!(value_int, 42);
        let spanned_string = Spanned {
            span: rug_fuzz_3..rug_fuzz_4,
            value: String::from(rug_fuzz_5),
        };
        let value_string = spanned_string.into_inner();
        debug_assert_eq!(value_string, "Hello, World!");
        let spanned_vec = Spanned {
            span: rug_fuzz_6..rug_fuzz_7,
            value: vec![rug_fuzz_8, 2, 3],
        };
        let value_vec = spanned_vec.into_inner();
        debug_assert_eq!(value_vec, vec![1, 2, 3]);
        let _rug_ed_tests_llm_16_10_rrrruuuugggg_test_into_inner = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_11 {
    use crate::Spanned;
    use std::ops::Range;
    #[test]
    fn test_span() {
        let _rug_st_tests_llm_16_11_rrrruuuugggg_test_span = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = "example";
        let rug_fuzz_3 = 5;
        let rug_fuzz_4 = 10;
        let spanned_value: Spanned<String> = Spanned {
            span: Range {
                start: rug_fuzz_0,
                end: rug_fuzz_1,
            },
            value: rug_fuzz_2.to_string(),
        };
        let span = spanned_value.span();
        debug_assert_eq!(rug_fuzz_3, span.start);
        debug_assert_eq!(rug_fuzz_4, span.end);
        let _rug_ed_tests_llm_16_11_rrrruuuugggg_test_span = 0;
    }
}
