use core::num::Wrapping;
use core::{f32, f64};
use core::{i128, i16, i32, i64, i8, isize};
use core::{u128, u16, u32, u64, u8, usize};
/// Numbers which have upper and lower bounds
pub trait Bounded {
    /// Returns the smallest finite number this type can represent
    fn min_value() -> Self;
    /// Returns the largest finite number this type can represent
    fn max_value() -> Self;
}
/// Numbers which have lower bounds
pub trait LowerBounded {
    /// Returns the smallest finite number this type can represent
    fn min_value() -> Self;
}
impl<T: Bounded> LowerBounded for T {
    fn min_value() -> T {
        Bounded::min_value()
    }
}
/// Numbers which have upper bounds
pub trait UpperBounded {
    /// Returns the largest finite number this type can represent
    fn max_value() -> Self;
}
impl<T: Bounded> UpperBounded for T {
    fn max_value() -> T {
        Bounded::max_value()
    }
}
macro_rules! bounded_impl {
    ($t:ty, $min:expr, $max:expr) => {
        impl Bounded for $t { #[inline] fn min_value() -> $t { $min } #[inline] fn
        max_value() -> $t { $max } }
    };
}
bounded_impl!(usize, usize::MIN, usize::MAX);
bounded_impl!(u8, u8::MIN, u8::MAX);
bounded_impl!(u16, u16::MIN, u16::MAX);
bounded_impl!(u32, u32::MIN, u32::MAX);
bounded_impl!(u64, u64::MIN, u64::MAX);
bounded_impl!(u128, u128::MIN, u128::MAX);
bounded_impl!(isize, isize::MIN, isize::MAX);
bounded_impl!(i8, i8::MIN, i8::MAX);
bounded_impl!(i16, i16::MIN, i16::MAX);
bounded_impl!(i32, i32::MIN, i32::MAX);
bounded_impl!(i64, i64::MIN, i64::MAX);
bounded_impl!(i128, i128::MIN, i128::MAX);
impl<T: Bounded> Bounded for Wrapping<T> {
    fn min_value() -> Self {
        Wrapping(T::min_value())
    }
    fn max_value() -> Self {
        Wrapping(T::max_value())
    }
}
bounded_impl!(f32, f32::MIN, f32::MAX);
macro_rules! for_each_tuple_ {
    ($m:ident !!) => {
        $m ! {}
    };
    ($m:ident !! $h:ident, $($t:ident,)*) => {
        $m ! { $h $($t)* } for_each_tuple_! { $m !! $($t,)* }
    };
}
macro_rules! for_each_tuple {
    ($m:ident) => {
        for_each_tuple_! { $m !! A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S,
        T, }
    };
}
macro_rules! bounded_tuple {
    ($($name:ident)*) => {
        impl <$($name : Bounded,)*> Bounded for ($($name,)*) { #[inline] fn min_value()
        -> Self { ($($name ::min_value(),)*) } #[inline] fn max_value() -> Self {
        ($($name ::max_value(),)*) } }
    };
}
for_each_tuple!(bounded_tuple);
bounded_impl!(f64, f64::MIN, f64::MAX);
#[test]
fn wrapping_bounded() {
    macro_rules! test_wrapping_bounded {
        ($($t:ty)+) => {
            $(assert_eq!(< Wrapping <$t > as Bounded >::min_value().0, <$t
            >::min_value()); assert_eq!(< Wrapping <$t > as Bounded >::max_value().0, <$t
            >::max_value());)+
        };
    }
    test_wrapping_bounded!(usize u8 u16 u32 u64 isize i8 i16 i32 i64);
}
#[test]
fn wrapping_bounded_i128() {
    macro_rules! test_wrapping_bounded {
        ($($t:ty)+) => {
            $(assert_eq!(< Wrapping <$t > as Bounded >::min_value().0, <$t
            >::min_value()); assert_eq!(< Wrapping <$t > as Bounded >::max_value().0, <$t
            >::max_value());)+
        };
    }
    test_wrapping_bounded!(u128 i128);
}
#[test]
fn wrapping_is_bounded() {
    fn require_bounded<T: Bounded>(_: &T) {}
    require_bounded(&Wrapping(42_u32));
    require_bounded(&Wrapping(-42));
}
#[cfg(test)]
mod tests_llm_16_147_llm_16_147 {
    use super::*;
    use crate::*;
    use crate::Bounded;
    #[test]
    fn test_max_value_i32() {
        let _rug_st_tests_llm_16_147_llm_16_147_rrrruuuugggg_test_max_value_i32 = 0;
        debug_assert_eq!(< i32 as Bounded > ::max_value(), i32::max_value());
        let _rug_ed_tests_llm_16_147_llm_16_147_rrrruuuugggg_test_max_value_i32 = 0;
    }
    #[test]
    fn test_max_value_u32() {
        let _rug_st_tests_llm_16_147_llm_16_147_rrrruuuugggg_test_max_value_u32 = 0;
        debug_assert_eq!(< u32 as Bounded > ::max_value(), u32::max_value());
        let _rug_ed_tests_llm_16_147_llm_16_147_rrrruuuugggg_test_max_value_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_148 {
    use super::*;
    use crate::*;
    #[test]
    fn test_min_value() {
        let _rug_st_tests_llm_16_148_rrrruuuugggg_test_min_value = 0;
        debug_assert_eq!(< () as bounds::Bounded > ::min_value(), ());
        let _rug_ed_tests_llm_16_148_rrrruuuugggg_test_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_153_llm_16_153 {
    use crate::bounds::Bounded;
    #[test]
    fn max_value_test() {
        let _rug_st_tests_llm_16_153_llm_16_153_rrrruuuugggg_max_value_test = 0;
        let max_value = <(
            i8,
            i16,
            i32,
            i64,
            i128,
            u8,
            u16,
            u32,
            u64,
            u128,
        ) as Bounded>::max_value();
        debug_assert_eq!(
            max_value, (i8::max_value(), i16::max_value(), i32::max_value(),
            i64::max_value(), i128::max_value(), u8::max_value(), u16::max_value(),
            u32::max_value(), u64::max_value(), u128::max_value(),)
        );
        let _rug_ed_tests_llm_16_153_llm_16_153_rrrruuuugggg_max_value_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_155_llm_16_155 {
    use crate::bounds::Bounded;
    #[test]
    fn test_max_value() {
        let _rug_st_tests_llm_16_155_llm_16_155_rrrruuuugggg_test_max_value = 0;
        debug_assert_eq!(
            (< (i32, u32, i16, u16, i8, u8, isize, usize, i64, u64, i128, u128) as
            Bounded > ::max_value()), (i32::max_value(), u32::max_value(),
            i16::max_value(), u16::max_value(), i8::max_value(), u8::max_value(),
            isize::max_value(), usize::max_value(), i64::max_value(), u64::max_value(),
            i128::max_value(), u128::max_value(),)
        );
        debug_assert_eq!(
            (< (f32, f64) as Bounded > ::max_value()), (< f32 as crate ::float::Float >
            ::max_value(), < f64 as crate ::float::Float > ::max_value(),)
        );
        let _rug_ed_tests_llm_16_155_llm_16_155_rrrruuuugggg_test_max_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_157_llm_16_157 {
    use crate::bounds::Bounded;
    #[test]
    fn test_max_value() {
        let _rug_st_tests_llm_16_157_llm_16_157_rrrruuuugggg_test_max_value = 0;
        let max_value = <(
            i32,
            u32,
            i64,
            u64,
            i8,
            u8,
            i16,
            u16,
            isize,
            usize,
            f32,
            f64,
        ) as Bounded>::max_value();
        debug_assert_eq!(
            max_value, (i32::max_value(), u32::max_value(), i64::max_value(),
            u64::max_value(), i8::max_value(), u8::max_value(), i16::max_value(),
            u16::max_value(), isize::max_value(), usize::max_value(), f32::MAX,
            f64::MAX,)
        );
        let _rug_ed_tests_llm_16_157_llm_16_157_rrrruuuugggg_test_max_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_158_llm_16_158 {
    use crate::bounds::Bounded;
    #[test]
    fn test_min_value() {
        let _rug_st_tests_llm_16_158_llm_16_158_rrrruuuugggg_test_min_value = 0;
        debug_assert_eq!(
            (< (u8, u16, u32, u64, usize, i8, i16, i32, i64, isize) as Bounded >
            ::min_value()), (u8::min_value(), u16::min_value(), u32::min_value(),
            u64::min_value(), usize::min_value(), i8::min_value(), i16::min_value(),
            i32::min_value(), i64::min_value(), isize::min_value(),)
        );
        let _rug_ed_tests_llm_16_158_llm_16_158_rrrruuuugggg_test_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_159_llm_16_159 {
    use crate::bounds::Bounded;
    #[test]
    fn test_max_value() {
        let _rug_st_tests_llm_16_159_llm_16_159_rrrruuuugggg_test_max_value = 0;
        let max_val = <(
            i32,
            u32,
            i64,
            u64,
            i8,
            u8,
            i16,
            u16,
            isize,
            usize,
            i128,
            u128,
        ) as Bounded>::max_value();
        debug_assert_eq!(
            max_val, (i32::max_value(), u32::max_value(), i64::max_value(),
            u64::max_value(), i8::max_value(), u8::max_value(), i16::max_value(),
            u16::max_value(), isize::max_value(), usize::max_value(), i128::max_value(),
            u128::max_value(),)
        );
        let _rug_ed_tests_llm_16_159_llm_16_159_rrrruuuugggg_test_max_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_164_llm_16_164 {
    use crate::bounds::Bounded;
    #[test]
    fn test_min_value() {
        let _rug_st_tests_llm_16_164_llm_16_164_rrrruuuugggg_test_min_value = 0;
        debug_assert_eq!(
            < (i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64) as Bounded >
            ::min_value(), (i8::min_value(), i16::min_value(), i32::min_value(),
            i64::min_value(), i128::min_value(), u8::min_value(), u16::min_value(),
            u32::min_value(), u64::min_value(), u128::min_value(), f32::MIN, f64::MIN,)
        );
        let _rug_ed_tests_llm_16_164_llm_16_164_rrrruuuugggg_test_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_165_llm_16_165 {
    use crate::Bounded;
    #[test]
    fn test_max_value() {
        let _rug_st_tests_llm_16_165_llm_16_165_rrrruuuugggg_test_max_value = 0;
        let max_values = <(
            i32,
            i64,
            u32,
            u64,
            i128,
            u128,
            isize,
            usize,
        ) as Bounded>::max_value();
        debug_assert_eq!(
            max_values, (i32::max_value(), i64::max_value(), u32::max_value(),
            u64::max_value(), i128::max_value(), u128::max_value(), isize::max_value(),
            usize::max_value(),)
        );
        let _rug_ed_tests_llm_16_165_llm_16_165_rrrruuuugggg_test_max_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_166_llm_16_166 {
    use crate::bounds::Bounded;
    #[test]
    fn test_min_value() {
        let _rug_st_tests_llm_16_166_llm_16_166_rrrruuuugggg_test_min_value = 0;
        type Tuple = (i32, i64, i8, i16, u32, u64, u8, u16, usize, isize);
        let result = <Tuple as Bounded>::min_value();
        let expected = (
            i32::min_value(),
            i64::min_value(),
            i8::min_value(),
            i16::min_value(),
            u32::min_value(),
            u64::min_value(),
            u8::min_value(),
            u16::min_value(),
            usize::min_value(),
            isize::min_value(),
        );
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_166_llm_16_166_rrrruuuugggg_test_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_167_llm_16_167 {
    use crate::Bounded;
    #[test]
    fn test_max_value() {
        let _rug_st_tests_llm_16_167_llm_16_167_rrrruuuugggg_test_max_value = 0;
        let max: (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = Bounded::max_value();
        debug_assert_eq!(
            max, (i32::max_value(), i32::max_value(), i32::max_value(), i32::max_value(),
            i32::max_value(), i32::max_value(), i32::max_value(), i32::max_value(),
            i32::max_value(), i32::max_value(), i32::max_value())
        );
        let _rug_ed_tests_llm_16_167_llm_16_167_rrrruuuugggg_test_max_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_168_llm_16_168 {
    use crate::bounds::Bounded;
    #[test]
    fn test_min_value() {
        let _rug_st_tests_llm_16_168_llm_16_168_rrrruuuugggg_test_min_value = 0;
        let min_val = (<(
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
        ) as Bounded>::min_value)();
        debug_assert_eq!(
            min_val, (i32::min_value(), i32::min_value(), i32::min_value(),
            i32::min_value(), i32::min_value(), i32::min_value(), i32::min_value(),
            i32::min_value(), i32::min_value(), i32::min_value(), i32::min_value(),)
        );
        let _rug_ed_tests_llm_16_168_llm_16_168_rrrruuuugggg_test_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_169_llm_16_169 {
    use crate::bounds::Bounded;
    #[test]
    fn test_max_value() {
        let _rug_st_tests_llm_16_169_llm_16_169_rrrruuuugggg_test_max_value = 0;
        let max_val = <(
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
        ) as Bounded>::max_value();
        debug_assert_eq!(
            max_val, (i32::max_value(), i32::max_value(), i32::max_value(),
            i32::max_value(), i32::max_value(), i32::max_value(), i32::max_value(),
            i32::max_value(), i32::max_value(), i32::max_value())
        );
        let _rug_ed_tests_llm_16_169_llm_16_169_rrrruuuugggg_test_max_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_170_llm_16_170 {
    use crate::bounds::Bounded;
    #[test]
    fn test_min_value() {
        let _rug_st_tests_llm_16_170_llm_16_170_rrrruuuugggg_test_min_value = 0;
        let min_value = <(
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32,
        ) as Bounded>::min_value();
        debug_assert_eq!(
            min_value, (i32::min_value(), i32::min_value(), i32::min_value(),
            i32::min_value(), i32::min_value(), i32::min_value(), i32::min_value(),
            i32::min_value(), i32::min_value(), i32::min_value())
        );
        let _rug_ed_tests_llm_16_170_llm_16_170_rrrruuuugggg_test_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_171_llm_16_171 {
    use crate::bounds::Bounded;
    #[test]
    fn test_max_value() {
        let _rug_st_tests_llm_16_171_llm_16_171_rrrruuuugggg_test_max_value = 0;
        debug_assert_eq!(
            < (i8, i16, i32, i64, i128, u8, u16, u32, u64) as Bounded > ::max_value(),
            (i8::max_value(), i16::max_value(), i32::max_value(), i64::max_value(),
            i128::max_value(), u8::max_value(), u16::max_value(), u32::max_value(),
            u64::max_value())
        );
        let _rug_ed_tests_llm_16_171_llm_16_171_rrrruuuugggg_test_max_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_172_llm_16_172 {
    use crate::Bounded;
    #[test]
    fn test_min_value() {
        let _rug_st_tests_llm_16_172_llm_16_172_rrrruuuugggg_test_min_value = 0;
        let min_value = <(
            u8,
            i16,
            u32,
            i64,
            u128,
            isize,
            usize,
            f32,
            f64,
        ) as Bounded>::min_value();
        debug_assert_eq!(
            min_value, (u8::min_value(), i16::min_value(), u32::min_value(),
            i64::min_value(), u128::min_value(), isize::min_value(), usize::min_value(),
            f32::MIN, f64::MIN,)
        );
        let _rug_ed_tests_llm_16_172_llm_16_172_rrrruuuugggg_test_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_173_llm_16_173 {
    use crate::Bounded;
    #[test]
    fn test_max_value() {
        let _rug_st_tests_llm_16_173_llm_16_173_rrrruuuugggg_test_max_value = 0;
        debug_assert_eq!(
            (< (i32, i64, u32, u64, i8, i16, u8, u16) as Bounded > ::max_value()),
            (i32::max_value(), i64::max_value(), u32::max_value(), u64::max_value(),
            i8::max_value(), i16::max_value(), u8::max_value(), u16::max_value(),)
        );
        let _rug_ed_tests_llm_16_173_llm_16_173_rrrruuuugggg_test_max_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_174_llm_16_174 {
    use crate::bounds::Bounded;
    #[test]
    fn test_min_value() {
        let _rug_st_tests_llm_16_174_llm_16_174_rrrruuuugggg_test_min_value = 0;
        debug_assert_eq!(
            < (i32, i32, i32, i32, i32, i32, i32, i32) as Bounded > ::min_value(),
            (i32::min_value(), i32::min_value(), i32::min_value(), i32::min_value(),
            i32::min_value(), i32::min_value(), i32::min_value(), i32::min_value(),)
        );
        let _rug_ed_tests_llm_16_174_llm_16_174_rrrruuuugggg_test_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_175_llm_16_175 {
    use crate::Bounded;
    #[test]
    fn test_max_value() {
        let _rug_st_tests_llm_16_175_llm_16_175_rrrruuuugggg_test_max_value = 0;
        let max = <(i32, u32, i64, u64, i8, u8, i16) as Bounded>::max_value();
        debug_assert_eq!(
            max, (i32::max_value(), u32::max_value(), i64::max_value(), u64::max_value(),
            i8::max_value(), u8::max_value(), i16::max_value(),)
        );
        let _rug_ed_tests_llm_16_175_llm_16_175_rrrruuuugggg_test_max_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_176_llm_16_176 {
    use super::*;
    use crate::*;
    use crate::bounds::Bounded;
    #[test]
    fn test_min_value() {
        let _rug_st_tests_llm_16_176_llm_16_176_rrrruuuugggg_test_min_value = 0;
        let expected_min_values = (
            i32::min_value(),
            <f32 as crate::bounds::LowerBounded>::min_value(),
        );
        let min_values: (i32, f32) = Bounded::min_value();
        debug_assert_eq!(min_values, expected_min_values);
        let _rug_ed_tests_llm_16_176_llm_16_176_rrrruuuugggg_test_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_177_llm_16_177 {
    use crate::bounds::Bounded;
    #[test]
    fn test_max_value() {
        let _rug_st_tests_llm_16_177_llm_16_177_rrrruuuugggg_test_max_value = 0;
        let max = <(i32, f32, u32, i64, f64, u64) as Bounded>::max_value();
        debug_assert_eq!(
            max, (i32::max_value(), f32::MAX, u32::max_value(), i64::max_value(),
            f64::MAX, u64::max_value(),)
        );
        let _rug_ed_tests_llm_16_177_llm_16_177_rrrruuuugggg_test_max_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_178_llm_16_178 {
    use super::*;
    use crate::*;
    #[test]
    fn test_min_value() {
        let _rug_st_tests_llm_16_178_llm_16_178_rrrruuuugggg_test_min_value = 0;
        let min_value = <(i32, i64, u32, u64, i8, u8) as Bounded>::min_value();
        debug_assert_eq!(
            min_value, (i32::min_value(), i64::min_value(), u32::min_value(),
            u64::min_value(), i8::min_value(), u8::min_value())
        );
        let _rug_ed_tests_llm_16_178_llm_16_178_rrrruuuugggg_test_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_179_llm_16_179 {
    use super::*;
    use crate::*;
    use crate::bounds::Bounded;
    #[test]
    fn test_max_value() {
        let _rug_st_tests_llm_16_179_llm_16_179_rrrruuuugggg_test_max_value = 0;
        let max = <(i32, u32, i64, u64, i8) as Bounded>::max_value();
        debug_assert_eq!(
            max, (i32::max_value(), u32::max_value(), i64::max_value(), u64::max_value(),
            i8::max_value())
        );
        let _rug_ed_tests_llm_16_179_llm_16_179_rrrruuuugggg_test_max_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_180_llm_16_180 {
    use super::*;
    use crate::*;
    use crate::bounds::Bounded;
    #[test]
    fn test_min_value() {
        let _rug_st_tests_llm_16_180_llm_16_180_rrrruuuugggg_test_min_value = 0;
        let min_value = <(i32, i64, i8, i16, u32) as Bounded>::min_value();
        debug_assert_eq!(
            min_value, (i32::min_value(), i64::min_value(), i8::min_value(),
            i16::min_value(), u32::min_value(),)
        );
        let _rug_ed_tests_llm_16_180_llm_16_180_rrrruuuugggg_test_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_182_llm_16_182 {
    use crate::bounds::Bounded;
    #[test]
    fn test_min_value() {
        let _rug_st_tests_llm_16_182_llm_16_182_rrrruuuugggg_test_min_value = 0;
        debug_assert_eq!(
            < (i32, u64, i8, u16) as Bounded > ::min_value(), (i32::min_value(),
            u64::min_value(), i8::min_value(), u16::min_value())
        );
        let _rug_ed_tests_llm_16_182_llm_16_182_rrrruuuugggg_test_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_183_llm_16_183 {
    use crate::bounds::Bounded;
    #[test]
    fn test_max_value() {
        let _rug_st_tests_llm_16_183_llm_16_183_rrrruuuugggg_test_max_value = 0;
        let max_val: (i32, f64, u8) = Bounded::max_value();
        debug_assert_eq!(max_val, (i32::max_value(), f64::MAX, u8::max_value()));
        let _rug_ed_tests_llm_16_183_llm_16_183_rrrruuuugggg_test_max_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_184_llm_16_184 {
    use crate::Bounded;
    #[test]
    fn test_min_value() {
        let _rug_st_tests_llm_16_184_llm_16_184_rrrruuuugggg_test_min_value = 0;
        let min_value = <(i32, i32, i32) as Bounded>::min_value();
        debug_assert_eq!(
            min_value, (i32::min_value(), i32::min_value(), i32::min_value())
        );
        let min_value = <(u32, u32, u32) as Bounded>::min_value();
        debug_assert_eq!(
            min_value, (u32::min_value(), u32::min_value(), u32::min_value())
        );
        let min_value = <(f32, f32, f32) as Bounded>::min_value();
        debug_assert!(min_value.0.is_infinite() && min_value.0.is_sign_negative());
        debug_assert!(min_value.1.is_infinite() && min_value.1.is_sign_negative());
        debug_assert!(min_value.2.is_infinite() && min_value.2.is_sign_negative());
        let _rug_ed_tests_llm_16_184_llm_16_184_rrrruuuugggg_test_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_185_llm_16_185 {
    use crate::bounds::Bounded;
    #[test]
    fn test_max_value() {
        let _rug_st_tests_llm_16_185_llm_16_185_rrrruuuugggg_test_max_value = 0;
        #[derive(Debug, PartialEq)]
        struct TestType;
        impl Bounded for TestType {
            fn min_value() -> Self {
                TestType
            }
            fn max_value() -> Self {
                TestType
            }
        }
        type S = TestType;
        type T = TestType;
        let max_value = <(S, T) as Bounded>::max_value();
        let expected_max_value_s = S::max_value();
        let expected_max_value_t = T::max_value();
        debug_assert_eq!(max_value, (expected_max_value_s, expected_max_value_t));
        let _rug_ed_tests_llm_16_185_llm_16_185_rrrruuuugggg_test_max_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_186_llm_16_186 {
    use crate::Bounded;
    #[test]
    fn test_min_value() {
        let _rug_st_tests_llm_16_186_llm_16_186_rrrruuuugggg_test_min_value = 0;
        debug_assert_eq!(
            < (i32, i32) as Bounded > ::min_value(), (i32::min_value(), i32::min_value())
        );
        debug_assert_eq!(
            < (u32, f64) as Bounded > ::min_value(), (u32::min_value(), f64::MIN)
        );
        let _rug_ed_tests_llm_16_186_llm_16_186_rrrruuuugggg_test_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_187_llm_16_187 {
    use crate::Bounded;
    #[test]
    fn test_max_value() {
        let _rug_st_tests_llm_16_187_llm_16_187_rrrruuuugggg_test_max_value = 0;
        debug_assert_eq!((i32::max_value(),), < (i32,) as Bounded > ::max_value());
        debug_assert_eq!((f32::MAX,), < (f32,) as Bounded > ::max_value());
        debug_assert_eq!((u64::max_value(),), < (u64,) as Bounded > ::max_value());
        let _rug_ed_tests_llm_16_187_llm_16_187_rrrruuuugggg_test_max_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_188_llm_16_188 {
    use crate::Bounded;
    #[test]
    fn test_min_value() {
        let _rug_st_tests_llm_16_188_llm_16_188_rrrruuuugggg_test_min_value = 0;
        debug_assert_eq!(< (i32,) as Bounded > ::min_value(), (i32::min_value(),));
        debug_assert_eq!(< (u32,) as Bounded > ::min_value(), (u32::min_value(),));
        debug_assert_eq!(
            < (i32, i64) as Bounded > ::min_value(), (i32::min_value(), i64::min_value())
        );
        debug_assert_eq!(
            < (u32, u64) as Bounded > ::min_value(), (u32::min_value(), u64::min_value())
        );
        let _rug_ed_tests_llm_16_188_llm_16_188_rrrruuuugggg_test_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_189_llm_16_189 {
    use crate::bounds::LowerBounded;
    #[test]
    fn test_min_value() {
        let _rug_st_tests_llm_16_189_llm_16_189_rrrruuuugggg_test_min_value = 0;
        debug_assert_eq!(< i32 as LowerBounded > ::min_value(), i32::min_value());
        debug_assert_eq!(< u32 as LowerBounded > ::min_value(), u32::min_value());
        let _rug_ed_tests_llm_16_189_llm_16_189_rrrruuuugggg_test_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_190_llm_16_190 {
    use crate::bounds::{Bounded, UpperBounded};
    #[test]
    fn max_value_test() {
        let _rug_st_tests_llm_16_190_llm_16_190_rrrruuuugggg_max_value_test = 0;
        debug_assert_eq!(< u32 as UpperBounded > ::max_value(), u32::MAX);
        debug_assert_eq!(< i32 as UpperBounded > ::max_value(), i32::MAX);
        debug_assert_eq!(< f32 as UpperBounded > ::max_value(), f32::INFINITY);
        let _rug_ed_tests_llm_16_190_llm_16_190_rrrruuuugggg_max_value_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_266 {
    use super::*;
    use crate::*;
    #[test]
    fn test_max_value() {
        let _rug_st_tests_llm_16_266_rrrruuuugggg_test_max_value = 0;
        let max_float: f32 = f32::MAX;
        debug_assert_eq!(< f32 as bounds::Bounded > ::max_value(), max_float);
        let _rug_ed_tests_llm_16_266_rrrruuuugggg_test_max_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_267 {
    use super::*;
    use crate::*;
    #[test]
    fn test_f32_min_value() {
        let _rug_st_tests_llm_16_267_rrrruuuugggg_test_f32_min_value = 0;
        let min_val: f32 = <f32 as bounds::Bounded>::min_value();
        debug_assert_eq!(min_val, f32::MIN);
        let _rug_ed_tests_llm_16_267_rrrruuuugggg_test_f32_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_434 {
    use crate::bounds::Bounded;
    #[test]
    fn test_f64_max_value() {
        let _rug_st_tests_llm_16_434_rrrruuuugggg_test_f64_max_value = 0;
        let max_val: f64 = <f64 as Bounded>::max_value();
        debug_assert_eq!(max_val, f64::MAX);
        let _rug_ed_tests_llm_16_434_rrrruuuugggg_test_f64_max_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_435_llm_16_435 {
    use crate::Bounded;
    #[test]
    fn test_min_value_for_f64() {
        let _rug_st_tests_llm_16_435_llm_16_435_rrrruuuugggg_test_min_value_for_f64 = 0;
        let min_val = <f64 as Bounded>::min_value();
        debug_assert_eq!(min_val, std::f64::MIN);
        let _rug_ed_tests_llm_16_435_llm_16_435_rrrruuuugggg_test_min_value_for_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_600_llm_16_600 {
    use crate::Bounded;
    #[test]
    fn test_max_value_i128() {
        let _rug_st_tests_llm_16_600_llm_16_600_rrrruuuugggg_test_max_value_i128 = 0;
        debug_assert_eq!(< i128 as Bounded > ::max_value(), i128::MAX);
        let _rug_ed_tests_llm_16_600_llm_16_600_rrrruuuugggg_test_max_value_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_601 {
    use super::*;
    use crate::*;
    #[test]
    fn test_min_value_for_i128() {
        let _rug_st_tests_llm_16_601_rrrruuuugggg_test_min_value_for_i128 = 0;
        debug_assert_eq!(i128::min_value(), std::i128::MIN);
        let _rug_ed_tests_llm_16_601_rrrruuuugggg_test_min_value_for_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_710 {
    use super::*;
    use crate::*;
    #[test]
    fn test_max_value_i16() {
        let _rug_st_tests_llm_16_710_rrrruuuugggg_test_max_value_i16 = 0;
        debug_assert_eq!(< i16 as bounds::Bounded > ::max_value(), i16::MAX);
        let _rug_ed_tests_llm_16_710_rrrruuuugggg_test_max_value_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_711_llm_16_711 {
    use crate::bounds::Bounded;
    #[test]
    fn min_value_test() {
        let _rug_st_tests_llm_16_711_llm_16_711_rrrruuuugggg_min_value_test = 0;
        debug_assert_eq!(< i16 as Bounded > ::min_value(), i16::MIN);
        let _rug_ed_tests_llm_16_711_llm_16_711_rrrruuuugggg_min_value_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_820 {
    use super::*;
    use crate::*;
    #[test]
    fn test_max_value() {
        let _rug_st_tests_llm_16_820_rrrruuuugggg_test_max_value = 0;
        debug_assert_eq!(< i32 as bounds::Bounded > ::max_value(), i32::MAX);
        let _rug_ed_tests_llm_16_820_rrrruuuugggg_test_max_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_821 {
    use super::*;
    use crate::*;
    #[test]
    fn test_min_value() {
        let _rug_st_tests_llm_16_821_rrrruuuugggg_test_min_value = 0;
        debug_assert_eq!(< i32 as bounds::Bounded > ::min_value(), i32::MIN);
        let _rug_ed_tests_llm_16_821_rrrruuuugggg_test_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_930_llm_16_930 {
    use crate::bounds::Bounded;
    #[test]
    fn test_max_value_for_i64() {
        let _rug_st_tests_llm_16_930_llm_16_930_rrrruuuugggg_test_max_value_for_i64 = 0;
        debug_assert_eq!(< i64 as Bounded > ::max_value(), i64::MAX);
        let _rug_ed_tests_llm_16_930_llm_16_930_rrrruuuugggg_test_max_value_for_i64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_931 {
    use super::*;
    use crate::*;
    #[test]
    fn test_min_value() {
        let _rug_st_tests_llm_16_931_rrrruuuugggg_test_min_value = 0;
        debug_assert_eq!(< i64 as bounds::Bounded > ::min_value(), i64::MIN);
        let _rug_ed_tests_llm_16_931_rrrruuuugggg_test_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1040 {
    use super::*;
    use crate::*;
    #[test]
    fn test_max_value_i8() {
        let _rug_st_tests_llm_16_1040_rrrruuuugggg_test_max_value_i8 = 0;
        debug_assert_eq!(i8::max_value(), 127);
        let _rug_ed_tests_llm_16_1040_rrrruuuugggg_test_max_value_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1041 {
    use super::*;
    use crate::*;
    #[test]
    fn test_min_value() {
        let _rug_st_tests_llm_16_1041_rrrruuuugggg_test_min_value = 0;
        debug_assert_eq!(i8::min_value(), i8::MIN);
        let _rug_ed_tests_llm_16_1041_rrrruuuugggg_test_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1150_llm_16_1150 {
    use crate::Bounded;
    #[test]
    fn test_isize_max_value() {
        let _rug_st_tests_llm_16_1150_llm_16_1150_rrrruuuugggg_test_isize_max_value = 0;
        debug_assert_eq!(isize::max_value(), < isize as Bounded > ::max_value());
        let _rug_ed_tests_llm_16_1150_llm_16_1150_rrrruuuugggg_test_isize_max_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1151_llm_16_1151 {
    use crate::Bounded;
    #[test]
    fn test_min_value() {
        let _rug_st_tests_llm_16_1151_llm_16_1151_rrrruuuugggg_test_min_value = 0;
        debug_assert_eq!(< isize as Bounded > ::min_value(), isize::MIN);
        let _rug_ed_tests_llm_16_1151_llm_16_1151_rrrruuuugggg_test_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1260_llm_16_1260 {
    use crate::bounds::Bounded;
    use core::num::Wrapping;
    #[test]
    fn test_max_value() {
        let _rug_st_tests_llm_16_1260_llm_16_1260_rrrruuuugggg_test_max_value = 0;
        debug_assert_eq!(Wrapping(i32::max_value()), Bounded::max_value());
        debug_assert_eq!(Wrapping(u32::max_value()), Bounded::max_value());
        debug_assert_eq!(Wrapping(i64::max_value()), Bounded::max_value());
        debug_assert_eq!(Wrapping(u64::max_value()), Bounded::max_value());
        debug_assert_eq!(Wrapping(isize::max_value()), Bounded::max_value());
        debug_assert_eq!(Wrapping(usize::max_value()), Bounded::max_value());
        debug_assert_eq!(Wrapping(i8::max_value()), Bounded::max_value());
        debug_assert_eq!(Wrapping(u8::max_value()), Bounded::max_value());
        debug_assert_eq!(Wrapping(i16::max_value()), Bounded::max_value());
        debug_assert_eq!(Wrapping(u16::max_value()), Bounded::max_value());
        let _rug_ed_tests_llm_16_1260_llm_16_1260_rrrruuuugggg_test_max_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1261_llm_16_1261 {
    use std::num::Wrapping;
    use crate::Bounded;
    #[test]
    fn test_wrapping_min_value() {
        let _rug_st_tests_llm_16_1261_llm_16_1261_rrrruuuugggg_test_wrapping_min_value = 0;
        debug_assert_eq!(
            Wrapping(i8::min_value()), < Wrapping < i8 > as Bounded > ::min_value()
        );
        debug_assert_eq!(
            Wrapping(i16::min_value()), < Wrapping < i16 > as Bounded > ::min_value()
        );
        debug_assert_eq!(
            Wrapping(i32::min_value()), < Wrapping < i32 > as Bounded > ::min_value()
        );
        debug_assert_eq!(
            Wrapping(i64::min_value()), < Wrapping < i64 > as Bounded > ::min_value()
        );
        debug_assert_eq!(
            Wrapping(i128::min_value()), < Wrapping < i128 > as Bounded > ::min_value()
        );
        debug_assert_eq!(
            Wrapping(u8::min_value()), < Wrapping < u8 > as Bounded > ::min_value()
        );
        debug_assert_eq!(
            Wrapping(u16::min_value()), < Wrapping < u16 > as Bounded > ::min_value()
        );
        debug_assert_eq!(
            Wrapping(u32::min_value()), < Wrapping < u32 > as Bounded > ::min_value()
        );
        debug_assert_eq!(
            Wrapping(u64::min_value()), < Wrapping < u64 > as Bounded > ::min_value()
        );
        debug_assert_eq!(
            Wrapping(u128::min_value()), < Wrapping < u128 > as Bounded > ::min_value()
        );
        let _rug_ed_tests_llm_16_1261_llm_16_1261_rrrruuuugggg_test_wrapping_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1356 {
    use super::*;
    use crate::*;
    #[test]
    fn test_max_value() {
        let _rug_st_tests_llm_16_1356_rrrruuuugggg_test_max_value = 0;
        debug_assert_eq!(u128::max_value(), std::u128::MAX);
        let _rug_ed_tests_llm_16_1356_rrrruuuugggg_test_max_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1357 {
    use super::*;
    use crate::*;
    #[test]
    fn test_u128_min_value() {
        let _rug_st_tests_llm_16_1357_rrrruuuugggg_test_u128_min_value = 0;
        debug_assert_eq!(u128::min_value(), 0u128);
        let _rug_ed_tests_llm_16_1357_rrrruuuugggg_test_u128_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1461 {
    use super::*;
    use crate::*;
    #[test]
    fn test_max_value_u16() {
        let _rug_st_tests_llm_16_1461_rrrruuuugggg_test_max_value_u16 = 0;
        debug_assert_eq!(u16::max_value(), u16::MAX);
        let _rug_ed_tests_llm_16_1461_rrrruuuugggg_test_max_value_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1462_llm_16_1462 {
    use crate::bounds::Bounded;
    #[test]
    fn test_min_value_for_u16() {
        let _rug_st_tests_llm_16_1462_llm_16_1462_rrrruuuugggg_test_min_value_for_u16 = 0;
        debug_assert_eq!(< u16 as Bounded > ::min_value(), 0u16);
        let _rug_ed_tests_llm_16_1462_llm_16_1462_rrrruuuugggg_test_min_value_for_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1566 {
    use crate::Bounded;
    #[test]
    fn test_u32_max_value() {
        let _rug_st_tests_llm_16_1566_rrrruuuugggg_test_u32_max_value = 0;
        debug_assert_eq!(u32::max_value(), std::u32::MAX);
        let _rug_ed_tests_llm_16_1566_rrrruuuugggg_test_u32_max_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1567 {
    use super::*;
    use crate::*;
    #[test]
    fn test_min_value() {
        let _rug_st_tests_llm_16_1567_rrrruuuugggg_test_min_value = 0;
        debug_assert_eq!(< u32 as bounds::Bounded > ::min_value(), u32::min_value());
        let _rug_ed_tests_llm_16_1567_rrrruuuugggg_test_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1671 {
    use super::*;
    use crate::*;
    #[test]
    fn test_max_value() {
        let _rug_st_tests_llm_16_1671_rrrruuuugggg_test_max_value = 0;
        debug_assert_eq!(< u64 as bounds::Bounded > ::max_value(), u64::MAX);
        let _rug_ed_tests_llm_16_1671_rrrruuuugggg_test_max_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1672 {
    use super::*;
    use crate::*;
    #[test]
    fn test_min_value() {
        let _rug_st_tests_llm_16_1672_rrrruuuugggg_test_min_value = 0;
        debug_assert_eq!(< u64 as bounds::Bounded > ::min_value(), u64::MIN);
        let _rug_ed_tests_llm_16_1672_rrrruuuugggg_test_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1776 {
    use super::*;
    use crate::*;
    #[test]
    fn test_max_value_u8() {
        let _rug_st_tests_llm_16_1776_rrrruuuugggg_test_max_value_u8 = 0;
        debug_assert_eq!(u8::max_value(), 255);
        let _rug_ed_tests_llm_16_1776_rrrruuuugggg_test_max_value_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1777 {
    use super::*;
    use crate::*;
    #[test]
    fn test_min_value_for_u8() {
        let _rug_st_tests_llm_16_1777_rrrruuuugggg_test_min_value_for_u8 = 0;
        debug_assert_eq!(< u8 as bounds::Bounded > ::min_value(), 0u8);
        let _rug_ed_tests_llm_16_1777_rrrruuuugggg_test_min_value_for_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1882 {
    use crate::bounds::Bounded;
    #[test]
    fn max_value_test() {
        let _rug_st_tests_llm_16_1882_rrrruuuugggg_max_value_test = 0;
        debug_assert_eq!(< usize as Bounded > ::max_value(), usize::MAX);
        let _rug_ed_tests_llm_16_1882_rrrruuuugggg_max_value_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1883 {
    use super::*;
    use crate::*;
    #[test]
    fn test_min_value() {
        let _rug_st_tests_llm_16_1883_rrrruuuugggg_test_min_value = 0;
        debug_assert_eq!(usize::min_value(), 0);
        let _rug_ed_tests_llm_16_1883_rrrruuuugggg_test_min_value = 0;
    }
}
#[cfg(test)]
mod tests_rug_148 {
    use super::*;
    use crate::Bounded;
    #[test]
    fn test_max_value() {
        let _rug_st_tests_rug_148_rrrruuuugggg_test_max_value = 0;
        type Q = u32;
        type R = i32;
        type S = u8;
        type T = i8;
        let max_values = <(Q, R, S, T) as Bounded>::max_value();
        debug_assert_eq!(
            max_values, (u32::max_value(), i32::max_value(), u8::max_value(),
            i8::max_value())
        );
        let _rug_ed_tests_rug_148_rrrruuuugggg_test_max_value = 0;
    }
}
