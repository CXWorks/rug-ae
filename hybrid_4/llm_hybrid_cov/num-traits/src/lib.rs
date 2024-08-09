//! Numeric traits for generic mathematics
//!
//! ## Compatibility
//!
//! The `num-traits` crate is tested for rustc 1.31 and greater.
#![doc(html_root_url = "https://docs.rs/num-traits/0.2")]
#[cfg(feature = "std")]
extern crate std;
use core::fmt;
use core::num::Wrapping;
use core::ops::{Add, Div, Mul, Rem, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
pub use crate::bounds::Bounded;
#[cfg(any(feature = "std", feature = "libm"))]
pub use crate::float::Float;
pub use crate::float::FloatConst;
pub use crate::cast::{cast, AsPrimitive, FromPrimitive, NumCast, ToPrimitive};
pub use crate::identities::{one, zero, One, Zero};
pub use crate::int::PrimInt;
pub use crate::ops::checked::{
    CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedRem, CheckedShl, CheckedShr,
    CheckedSub,
};
pub use crate::ops::euclid::{CheckedEuclid, Euclid};
pub use crate::ops::inv::Inv;
pub use crate::ops::mul_add::{MulAdd, MulAddAssign};
pub use crate::ops::saturating::{
    Saturating, SaturatingAdd, SaturatingMul, SaturatingSub,
};
pub use crate::ops::wrapping::{
    WrappingAdd, WrappingMul, WrappingNeg, WrappingShl, WrappingShr, WrappingSub,
};
pub use crate::pow::{checked_pow, pow, Pow};
pub use crate::sign::{abs, abs_sub, signum, Signed, Unsigned};
#[macro_use]
mod macros;
pub mod bounds;
pub mod cast;
pub mod float;
pub mod identities;
pub mod int;
pub mod ops;
pub mod pow;
pub mod real;
pub mod sign;
/// The base trait for numeric types, covering `0` and `1` values,
/// comparisons, basic numeric operations, and string conversion.
pub trait Num: PartialEq + Zero + One + NumOps {
    type FromStrRadixErr;
    /// Convert from a string and radix (typically `2..=36`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use num_traits::Num;
    ///
    /// let result = <i32 as Num>::from_str_radix("27", 10);
    /// assert_eq!(result, Ok(27));
    ///
    /// let result = <i32 as Num>::from_str_radix("foo", 10);
    /// assert!(result.is_err());
    /// ```
    ///
    /// # Supported radices
    ///
    /// The exact range of supported radices is at the discretion of each type implementation. For
    /// primitive integers, this is implemented by the inherent `from_str_radix` methods in the
    /// standard library, which **panic** if the radix is not in the range from 2 to 36. The
    /// implementation in this crate for primitive floats is similar.
    ///
    /// For third-party types, it is suggested that implementations should follow suit and at least
    /// accept `2..=36` without panicking, but an `Err` may be returned for any unsupported radix.
    /// It's possible that a type might not even support the common radix 10, nor any, if string
    /// parsing doesn't make sense for that type.
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr>;
}
/// Generic trait for types implementing basic numeric operations
///
/// This is automatically implemented for types which implement the operators.
pub trait NumOps<
    Rhs = Self,
    Output = Self,
>: Add<
        Rhs,
        Output = Output,
    > + Sub<
        Rhs,
        Output = Output,
    > + Mul<
        Rhs,
        Output = Output,
    > + Div<Rhs, Output = Output> + Rem<Rhs, Output = Output> {}
impl<T, Rhs, Output> NumOps<Rhs, Output> for T
where
    T: Add<Rhs, Output = Output> + Sub<Rhs, Output = Output> + Mul<Rhs, Output = Output>
        + Div<Rhs, Output = Output> + Rem<Rhs, Output = Output>,
{}
/// The trait for `Num` types which also implement numeric operations taking
/// the second operand by reference.
///
/// This is automatically implemented for types which implement the operators.
pub trait NumRef: Num + for<'r> NumOps<&'r Self> {}
impl<T> NumRef for T
where
    T: Num + for<'r> NumOps<&'r T>,
{}
/// The trait for `Num` references which implement numeric operations, taking the
/// second operand either by value or by reference.
///
/// This is automatically implemented for all types which implement the operators. It covers
/// every type implementing the operations though, regardless of it being a reference or
/// related to `Num`.
pub trait RefNum<Base>: NumOps<Base, Base> + for<'r> NumOps<&'r Base, Base> {}
impl<T, Base> RefNum<Base> for T
where
    T: NumOps<Base, Base> + for<'r> NumOps<&'r Base, Base>,
{}
/// Generic trait for types implementing numeric assignment operators (like `+=`).
///
/// This is automatically implemented for types which implement the operators.
pub trait NumAssignOps<
    Rhs = Self,
>: AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs> + RemAssign<Rhs> {}
impl<T, Rhs> NumAssignOps<Rhs> for T
where
    T: AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs>
        + RemAssign<Rhs>,
{}
/// The trait for `Num` types which also implement assignment operators.
///
/// This is automatically implemented for types which implement the operators.
pub trait NumAssign: Num + NumAssignOps {}
impl<T> NumAssign for T
where
    T: Num + NumAssignOps,
{}
/// The trait for `NumAssign` types which also implement assignment operations
/// taking the second operand by reference.
///
/// This is automatically implemented for types which implement the operators.
pub trait NumAssignRef: NumAssign + for<'r> NumAssignOps<&'r Self> {}
impl<T> NumAssignRef for T
where
    T: NumAssign + for<'r> NumAssignOps<&'r T>,
{}
macro_rules! int_trait_impl {
    ($name:ident for $($t:ty)*) => {
        $(impl $name for $t { type FromStrRadixErr = ::core::num::ParseIntError;
        #[inline] fn from_str_radix(s : & str, radix : u32) -> Result < Self,
        ::core::num::ParseIntError > { <$t >::from_str_radix(s, radix) } })*
    };
}
int_trait_impl!(Num for usize u8 u16 u32 u64 u128);
int_trait_impl!(Num for isize i8 i16 i32 i64 i128);
impl<T: Num> Num for Wrapping<T>
where
    Wrapping<T>: NumOps,
{
    type FromStrRadixErr = T::FromStrRadixErr;
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        T::from_str_radix(str, radix).map(Wrapping)
    }
}
#[derive(Debug)]
pub enum FloatErrorKind {
    Empty,
    Invalid,
}
#[derive(Debug)]
pub struct ParseFloatError {
    pub kind: FloatErrorKind,
}
impl fmt::Display for ParseFloatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self.kind {
            FloatErrorKind::Empty => "cannot parse float from empty string",
            FloatErrorKind::Invalid => "invalid float literal",
        };
        description.fmt(f)
    }
}
fn str_to_ascii_lower_eq_str(a: &str, b: &str) -> bool {
    a.len() == b.len()
        && a
            .bytes()
            .zip(b.bytes())
            .all(|(a, b)| {
                let a_to_ascii_lower = a | (((b'A' <= a && a <= b'Z') as u8) << 5);
                a_to_ascii_lower == b
            })
}
macro_rules! float_trait_impl {
    ($name:ident for $($t:ident)*) => {
        $(impl $name for $t { type FromStrRadixErr = ParseFloatError; fn
        from_str_radix(src : & str, radix : u32) -> Result < Self, Self::FromStrRadixErr
        > { use self::FloatErrorKind::*; use self::ParseFloatError as PFE; if radix == 10
        { return src.parse().map_err(| _ | PFE { kind : if src.is_empty() { Empty } else
        { Invalid }, }); } if str_to_ascii_lower_eq_str(src, "inf") ||
        str_to_ascii_lower_eq_str(src, "infinity") { return Ok(core::$t ::INFINITY); }
        else if str_to_ascii_lower_eq_str(src, "-inf") || str_to_ascii_lower_eq_str(src,
        "-infinity") { return Ok(core::$t ::NEG_INFINITY); } else if
        str_to_ascii_lower_eq_str(src, "nan") { return Ok(core::$t ::NAN); } else if
        str_to_ascii_lower_eq_str(src, "-nan") { return Ok(- core::$t ::NAN); } fn
        slice_shift_char(src : & str) -> Option < (char, & str) > { let mut chars = src
        .chars(); Some((chars.next() ?, chars.as_str())) } let (is_positive, src) = match
        slice_shift_char(src) { None => return Err(PFE { kind : Empty }), Some(('-', ""))
        => return Err(PFE { kind : Empty }), Some(('-', src)) => (false, src), Some((_,
        _)) => (true, src), }; let mut sig = if is_positive { 0.0 } else { - 0.0 }; let
        mut prev_sig = sig; let mut cs = src.chars().enumerate(); let mut exp_info =
        None::< (char, usize) >; for (i, c) in cs.by_ref() { match c.to_digit(radix) {
        Some(digit) => { sig *= radix as $t; if is_positive { sig += (digit as isize) as
        $t; } else { sig -= (digit as isize) as $t; } if prev_sig != 0.0 { if is_positive
        && sig <= prev_sig { return Ok(core::$t ::INFINITY); } if ! is_positive && sig >=
        prev_sig { return Ok(core::$t ::NEG_INFINITY); } if is_positive && (prev_sig !=
        (sig - digit as $t) / radix as $t) { return Ok(core::$t ::INFINITY); } if !
        is_positive && (prev_sig != (sig + digit as $t) / radix as $t) { return
        Ok(core::$t ::NEG_INFINITY); } } prev_sig = sig; }, None => match c { 'e' | 'E' |
        'p' | 'P' => { exp_info = Some((c, i + 1)); break; }, '.' => { break; }, _ => {
        return Err(PFE { kind : Invalid }); }, }, } } if exp_info.is_none() { let mut
        power = 1.0; for (i, c) in cs.by_ref() { match c.to_digit(radix) { Some(digit) =>
        { power /= radix as $t; sig = if is_positive { sig + (digit as $t) * power } else
        { sig - (digit as $t) * power }; if is_positive && sig < prev_sig { return
        Ok(core::$t ::INFINITY); } if ! is_positive && sig > prev_sig { return
        Ok(core::$t ::NEG_INFINITY); } prev_sig = sig; }, None => match c { 'e' | 'E' |
        'p' | 'P' => { exp_info = Some((c, i + 1)); break; }, _ => { return Err(PFE {
        kind : Invalid }); }, }, } } } let exp = match exp_info { Some((c, offset)) => {
        let base = match c { 'E' | 'e' if radix == 10 => 10.0, 'P' | 'p' if radix == 16
        => 2.0, _ => return Err(PFE { kind : Invalid }), }; let src = & src[offset..];
        let (is_positive, exp) = match slice_shift_char(src) { Some(('-', src)) =>
        (false, src.parse::< usize > ()), Some(('+', src)) => (true, src.parse::< usize >
        ()), Some((_, _)) => (true, src.parse::< usize > ()), None => return Err(PFE {
        kind : Invalid }), }; #[cfg(feature = "std")] fn pow(base : $t, exp : usize) ->
        $t { Float::powi(base, exp as i32) } match (is_positive, exp) { (true, Ok(exp))
        => pow(base, exp), (false, Ok(exp)) => 1.0 / pow(base, exp), (_, Err(_)) =>
        return Err(PFE { kind : Invalid }), } }, None => 1.0, }; Ok(sig * exp) } })*
    };
}
float_trait_impl!(Num for f32 f64);
/// A value bounded by a minimum and a maximum
///
///  If input is less than min then this returns min.
///  If input is greater than max then this returns max.
///  Otherwise this returns input.
///
/// **Panics** in debug mode if `!(min <= max)`.
#[inline]
pub fn clamp<T: PartialOrd>(input: T, min: T, max: T) -> T {
    debug_assert!(min <= max, "min must be less than or equal to max");
    if input < min { min } else if input > max { max } else { input }
}
/// A value bounded by a minimum value
///
///  If input is less than min then this returns min.
///  Otherwise this returns input.
///  `clamp_min(std::f32::NAN, 1.0)` preserves `NAN` different from `f32::min(std::f32::NAN, 1.0)`.
///
/// **Panics** in debug mode if `!(min == min)`. (This occurs if `min` is `NAN`.)
#[inline]
#[allow(clippy::eq_op)]
pub fn clamp_min<T: PartialOrd>(input: T, min: T) -> T {
    debug_assert!(min == min, "min must not be NAN");
    if input < min { min } else { input }
}
/// A value bounded by a maximum value
///
///  If input is greater than max then this returns max.
///  Otherwise this returns input.
///  `clamp_max(std::f32::NAN, 1.0)` preserves `NAN` different from `f32::max(std::f32::NAN, 1.0)`.
///
/// **Panics** in debug mode if `!(max == max)`. (This occurs if `max` is `NAN`.)
#[inline]
#[allow(clippy::eq_op)]
pub fn clamp_max<T: PartialOrd>(input: T, max: T) -> T {
    debug_assert!(max == max, "max must not be NAN");
    if input > max { max } else { input }
}
#[test]
fn clamp_test() {
    assert_eq!(1, clamp(1, - 1, 2));
    assert_eq!(- 1, clamp(- 2, - 1, 2));
    assert_eq!(2, clamp(3, - 1, 2));
    assert_eq!(1, clamp_min(1, - 1));
    assert_eq!(- 1, clamp_min(- 2, - 1));
    assert_eq!(- 1, clamp_max(1, - 1));
    assert_eq!(- 2, clamp_max(- 2, - 1));
    assert_eq!(1.0, clamp(1.0, - 1.0, 2.0));
    assert_eq!(- 1.0, clamp(- 2.0, - 1.0, 2.0));
    assert_eq!(2.0, clamp(3.0, - 1.0, 2.0));
    assert_eq!(1.0, clamp_min(1.0, - 1.0));
    assert_eq!(- 1.0, clamp_min(- 2.0, - 1.0));
    assert_eq!(- 1.0, clamp_max(1.0, - 1.0));
    assert_eq!(- 2.0, clamp_max(- 2.0, - 1.0));
    assert!(clamp(::core::f32::NAN, - 1.0, 1.0).is_nan());
    assert!(clamp_min(::core::f32::NAN, 1.0).is_nan());
    assert!(clamp_max(::core::f32::NAN, 1.0).is_nan());
}
#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn clamp_nan_min() {
    clamp(0., ::core::f32::NAN, 1.);
}
#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn clamp_nan_max() {
    clamp(0., -1., ::core::f32::NAN);
}
#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn clamp_nan_min_max() {
    clamp(0., ::core::f32::NAN, ::core::f32::NAN);
}
#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn clamp_min_nan_min() {
    clamp_min(0., ::core::f32::NAN);
}
#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn clamp_max_nan_max() {
    clamp_max(0., ::core::f32::NAN);
}
#[test]
fn from_str_radix_unwrap() {
    let i: i32 = Num::from_str_radix("0", 10).unwrap();
    assert_eq!(i, 0);
    let f: f32 = Num::from_str_radix("0.0", 10).unwrap();
    assert_eq!(f, 0.0);
}
#[test]
fn from_str_radix_multi_byte_fail() {
    assert!(f32::from_str_radix("™0.2", 10).is_err());
    assert!(f32::from_str_radix("0.2E™1", 10).is_err());
}
#[test]
fn from_str_radix_ignore_case() {
    assert_eq!(f32::from_str_radix("InF", 16).unwrap(), ::core::f32::INFINITY);
    assert_eq!(f32::from_str_radix("InfinitY", 16).unwrap(), ::core::f32::INFINITY);
    assert_eq!(f32::from_str_radix("-InF", 8).unwrap(), ::core::f32::NEG_INFINITY);
    assert_eq!(f32::from_str_radix("-InfinitY", 8).unwrap(), ::core::f32::NEG_INFINITY);
    assert!(f32::from_str_radix("nAn", 4).unwrap().is_nan());
    assert!(f32::from_str_radix("-nAn", 4).unwrap().is_nan());
}
#[test]
fn wrapping_is_num() {
    fn require_num<T: Num>(_: &T) {}
    require_num(&Wrapping(42_u32));
    require_num(&Wrapping(-42));
}
#[test]
fn wrapping_from_str_radix() {
    macro_rules! test_wrapping_from_str_radix {
        ($($t:ty)+) => {
            $(for & (s, r) in & [("42", 10), ("42", 2), ("-13.0", 10), ("foo", 10)] { let
            w = Wrapping::<$t >::from_str_radix(s, r).map(| w | w.0); assert_eq!(w, <$t
            as Num >::from_str_radix(s, r)); })+
        };
    }
    test_wrapping_from_str_radix!(usize u8 u16 u32 u64 isize i8 i16 i32 i64);
}
#[test]
fn check_num_ops() {
    fn compute<T: Num + Copy>(x: T, y: T) -> T {
        x * y / y % y + y - y
    }
    assert_eq!(compute(1, 2), 1)
}
#[test]
fn check_numref_ops() {
    fn compute<T: NumRef>(x: T, y: &T) -> T {
        x * y / y % y + y - y
    }
    assert_eq!(compute(1, & 2), 1)
}
#[test]
fn check_refnum_ops() {
    fn compute<T: Copy>(x: &T, y: T) -> T
    where
        for<'a> &'a T: RefNum<T>,
    {
        &(&(&(&(x * y) / y) % y) + y) - y
    }
    assert_eq!(compute(& 1, 2), 1)
}
#[test]
fn check_refref_ops() {
    fn compute<T>(x: &T, y: &T) -> T
    where
        for<'a> &'a T: RefNum<T>,
    {
        &(&(&(&(x * y) / y) % y) + y) - y
    }
    assert_eq!(compute(& 1, & 2), 1)
}
#[test]
fn check_numassign_ops() {
    fn compute<T: NumAssign + Copy>(mut x: T, y: T) -> T {
        x *= y;
        x /= y;
        x %= y;
        x += y;
        x -= y;
        x
    }
    assert_eq!(compute(1, 2), 1)
}
#[test]
fn check_numassignref_ops() {
    fn compute<T: NumAssignRef + Copy>(mut x: T, y: &T) -> T {
        x *= y;
        x /= y;
        x %= y;
        x += y;
        x -= y;
        x
    }
    assert_eq!(compute(1, & 2), 1)
}
#[cfg(test)]
mod tests_llm_16_263_llm_16_263 {
    use crate::Num;
    #[test]
    fn from_str_radix_normal() {
        let _rug_st_tests_llm_16_263_llm_16_263_rrrruuuugggg_from_str_radix_normal = 0;
        let rug_fuzz_0 = "123";
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = "1101";
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = "a";
        let rug_fuzz_5 = 16;
        debug_assert_eq!(
            < f32 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1).unwrap(), 123f32
        );
        debug_assert_eq!(
            < f32 as Num > ::from_str_radix(rug_fuzz_2, rug_fuzz_3).unwrap(), 13f32
        );
        debug_assert_eq!(
            < f32 as Num > ::from_str_radix(rug_fuzz_4, rug_fuzz_5).unwrap(), 10f32
        );
        let _rug_ed_tests_llm_16_263_llm_16_263_rrrruuuugggg_from_str_radix_normal = 0;
    }
    #[test]
    fn from_str_radix_special() {
        let _rug_st_tests_llm_16_263_llm_16_263_rrrruuuugggg_from_str_radix_special = 0;
        let rug_fuzz_0 = "inf";
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = "-inf";
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = "nan";
        let rug_fuzz_5 = 10;
        let rug_fuzz_6 = "-nan";
        let rug_fuzz_7 = 10;
        debug_assert!(
            < f32 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1).unwrap()
            .is_infinite()
        );
        debug_assert!(
            < f32 as Num > ::from_str_radix(rug_fuzz_2, rug_fuzz_3).unwrap()
            .is_infinite()
        );
        debug_assert!(
            < f32 as Num > ::from_str_radix(rug_fuzz_4, rug_fuzz_5).unwrap().is_nan()
        );
        debug_assert!(
            < f32 as Num > ::from_str_radix(rug_fuzz_6, rug_fuzz_7).unwrap().is_nan()
        );
        let _rug_ed_tests_llm_16_263_llm_16_263_rrrruuuugggg_from_str_radix_special = 0;
    }
    #[test]
    fn from_str_radix_empty() {
        let _rug_st_tests_llm_16_263_llm_16_263_rrrruuuugggg_from_str_radix_empty = 0;
        let rug_fuzz_0 = "";
        let rug_fuzz_1 = 10;
        debug_assert!(< f32 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1).is_err());
        let _rug_ed_tests_llm_16_263_llm_16_263_rrrruuuugggg_from_str_radix_empty = 0;
    }
    #[test]
    fn from_str_radix_invalid() {
        let _rug_st_tests_llm_16_263_llm_16_263_rrrruuuugggg_from_str_radix_invalid = 0;
        let rug_fuzz_0 = "not a number";
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = "123";
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = "123";
        let rug_fuzz_5 = 37;
        debug_assert!(< f32 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1).is_err());
        debug_assert!(< f32 as Num > ::from_str_radix(rug_fuzz_2, rug_fuzz_3).is_err());
        debug_assert!(< f32 as Num > ::from_str_radix(rug_fuzz_4, rug_fuzz_5).is_err());
        let _rug_ed_tests_llm_16_263_llm_16_263_rrrruuuugggg_from_str_radix_invalid = 0;
    }
    #[test]
    fn from_str_radix_invalid_radix() {
        let _rug_st_tests_llm_16_263_llm_16_263_rrrruuuugggg_from_str_radix_invalid_radix = 0;
        let rug_fuzz_0 = "100";
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = "100";
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = "100";
        let rug_fuzz_5 = 37;
        debug_assert!(< f32 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1).is_err());
        debug_assert!(< f32 as Num > ::from_str_radix(rug_fuzz_2, rug_fuzz_3).is_err());
        debug_assert!(< f32 as Num > ::from_str_radix(rug_fuzz_4, rug_fuzz_5).is_err());
        let _rug_ed_tests_llm_16_263_llm_16_263_rrrruuuugggg_from_str_radix_invalid_radix = 0;
    }
    #[test]
    fn from_str_radix_invalid_char() {
        let _rug_st_tests_llm_16_263_llm_16_263_rrrruuuugggg_from_str_radix_invalid_char = 0;
        let rug_fuzz_0 = "123x";
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = "1.0.1";
        let rug_fuzz_3 = 10;
        debug_assert!(< f32 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1).is_err());
        debug_assert!(< f32 as Num > ::from_str_radix(rug_fuzz_2, rug_fuzz_3).is_err());
        let _rug_ed_tests_llm_16_263_llm_16_263_rrrruuuugggg_from_str_radix_invalid_char = 0;
    }
    #[test]
    fn from_str_radix_invalid_exponent() {
        let _rug_st_tests_llm_16_263_llm_16_263_rrrruuuugggg_from_str_radix_invalid_exponent = 0;
        let rug_fuzz_0 = "123e";
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = "123p";
        let rug_fuzz_3 = 16;
        debug_assert!(< f32 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1).is_err());
        debug_assert!(< f32 as Num > ::from_str_radix(rug_fuzz_2, rug_fuzz_3).is_err());
        let _rug_ed_tests_llm_16_263_llm_16_263_rrrruuuugggg_from_str_radix_invalid_exponent = 0;
    }
    #[test]
    fn from_str_radix_overflow() {
        let _rug_st_tests_llm_16_263_llm_16_263_rrrruuuugggg_from_str_radix_overflow = 0;
        let rug_fuzz_0 = "340282356779733661637539395458142568447";
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = "-340282356779733661637539395458142568447";
        let rug_fuzz_3 = 10;
        debug_assert!(
            < f32 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1).unwrap()
            .is_infinite()
        );
        debug_assert!(
            < f32 as Num > ::from_str_radix(rug_fuzz_2, rug_fuzz_3).unwrap()
            .is_infinite()
        );
        let _rug_ed_tests_llm_16_263_llm_16_263_rrrruuuugggg_from_str_radix_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_264_llm_16_264 {
    use crate::pow;
    use crate::float::FloatCore;
    #[test]
    fn test_pow() {
        let _rug_st_tests_llm_16_264_llm_16_264_rrrruuuugggg_test_pow = 0;
        let rug_fuzz_0 = 2.0_f32;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 2.0_f32;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 2.0_f32;
        let rug_fuzz_5 = 2;
        let rug_fuzz_6 = 2.0_f32;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 3.0_f32;
        let rug_fuzz_9 = 2;
        let rug_fuzz_10 = 2.0_f32;
        let rug_fuzz_11 = 2;
        let rug_fuzz_12 = 2.0_f32;
        let rug_fuzz_13 = 3;
        let rug_fuzz_14 = 2.5_f32;
        let rug_fuzz_15 = 2;
        let rug_fuzz_16 = 0.0_f32;
        let rug_fuzz_17 = 0;
        let rug_fuzz_18 = 0.0_f32;
        let rug_fuzz_19 = 1;
        let rug_fuzz_20 = 2.0_f32;
        let rug_fuzz_21 = 10;
        let rug_fuzz_22 = 2.0_f32;
        debug_assert_eq!(pow(rug_fuzz_0, rug_fuzz_1), 1.0);
        debug_assert_eq!(pow(rug_fuzz_2, rug_fuzz_3), 2.0);
        debug_assert_eq!(pow(rug_fuzz_4, rug_fuzz_5), 4.0);
        debug_assert_eq!(pow(rug_fuzz_6, rug_fuzz_7), 8.0);
        debug_assert_eq!(pow(rug_fuzz_8, rug_fuzz_9), 9.0);
        debug_assert_eq!(pow(- rug_fuzz_10, rug_fuzz_11), 4.0);
        debug_assert_eq!(pow(- rug_fuzz_12, rug_fuzz_13), - 8.0);
        debug_assert_eq!(pow(rug_fuzz_14, rug_fuzz_15), 6.25);
        debug_assert_eq!(pow(rug_fuzz_16, rug_fuzz_17), 1.0);
        debug_assert_eq!(pow(rug_fuzz_18, rug_fuzz_19), 0.0);
        debug_assert_eq!(pow(rug_fuzz_20, rug_fuzz_21), 1024.0);
        debug_assert!(pow(rug_fuzz_22, usize::MAX).is_infinite());
        let _rug_ed_tests_llm_16_264_llm_16_264_rrrruuuugggg_test_pow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_431_llm_16_431 {
    use crate::Num;
    use core::num::ParseFloatError;
    fn almost_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < f64::EPSILON
    }
    #[test]
    fn test_from_str_radix_normal() {
        assert_eq!(< f64 as Num >::from_str_radix("10", 10).unwrap(), 10.0);
        assert_eq!(< f64 as Num >::from_str_radix("A", 16).unwrap(), 10.0);
    }
    #[test]
    fn test_from_str_radix_edge_case_radix_10() {
        assert_eq!(< f64 as Num >::from_str_radix("10", 10).unwrap(), 10.0);
    }
    #[test]
    fn test_from_str_radix_special_values() {
        assert!(
            almost_eq(< f64 as Num >::from_str_radix("inf", 10).unwrap(), f64::INFINITY)
        );
        assert!(
            almost_eq(< f64 as Num >::from_str_radix("-inf", 10).unwrap(),
            f64::NEG_INFINITY)
        );
        assert!(< f64 as Num >::from_str_radix("nan", 10).unwrap().is_nan());
        assert!(< f64 as Num >::from_str_radix("-nan", 10).unwrap().is_nan());
    }
    #[test]
    fn test_from_str_radix_empty_string() {
        assert!(< f64 as Num >::from_str_radix("", 10).is_err());
    }
    #[test]
    fn test_from_str_radix_invalid_string() {
        assert!(< f64 as Num >::from_str_radix("invalid", 10).is_err());
    }
    #[test]
    fn test_from_str_radix_invalid_radix() {
        assert!(< f64 as Num >::from_str_radix("10", 37).is_err());
        assert!(< f64 as Num >::from_str_radix("10", 1).is_err());
    }
    #[test]
    fn test_from_str_radix_with_exponent() {
        assert_eq!(< f64 as Num >::from_str_radix("1e4", 10).unwrap(), 10000.0);
        assert_eq!(< f64 as Num >::from_str_radix("1p4", 16).unwrap(), 16.0);
    }
    #[test]
    fn test_from_str_radix_overflow() {
        assert!(
            almost_eq(< f64 as Num >::from_str_radix("1e309", 10).unwrap(),
            f64::INFINITY)
        );
        assert!(
            almost_eq(< f64 as Num >::from_str_radix("-1e309", 10).unwrap(),
            f64::NEG_INFINITY)
        );
    }
}
#[cfg(test)]
mod tests_llm_16_432_llm_16_432 {
    use super::*;
    use crate::*;
    use crate::float::Float;
    #[test]
    fn test_pow_positive_exponent() {
        let _rug_st_tests_llm_16_432_llm_16_432_rrrruuuugggg_test_pow_positive_exponent = 0;
        let rug_fuzz_0 = 2.0_f64;
        let rug_fuzz_1 = 3;
        debug_assert_eq!(rug_fuzz_0.pow(rug_fuzz_1), 8.0_f64);
        let _rug_ed_tests_llm_16_432_llm_16_432_rrrruuuugggg_test_pow_positive_exponent = 0;
    }
    #[test]
    fn test_pow_zero_exponent() {
        let _rug_st_tests_llm_16_432_llm_16_432_rrrruuuugggg_test_pow_zero_exponent = 0;
        let rug_fuzz_0 = 2.0_f64;
        let rug_fuzz_1 = 0;
        debug_assert_eq!(rug_fuzz_0.pow(rug_fuzz_1), 1.0_f64);
        let _rug_ed_tests_llm_16_432_llm_16_432_rrrruuuugggg_test_pow_zero_exponent = 0;
    }
    #[test]
    fn test_pow_one_exponent() {
        let _rug_st_tests_llm_16_432_llm_16_432_rrrruuuugggg_test_pow_one_exponent = 0;
        let rug_fuzz_0 = 2.0_f64;
        let rug_fuzz_1 = 1;
        debug_assert_eq!(rug_fuzz_0.pow(rug_fuzz_1), 2.0_f64);
        let _rug_ed_tests_llm_16_432_llm_16_432_rrrruuuugggg_test_pow_one_exponent = 0;
    }
    #[test]
    fn test_pow_large_exponent() {
        let _rug_st_tests_llm_16_432_llm_16_432_rrrruuuugggg_test_pow_large_exponent = 0;
        let rug_fuzz_0 = 2.0_f64;
        let rug_fuzz_1 = 10;
        debug_assert_eq!(rug_fuzz_0.pow(rug_fuzz_1), 1024.0_f64);
        let _rug_ed_tests_llm_16_432_llm_16_432_rrrruuuugggg_test_pow_large_exponent = 0;
    }
    #[test]
    fn test_pow_zero_base() {
        let _rug_st_tests_llm_16_432_llm_16_432_rrrruuuugggg_test_pow_zero_base = 0;
        let rug_fuzz_0 = 0.0_f64;
        let rug_fuzz_1 = 2;
        debug_assert_eq!(rug_fuzz_0.pow(rug_fuzz_1), 0.0_f64);
        let _rug_ed_tests_llm_16_432_llm_16_432_rrrruuuugggg_test_pow_zero_base = 0;
    }
    #[test]
    fn test_pow_one_base() {
        let _rug_st_tests_llm_16_432_llm_16_432_rrrruuuugggg_test_pow_one_base = 0;
        let rug_fuzz_0 = 1.0_f64;
        let rug_fuzz_1 = 2;
        debug_assert_eq!(rug_fuzz_0.pow(rug_fuzz_1), 1.0_f64);
        let _rug_ed_tests_llm_16_432_llm_16_432_rrrruuuugggg_test_pow_one_base = 0;
    }
    #[test]
    fn test_pow_base_one_exponent_zero() {
        let _rug_st_tests_llm_16_432_llm_16_432_rrrruuuugggg_test_pow_base_one_exponent_zero = 0;
        let rug_fuzz_0 = 1.0_f64;
        let rug_fuzz_1 = 0;
        debug_assert_eq!(rug_fuzz_0.pow(rug_fuzz_1), 1.0_f64);
        let _rug_ed_tests_llm_16_432_llm_16_432_rrrruuuugggg_test_pow_base_one_exponent_zero = 0;
    }
    #[test]
    fn test_pow_base_zero_exponent_zero() {
        let _rug_st_tests_llm_16_432_llm_16_432_rrrruuuugggg_test_pow_base_zero_exponent_zero = 0;
        let rug_fuzz_0 = 0.0_f64;
        let rug_fuzz_1 = 0;
        debug_assert_eq!(rug_fuzz_0.pow(rug_fuzz_1), 1.0_f64);
        let _rug_ed_tests_llm_16_432_llm_16_432_rrrruuuugggg_test_pow_base_zero_exponent_zero = 0;
    }
    #[test]
    fn test_pow_negative_base_even_exponent() {
        let _rug_st_tests_llm_16_432_llm_16_432_rrrruuuugggg_test_pow_negative_base_even_exponent = 0;
        let rug_fuzz_0 = 2.0_f64;
        let rug_fuzz_1 = 2;
        debug_assert_eq!((- rug_fuzz_0).pow(rug_fuzz_1), 4.0_f64);
        let _rug_ed_tests_llm_16_432_llm_16_432_rrrruuuugggg_test_pow_negative_base_even_exponent = 0;
    }
    #[test]
    fn test_pow_negative_base_odd_exponent() {
        let _rug_st_tests_llm_16_432_llm_16_432_rrrruuuugggg_test_pow_negative_base_odd_exponent = 0;
        let rug_fuzz_0 = 2.0_f64;
        let rug_fuzz_1 = 3;
        debug_assert_eq!((- rug_fuzz_0).pow(rug_fuzz_1), - 8.0_f64);
        let _rug_ed_tests_llm_16_432_llm_16_432_rrrruuuugggg_test_pow_negative_base_odd_exponent = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_599_llm_16_599 {
    use crate::Num;
    #[test]
    fn test_from_str_radix_valid_hex() {
        let _rug_st_tests_llm_16_599_llm_16_599_rrrruuuugggg_test_from_str_radix_valid_hex = 0;
        let rug_fuzz_0 = "7f";
        let rug_fuzz_1 = 16;
        debug_assert_eq!(i128::from_str_radix(rug_fuzz_0, rug_fuzz_1), Ok(127));
        let _rug_ed_tests_llm_16_599_llm_16_599_rrrruuuugggg_test_from_str_radix_valid_hex = 0;
    }
    #[test]
    fn test_from_str_radix_valid_binary() {
        let _rug_st_tests_llm_16_599_llm_16_599_rrrruuuugggg_test_from_str_radix_valid_binary = 0;
        let rug_fuzz_0 = "1101";
        let rug_fuzz_1 = 2;
        debug_assert_eq!(i128::from_str_radix(rug_fuzz_0, rug_fuzz_1), Ok(13));
        let _rug_ed_tests_llm_16_599_llm_16_599_rrrruuuugggg_test_from_str_radix_valid_binary = 0;
    }
    #[test]
    fn test_from_str_radix_valid_decimal() {
        let _rug_st_tests_llm_16_599_llm_16_599_rrrruuuugggg_test_from_str_radix_valid_decimal = 0;
        let rug_fuzz_0 = "123";
        let rug_fuzz_1 = 10;
        debug_assert_eq!(i128::from_str_radix(rug_fuzz_0, rug_fuzz_1), Ok(123));
        let _rug_ed_tests_llm_16_599_llm_16_599_rrrruuuugggg_test_from_str_radix_valid_decimal = 0;
    }
    #[test]
    fn test_from_str_radix_invalid_number() {
        let _rug_st_tests_llm_16_599_llm_16_599_rrrruuuugggg_test_from_str_radix_invalid_number = 0;
        let rug_fuzz_0 = "xyz";
        let rug_fuzz_1 = 10;
        debug_assert!(i128::from_str_radix(rug_fuzz_0, rug_fuzz_1).is_err());
        let _rug_ed_tests_llm_16_599_llm_16_599_rrrruuuugggg_test_from_str_radix_invalid_number = 0;
    }
    #[test]
    fn test_from_str_radix_invalid_radix() {
        let _rug_st_tests_llm_16_599_llm_16_599_rrrruuuugggg_test_from_str_radix_invalid_radix = 0;
        let rug_fuzz_0 = "123";
        let rug_fuzz_1 = 37;
        debug_assert!(i128::from_str_radix(rug_fuzz_0, rug_fuzz_1).is_err());
        let _rug_ed_tests_llm_16_599_llm_16_599_rrrruuuugggg_test_from_str_radix_invalid_radix = 0;
    }
    #[test]
    fn test_from_str_radix_radix_too_low() {
        let _rug_st_tests_llm_16_599_llm_16_599_rrrruuuugggg_test_from_str_radix_radix_too_low = 0;
        let rug_fuzz_0 = "123";
        let rug_fuzz_1 = 1;
        debug_assert!(i128::from_str_radix(rug_fuzz_0, rug_fuzz_1).is_err());
        let _rug_ed_tests_llm_16_599_llm_16_599_rrrruuuugggg_test_from_str_radix_radix_too_low = 0;
    }
    #[test]
    fn test_from_str_radix_negative() {
        let _rug_st_tests_llm_16_599_llm_16_599_rrrruuuugggg_test_from_str_radix_negative = 0;
        let rug_fuzz_0 = "-123";
        let rug_fuzz_1 = 10;
        debug_assert_eq!(i128::from_str_radix(rug_fuzz_0, rug_fuzz_1), Ok(- 123));
        let _rug_ed_tests_llm_16_599_llm_16_599_rrrruuuugggg_test_from_str_radix_negative = 0;
    }
    #[test]
    fn test_from_str_radix_max_value() {
        let _rug_st_tests_llm_16_599_llm_16_599_rrrruuuugggg_test_from_str_radix_max_value = 0;
        let rug_fuzz_0 = "170141183460469231731687303715884105727";
        let rug_fuzz_1 = 10;
        debug_assert_eq!(i128::from_str_radix(rug_fuzz_0, rug_fuzz_1), Ok(i128::MAX));
        let _rug_ed_tests_llm_16_599_llm_16_599_rrrruuuugggg_test_from_str_radix_max_value = 0;
    }
    #[test]
    fn test_from_str_radix_min_value() {
        let _rug_st_tests_llm_16_599_llm_16_599_rrrruuuugggg_test_from_str_radix_min_value = 0;
        let rug_fuzz_0 = "-170141183460469231731687303715884105728";
        let rug_fuzz_1 = 10;
        debug_assert_eq!(i128::from_str_radix(rug_fuzz_0, rug_fuzz_1), Ok(i128::MIN));
        let _rug_ed_tests_llm_16_599_llm_16_599_rrrruuuugggg_test_from_str_radix_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_709_llm_16_709 {
    use crate::Num;
    #[test]
    fn test_from_str_radix_valid() {
        let _rug_st_tests_llm_16_709_llm_16_709_rrrruuuugggg_test_from_str_radix_valid = 0;
        let rug_fuzz_0 = "A";
        let rug_fuzz_1 = 16;
        let rug_fuzz_2 = "10";
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = "1010";
        let rug_fuzz_5 = 2;
        let rug_fuzz_6 = "12";
        let rug_fuzz_7 = 8;
        debug_assert_eq!(
            < i16 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1), Ok(10)
        );
        debug_assert_eq!(
            < i16 as Num > ::from_str_radix(rug_fuzz_2, rug_fuzz_3), Ok(10)
        );
        debug_assert_eq!(
            < i16 as Num > ::from_str_radix(rug_fuzz_4, rug_fuzz_5), Ok(10)
        );
        debug_assert_eq!(
            < i16 as Num > ::from_str_radix(rug_fuzz_6, rug_fuzz_7), Ok(10)
        );
        let _rug_ed_tests_llm_16_709_llm_16_709_rrrruuuugggg_test_from_str_radix_valid = 0;
    }
    #[test]
    fn test_from_str_radix_invalid_radix() {
        let _rug_st_tests_llm_16_709_llm_16_709_rrrruuuugggg_test_from_str_radix_invalid_radix = 0;
        let rug_fuzz_0 = "10";
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = "10";
        let rug_fuzz_3 = 37;
        debug_assert!(< i16 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1).is_err());
        debug_assert!(< i16 as Num > ::from_str_radix(rug_fuzz_2, rug_fuzz_3).is_err());
        let _rug_ed_tests_llm_16_709_llm_16_709_rrrruuuugggg_test_from_str_radix_invalid_radix = 0;
    }
    #[test]
    fn test_from_str_radix_invalid_value() {
        let _rug_st_tests_llm_16_709_llm_16_709_rrrruuuugggg_test_from_str_radix_invalid_value = 0;
        let rug_fuzz_0 = "Z";
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = "10.1";
        let rug_fuzz_3 = 10;
        debug_assert!(< i16 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1).is_err());
        debug_assert!(< i16 as Num > ::from_str_radix(rug_fuzz_2, rug_fuzz_3).is_err());
        let _rug_ed_tests_llm_16_709_llm_16_709_rrrruuuugggg_test_from_str_radix_invalid_value = 0;
    }
    #[test]
    fn test_from_str_radix_empty_string() {
        let _rug_st_tests_llm_16_709_llm_16_709_rrrruuuugggg_test_from_str_radix_empty_string = 0;
        let rug_fuzz_0 = "";
        let rug_fuzz_1 = 10;
        debug_assert!(< i16 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1).is_err());
        let _rug_ed_tests_llm_16_709_llm_16_709_rrrruuuugggg_test_from_str_radix_empty_string = 0;
    }
    #[test]
    fn test_from_str_radix_negatives() {
        let _rug_st_tests_llm_16_709_llm_16_709_rrrruuuugggg_test_from_str_radix_negatives = 0;
        let rug_fuzz_0 = "-A";
        let rug_fuzz_1 = 16;
        let rug_fuzz_2 = "-10";
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = "-1010";
        let rug_fuzz_5 = 2;
        let rug_fuzz_6 = "-12";
        let rug_fuzz_7 = 8;
        debug_assert_eq!(
            < i16 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1), Ok(- 10)
        );
        debug_assert_eq!(
            < i16 as Num > ::from_str_radix(rug_fuzz_2, rug_fuzz_3), Ok(- 10)
        );
        debug_assert_eq!(
            < i16 as Num > ::from_str_radix(rug_fuzz_4, rug_fuzz_5), Ok(- 10)
        );
        debug_assert_eq!(
            < i16 as Num > ::from_str_radix(rug_fuzz_6, rug_fuzz_7), Ok(- 10)
        );
        let _rug_ed_tests_llm_16_709_llm_16_709_rrrruuuugggg_test_from_str_radix_negatives = 0;
    }
    #[test]
    fn test_from_str_radix_edge_cases() {
        let _rug_st_tests_llm_16_709_llm_16_709_rrrruuuugggg_test_from_str_radix_edge_cases = 0;
        let rug_fuzz_0 = "-8000";
        let rug_fuzz_1 = 16;
        let rug_fuzz_2 = "-8001";
        let rug_fuzz_3 = 16;
        let rug_fuzz_4 = "7FFF";
        let rug_fuzz_5 = 16;
        let rug_fuzz_6 = "8000";
        let rug_fuzz_7 = 16;
        debug_assert_eq!(
            < i16 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1), Ok(- 32768)
        );
        debug_assert!(< i16 as Num > ::from_str_radix(rug_fuzz_2, rug_fuzz_3).is_err());
        debug_assert_eq!(
            < i16 as Num > ::from_str_radix(rug_fuzz_4, rug_fuzz_5), Ok(32767)
        );
        debug_assert!(< i16 as Num > ::from_str_radix(rug_fuzz_6, rug_fuzz_7).is_err());
        let _rug_ed_tests_llm_16_709_llm_16_709_rrrruuuugggg_test_from_str_radix_edge_cases = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_819_llm_16_819 {
    use crate::Num;
    #[test]
    fn test_from_str_radix_valid_integer() {
        let _rug_st_tests_llm_16_819_llm_16_819_rrrruuuugggg_test_from_str_radix_valid_integer = 0;
        let rug_fuzz_0 = "1234";
        let rug_fuzz_1 = 10;
        debug_assert_eq!(
            < i32 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1), Ok(1234)
        );
        let _rug_ed_tests_llm_16_819_llm_16_819_rrrruuuugggg_test_from_str_radix_valid_integer = 0;
    }
    #[test]
    fn test_from_str_radix_valid_hex() {
        let _rug_st_tests_llm_16_819_llm_16_819_rrrruuuugggg_test_from_str_radix_valid_hex = 0;
        let rug_fuzz_0 = "7b";
        let rug_fuzz_1 = 16;
        debug_assert_eq!(
            < i32 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1), Ok(123)
        );
        let _rug_ed_tests_llm_16_819_llm_16_819_rrrruuuugggg_test_from_str_radix_valid_hex = 0;
    }
    #[test]
    fn test_from_str_radix_valid_binary() {
        let _rug_st_tests_llm_16_819_llm_16_819_rrrruuuugggg_test_from_str_radix_valid_binary = 0;
        let rug_fuzz_0 = "1101";
        let rug_fuzz_1 = 2;
        debug_assert_eq!(
            < i32 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1), Ok(13)
        );
        let _rug_ed_tests_llm_16_819_llm_16_819_rrrruuuugggg_test_from_str_radix_valid_binary = 0;
    }
    #[test]
    fn test_from_str_radix_invalid_number() {
        let _rug_st_tests_llm_16_819_llm_16_819_rrrruuuugggg_test_from_str_radix_invalid_number = 0;
        let rug_fuzz_0 = "xyz";
        let rug_fuzz_1 = 10;
        debug_assert!(< i32 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1).is_err());
        let _rug_ed_tests_llm_16_819_llm_16_819_rrrruuuugggg_test_from_str_radix_invalid_number = 0;
    }
    #[test]
    fn test_from_str_radix_invalid_radix() {
        let _rug_st_tests_llm_16_819_llm_16_819_rrrruuuugggg_test_from_str_radix_invalid_radix = 0;
        let rug_fuzz_0 = "123";
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = "123";
        let rug_fuzz_3 = 37;
        debug_assert!(< i32 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1).is_err());
        debug_assert!(< i32 as Num > ::from_str_radix(rug_fuzz_2, rug_fuzz_3).is_err());
        let _rug_ed_tests_llm_16_819_llm_16_819_rrrruuuugggg_test_from_str_radix_invalid_radix = 0;
    }
    #[test]
    fn test_from_str_radix_empty_string() {
        let _rug_st_tests_llm_16_819_llm_16_819_rrrruuuugggg_test_from_str_radix_empty_string = 0;
        let rug_fuzz_0 = "";
        let rug_fuzz_1 = 10;
        debug_assert!(< i32 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1).is_err());
        let _rug_ed_tests_llm_16_819_llm_16_819_rrrruuuugggg_test_from_str_radix_empty_string = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_929_llm_16_929 {
    use crate::Num;
    #[test]
    fn test_from_str_radix_valid() {
        let _rug_st_tests_llm_16_929_llm_16_929_rrrruuuugggg_test_from_str_radix_valid = 0;
        let rug_fuzz_0 = "123";
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = "A";
        let rug_fuzz_3 = 16;
        let rug_fuzz_4 = "110";
        let rug_fuzz_5 = 2;
        let rug_fuzz_6 = "z";
        let rug_fuzz_7 = 36;
        debug_assert_eq!(
            < i64 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1), Ok(123)
        );
        debug_assert_eq!(
            < i64 as Num > ::from_str_radix(rug_fuzz_2, rug_fuzz_3), Ok(10)
        );
        debug_assert_eq!(< i64 as Num > ::from_str_radix(rug_fuzz_4, rug_fuzz_5), Ok(6));
        debug_assert_eq!(
            < i64 as Num > ::from_str_radix(rug_fuzz_6, rug_fuzz_7), Ok(35)
        );
        let _rug_ed_tests_llm_16_929_llm_16_929_rrrruuuugggg_test_from_str_radix_valid = 0;
    }
    #[test]
    fn test_from_str_radix_invalid_number() {
        let _rug_st_tests_llm_16_929_llm_16_929_rrrruuuugggg_test_from_str_radix_invalid_number = 0;
        let rug_fuzz_0 = "123";
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = "123";
        let rug_fuzz_3 = 37;
        let rug_fuzz_4 = "12A";
        let rug_fuzz_5 = 10;
        debug_assert!(< i64 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1).is_err());
        debug_assert!(< i64 as Num > ::from_str_radix(rug_fuzz_2, rug_fuzz_3).is_err());
        debug_assert!(< i64 as Num > ::from_str_radix(rug_fuzz_4, rug_fuzz_5).is_err());
        let _rug_ed_tests_llm_16_929_llm_16_929_rrrruuuugggg_test_from_str_radix_invalid_number = 0;
    }
    #[test]
    fn test_from_str_radix_invalid_radix() {
        let _rug_st_tests_llm_16_929_llm_16_929_rrrruuuugggg_test_from_str_radix_invalid_radix = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 37;
        let rug_fuzz_2 = "10";
        let invalid_radix_errors: [u32; 2] = [rug_fuzz_0, rug_fuzz_1];
        for &radix in &invalid_radix_errors {
            let result = <i64 as Num>::from_str_radix(rug_fuzz_2, radix);
            debug_assert!(result.is_err());
        }
        let _rug_ed_tests_llm_16_929_llm_16_929_rrrruuuugggg_test_from_str_radix_invalid_radix = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1039_llm_16_1039 {
    use crate::Num;
    #[test]
    fn test_from_str_radix() {
        let _rug_st_tests_llm_16_1039_llm_16_1039_rrrruuuugggg_test_from_str_radix = 0;
        let rug_fuzz_0 = "7";
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = "-8";
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = "10";
        let rug_fuzz_5 = 2;
        let rug_fuzz_6 = "ff";
        let rug_fuzz_7 = 16;
        let rug_fuzz_8 = "80";
        let rug_fuzz_9 = 16;
        let rug_fuzz_10 = "zz";
        let rug_fuzz_11 = 36;
        let rug_fuzz_12 = "128";
        let rug_fuzz_13 = 10;
        debug_assert_eq!(
            < i8 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1), Ok(7i8)
        );
        debug_assert_eq!(
            < i8 as Num > ::from_str_radix(rug_fuzz_2, rug_fuzz_3), Ok(- 8i8)
        );
        debug_assert_eq!(
            < i8 as Num > ::from_str_radix(rug_fuzz_4, rug_fuzz_5), Ok(2i8)
        );
        debug_assert_eq!(
            < i8 as Num > ::from_str_radix(rug_fuzz_6, rug_fuzz_7), Ok(- 1i8)
        );
        debug_assert_eq!(
            < i8 as Num > ::from_str_radix(rug_fuzz_8, rug_fuzz_9), Ok(- 128i8)
        );
        debug_assert_eq!(
            < i8 as Num > ::from_str_radix(rug_fuzz_10, rug_fuzz_11), Ok(35i8)
        );
        let error_cases = vec![
            (rug_fuzz_12, rug_fuzz_13), ("-129", 10), ("z", 10), ("", 10), ("7", 1),
            ("7", 37)
        ];
        for (input, radix) in error_cases {
            match <i8 as Num>::from_str_radix(input, radix) {
                Ok(_) => {
                    panic!("Test failed, expecting Err, got Ok for input: {}", input)
                }
                Err(_) => {}
            }
        }
        let _rug_ed_tests_llm_16_1039_llm_16_1039_rrrruuuugggg_test_from_str_radix = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1149_llm_16_1149 {
    use crate::Num;
    #[test]
    fn test_from_str_radix_valid_input() {
        let _rug_st_tests_llm_16_1149_llm_16_1149_rrrruuuugggg_test_from_str_radix_valid_input = 0;
        let rug_fuzz_0 = "42";
        let rug_fuzz_1 = 10;
        let numbers = vec![
            (rug_fuzz_0, rug_fuzz_1), ("1A", 16), ("101010", 2), ("052", 8)
        ];
        for &(num_str, radix) in &numbers {
            let result = <isize as Num>::from_str_radix(num_str, radix);
            debug_assert!(result.is_ok());
            let result = result.unwrap();
            debug_assert_eq!(result, isize::from_str_radix(num_str, radix).unwrap());
        }
        let _rug_ed_tests_llm_16_1149_llm_16_1149_rrrruuuugggg_test_from_str_radix_valid_input = 0;
    }
    #[test]
    fn test_from_str_radix_invalid_input() {
        let _rug_st_tests_llm_16_1149_llm_16_1149_rrrruuuugggg_test_from_str_radix_invalid_input = 0;
        let rug_fuzz_0 = "12";
        let rug_fuzz_1 = 1;
        let numbers = vec![
            (rug_fuzz_0, rug_fuzz_1), ("ZZ", 35), ("is_not_a_number", 10)
        ];
        for &(num_str, radix) in &numbers {
            let result = <isize as Num>::from_str_radix(num_str, radix);
            debug_assert!(result.is_err());
        }
        let _rug_ed_tests_llm_16_1149_llm_16_1149_rrrruuuugggg_test_from_str_radix_invalid_input = 0;
    }
    #[test]
    fn test_from_str_radix_invalid_radix() {
        let _rug_st_tests_llm_16_1149_llm_16_1149_rrrruuuugggg_test_from_str_radix_invalid_radix = 0;
        let rug_fuzz_0 = "10";
        let rug_fuzz_1 = 37;
        let rug_fuzz_2 = "10";
        let rug_fuzz_3 = 0;
        let result = <isize as Num>::from_str_radix(rug_fuzz_0, rug_fuzz_1);
        debug_assert!(result.is_err());
        let result = <isize as Num>::from_str_radix(rug_fuzz_2, rug_fuzz_3);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_1149_llm_16_1149_rrrruuuugggg_test_from_str_radix_invalid_radix = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1259_llm_16_1259 {
    use super::*;
    use crate::*;
    use std::num::Wrapping;
    use crate::Num;
    #[test]
    fn test_from_str_radix_success() {
        let _rug_st_tests_llm_16_1259_llm_16_1259_rrrruuuugggg_test_from_str_radix_success = 0;
        let rug_fuzz_0 = "1234";
        let rug_fuzz_1 = 10;
        let value = rug_fuzz_0;
        let radix = rug_fuzz_1;
        let result = <Wrapping<u32> as Num>::from_str_radix(value, radix);
        debug_assert_eq!(result, Ok(Wrapping(1234u32)));
        let _rug_ed_tests_llm_16_1259_llm_16_1259_rrrruuuugggg_test_from_str_radix_success = 0;
    }
    #[test]
    fn test_from_str_radix_invalid_number() {
        let _rug_st_tests_llm_16_1259_llm_16_1259_rrrruuuugggg_test_from_str_radix_invalid_number = 0;
        let rug_fuzz_0 = "1z34";
        let rug_fuzz_1 = 10;
        let value = rug_fuzz_0;
        let radix = rug_fuzz_1;
        let result = <Wrapping<u32> as Num>::from_str_radix(value, radix);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_1259_llm_16_1259_rrrruuuugggg_test_from_str_radix_invalid_number = 0;
    }
    #[test]
    fn test_from_str_radix_invalid_radix() {
        let _rug_st_tests_llm_16_1259_llm_16_1259_rrrruuuugggg_test_from_str_radix_invalid_radix = 0;
        let rug_fuzz_0 = "1234";
        let rug_fuzz_1 = 1;
        let value = rug_fuzz_0;
        let radix = rug_fuzz_1;
        let result = <Wrapping<u32> as Num>::from_str_radix(value, radix);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_1259_llm_16_1259_rrrruuuugggg_test_from_str_radix_invalid_radix = 0;
    }
    #[test]
    fn test_from_str_radix_hex() {
        let _rug_st_tests_llm_16_1259_llm_16_1259_rrrruuuugggg_test_from_str_radix_hex = 0;
        let rug_fuzz_0 = "1A";
        let rug_fuzz_1 = 16;
        let value = rug_fuzz_0;
        let radix = rug_fuzz_1;
        let result = <Wrapping<u32> as Num>::from_str_radix(value, radix);
        debug_assert_eq!(result, Ok(Wrapping(26u32)));
        let _rug_ed_tests_llm_16_1259_llm_16_1259_rrrruuuugggg_test_from_str_radix_hex = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1355_llm_16_1355 {
    use crate::Num;
    #[test]
    fn test_from_str_radix_u128() {
        let _rug_st_tests_llm_16_1355_llm_16_1355_rrrruuuugggg_test_from_str_radix_u128 = 0;
        let rug_fuzz_0 = "10";
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = "A";
        let rug_fuzz_3 = 16;
        let rug_fuzz_4 = "Z";
        let rug_fuzz_5 = 36;
        let rug_fuzz_6 = "";
        let rug_fuzz_7 = 10;
        let rug_fuzz_8 = " ";
        let rug_fuzz_9 = 10;
        let rug_fuzz_10 = "10";
        let rug_fuzz_11 = 1;
        let rug_fuzz_12 = "10";
        let rug_fuzz_13 = 37;
        debug_assert_eq!(
            < u128 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1), Ok(10u128)
        );
        debug_assert_eq!(
            < u128 as Num > ::from_str_radix(rug_fuzz_2, rug_fuzz_3), Ok(10u128)
        );
        debug_assert_eq!(
            < u128 as Num > ::from_str_radix(rug_fuzz_4, rug_fuzz_5), Ok(35u128)
        );
        debug_assert!(< u128 as Num > ::from_str_radix(rug_fuzz_6, rug_fuzz_7).is_err());
        debug_assert!(< u128 as Num > ::from_str_radix(rug_fuzz_8, rug_fuzz_9).is_err());
        debug_assert!(
            < u128 as Num > ::from_str_radix(rug_fuzz_10, rug_fuzz_11).is_err()
        );
        debug_assert!(
            < u128 as Num > ::from_str_radix(rug_fuzz_12, rug_fuzz_13).is_err()
        );
        let _rug_ed_tests_llm_16_1355_llm_16_1355_rrrruuuugggg_test_from_str_radix_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1460_llm_16_1460 {
    use crate::Num;
    #[test]
    fn test_u16_from_str_radix_valid_input() {
        let _rug_st_tests_llm_16_1460_llm_16_1460_rrrruuuugggg_test_u16_from_str_radix_valid_input = 0;
        let rug_fuzz_0 = "A";
        let rug_fuzz_1 = 16;
        let rug_fuzz_2 = "101";
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = "z";
        let rug_fuzz_5 = 36;
        debug_assert_eq!(
            < u16 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1), Ok(10)
        );
        debug_assert_eq!(< u16 as Num > ::from_str_radix(rug_fuzz_2, rug_fuzz_3), Ok(5));
        debug_assert_eq!(
            < u16 as Num > ::from_str_radix(rug_fuzz_4, rug_fuzz_5), Ok(35)
        );
        let _rug_ed_tests_llm_16_1460_llm_16_1460_rrrruuuugggg_test_u16_from_str_radix_valid_input = 0;
    }
    #[test]
    fn test_u16_from_str_radix_invalid_input() {
        let _rug_st_tests_llm_16_1460_llm_16_1460_rrrruuuugggg_test_u16_from_str_radix_invalid_input = 0;
        let rug_fuzz_0 = "G";
        let rug_fuzz_1 = 16;
        let rug_fuzz_2 = "2";
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = " ";
        let rug_fuzz_5 = 10;
        debug_assert!(< u16 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1).is_err());
        debug_assert!(< u16 as Num > ::from_str_radix(rug_fuzz_2, rug_fuzz_3).is_err());
        debug_assert!(< u16 as Num > ::from_str_radix(rug_fuzz_4, rug_fuzz_5).is_err());
        let _rug_ed_tests_llm_16_1460_llm_16_1460_rrrruuuugggg_test_u16_from_str_radix_invalid_input = 0;
    }
    #[test]
    fn test_u16_from_str_radix_invalid_radix() {
        let _rug_st_tests_llm_16_1460_llm_16_1460_rrrruuuugggg_test_u16_from_str_radix_invalid_radix = 0;
        let rug_fuzz_0 = "10";
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = "10";
        let rug_fuzz_3 = 37;
        debug_assert!(< u16 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1).is_err());
        debug_assert!(< u16 as Num > ::from_str_radix(rug_fuzz_2, rug_fuzz_3).is_err());
        let _rug_ed_tests_llm_16_1460_llm_16_1460_rrrruuuugggg_test_u16_from_str_radix_invalid_radix = 0;
    }
    #[test]
    fn test_u16_from_str_radix_edge_cases() {
        let _rug_st_tests_llm_16_1460_llm_16_1460_rrrruuuugggg_test_u16_from_str_radix_edge_cases = 0;
        let rug_fuzz_0 = "0";
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = "65535";
        let rug_fuzz_3 = 10;
        debug_assert_eq!(< u16 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1), Ok(0));
        debug_assert_eq!(
            < u16 as Num > ::from_str_radix(rug_fuzz_2, rug_fuzz_3), Ok(u16::MAX)
        );
        let _rug_ed_tests_llm_16_1460_llm_16_1460_rrrruuuugggg_test_u16_from_str_radix_edge_cases = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1565_llm_16_1565 {
    use crate::Num;
    #[test]
    fn test_from_str_radix() {
        let _rug_st_tests_llm_16_1565_llm_16_1565_rrrruuuugggg_test_from_str_radix = 0;
        let rug_fuzz_0 = "A";
        let rug_fuzz_1 = 16;
        let rug_fuzz_2 = "z";
        let rug_fuzz_3 = 36;
        let rug_fuzz_4 = "100";
        let rug_fuzz_5 = 10;
        let rug_fuzz_6 = "100";
        let rug_fuzz_7 = 2;
        let rug_fuzz_8 = "cake";
        let rug_fuzz_9 = 16;
        let rug_fuzz_10 = "10";
        let rug_fuzz_11 = 1;
        let rug_fuzz_12 = "10";
        let rug_fuzz_13 = 37;
        debug_assert_eq!(u32::from_str_radix(rug_fuzz_0, rug_fuzz_1), Ok(10));
        debug_assert_eq!(u32::from_str_radix(rug_fuzz_2, rug_fuzz_3), Ok(35));
        debug_assert_eq!(u32::from_str_radix(rug_fuzz_4, rug_fuzz_5), Ok(100));
        debug_assert_eq!(u32::from_str_radix(rug_fuzz_6, rug_fuzz_7), Ok(4));
        debug_assert_eq!(u32::from_str_radix(rug_fuzz_8, rug_fuzz_9).is_err(), true);
        debug_assert_eq!(u32::from_str_radix(rug_fuzz_10, rug_fuzz_11).is_err(), true);
        debug_assert_eq!(u32::from_str_radix(rug_fuzz_12, rug_fuzz_13).is_err(), true);
        let _rug_ed_tests_llm_16_1565_llm_16_1565_rrrruuuugggg_test_from_str_radix = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1670_llm_16_1670 {
    use crate::Num;
    #[test]
    fn test_from_str_radix_valid() {
        let _rug_st_tests_llm_16_1670_llm_16_1670_rrrruuuugggg_test_from_str_radix_valid = 0;
        let rug_fuzz_0 = "A";
        let rug_fuzz_1 = 16;
        let rug_fuzz_2 = "10";
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = "123";
        let rug_fuzz_5 = 10;
        debug_assert_eq!(
            < u64 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1), Ok(10)
        );
        debug_assert_eq!(< u64 as Num > ::from_str_radix(rug_fuzz_2, rug_fuzz_3), Ok(2));
        debug_assert_eq!(
            < u64 as Num > ::from_str_radix(rug_fuzz_4, rug_fuzz_5), Ok(123)
        );
        let _rug_ed_tests_llm_16_1670_llm_16_1670_rrrruuuugggg_test_from_str_radix_valid = 0;
    }
    #[test]
    fn test_from_str_radix_invalid_number() {
        let _rug_st_tests_llm_16_1670_llm_16_1670_rrrruuuugggg_test_from_str_radix_invalid_number = 0;
        let rug_fuzz_0 = "Z";
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = "12A";
        let rug_fuzz_3 = 10;
        debug_assert!(< u64 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1).is_err());
        debug_assert!(< u64 as Num > ::from_str_radix(rug_fuzz_2, rug_fuzz_3).is_err());
        let _rug_ed_tests_llm_16_1670_llm_16_1670_rrrruuuugggg_test_from_str_radix_invalid_number = 0;
    }
    #[test]
    fn test_from_str_radix_invalid_radix() {
        let _rug_st_tests_llm_16_1670_llm_16_1670_rrrruuuugggg_test_from_str_radix_invalid_radix = 0;
        let rug_fuzz_0 = "10";
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = "10";
        let rug_fuzz_3 = 37;
        debug_assert!(< u64 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1).is_err());
        debug_assert!(< u64 as Num > ::from_str_radix(rug_fuzz_2, rug_fuzz_3).is_err());
        let _rug_ed_tests_llm_16_1670_llm_16_1670_rrrruuuugggg_test_from_str_radix_invalid_radix = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1775_llm_16_1775 {
    use crate::Num;
    use std::num::ParseIntError;
    #[test]
    fn test_from_str_radix_valid() {
        let _rug_st_tests_llm_16_1775_llm_16_1775_rrrruuuugggg_test_from_str_radix_valid = 0;
        let rug_fuzz_0 = "7";
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = "A";
        let rug_fuzz_3 = 16;
        let rug_fuzz_4 = "10";
        let rug_fuzz_5 = 2;
        debug_assert_eq!(< u8 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1), Ok(7));
        debug_assert_eq!(< u8 as Num > ::from_str_radix(rug_fuzz_2, rug_fuzz_3), Ok(10));
        debug_assert_eq!(< u8 as Num > ::from_str_radix(rug_fuzz_4, rug_fuzz_5), Ok(2));
        let _rug_ed_tests_llm_16_1775_llm_16_1775_rrrruuuugggg_test_from_str_radix_valid = 0;
    }
    #[test]
    fn test_from_str_radix_invalid_number() {
        let _rug_st_tests_llm_16_1775_llm_16_1775_rrrruuuugggg_test_from_str_radix_invalid_number = 0;
        let rug_fuzz_0 = "256";
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = "G";
        let rug_fuzz_3 = 16;
        debug_assert!(< u8 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1).is_err());
        debug_assert!(< u8 as Num > ::from_str_radix(rug_fuzz_2, rug_fuzz_3).is_err());
        let _rug_ed_tests_llm_16_1775_llm_16_1775_rrrruuuugggg_test_from_str_radix_invalid_number = 0;
    }
    #[test]
    fn test_from_str_radix_invalid_radix() {
        let _rug_st_tests_llm_16_1775_llm_16_1775_rrrruuuugggg_test_from_str_radix_invalid_radix = 0;
        let rug_fuzz_0 = "7";
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = "7";
        let rug_fuzz_3 = 37;
        debug_assert!(
            matches!(< u8 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1),
            Err(ParseIntError { .. }))
        );
        debug_assert!(
            matches!(< u8 as Num > ::from_str_radix(rug_fuzz_2, rug_fuzz_3),
            Err(ParseIntError { .. }))
        );
        let _rug_ed_tests_llm_16_1775_llm_16_1775_rrrruuuugggg_test_from_str_radix_invalid_radix = 0;
    }
    #[test]
    fn test_from_str_radix_empty() {
        let _rug_st_tests_llm_16_1775_llm_16_1775_rrrruuuugggg_test_from_str_radix_empty = 0;
        let rug_fuzz_0 = "";
        let rug_fuzz_1 = 10;
        debug_assert!(< u8 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1).is_err());
        let _rug_ed_tests_llm_16_1775_llm_16_1775_rrrruuuugggg_test_from_str_radix_empty = 0;
    }
    #[test]
    fn test_from_str_radix_whitespace() {
        let _rug_st_tests_llm_16_1775_llm_16_1775_rrrruuuugggg_test_from_str_radix_whitespace = 0;
        let rug_fuzz_0 = "   ";
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = "\t";
        let rug_fuzz_3 = 10;
        debug_assert!(< u8 as Num > ::from_str_radix(rug_fuzz_0, rug_fuzz_1).is_err());
        debug_assert!(< u8 as Num > ::from_str_radix(rug_fuzz_2, rug_fuzz_3).is_err());
        let _rug_ed_tests_llm_16_1775_llm_16_1775_rrrruuuugggg_test_from_str_radix_whitespace = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1881_llm_16_1881 {
    use crate::Num;
    use std::num::ParseIntError;
    #[test]
    fn from_str_radix_valid_hex() -> Result<(), ParseIntError> {
        let num_str = "1f";
        let num = <usize as Num>::from_str_radix(num_str, 16)?;
        assert_eq!(num, 31);
        Ok(())
    }
    #[test]
    fn from_str_radix_valid_binary() -> Result<(), ParseIntError> {
        let num_str = "1011";
        let num = <usize as Num>::from_str_radix(num_str, 2)?;
        assert_eq!(num, 11);
        Ok(())
    }
    #[test]
    fn from_str_radix_valid_decimal() -> Result<(), ParseIntError> {
        let num_str = "123";
        let num = <usize as Num>::from_str_radix(num_str, 10)?;
        assert_eq!(num, 123);
        Ok(())
    }
    #[test]
    fn from_str_radix_invalid_number() {
        let num_str = "1z";
        let result = <usize as Num>::from_str_radix(num_str, 36);
        assert!(result.is_err());
    }
    #[test]
    fn from_str_radix_invalid_radix_too_low() {
        let num_str = "123";
        let result = <usize as Num>::from_str_radix(num_str, 1);
        assert!(result.is_err());
    }
    #[test]
    fn from_str_radix_invalid_radix_too_high() {
        let num_str = "123";
        let result = <usize as Num>::from_str_radix(num_str, 37);
        assert!(result.is_err());
    }
}
#[cfg(test)]
mod tests_llm_16_2011 {
    use crate::clamp;
    #[test]
    fn test_clamp_within_bounds() {
        let _rug_st_tests_llm_16_2011_rrrruuuugggg_test_clamp_within_bounds = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 42;
        let rug_fuzz_4 = 42;
        let rug_fuzz_5 = 42;
        let rug_fuzz_6 = 0.5;
        let rug_fuzz_7 = 0.0;
        let rug_fuzz_8 = 1.0;
        debug_assert_eq!(clamp(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), 5);
        debug_assert_eq!(clamp(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5), 42);
        debug_assert_eq!(clamp(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8), 0.5);
        let _rug_ed_tests_llm_16_2011_rrrruuuugggg_test_clamp_within_bounds = 0;
    }
    #[test]
    fn test_clamp_at_bounds() {
        let _rug_st_tests_llm_16_2011_rrrruuuugggg_test_clamp_at_bounds = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 10;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 10;
        let rug_fuzz_6 = 0.0;
        let rug_fuzz_7 = 0.0;
        let rug_fuzz_8 = 1.0;
        let rug_fuzz_9 = 1.0;
        let rug_fuzz_10 = 0.0;
        let rug_fuzz_11 = 1.0;
        debug_assert_eq!(clamp(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), 1);
        debug_assert_eq!(clamp(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5), 10);
        debug_assert_eq!(clamp(rug_fuzz_6, rug_fuzz_7, rug_fuzz_8), 0.0);
        debug_assert_eq!(clamp(rug_fuzz_9, rug_fuzz_10, rug_fuzz_11), 1.0);
        let _rug_ed_tests_llm_16_2011_rrrruuuugggg_test_clamp_at_bounds = 0;
    }
    #[test]
    fn test_clamp_below_bounds() {
        let _rug_st_tests_llm_16_2011_rrrruuuugggg_test_clamp_below_bounds = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 1.0;
        let rug_fuzz_4 = 0.0;
        let rug_fuzz_5 = 1.0;
        debug_assert_eq!(clamp(- rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), 1);
        debug_assert_eq!(clamp(- rug_fuzz_3, rug_fuzz_4, rug_fuzz_5), 0.0);
        let _rug_ed_tests_llm_16_2011_rrrruuuugggg_test_clamp_below_bounds = 0;
    }
    #[test]
    fn test_clamp_above_bounds() {
        let _rug_st_tests_llm_16_2011_rrrruuuugggg_test_clamp_above_bounds = 0;
        let rug_fuzz_0 = 15;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 10;
        let rug_fuzz_3 = 1.5;
        let rug_fuzz_4 = 0.0;
        let rug_fuzz_5 = 1.0;
        debug_assert_eq!(clamp(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), 10);
        debug_assert_eq!(clamp(rug_fuzz_3, rug_fuzz_4, rug_fuzz_5), 1.0);
        let _rug_ed_tests_llm_16_2011_rrrruuuugggg_test_clamp_above_bounds = 0;
    }
    #[test]
    #[should_panic(expected = "min must be less than or equal to max")]
    fn test_clamp_invalid_bounds() {
        let _rug_st_tests_llm_16_2011_rrrruuuugggg_test_clamp_invalid_bounds = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 10;
        let rug_fuzz_2 = 1;
        clamp(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let _rug_ed_tests_llm_16_2011_rrrruuuugggg_test_clamp_invalid_bounds = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2012 {
    use crate::clamp_max;
    #[test]
    fn test_clamp_max_int() {
        let _rug_st_tests_llm_16_2012_rrrruuuugggg_test_clamp_max_int = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 3;
        let rug_fuzz_5 = 3;
        debug_assert_eq!(clamp_max(rug_fuzz_0, rug_fuzz_1), 3);
        debug_assert_eq!(clamp_max(rug_fuzz_2, rug_fuzz_3), 1);
        debug_assert_eq!(clamp_max(rug_fuzz_4, rug_fuzz_5), 3);
        let _rug_ed_tests_llm_16_2012_rrrruuuugggg_test_clamp_max_int = 0;
    }
    #[test]
    fn test_clamp_max_float() {
        let _rug_st_tests_llm_16_2012_rrrruuuugggg_test_clamp_max_float = 0;
        let rug_fuzz_0 = 5.5;
        let rug_fuzz_1 = 3.3;
        let rug_fuzz_2 = 1.1;
        let rug_fuzz_3 = 3.3;
        let rug_fuzz_4 = 3.3;
        let rug_fuzz_5 = 3.3;
        debug_assert_eq!(clamp_max(rug_fuzz_0, rug_fuzz_1), 3.3);
        debug_assert_eq!(clamp_max(rug_fuzz_2, rug_fuzz_3), 1.1);
        debug_assert_eq!(clamp_max(rug_fuzz_4, rug_fuzz_5), 3.3);
        let _rug_ed_tests_llm_16_2012_rrrruuuugggg_test_clamp_max_float = 0;
    }
    #[test]
    fn test_clamp_max_edge_cases() {
        let _rug_st_tests_llm_16_2012_rrrruuuugggg_test_clamp_max_edge_cases = 0;
        let rug_fuzz_0 = 1.0;
        let rug_fuzz_1 = 1.0;
        debug_assert_eq!(clamp_max(std::f32::INFINITY, rug_fuzz_0), 1.0);
        debug_assert!(clamp_max(std::f32::NAN, rug_fuzz_1).is_nan());
        let _rug_ed_tests_llm_16_2012_rrrruuuugggg_test_clamp_max_edge_cases = 0;
    }
    #[test]
    #[should_panic(expected = "max must not be NAN")]
    fn test_clamp_max_nan_max() {
        let _rug_st_tests_llm_16_2012_rrrruuuugggg_test_clamp_max_nan_max = 0;
        let rug_fuzz_0 = 1.0;
        let _ = clamp_max(rug_fuzz_0, std::f32::NAN);
        let _rug_ed_tests_llm_16_2012_rrrruuuugggg_test_clamp_max_nan_max = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2013 {
    use super::*;
    use crate::*;
    #[test]
    fn test_clamp_min_greater_than_min() {
        let _rug_st_tests_llm_16_2013_rrrruuuugggg_test_clamp_min_greater_than_min = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 20;
        let min = rug_fuzz_0;
        let input = rug_fuzz_1;
        debug_assert_eq!(clamp_min(input, min), input);
        let _rug_ed_tests_llm_16_2013_rrrruuuugggg_test_clamp_min_greater_than_min = 0;
    }
    #[test]
    fn test_clamp_min_less_than_min() {
        let _rug_st_tests_llm_16_2013_rrrruuuugggg_test_clamp_min_less_than_min = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 5;
        let min = rug_fuzz_0;
        let input = rug_fuzz_1;
        debug_assert_eq!(clamp_min(input, min), min);
        let _rug_ed_tests_llm_16_2013_rrrruuuugggg_test_clamp_min_less_than_min = 0;
    }
    #[test]
    fn test_clamp_min_equal_to_min() {
        let _rug_st_tests_llm_16_2013_rrrruuuugggg_test_clamp_min_equal_to_min = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = 10;
        let min = rug_fuzz_0;
        let input = rug_fuzz_1;
        debug_assert_eq!(clamp_min(input, min), input);
        let _rug_ed_tests_llm_16_2013_rrrruuuugggg_test_clamp_min_equal_to_min = 0;
    }
    #[test]
    fn test_clamp_min_float_greater_than_min() {
        let _rug_st_tests_llm_16_2013_rrrruuuugggg_test_clamp_min_float_greater_than_min = 0;
        let rug_fuzz_0 = 10.0;
        let rug_fuzz_1 = 20.0;
        let min = rug_fuzz_0;
        let input = rug_fuzz_1;
        debug_assert_eq!(clamp_min(input, min), input);
        let _rug_ed_tests_llm_16_2013_rrrruuuugggg_test_clamp_min_float_greater_than_min = 0;
    }
    #[test]
    fn test_clamp_min_float_less_than_min() {
        let _rug_st_tests_llm_16_2013_rrrruuuugggg_test_clamp_min_float_less_than_min = 0;
        let rug_fuzz_0 = 10.0;
        let rug_fuzz_1 = 5.0;
        let min = rug_fuzz_0;
        let input = rug_fuzz_1;
        debug_assert_eq!(clamp_min(input, min), min);
        let _rug_ed_tests_llm_16_2013_rrrruuuugggg_test_clamp_min_float_less_than_min = 0;
    }
    #[test]
    fn test_clamp_min_float_equal_to_min() {
        let _rug_st_tests_llm_16_2013_rrrruuuugggg_test_clamp_min_float_equal_to_min = 0;
        let rug_fuzz_0 = 10.0;
        let rug_fuzz_1 = 10.0;
        let min = rug_fuzz_0;
        let input = rug_fuzz_1;
        debug_assert_eq!(clamp_min(input, min), input);
        let _rug_ed_tests_llm_16_2013_rrrruuuugggg_test_clamp_min_float_equal_to_min = 0;
    }
    #[test]
    fn test_clamp_min_nan_input() {
        let _rug_st_tests_llm_16_2013_rrrruuuugggg_test_clamp_min_nan_input = 0;
        let rug_fuzz_0 = 1.0;
        let min = rug_fuzz_0;
        let input = std::f32::NAN;
        debug_assert!(clamp_min(input, min).is_nan());
        let _rug_ed_tests_llm_16_2013_rrrruuuugggg_test_clamp_min_nan_input = 0;
    }
    #[test]
    #[should_panic(expected = "min must not be NAN")]
    fn test_clamp_min_nan_min() {
        let _rug_st_tests_llm_16_2013_rrrruuuugggg_test_clamp_min_nan_min = 0;
        let rug_fuzz_0 = 1.0;
        let min = std::f32::NAN;
        let input = rug_fuzz_0;
        let _ = clamp_min(input, min);
        let _rug_ed_tests_llm_16_2013_rrrruuuugggg_test_clamp_min_nan_min = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2107 {
    use crate::str_to_ascii_lower_eq_str;
    #[test]
    fn test_empty_strings() {
        let _rug_st_tests_llm_16_2107_rrrruuuugggg_test_empty_strings = 0;
        let rug_fuzz_0 = "";
        let rug_fuzz_1 = "";
        debug_assert!(str_to_ascii_lower_eq_str(rug_fuzz_0, rug_fuzz_1));
        let _rug_ed_tests_llm_16_2107_rrrruuuugggg_test_empty_strings = 0;
    }
    #[test]
    fn test_equal_strings() {
        let _rug_st_tests_llm_16_2107_rrrruuuugggg_test_equal_strings = 0;
        let rug_fuzz_0 = "rust";
        let rug_fuzz_1 = "rust";
        debug_assert!(str_to_ascii_lower_eq_str(rug_fuzz_0, rug_fuzz_1));
        let _rug_ed_tests_llm_16_2107_rrrruuuugggg_test_equal_strings = 0;
    }
    #[test]
    fn test_lowercase_to_uppercase() {
        let _rug_st_tests_llm_16_2107_rrrruuuugggg_test_lowercase_to_uppercase = 0;
        let rug_fuzz_0 = "rust";
        let rug_fuzz_1 = "RUST";
        debug_assert!(str_to_ascii_lower_eq_str(rug_fuzz_0, rug_fuzz_1));
        let _rug_ed_tests_llm_16_2107_rrrruuuugggg_test_lowercase_to_uppercase = 0;
    }
    #[test]
    fn test_uppercase_to_lowercase() {
        let _rug_st_tests_llm_16_2107_rrrruuuugggg_test_uppercase_to_lowercase = 0;
        let rug_fuzz_0 = "RUST";
        let rug_fuzz_1 = "rust";
        debug_assert!(str_to_ascii_lower_eq_str(rug_fuzz_0, rug_fuzz_1));
        let _rug_ed_tests_llm_16_2107_rrrruuuugggg_test_uppercase_to_lowercase = 0;
    }
    #[test]
    fn test_mixed_case() {
        let _rug_st_tests_llm_16_2107_rrrruuuugggg_test_mixed_case = 0;
        let rug_fuzz_0 = "RuSt";
        let rug_fuzz_1 = "rust";
        debug_assert!(str_to_ascii_lower_eq_str(rug_fuzz_0, rug_fuzz_1));
        let _rug_ed_tests_llm_16_2107_rrrruuuugggg_test_mixed_case = 0;
    }
    #[test]
    fn test_non_ascii() {
        let _rug_st_tests_llm_16_2107_rrrruuuugggg_test_non_ascii = 0;
        let rug_fuzz_0 = "фу";
        let rug_fuzz_1 = "ФУ";
        debug_assert!(! str_to_ascii_lower_eq_str(rug_fuzz_0, rug_fuzz_1));
        let _rug_ed_tests_llm_16_2107_rrrruuuugggg_test_non_ascii = 0;
    }
    #[test]
    fn test_different_lengths() {
        let _rug_st_tests_llm_16_2107_rrrruuuugggg_test_different_lengths = 0;
        let rug_fuzz_0 = "hello";
        let rug_fuzz_1 = "helloworld";
        debug_assert!(! str_to_ascii_lower_eq_str(rug_fuzz_0, rug_fuzz_1));
        let _rug_ed_tests_llm_16_2107_rrrruuuugggg_test_different_lengths = 0;
    }
    #[test]
    fn test_shared_prefix() {
        let _rug_st_tests_llm_16_2107_rrrruuuugggg_test_shared_prefix = 0;
        let rug_fuzz_0 = "hello";
        let rug_fuzz_1 = "hell";
        debug_assert!(! str_to_ascii_lower_eq_str(rug_fuzz_0, rug_fuzz_1));
        let _rug_ed_tests_llm_16_2107_rrrruuuugggg_test_shared_prefix = 0;
    }
    #[test]
    fn test_different_chars() {
        let _rug_st_tests_llm_16_2107_rrrruuuugggg_test_different_chars = 0;
        let rug_fuzz_0 = "hello";
        let rug_fuzz_1 = "world";
        debug_assert!(! str_to_ascii_lower_eq_str(rug_fuzz_0, rug_fuzz_1));
        let _rug_ed_tests_llm_16_2107_rrrruuuugggg_test_different_chars = 0;
    }
}
