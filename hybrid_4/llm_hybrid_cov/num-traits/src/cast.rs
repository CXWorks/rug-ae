use core::mem::size_of;
use core::num::Wrapping;
use core::{f32, f64};
use core::{i128, i16, i32, i64, i8, isize};
use core::{u128, u16, u32, u64, u8, usize};
/// A generic trait for converting a value to a number.
///
/// A value can be represented by the target type when it lies within
/// the range of scalars supported by the target type.
/// For example, a negative integer cannot be represented by an unsigned
/// integer type, and an `i64` with a very high magnitude might not be
/// convertible to an `i32`.
/// On the other hand, conversions with possible precision loss or truncation
/// are admitted, like an `f32` with a decimal part to an integer type, or
/// even a large `f64` saturating to `f32` infinity.
pub trait ToPrimitive {
    /// Converts the value of `self` to an `isize`. If the value cannot be
    /// represented by an `isize`, then `None` is returned.
    #[inline]
    fn to_isize(&self) -> Option<isize> {
        self.to_i64().as_ref().and_then(ToPrimitive::to_isize)
    }
    /// Converts the value of `self` to an `i8`. If the value cannot be
    /// represented by an `i8`, then `None` is returned.
    #[inline]
    fn to_i8(&self) -> Option<i8> {
        self.to_i64().as_ref().and_then(ToPrimitive::to_i8)
    }
    /// Converts the value of `self` to an `i16`. If the value cannot be
    /// represented by an `i16`, then `None` is returned.
    #[inline]
    fn to_i16(&self) -> Option<i16> {
        self.to_i64().as_ref().and_then(ToPrimitive::to_i16)
    }
    /// Converts the value of `self` to an `i32`. If the value cannot be
    /// represented by an `i32`, then `None` is returned.
    #[inline]
    fn to_i32(&self) -> Option<i32> {
        self.to_i64().as_ref().and_then(ToPrimitive::to_i32)
    }
    /// Converts the value of `self` to an `i64`. If the value cannot be
    /// represented by an `i64`, then `None` is returned.
    fn to_i64(&self) -> Option<i64>;
    /// Converts the value of `self` to an `i128`. If the value cannot be
    /// represented by an `i128` (`i64` under the default implementation), then
    /// `None` is returned.
    ///
    /// The default implementation converts through `to_i64()`. Types implementing
    /// this trait should override this method if they can represent a greater range.
    #[inline]
    fn to_i128(&self) -> Option<i128> {
        self.to_i64().map(From::from)
    }
    /// Converts the value of `self` to a `usize`. If the value cannot be
    /// represented by a `usize`, then `None` is returned.
    #[inline]
    fn to_usize(&self) -> Option<usize> {
        self.to_u64().as_ref().and_then(ToPrimitive::to_usize)
    }
    /// Converts the value of `self` to a `u8`. If the value cannot be
    /// represented by a `u8`, then `None` is returned.
    #[inline]
    fn to_u8(&self) -> Option<u8> {
        self.to_u64().as_ref().and_then(ToPrimitive::to_u8)
    }
    /// Converts the value of `self` to a `u16`. If the value cannot be
    /// represented by a `u16`, then `None` is returned.
    #[inline]
    fn to_u16(&self) -> Option<u16> {
        self.to_u64().as_ref().and_then(ToPrimitive::to_u16)
    }
    /// Converts the value of `self` to a `u32`. If the value cannot be
    /// represented by a `u32`, then `None` is returned.
    #[inline]
    fn to_u32(&self) -> Option<u32> {
        self.to_u64().as_ref().and_then(ToPrimitive::to_u32)
    }
    /// Converts the value of `self` to a `u64`. If the value cannot be
    /// represented by a `u64`, then `None` is returned.
    fn to_u64(&self) -> Option<u64>;
    /// Converts the value of `self` to a `u128`. If the value cannot be
    /// represented by a `u128` (`u64` under the default implementation), then
    /// `None` is returned.
    ///
    /// The default implementation converts through `to_u64()`. Types implementing
    /// this trait should override this method if they can represent a greater range.
    #[inline]
    fn to_u128(&self) -> Option<u128> {
        self.to_u64().map(From::from)
    }
    /// Converts the value of `self` to an `f32`. Overflows may map to positive
    /// or negative inifinity, otherwise `None` is returned if the value cannot
    /// be represented by an `f32`.
    #[inline]
    fn to_f32(&self) -> Option<f32> {
        self.to_f64().as_ref().and_then(ToPrimitive::to_f32)
    }
    /// Converts the value of `self` to an `f64`. Overflows may map to positive
    /// or negative inifinity, otherwise `None` is returned if the value cannot
    /// be represented by an `f64`.
    ///
    /// The default implementation tries to convert through `to_i64()`, and
    /// failing that through `to_u64()`. Types implementing this trait should
    /// override this method if they can represent a greater range.
    #[inline]
    fn to_f64(&self) -> Option<f64> {
        match self.to_i64() {
            Some(i) => i.to_f64(),
            None => self.to_u64().as_ref().and_then(ToPrimitive::to_f64),
        }
    }
}
macro_rules! impl_to_primitive_int_to_int {
    ($SrcT:ident : $($(#[$cfg:meta])* fn $method:ident -> $DstT:ident;)*) => {
        $(#[inline] $(#[$cfg])* fn $method (& self) -> Option <$DstT > { let min = $DstT
        ::MIN as $SrcT; let max = $DstT ::MAX as $SrcT; if size_of::<$SrcT > () <=
        size_of::<$DstT > () || (min <= * self && * self <= max) { Some(* self as $DstT)
        } else { None } })*
    };
}
macro_rules! impl_to_primitive_int_to_uint {
    ($SrcT:ident : $($(#[$cfg:meta])* fn $method:ident -> $DstT:ident;)*) => {
        $(#[inline] $(#[$cfg])* fn $method (& self) -> Option <$DstT > { let max = $DstT
        ::MAX as $SrcT; if 0 <= * self && (size_of::<$SrcT > () <= size_of::<$DstT > ()
        || * self <= max) { Some(* self as $DstT) } else { None } })*
    };
}
macro_rules! impl_to_primitive_int {
    ($T:ident) => {
        impl ToPrimitive for $T { impl_to_primitive_int_to_int! { $T : fn to_isize ->
        isize; fn to_i8 -> i8; fn to_i16 -> i16; fn to_i32 -> i32; fn to_i64 -> i64; fn
        to_i128 -> i128; } impl_to_primitive_int_to_uint! { $T : fn to_usize -> usize; fn
        to_u8 -> u8; fn to_u16 -> u16; fn to_u32 -> u32; fn to_u64 -> u64; fn to_u128 ->
        u128; } #[inline] fn to_f32(& self) -> Option < f32 > { Some(* self as f32) }
        #[inline] fn to_f64(& self) -> Option < f64 > { Some(* self as f64) } }
    };
}
impl_to_primitive_int!(isize);
impl_to_primitive_int!(i8);
impl_to_primitive_int!(i16);
impl_to_primitive_int!(i32);
impl_to_primitive_int!(i64);
impl_to_primitive_int!(i128);
macro_rules! impl_to_primitive_uint_to_int {
    ($SrcT:ident : $($(#[$cfg:meta])* fn $method:ident -> $DstT:ident;)*) => {
        $(#[inline] $(#[$cfg])* fn $method (& self) -> Option <$DstT > { let max = $DstT
        ::MAX as $SrcT; if size_of::<$SrcT > () < size_of::<$DstT > () || * self <= max {
        Some(* self as $DstT) } else { None } })*
    };
}
macro_rules! impl_to_primitive_uint_to_uint {
    ($SrcT:ident : $($(#[$cfg:meta])* fn $method:ident -> $DstT:ident;)*) => {
        $(#[inline] $(#[$cfg])* fn $method (& self) -> Option <$DstT > { let max = $DstT
        ::MAX as $SrcT; if size_of::<$SrcT > () <= size_of::<$DstT > () || * self <= max
        { Some(* self as $DstT) } else { None } })*
    };
}
macro_rules! impl_to_primitive_uint {
    ($T:ident) => {
        impl ToPrimitive for $T { impl_to_primitive_uint_to_int! { $T : fn to_isize ->
        isize; fn to_i8 -> i8; fn to_i16 -> i16; fn to_i32 -> i32; fn to_i64 -> i64; fn
        to_i128 -> i128; } impl_to_primitive_uint_to_uint! { $T : fn to_usize -> usize;
        fn to_u8 -> u8; fn to_u16 -> u16; fn to_u32 -> u32; fn to_u64 -> u64; fn to_u128
        -> u128; } #[inline] fn to_f32(& self) -> Option < f32 > { Some(* self as f32) }
        #[inline] fn to_f64(& self) -> Option < f64 > { Some(* self as f64) } }
    };
}
impl_to_primitive_uint!(usize);
impl_to_primitive_uint!(u8);
impl_to_primitive_uint!(u16);
impl_to_primitive_uint!(u32);
impl_to_primitive_uint!(u64);
impl_to_primitive_uint!(u128);
macro_rules! impl_to_primitive_float_to_float {
    ($SrcT:ident : $(fn $method:ident -> $DstT:ident;)*) => {
        $(#[inline] fn $method (& self) -> Option <$DstT > { Some(* self as $DstT) })*
    };
}
#[cfg(has_to_int_unchecked)]
macro_rules! float_to_int_unchecked {
    ($float:expr => $int:ty) => {
        unsafe { $float .to_int_unchecked::<$int > () }
    };
}
#[cfg(not(has_to_int_unchecked))]
macro_rules! float_to_int_unchecked {
    ($float:expr => $int:ty) => {
        $float as $int
    };
}
macro_rules! impl_to_primitive_float_to_signed_int {
    ($f:ident : $($(#[$cfg:meta])* fn $method:ident -> $i:ident;)*) => {
        $(#[inline] $(#[$cfg])* fn $method (& self) -> Option <$i > { if size_of::<$f >
        () > size_of::<$i > () { const MIN_M1 : $f = $i ::MIN as $f - 1.0; const MAX_P1 :
        $f = $i ::MAX as $f + 1.0; if * self > MIN_M1 && * self < MAX_P1 { return
        Some(float_to_int_unchecked!(* self => $i)); } } else { const MIN : $f = $i ::MIN
        as $f; const MAX_P1 : $f = $i ::MAX as $f; if * self >= MIN && * self < MAX_P1 {
        return Some(float_to_int_unchecked!(* self => $i)); } } None })*
    };
}
macro_rules! impl_to_primitive_float_to_unsigned_int {
    ($f:ident : $($(#[$cfg:meta])* fn $method:ident -> $u:ident;)*) => {
        $(#[inline] $(#[$cfg])* fn $method (& self) -> Option <$u > { if size_of::<$f >
        () > size_of::<$u > () { const MAX_P1 : $f = $u ::MAX as $f + 1.0; if * self > -
        1.0 && * self < MAX_P1 { return Some(float_to_int_unchecked!(* self => $u)); } }
        else { const MAX_P1 : $f = $u ::MAX as $f; if * self > - 1.0 && * self < MAX_P1 {
        return Some(float_to_int_unchecked!(* self => $u)); } } None })*
    };
}
macro_rules! impl_to_primitive_float {
    ($T:ident) => {
        impl ToPrimitive for $T { impl_to_primitive_float_to_signed_int! { $T : fn
        to_isize -> isize; fn to_i8 -> i8; fn to_i16 -> i16; fn to_i32 -> i32; fn to_i64
        -> i64; fn to_i128 -> i128; } impl_to_primitive_float_to_unsigned_int! { $T : fn
        to_usize -> usize; fn to_u8 -> u8; fn to_u16 -> u16; fn to_u32 -> u32; fn to_u64
        -> u64; fn to_u128 -> u128; } impl_to_primitive_float_to_float! { $T : fn to_f32
        -> f32; fn to_f64 -> f64; } }
    };
}
impl_to_primitive_float!(f32);
impl_to_primitive_float!(f64);
/// A generic trait for converting a number to a value.
///
/// A value can be represented by the target type when it lies within
/// the range of scalars supported by the target type.
/// For example, a negative integer cannot be represented by an unsigned
/// integer type, and an `i64` with a very high magnitude might not be
/// convertible to an `i32`.
/// On the other hand, conversions with possible precision loss or truncation
/// are admitted, like an `f32` with a decimal part to an integer type, or
/// even a large `f64` saturating to `f32` infinity.
pub trait FromPrimitive: Sized {
    /// Converts an `isize` to return an optional value of this type. If the
    /// value cannot be represented by this type, then `None` is returned.
    #[inline]
    fn from_isize(n: isize) -> Option<Self> {
        n.to_i64().and_then(FromPrimitive::from_i64)
    }
    /// Converts an `i8` to return an optional value of this type. If the
    /// value cannot be represented by this type, then `None` is returned.
    #[inline]
    fn from_i8(n: i8) -> Option<Self> {
        FromPrimitive::from_i64(From::from(n))
    }
    /// Converts an `i16` to return an optional value of this type. If the
    /// value cannot be represented by this type, then `None` is returned.
    #[inline]
    fn from_i16(n: i16) -> Option<Self> {
        FromPrimitive::from_i64(From::from(n))
    }
    /// Converts an `i32` to return an optional value of this type. If the
    /// value cannot be represented by this type, then `None` is returned.
    #[inline]
    fn from_i32(n: i32) -> Option<Self> {
        FromPrimitive::from_i64(From::from(n))
    }
    /// Converts an `i64` to return an optional value of this type. If the
    /// value cannot be represented by this type, then `None` is returned.
    fn from_i64(n: i64) -> Option<Self>;
    /// Converts an `i128` to return an optional value of this type. If the
    /// value cannot be represented by this type, then `None` is returned.
    ///
    /// The default implementation converts through `from_i64()`. Types implementing
    /// this trait should override this method if they can represent a greater range.
    #[inline]
    fn from_i128(n: i128) -> Option<Self> {
        n.to_i64().and_then(FromPrimitive::from_i64)
    }
    /// Converts a `usize` to return an optional value of this type. If the
    /// value cannot be represented by this type, then `None` is returned.
    #[inline]
    fn from_usize(n: usize) -> Option<Self> {
        n.to_u64().and_then(FromPrimitive::from_u64)
    }
    /// Converts an `u8` to return an optional value of this type. If the
    /// value cannot be represented by this type, then `None` is returned.
    #[inline]
    fn from_u8(n: u8) -> Option<Self> {
        FromPrimitive::from_u64(From::from(n))
    }
    /// Converts an `u16` to return an optional value of this type. If the
    /// value cannot be represented by this type, then `None` is returned.
    #[inline]
    fn from_u16(n: u16) -> Option<Self> {
        FromPrimitive::from_u64(From::from(n))
    }
    /// Converts an `u32` to return an optional value of this type. If the
    /// value cannot be represented by this type, then `None` is returned.
    #[inline]
    fn from_u32(n: u32) -> Option<Self> {
        FromPrimitive::from_u64(From::from(n))
    }
    /// Converts an `u64` to return an optional value of this type. If the
    /// value cannot be represented by this type, then `None` is returned.
    fn from_u64(n: u64) -> Option<Self>;
    /// Converts an `u128` to return an optional value of this type. If the
    /// value cannot be represented by this type, then `None` is returned.
    ///
    /// The default implementation converts through `from_u64()`. Types implementing
    /// this trait should override this method if they can represent a greater range.
    #[inline]
    fn from_u128(n: u128) -> Option<Self> {
        n.to_u64().and_then(FromPrimitive::from_u64)
    }
    /// Converts a `f32` to return an optional value of this type. If the
    /// value cannot be represented by this type, then `None` is returned.
    #[inline]
    fn from_f32(n: f32) -> Option<Self> {
        FromPrimitive::from_f64(From::from(n))
    }
    /// Converts a `f64` to return an optional value of this type. If the
    /// value cannot be represented by this type, then `None` is returned.
    ///
    /// The default implementation tries to convert through `from_i64()`, and
    /// failing that through `from_u64()`. Types implementing this trait should
    /// override this method if they can represent a greater range.
    #[inline]
    fn from_f64(n: f64) -> Option<Self> {
        match n.to_i64() {
            Some(i) => FromPrimitive::from_i64(i),
            None => n.to_u64().and_then(FromPrimitive::from_u64),
        }
    }
}
macro_rules! impl_from_primitive {
    ($T:ty, $to_ty:ident) => {
        #[allow(deprecated)] impl FromPrimitive for $T { #[inline] fn from_isize(n :
        isize) -> Option <$T > { n.$to_ty () } #[inline] fn from_i8(n : i8) -> Option <$T
        > { n.$to_ty () } #[inline] fn from_i16(n : i16) -> Option <$T > { n.$to_ty () }
        #[inline] fn from_i32(n : i32) -> Option <$T > { n.$to_ty () } #[inline] fn
        from_i64(n : i64) -> Option <$T > { n.$to_ty () } #[inline] fn from_i128(n :
        i128) -> Option <$T > { n.$to_ty () } #[inline] fn from_usize(n : usize) ->
        Option <$T > { n.$to_ty () } #[inline] fn from_u8(n : u8) -> Option <$T > { n
        .$to_ty () } #[inline] fn from_u16(n : u16) -> Option <$T > { n.$to_ty () }
        #[inline] fn from_u32(n : u32) -> Option <$T > { n.$to_ty () } #[inline] fn
        from_u64(n : u64) -> Option <$T > { n.$to_ty () } #[inline] fn from_u128(n :
        u128) -> Option <$T > { n.$to_ty () } #[inline] fn from_f32(n : f32) -> Option
        <$T > { n.$to_ty () } #[inline] fn from_f64(n : f64) -> Option <$T > { n.$to_ty
        () } }
    };
}
impl_from_primitive!(isize, to_isize);
impl_from_primitive!(i8, to_i8);
impl_from_primitive!(i16, to_i16);
impl_from_primitive!(i32, to_i32);
impl_from_primitive!(i64, to_i64);
impl_from_primitive!(i128, to_i128);
impl_from_primitive!(usize, to_usize);
impl_from_primitive!(u8, to_u8);
impl_from_primitive!(u16, to_u16);
impl_from_primitive!(u32, to_u32);
impl_from_primitive!(u64, to_u64);
impl_from_primitive!(u128, to_u128);
impl_from_primitive!(f32, to_f32);
impl_from_primitive!(f64, to_f64);
macro_rules! impl_to_primitive_wrapping {
    ($($(#[$cfg:meta])* fn $method:ident -> $i:ident;)*) => {
        $(#[inline] $(#[$cfg])* fn $method (& self) -> Option <$i > { (self.0).$method ()
        })*
    };
}
impl<T: ToPrimitive> ToPrimitive for Wrapping<T> {
    impl_to_primitive_wrapping! {
        fn to_isize -> isize; fn to_i8 -> i8; fn to_i16 -> i16; fn to_i32 -> i32; fn
        to_i64 -> i64; fn to_i128 -> i128; fn to_usize -> usize; fn to_u8 -> u8; fn
        to_u16 -> u16; fn to_u32 -> u32; fn to_u64 -> u64; fn to_u128 -> u128; fn to_f32
        -> f32; fn to_f64 -> f64;
    }
}
macro_rules! impl_from_primitive_wrapping {
    ($($(#[$cfg:meta])* fn $method:ident ($i:ident);)*) => {
        $(#[inline] $(#[$cfg])* fn $method (n : $i) -> Option < Self > { T::$method (n)
        .map(Wrapping) })*
    };
}
impl<T: FromPrimitive> FromPrimitive for Wrapping<T> {
    impl_from_primitive_wrapping! {
        fn from_isize(isize); fn from_i8(i8); fn from_i16(i16); fn from_i32(i32); fn
        from_i64(i64); fn from_i128(i128); fn from_usize(usize); fn from_u8(u8); fn
        from_u16(u16); fn from_u32(u32); fn from_u64(u64); fn from_u128(u128); fn
        from_f32(f32); fn from_f64(f64);
    }
}
/// Cast from one machine scalar to another.
///
/// # Examples
///
/// ```
/// # use num_traits as num;
/// let twenty: f32 = num::cast(0x14).unwrap();
/// assert_eq!(twenty, 20f32);
/// ```
///
#[inline]
pub fn cast<T: NumCast, U: NumCast>(n: T) -> Option<U> {
    NumCast::from(n)
}
/// An interface for casting between machine scalars.
pub trait NumCast: Sized + ToPrimitive {
    /// Creates a number from another value that can be converted into
    /// a primitive via the `ToPrimitive` trait. If the source value cannot be
    /// represented by the target type, then `None` is returned.
    ///
    /// A value can be represented by the target type when it lies within
    /// the range of scalars supported by the target type.
    /// For example, a negative integer cannot be represented by an unsigned
    /// integer type, and an `i64` with a very high magnitude might not be
    /// convertible to an `i32`.
    /// On the other hand, conversions with possible precision loss or truncation
    /// are admitted, like an `f32` with a decimal part to an integer type, or
    /// even a large `f64` saturating to `f32` infinity.
    fn from<T: ToPrimitive>(n: T) -> Option<Self>;
}
macro_rules! impl_num_cast {
    ($T:ty, $conv:ident) => {
        impl NumCast for $T { #[inline] #[allow(deprecated)] fn from < N : ToPrimitive >
        (n : N) -> Option <$T > { n.$conv () } }
    };
}
impl_num_cast!(u8, to_u8);
impl_num_cast!(u16, to_u16);
impl_num_cast!(u32, to_u32);
impl_num_cast!(u64, to_u64);
impl_num_cast!(u128, to_u128);
impl_num_cast!(usize, to_usize);
impl_num_cast!(i8, to_i8);
impl_num_cast!(i16, to_i16);
impl_num_cast!(i32, to_i32);
impl_num_cast!(i64, to_i64);
impl_num_cast!(i128, to_i128);
impl_num_cast!(isize, to_isize);
impl_num_cast!(f32, to_f32);
impl_num_cast!(f64, to_f64);
impl<T: NumCast> NumCast for Wrapping<T> {
    fn from<U: ToPrimitive>(n: U) -> Option<Self> {
        T::from(n).map(Wrapping)
    }
}
/// A generic interface for casting between machine scalars with the
/// `as` operator, which admits narrowing and precision loss.
/// Implementers of this trait `AsPrimitive` should behave like a primitive
/// numeric type (e.g. a newtype around another primitive), and the
/// intended conversion must never fail.
///
/// # Examples
///
/// ```
/// # use num_traits::AsPrimitive;
/// let three: i32 = (3.14159265f32).as_();
/// assert_eq!(three, 3);
/// ```
///
/// # Safety
///
/// **In Rust versions before 1.45.0**, some uses of the `as` operator were not entirely safe.
/// In particular, it was undefined behavior if
/// a truncated floating point value could not fit in the target integer
/// type ([#10184](https://github.com/rust-lang/rust/issues/10184)).
///
/// ```ignore
/// # use num_traits::AsPrimitive;
/// let x: u8 = (1.04E+17).as_(); // UB
/// ```
///
pub trait AsPrimitive<T>: 'static + Copy
where
    T: 'static + Copy,
{
    /// Convert a value to another, using the `as` operator.
    fn as_(self) -> T;
}
macro_rules! impl_as_primitive {
    (@ $T:ty => $(#[$cfg:meta])* impl $U:ty) => {
        $(#[$cfg])* impl AsPrimitive <$U > for $T { #[inline] fn as_(self) -> $U { self
        as $U } }
    };
    (@ $T:ty => { $($U:ty),* }) => {
        $(impl_as_primitive!(@ $T => impl $U);)*
    };
    ($T:ty => { $($U:ty),* }) => {
        impl_as_primitive!(@ $T => { $($U),* }); impl_as_primitive!(@ $T => { u8, u16,
        u32, u64, u128, usize }); impl_as_primitive!(@ $T => { i8, i16, i32, i64, i128,
        isize });
    };
}
impl_as_primitive!(u8 => { char, f32, f64 });
impl_as_primitive!(i8 => { f32, f64 });
impl_as_primitive!(u16 => { f32, f64 });
impl_as_primitive!(i16 => { f32, f64 });
impl_as_primitive!(u32 => { f32, f64 });
impl_as_primitive!(i32 => { f32, f64 });
impl_as_primitive!(u64 => { f32, f64 });
impl_as_primitive!(i64 => { f32, f64 });
impl_as_primitive!(u128 => { f32, f64 });
impl_as_primitive!(i128 => { f32, f64 });
impl_as_primitive!(usize => { f32, f64 });
impl_as_primitive!(isize => { f32, f64 });
impl_as_primitive!(f32 => { f32, f64 });
impl_as_primitive!(f64 => { f32, f64 });
impl_as_primitive!(char => { char });
impl_as_primitive!(bool => {});
#[cfg(test)]
mod tests_llm_16_238_llm_16_238 {
    use super::*;
    use crate::*;
    use crate::cast::AsPrimitive;
    #[test]
    fn test_bool_as_i128() {
        let _rug_st_tests_llm_16_238_llm_16_238_rrrruuuugggg_test_bool_as_i128 = 0;
        let rug_fuzz_0 = false;
        let rug_fuzz_1 = true;
        debug_assert_eq!(< bool as AsPrimitive < i128 > > ::as_(rug_fuzz_0), 0i128);
        debug_assert_eq!(< bool as AsPrimitive < i128 > > ::as_(rug_fuzz_1), 1i128);
        let _rug_ed_tests_llm_16_238_llm_16_238_rrrruuuugggg_test_bool_as_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_240_llm_16_240 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_from_bool_to_i32() {
        let _rug_st_tests_llm_16_240_llm_16_240_rrrruuuugggg_test_as_primitive_from_bool_to_i32 = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = false;
        debug_assert_eq!(AsPrimitive:: < i32 > ::as_(rug_fuzz_0), 1i32);
        debug_assert_eq!(AsPrimitive:: < i32 > ::as_(rug_fuzz_1), 0i32);
        let _rug_ed_tests_llm_16_240_llm_16_240_rrrruuuugggg_test_as_primitive_from_bool_to_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_241_llm_16_241 {
    use crate::AsPrimitive;
    #[test]
    fn test_bool_as_i64() {
        let _rug_st_tests_llm_16_241_llm_16_241_rrrruuuugggg_test_bool_as_i64 = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = false;
        debug_assert_eq!(< bool as AsPrimitive < i64 > > ::as_(rug_fuzz_0), 1i64);
        debug_assert_eq!(< bool as AsPrimitive < i64 > > ::as_(rug_fuzz_1), 0i64);
        let _rug_ed_tests_llm_16_241_llm_16_241_rrrruuuugggg_test_bool_as_i64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_242_llm_16_242 {
    use crate::cast::AsPrimitive;
    #[test]
    fn bool_as_i8() {
        let _rug_st_tests_llm_16_242_llm_16_242_rrrruuuugggg_bool_as_i8 = 0;
        let rug_fuzz_0 = false;
        let rug_fuzz_1 = true;
        debug_assert_eq!(< bool as AsPrimitive < i8 > > ::as_(rug_fuzz_0), 0i8);
        debug_assert_eq!(< bool as AsPrimitive < i8 > > ::as_(rug_fuzz_1), 1i8);
        let _rug_ed_tests_llm_16_242_llm_16_242_rrrruuuugggg_bool_as_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_243_llm_16_243 {
    use crate::AsPrimitive;
    #[test]
    fn test_bool_as_isize() {
        let _rug_st_tests_llm_16_243_llm_16_243_rrrruuuugggg_test_bool_as_isize = 0;
        let rug_fuzz_0 = false;
        let rug_fuzz_1 = true;
        debug_assert_eq!(< bool as AsPrimitive < isize > > ::as_(rug_fuzz_0), 0isize);
        debug_assert_eq!(< bool as AsPrimitive < isize > > ::as_(rug_fuzz_1), 1isize);
        let _rug_ed_tests_llm_16_243_llm_16_243_rrrruuuugggg_test_bool_as_isize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_245_llm_16_245 {
    use crate::cast::AsPrimitive;
    #[test]
    fn bool_as_u16() {
        let _rug_st_tests_llm_16_245_llm_16_245_rrrruuuugggg_bool_as_u16 = 0;
        let rug_fuzz_0 = false;
        let rug_fuzz_1 = true;
        debug_assert_eq!(AsPrimitive:: < u16 > ::as_(rug_fuzz_0), 0u16);
        debug_assert_eq!(AsPrimitive:: < u16 > ::as_(rug_fuzz_1), 1u16);
        let _rug_ed_tests_llm_16_245_llm_16_245_rrrruuuugggg_bool_as_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_246_llm_16_246 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_bool_to_u32() {
        let _rug_st_tests_llm_16_246_llm_16_246_rrrruuuugggg_test_as_primitive_bool_to_u32 = 0;
        let rug_fuzz_0 = false;
        let rug_fuzz_1 = true;
        debug_assert_eq!(AsPrimitive:: < u32 > ::as_(rug_fuzz_0), 0_u32);
        debug_assert_eq!(AsPrimitive:: < u32 > ::as_(rug_fuzz_1), 1_u32);
        let _rug_ed_tests_llm_16_246_llm_16_246_rrrruuuugggg_test_as_primitive_bool_to_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_247_llm_16_247 {
    use crate::cast::AsPrimitive;
    #[test]
    fn bool_as_u64() {
        let _rug_st_tests_llm_16_247_llm_16_247_rrrruuuugggg_bool_as_u64 = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = false;
        debug_assert_eq!(< bool as AsPrimitive < u64 > > ::as_(rug_fuzz_0), 1u64);
        debug_assert_eq!(< bool as AsPrimitive < u64 > > ::as_(rug_fuzz_1), 0u64);
        let _rug_ed_tests_llm_16_247_llm_16_247_rrrruuuugggg_bool_as_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_248_llm_16_248 {
    use crate::AsPrimitive;
    #[test]
    fn bool_to_u8_as_primitive() {
        let _rug_st_tests_llm_16_248_llm_16_248_rrrruuuugggg_bool_to_u8_as_primitive = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = false;
        debug_assert_eq!(< bool as AsPrimitive < u8 > > ::as_(rug_fuzz_0), 1u8);
        debug_assert_eq!(< bool as AsPrimitive < u8 > > ::as_(rug_fuzz_1), 0u8);
        let _rug_ed_tests_llm_16_248_llm_16_248_rrrruuuugggg_bool_to_u8_as_primitive = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_249_llm_16_249 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_bool_as_usize() {
        let _rug_st_tests_llm_16_249_llm_16_249_rrrruuuugggg_test_bool_as_usize = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = false;
        debug_assert_eq!(< bool as AsPrimitive < usize > > ::as_(rug_fuzz_0), 1);
        debug_assert_eq!(< bool as AsPrimitive < usize > > ::as_(rug_fuzz_1), 0);
        let _rug_ed_tests_llm_16_249_llm_16_249_rrrruuuugggg_test_bool_as_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_250_llm_16_250 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_char_as_primitive_char() {
        let _rug_st_tests_llm_16_250_llm_16_250_rrrruuuugggg_test_char_as_primitive_char = 0;
        let rug_fuzz_0 = 'a';
        let x: char = rug_fuzz_0;
        let y: char = AsPrimitive::<char>::as_(x);
        debug_assert_eq!(x, y);
        let _rug_ed_tests_llm_16_250_llm_16_250_rrrruuuugggg_test_char_as_primitive_char = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_251_llm_16_251 {
    use crate::cast::AsPrimitive;
    #[test]
    fn char_as_i128() {
        let _rug_st_tests_llm_16_251_llm_16_251_rrrruuuugggg_char_as_i128 = 0;
        let rug_fuzz_0 = 'a';
        let rug_fuzz_1 = 'a';
        let c = rug_fuzz_0;
        let actual = <char as AsPrimitive<i128>>::as_(c);
        let expected = rug_fuzz_1 as i128;
        debug_assert_eq!(actual, expected);
        let _rug_ed_tests_llm_16_251_llm_16_251_rrrruuuugggg_char_as_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_252_llm_16_252 {
    use super::*;
    use crate::*;
    use crate::cast::AsPrimitive;
    #[test]
    fn test_char_as_primitive_i16() {
        let _rug_st_tests_llm_16_252_llm_16_252_rrrruuuugggg_test_char_as_primitive_i16 = 0;
        let rug_fuzz_0 = 'a';
        let ch = rug_fuzz_0;
        let value_as_i16: i16 = AsPrimitive::as_(ch);
        debug_assert_eq!(value_as_i16, 'a' as i16);
        let _rug_ed_tests_llm_16_252_llm_16_252_rrrruuuugggg_test_char_as_primitive_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_253_llm_16_253 {
    use crate::cast::AsPrimitive;
    #[test]
    fn char_as_i32() {
        let _rug_st_tests_llm_16_253_llm_16_253_rrrruuuugggg_char_as_i32 = 0;
        let rug_fuzz_0 = 'a';
        let rug_fuzz_1 = 'a';
        let c: char = rug_fuzz_0;
        let expected: i32 = rug_fuzz_1 as i32;
        let result: i32 = AsPrimitive::<i32>::as_(c);
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_253_llm_16_253_rrrruuuugggg_char_as_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_254_llm_16_254 {
    use crate::cast::AsPrimitive;
    #[test]
    fn char_as_i64() {
        let _rug_st_tests_llm_16_254_llm_16_254_rrrruuuugggg_char_as_i64 = 0;
        let rug_fuzz_0 = 'A';
        let rug_fuzz_1 = 65;
        let char_val = rug_fuzz_0;
        let i64_val: i64 = AsPrimitive::<i64>::as_(char_val);
        let expected_i64_val: i64 = rug_fuzz_1;
        debug_assert_eq!(i64_val, expected_i64_val);
        let _rug_ed_tests_llm_16_254_llm_16_254_rrrruuuugggg_char_as_i64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_255_llm_16_255 {
    use crate::cast::AsPrimitive;
    #[test]
    fn as_primitive_char_to_i8() {
        let _rug_st_tests_llm_16_255_llm_16_255_rrrruuuugggg_as_primitive_char_to_i8 = 0;
        let rug_fuzz_0 = 'a';
        let rug_fuzz_1 = 97i8;
        let rug_fuzz_2 = 'z';
        let rug_fuzz_3 = 122i8;
        let rug_fuzz_4 = '\u{1234}';
        let c = rug_fuzz_0;
        let expected = rug_fuzz_1;
        let result: i8 = c.as_();
        debug_assert_eq!(result, expected, "Casting 'a' as i8 failed");
        let c = rug_fuzz_2;
        let expected = rug_fuzz_3;
        let result: i8 = c.as_();
        debug_assert_eq!(result, expected, "Casting 'z' as i8 failed");
        let c = rug_fuzz_4;
        debug_assert!(
            std::panic::catch_unwind(| | { let _ : i8 = c.as_(); }).is_err(),
            "Casting a char outside of the i8 range should panic"
        );
        let _rug_ed_tests_llm_16_255_llm_16_255_rrrruuuugggg_as_primitive_char_to_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_256_llm_16_256 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_char_to_isize() {
        let _rug_st_tests_llm_16_256_llm_16_256_rrrruuuugggg_test_as_primitive_char_to_isize = 0;
        let rug_fuzz_0 = 'a';
        let c = rug_fuzz_0;
        let value: isize = c.as_();
        debug_assert_eq!(value, 'a' as isize);
        let _rug_ed_tests_llm_16_256_llm_16_256_rrrruuuugggg_test_as_primitive_char_to_isize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_257_llm_16_257 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_for_char_to_u128() {
        let _rug_st_tests_llm_16_257_llm_16_257_rrrruuuugggg_test_as_primitive_for_char_to_u128 = 0;
        let rug_fuzz_0 = 'a';
        let rug_fuzz_1 = 97;
        let rug_fuzz_2 = '\u{10000}';
        let rug_fuzz_3 = 0x10000;
        let rug_fuzz_4 = '0';
        let rug_fuzz_5 = 48;
        let rug_fuzz_6 = '\u{0}';
        let rug_fuzz_7 = 0;
        let c = rug_fuzz_0;
        let expected: u128 = rug_fuzz_1;
        let result: u128 = AsPrimitive::<u128>::as_(c);
        debug_assert_eq!(
            result, expected, "Casting 'a' as u128 did not produce the expected value."
        );
        let c = rug_fuzz_2;
        let expected: u128 = rug_fuzz_3;
        let result: u128 = AsPrimitive::<u128>::as_(c);
        debug_assert_eq!(
            result, expected,
            "Casting char with code 0x10000 as u128 did not produce the expected value."
        );
        let c = rug_fuzz_4;
        let expected: u128 = rug_fuzz_5;
        let result: u128 = AsPrimitive::<u128>::as_(c);
        debug_assert_eq!(
            result, expected, "Casting '0' as u128 did not produce the expected value."
        );
        let c = rug_fuzz_6;
        let expected: u128 = rug_fuzz_7;
        let result: u128 = AsPrimitive::<u128>::as_(c);
        debug_assert_eq!(
            result, expected,
            "Casting NULL char as u128 did not produce the expected value."
        );
        let _rug_ed_tests_llm_16_257_llm_16_257_rrrruuuugggg_test_as_primitive_for_char_to_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_258_llm_16_258 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive() {
        let _rug_st_tests_llm_16_258_llm_16_258_rrrruuuugggg_test_as_primitive = 0;
        let rug_fuzz_0 = 'a';
        let c = rug_fuzz_0;
        let value_as_u16: u16 = AsPrimitive::<u16>::as_(c);
        debug_assert_eq!(value_as_u16, 'a' as u16);
        let _rug_ed_tests_llm_16_258_llm_16_258_rrrruuuugggg_test_as_primitive = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_260_llm_16_260 {
    use crate::AsPrimitive;
    #[test]
    fn test_char_as_primitive_u64() {
        let _rug_st_tests_llm_16_260_llm_16_260_rrrruuuugggg_test_char_as_primitive_u64 = 0;
        let rug_fuzz_0 = 'A';
        let c = rug_fuzz_0;
        let value_u64: u64 = AsPrimitive::<u64>::as_(c);
        debug_assert_eq!(value_u64, 'A' as u64);
        let _rug_ed_tests_llm_16_260_llm_16_260_rrrruuuugggg_test_char_as_primitive_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_261_llm_16_261 {
    use crate::cast::AsPrimitive;
    #[test]
    fn char_as_u8() {
        let _rug_st_tests_llm_16_261_llm_16_261_rrrruuuugggg_char_as_u8 = 0;
        let rug_fuzz_0 = 'A';
        let rug_fuzz_1 = 65;
        let c = rug_fuzz_0;
        let expected: u8 = rug_fuzz_1;
        let result: u8 = AsPrimitive::<u8>::as_(c);
        debug_assert_eq!(expected, result);
        let _rug_ed_tests_llm_16_261_llm_16_261_rrrruuuugggg_char_as_u8 = 0;
    }
    #[test]
    #[should_panic]
    fn char_as_u8_non_ascii() {
        let _rug_st_tests_llm_16_261_llm_16_261_rrrruuuugggg_char_as_u8_non_ascii = 0;
        let rug_fuzz_0 = 'é';
        let c = rug_fuzz_0;
        let _: u8 = AsPrimitive::<u8>::as_(c);
        let _rug_ed_tests_llm_16_261_llm_16_261_rrrruuuugggg_char_as_u8_non_ascii = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_262_llm_16_262 {
    use crate::cast::AsPrimitive;
    #[test]
    fn char_as_usize() {
        let _rug_st_tests_llm_16_262_llm_16_262_rrrruuuugggg_char_as_usize = 0;
        let rug_fuzz_0 = 'a';
        let rug_fuzz_1 = 'a';
        let c = rug_fuzz_0;
        let expected_usize: usize = rug_fuzz_1 as usize;
        let as_usize: usize = AsPrimitive::<usize>::as_(c);
        debug_assert_eq!(as_usize, expected_usize, "Casting char 'a' as usize failed");
        let _rug_ed_tests_llm_16_262_llm_16_262_rrrruuuugggg_char_as_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_268_llm_16_268 {
    use crate::AsPrimitive;
    #[test]
    fn test_as_primitive_f32_to_f32() {
        let _rug_st_tests_llm_16_268_llm_16_268_rrrruuuugggg_test_as_primitive_f32_to_f32 = 0;
        let rug_fuzz_0 = 5.5;
        let original: f32 = rug_fuzz_0;
        let casted: f32 = original.as_();
        debug_assert_eq!(casted, 5.5_f32);
        let _rug_ed_tests_llm_16_268_llm_16_268_rrrruuuugggg_test_as_primitive_f32_to_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_269_llm_16_269 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_f32_to_f64() {
        let _rug_st_tests_llm_16_269_llm_16_269_rrrruuuugggg_test_as_primitive_f32_to_f64 = 0;
        let rug_fuzz_0 = 123.456_f32;
        let value: f32 = rug_fuzz_0;
        let result: f64 = value.as_();
        debug_assert_eq!(result, value as f64);
        let _rug_ed_tests_llm_16_269_llm_16_269_rrrruuuugggg_test_as_primitive_f32_to_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_270 {
    use crate::AsPrimitive;
    #[test]
    fn f32_as_i128() {
        let _rug_st_tests_llm_16_270_rrrruuuugggg_f32_as_i128 = 0;
        let rug_fuzz_0 = 42.0;
        let value: f32 = rug_fuzz_0;
        let result: i128 = AsPrimitive::<i128>::as_(value);
        debug_assert_eq!(result, 42i128);
        let _rug_ed_tests_llm_16_270_rrrruuuugggg_f32_as_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_271_llm_16_271 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_f32_to_i16() {
        let _rug_st_tests_llm_16_271_llm_16_271_rrrruuuugggg_test_as_primitive_f32_to_i16 = 0;
        let rug_fuzz_0 = 42.0;
        let value: f32 = rug_fuzz_0;
        let result: i16 = <f32 as AsPrimitive<i16>>::as_(value);
        debug_assert_eq!(result, 42i16);
        let _rug_ed_tests_llm_16_271_llm_16_271_rrrruuuugggg_test_as_primitive_f32_to_i16 = 0;
    }
    #[test]
    fn test_as_primitive_f32_to_i16_overflow() {
        let _rug_st_tests_llm_16_271_llm_16_271_rrrruuuugggg_test_as_primitive_f32_to_i16_overflow = 0;
        let rug_fuzz_0 = 1e20;
        let value: f32 = rug_fuzz_0;
        let result: i16 = <f32 as AsPrimitive<i16>>::as_(value);
        debug_assert!(result != value as i16);
        let _rug_ed_tests_llm_16_271_llm_16_271_rrrruuuugggg_test_as_primitive_f32_to_i16_overflow = 0;
    }
    #[test]
    fn test_as_primitive_f32_to_i16_underflow() {
        let _rug_st_tests_llm_16_271_llm_16_271_rrrruuuugggg_test_as_primitive_f32_to_i16_underflow = 0;
        let rug_fuzz_0 = 1e20;
        let value: f32 = -rug_fuzz_0;
        let result: i16 = <f32 as AsPrimitive<i16>>::as_(value);
        debug_assert!(result != value as i16);
        let _rug_ed_tests_llm_16_271_llm_16_271_rrrruuuugggg_test_as_primitive_f32_to_i16_underflow = 0;
    }
    #[test]
    fn test_as_primitive_f32_to_i16_edge_cases() {
        let _rug_st_tests_llm_16_271_llm_16_271_rrrruuuugggg_test_as_primitive_f32_to_i16_edge_cases = 0;
        let value_close_to_max: f32 = i16::MAX as f32;
        debug_assert_eq!(
            < f32 as AsPrimitive < i16 > > ::as_(value_close_to_max), i16::MAX
        );
        let value_close_to_min: f32 = i16::MIN as f32;
        debug_assert_eq!(
            < f32 as AsPrimitive < i16 > > ::as_(value_close_to_min), i16::MIN
        );
        let _rug_ed_tests_llm_16_271_llm_16_271_rrrruuuugggg_test_as_primitive_f32_to_i16_edge_cases = 0;
    }
    #[test]
    fn test_as_primitive_f32_to_i16_special_values() {
        let _rug_st_tests_llm_16_271_llm_16_271_rrrruuuugggg_test_as_primitive_f32_to_i16_special_values = 0;
        let nan: f32 = f32::NAN;
        let result: i16 = <f32 as AsPrimitive<i16>>::as_(nan);
        let positive_inf: f32 = f32::INFINITY;
        let result: i16 = <f32 as AsPrimitive<i16>>::as_(positive_inf);
        let negative_inf: f32 = f32::NEG_INFINITY;
        let result: i16 = <f32 as AsPrimitive<i16>>::as_(negative_inf);
        let _rug_ed_tests_llm_16_271_llm_16_271_rrrruuuugggg_test_as_primitive_f32_to_i16_special_values = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_272_llm_16_272 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_f32_to_i32() {
        let _rug_st_tests_llm_16_272_llm_16_272_rrrruuuugggg_test_as_primitive_f32_to_i32 = 0;
        let rug_fuzz_0 = 42.5;
        let value_f32: f32 = rug_fuzz_0;
        let value_i32: i32 = AsPrimitive::<i32>::as_(value_f32);
        debug_assert_eq!(value_i32, 42);
        let _rug_ed_tests_llm_16_272_llm_16_272_rrrruuuugggg_test_as_primitive_f32_to_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_273_llm_16_273 {
    use crate::cast::AsPrimitive;
    #[test]
    fn float_to_i64_cast() {
        let _rug_st_tests_llm_16_273_llm_16_273_rrrruuuugggg_float_to_i64_cast = 0;
        let rug_fuzz_0 = 42.0;
        let value: f32 = rug_fuzz_0;
        let result: i64 = <f32 as AsPrimitive<i64>>::as_(value);
        debug_assert_eq!(result, 42_i64);
        let _rug_ed_tests_llm_16_273_llm_16_273_rrrruuuugggg_float_to_i64_cast = 0;
    }
    #[test]
    fn float_to_i64_cast_truncation() {
        let _rug_st_tests_llm_16_273_llm_16_273_rrrruuuugggg_float_to_i64_cast_truncation = 0;
        let rug_fuzz_0 = 42.9;
        let value: f32 = rug_fuzz_0;
        let result: i64 = <f32 as AsPrimitive<i64>>::as_(value);
        debug_assert_eq!(result, 42_i64);
        let _rug_ed_tests_llm_16_273_llm_16_273_rrrruuuugggg_float_to_i64_cast_truncation = 0;
    }
    #[test]
    fn float_to_i64_cast_negative() {
        let _rug_st_tests_llm_16_273_llm_16_273_rrrruuuugggg_float_to_i64_cast_negative = 0;
        let rug_fuzz_0 = 42.0;
        let value: f32 = -rug_fuzz_0;
        let result: i64 = <f32 as AsPrimitive<i64>>::as_(value);
        debug_assert_eq!(result, - 42_i64);
        let _rug_ed_tests_llm_16_273_llm_16_273_rrrruuuugggg_float_to_i64_cast_negative = 0;
    }
    #[test]
    fn float_to_i64_cast_zero() {
        let _rug_st_tests_llm_16_273_llm_16_273_rrrruuuugggg_float_to_i64_cast_zero = 0;
        let rug_fuzz_0 = 0.0;
        let value: f32 = rug_fuzz_0;
        let result: i64 = <f32 as AsPrimitive<i64>>::as_(value);
        debug_assert_eq!(result, 0_i64);
        let _rug_ed_tests_llm_16_273_llm_16_273_rrrruuuugggg_float_to_i64_cast_zero = 0;
    }
    #[test]
    fn float_to_i64_cast_min_value() {
        let _rug_st_tests_llm_16_273_llm_16_273_rrrruuuugggg_float_to_i64_cast_min_value = 0;
        let value: f32 = i64::MIN as f32;
        let result: i64 = <f32 as AsPrimitive<i64>>::as_(value);
        debug_assert_eq!(result, i64::MIN);
        let _rug_ed_tests_llm_16_273_llm_16_273_rrrruuuugggg_float_to_i64_cast_min_value = 0;
    }
    #[test]
    fn float_to_i64_cast_max_value() {
        let _rug_st_tests_llm_16_273_llm_16_273_rrrruuuugggg_float_to_i64_cast_max_value = 0;
        let value: f32 = i64::MAX as f32;
        let result: i64 = <f32 as AsPrimitive<i64>>::as_(value);
        debug_assert!(result <= i64::MAX);
        let _rug_ed_tests_llm_16_273_llm_16_273_rrrruuuugggg_float_to_i64_cast_max_value = 0;
    }
    #[test]
    #[should_panic(expected = "attempt to cast with overflow")]
    fn float_to_i64_cast_overflow() {
        let _rug_st_tests_llm_16_273_llm_16_273_rrrruuuugggg_float_to_i64_cast_overflow = 0;
        let value: f32 = f32::MAX;
        let _result: i64 = <f32 as AsPrimitive<i64>>::as_(value);
        let _rug_ed_tests_llm_16_273_llm_16_273_rrrruuuugggg_float_to_i64_cast_overflow = 0;
    }
    #[test]
    #[should_panic(expected = "attempt to cast with overflow")]
    fn float_to_i64_cast_underflow() {
        let _rug_st_tests_llm_16_273_llm_16_273_rrrruuuugggg_float_to_i64_cast_underflow = 0;
        let value: f32 = f32::MIN;
        let _result: i64 = <f32 as AsPrimitive<i64>>::as_(value);
        let _rug_ed_tests_llm_16_273_llm_16_273_rrrruuuugggg_float_to_i64_cast_underflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_276_llm_16_276 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_f32_to_u128() {
        let _rug_st_tests_llm_16_276_llm_16_276_rrrruuuugggg_test_as_primitive_f32_to_u128 = 0;
        let rug_fuzz_0 = 42.0;
        let value: f32 = rug_fuzz_0;
        let result = AsPrimitive::<u128>::as_(value);
        debug_assert_eq!(result, 42u128);
        let _rug_ed_tests_llm_16_276_llm_16_276_rrrruuuugggg_test_as_primitive_f32_to_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_277_llm_16_277 {
    use super::*;
    use crate::*;
    #[test]
    fn test_f32_as_u16() {
        let _rug_st_tests_llm_16_277_llm_16_277_rrrruuuugggg_test_f32_as_u16 = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 1.0;
        let rug_fuzz_2 = 1.4;
        let rug_fuzz_3 = 1.0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 1;
        let rug_fuzz_7 = 0;
        let f_values: [f32; 4] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, -rug_fuzz_3];
        let u_values: [u16; 4] = [rug_fuzz_4, rug_fuzz_5, rug_fuzz_6, rug_fuzz_7];
        for (f, &u) in f_values.iter().zip(u_values.iter()) {
            let casted: u16 = AsPrimitive::<u16>::as_(*f);
            debug_assert_eq!(casted, u);
        }
        let _rug_ed_tests_llm_16_277_llm_16_277_rrrruuuugggg_test_f32_as_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_278 {
    use super::*;
    use crate::*;
    #[test]
    fn test_as_primitive_f32_to_u32() {
        let _rug_st_tests_llm_16_278_rrrruuuugggg_test_as_primitive_f32_to_u32 = 0;
        let rug_fuzz_0 = 42.0;
        let value: f32 = rug_fuzz_0;
        let result: u32 = AsPrimitive::<u32>::as_(value);
        debug_assert_eq!(result, 42u32);
        let _rug_ed_tests_llm_16_278_rrrruuuugggg_test_as_primitive_f32_to_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_279_llm_16_279 {
    use crate::cast::AsPrimitive;
    #[test]
    fn f32_as_u64() {
        let _rug_st_tests_llm_16_279_llm_16_279_rrrruuuugggg_f32_as_u64 = 0;
        let rug_fuzz_0 = 0.0f32;
        let rug_fuzz_1 = 0u64;
        let rug_fuzz_2 = 1.0f32;
        let rug_fuzz_3 = 1u64;
        let rug_fuzz_4 = 1.5f32;
        let rug_fuzz_5 = 1u64;
        let values: [(f32, u64); 3] = [
            (rug_fuzz_0, rug_fuzz_1),
            (rug_fuzz_2, rug_fuzz_3),
            (rug_fuzz_4, rug_fuzz_5),
        ];
        for &(input, expected) in &values {
            let result: u64 = input.as_();
            debug_assert_eq!(
                result, expected,
                "Casting {} to u64 did not produce the expected result {}", input,
                expected
            );
        }
        let _rug_ed_tests_llm_16_279_llm_16_279_rrrruuuugggg_f32_as_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_280_llm_16_280 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_f32_to_u8_cast() {
        let _rug_st_tests_llm_16_280_llm_16_280_rrrruuuugggg_test_f32_to_u8_cast = 0;
        let rug_fuzz_0 = 42.0;
        let value: f32 = rug_fuzz_0;
        let result: u8 = AsPrimitive::<u8>::as_(value);
        debug_assert_eq!(result, 42u8);
        let _rug_ed_tests_llm_16_280_llm_16_280_rrrruuuugggg_test_f32_to_u8_cast = 0;
    }
    #[test]
    fn test_f32_to_u8_cast_overflow() {
        let _rug_st_tests_llm_16_280_llm_16_280_rrrruuuugggg_test_f32_to_u8_cast_overflow = 0;
        let rug_fuzz_0 = 300.0;
        let value: f32 = rug_fuzz_0;
        let result: u8 = AsPrimitive::<u8>::as_(value);
        debug_assert_eq!(result, 255u8);
        let _rug_ed_tests_llm_16_280_llm_16_280_rrrruuuugggg_test_f32_to_u8_cast_overflow = 0;
    }
    #[test]
    #[should_panic(expected = "attempt to cast to type `u8` which cannot hold `300.0`")]
    fn test_f32_to_u8_cast_overflow_panic() {
        let _rug_st_tests_llm_16_280_llm_16_280_rrrruuuugggg_test_f32_to_u8_cast_overflow_panic = 0;
        let rug_fuzz_0 = 300.0;
        let value: f32 = rug_fuzz_0;
        let _: u8 = AsPrimitive::<u8>::as_(value);
        let _rug_ed_tests_llm_16_280_llm_16_280_rrrruuuugggg_test_f32_to_u8_cast_overflow_panic = 0;
    }
    #[test]
    fn test_f32_to_u8_cast_underflow() {
        let _rug_st_tests_llm_16_280_llm_16_280_rrrruuuugggg_test_f32_to_u8_cast_underflow = 0;
        let rug_fuzz_0 = 5.0;
        let value: f32 = -rug_fuzz_0;
        let result: u8 = AsPrimitive::<u8>::as_(value);
        debug_assert_eq!(result, 0u8);
        let _rug_ed_tests_llm_16_280_llm_16_280_rrrruuuugggg_test_f32_to_u8_cast_underflow = 0;
    }
    #[test]
    #[should_panic(expected = "attempt to cast to type `u8` which cannot hold `-5.0`")]
    fn test_f32_to_u8_cast_underflow_panic() {
        let _rug_st_tests_llm_16_280_llm_16_280_rrrruuuugggg_test_f32_to_u8_cast_underflow_panic = 0;
        let rug_fuzz_0 = 5.0;
        let value: f32 = -rug_fuzz_0;
        let _: u8 = AsPrimitive::<u8>::as_(value);
        let _rug_ed_tests_llm_16_280_llm_16_280_rrrruuuugggg_test_f32_to_u8_cast_underflow_panic = 0;
    }
    #[test]
    fn test_f32_to_u8_cast_fractional() {
        let _rug_st_tests_llm_16_280_llm_16_280_rrrruuuugggg_test_f32_to_u8_cast_fractional = 0;
        let rug_fuzz_0 = 42.99;
        let value: f32 = rug_fuzz_0;
        let result: u8 = AsPrimitive::<u8>::as_(value);
        debug_assert_eq!(result, 42u8);
        let _rug_ed_tests_llm_16_280_llm_16_280_rrrruuuugggg_test_f32_to_u8_cast_fractional = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_281_llm_16_281 {
    use crate::AsPrimitive;
    #[test]
    fn test_f32_to_usize_cast() {
        let _rug_st_tests_llm_16_281_llm_16_281_rrrruuuugggg_test_f32_to_usize_cast = 0;
        let rug_fuzz_0 = 42.0;
        let value_f32: f32 = rug_fuzz_0;
        let value_usize: usize = AsPrimitive::<usize>::as_(value_f32);
        debug_assert_eq!(value_usize, 42usize);
        let _rug_ed_tests_llm_16_281_llm_16_281_rrrruuuugggg_test_f32_to_usize_cast = 0;
    }
    #[test]
    #[should_panic]
    fn test_f32_to_usize_cast_overflow() {
        let _rug_st_tests_llm_16_281_llm_16_281_rrrruuuugggg_test_f32_to_usize_cast_overflow = 0;
        let rug_fuzz_0 = 1.0;
        let value_f32: f32 = std::usize::MAX as f32 + rug_fuzz_0;
        let _value_usize: usize = AsPrimitive::<usize>::as_(value_f32);
        let _rug_ed_tests_llm_16_281_llm_16_281_rrrruuuugggg_test_f32_to_usize_cast_overflow = 0;
    }
    #[test]
    fn test_f32_to_usize_cast_rounding() {
        let _rug_st_tests_llm_16_281_llm_16_281_rrrruuuugggg_test_f32_to_usize_cast_rounding = 0;
        let rug_fuzz_0 = 42.7;
        let value_f32: f32 = rug_fuzz_0;
        let value_usize: usize = AsPrimitive::<usize>::as_(value_f32);
        debug_assert_eq!(value_usize, 42usize);
        let _rug_ed_tests_llm_16_281_llm_16_281_rrrruuuugggg_test_f32_to_usize_cast_rounding = 0;
    }
    #[test]
    fn test_f32_to_usize_cast_negative() {
        let _rug_st_tests_llm_16_281_llm_16_281_rrrruuuugggg_test_f32_to_usize_cast_negative = 0;
        let rug_fuzz_0 = 1.0;
        let value_f32: f32 = -rug_fuzz_0;
        let value_usize: usize = AsPrimitive::<usize>::as_(value_f32);
        debug_assert_eq!(value_usize, 0usize);
        let _rug_ed_tests_llm_16_281_llm_16_281_rrrruuuugggg_test_f32_to_usize_cast_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_282_llm_16_282 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_f32() {
        let _rug_st_tests_llm_16_282_llm_16_282_rrrruuuugggg_test_from_f32 = 0;
        let rug_fuzz_0 = 123.456;
        let rug_fuzz_1 = 123.456_f32;
        let rug_fuzz_2 = 123.456_f32;
        let rug_fuzz_3 = 0.0_f32;
        let normal_f32: f32 = rug_fuzz_0;
        let converted_val: Option<i32> = i32::from_f32(normal_f32);
        debug_assert_eq!(converted_val, Some(normal_f32 as i32));
        let normal_f32: f32 = rug_fuzz_1;
        let converted_val: Option<u32> = u32::from_f32(normal_f32);
        debug_assert_eq!(converted_val, Some(normal_f32 as u32));
        let normal_f32: f32 = -rug_fuzz_2;
        let converted_val: Option<i32> = i32::from_f32(normal_f32);
        debug_assert_eq!(converted_val, Some(normal_f32 as i32));
        let edge_cases: [f32; 3] = [f32::MIN, f32::MAX, rug_fuzz_3];
        for &case in &edge_cases {
            let converted_val: Option<u32> = u32::from_f32(case);
            debug_assert_eq!(converted_val, Some(case as u32));
        }
        let nan = f32::NAN;
        let converted_val: Option<u32> = u32::from_f32(nan);
        debug_assert!(converted_val.is_none());
        let pos_inf = f32::INFINITY;
        let converted_val: Option<u32> = u32::from_f32(pos_inf);
        debug_assert!(converted_val.is_none());
        let neg_inf = f32::NEG_INFINITY;
        let converted_val: Option<u32> = u32::from_f32(neg_inf);
        debug_assert!(converted_val.is_none());
        let _rug_ed_tests_llm_16_282_llm_16_282_rrrruuuugggg_test_from_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_283_llm_16_283 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_f64() {
        let _rug_st_tests_llm_16_283_llm_16_283_rrrruuuugggg_test_from_f64 = 0;
        let rug_fuzz_0 = 123.456f64;
        let delta = f32::EPSILON;
        let num_f64 = rug_fuzz_0;
        let num_f32 = <f32 as FromPrimitive>::from_f64(num_f64);
        debug_assert!(num_f32.is_some());
        debug_assert!((num_f32.unwrap() - num_f64 as f32).abs() < delta);
        let num_f64 = f64::MAX;
        let num_f32 = <f32 as FromPrimitive>::from_f64(num_f64);
        debug_assert!(num_f32.is_none());
        let num_f64 = f64::MIN;
        let num_f32 = <f32 as FromPrimitive>::from_f64(num_f64);
        debug_assert!(num_f32.is_none());
        let num_f64 = f64::NAN;
        let num_f32 = <f32 as FromPrimitive>::from_f64(num_f64);
        debug_assert!(num_f32.unwrap().is_nan());
        let num_f64 = f64::INFINITY;
        let num_f32 = <f32 as FromPrimitive>::from_f64(num_f64);
        debug_assert_eq!(num_f32, Some(f32::INFINITY));
        let num_f64 = f64::NEG_INFINITY;
        let num_f32 = <f32 as FromPrimitive>::from_f64(num_f64);
        debug_assert_eq!(num_f32, Some(f32::NEG_INFINITY));
        let _rug_ed_tests_llm_16_283_llm_16_283_rrrruuuugggg_test_from_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_284_llm_16_284 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i128() {
        let _rug_st_tests_llm_16_284_llm_16_284_rrrruuuugggg_test_from_i128 = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 1.0;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 0.0;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 1.0;
        let values: &[(i128, Option<f32>)] = &[
            (i128::min_value(), None),
            (-rug_fuzz_0, Some(-rug_fuzz_1)),
            (rug_fuzz_2, Some(rug_fuzz_3)),
            (rug_fuzz_4, Some(rug_fuzz_5)),
            (i128::max_value(), None),
        ];
        for &(n, expected) in values.iter() {
            let result = <f32 as FromPrimitive>::from_i128(n);
            debug_assert_eq!(
                result, expected, "from_i128({}) did not return {:?}", n, expected
            );
        }
        let _rug_ed_tests_llm_16_284_llm_16_284_rrrruuuugggg_test_from_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_285_llm_16_285 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i16() {
        let _rug_st_tests_llm_16_285_llm_16_285_rrrruuuugggg_test_from_i16 = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        debug_assert_eq!(< f32 as FromPrimitive > ::from_i16(rug_fuzz_0), Some(0.0f32));
        debug_assert_eq!(< f32 as FromPrimitive > ::from_i16(rug_fuzz_1), Some(1.0f32));
        debug_assert_eq!(
            < f32 as FromPrimitive > ::from_i16(- rug_fuzz_2), Some(- 1.0f32)
        );
        debug_assert_eq!(
            < f32 as FromPrimitive > ::from_i16(i16::MAX), Some(i16::MAX as f32)
        );
        debug_assert_eq!(
            < f32 as FromPrimitive > ::from_i16(i16::MIN), Some(i16::MIN as f32)
        );
        let _rug_ed_tests_llm_16_285_llm_16_285_rrrruuuugggg_test_from_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_286_llm_16_286 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i32() {
        let _rug_st_tests_llm_16_286_llm_16_286_rrrruuuugggg_test_from_i32 = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = 42;
        debug_assert_eq!(< f32 as FromPrimitive > ::from_i32(rug_fuzz_0), Some(0.0f32));
        debug_assert_eq!(< f32 as FromPrimitive > ::from_i32(rug_fuzz_1), Some(42.0f32));
        debug_assert_eq!(
            < f32 as FromPrimitive > ::from_i32(- rug_fuzz_2), Some(- 42.0f32)
        );
        debug_assert_eq!(
            < f32 as FromPrimitive > ::from_i32(i32::MAX), Some(i32::MAX as f32)
        );
        debug_assert_eq!(
            < f32 as FromPrimitive > ::from_i32(i32::MIN), Some(i32::MIN as f32)
        );
        let _rug_ed_tests_llm_16_286_llm_16_286_rrrruuuugggg_test_from_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_287_llm_16_287 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i64() {
        let _rug_st_tests_llm_16_287_llm_16_287_rrrruuuugggg_test_from_i64 = 0;
        let rug_fuzz_0 = 0;
        let values: Vec<i64> = vec![rug_fuzz_0, 1, - 1, i64::MAX, i64::MIN];
        for &n in &values {
            let result: Option<f32> = <f32 as FromPrimitive>::from_i64(n);
            let expected: Option<f32> = Some(n as f32);
            debug_assert_eq!(result, expected, "Testing from_i64 with value: {}", n);
        }
        let _rug_ed_tests_llm_16_287_llm_16_287_rrrruuuugggg_test_from_i64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_288_llm_16_288 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i8() {
        assert_eq!(< f32 as FromPrimitive >::from_i8(0i8), Some(0f32));
        assert_eq!(< f32 as FromPrimitive >::from_i8(- 1i8), Some(- 1f32));
        assert_eq!(< f32 as FromPrimitive >::from_i8(127i8), Some(127f32));
        assert_eq!(< f32 as FromPrimitive >::from_i8(- 128i8), Some(- 128f32));
    }
}
#[cfg(test)]
mod tests_llm_16_289_llm_16_289 {
    use crate::cast::FromPrimitive;
    #[test]
    fn from_isize() {
        let _rug_st_tests_llm_16_289_llm_16_289_rrrruuuugggg_from_isize = 0;
        let rug_fuzz_0 = 100;
        let rug_fuzz_1 = 100;
        let a: isize = rug_fuzz_0;
        let b: isize = -rug_fuzz_1;
        let c: isize = isize::MAX;
        let d: isize = isize::MIN;
        let a_f32: Option<f32> = <f32 as FromPrimitive>::from_isize(a);
        let b_f32: Option<f32> = <f32 as FromPrimitive>::from_isize(b);
        let c_f32: Option<f32> = <f32 as FromPrimitive>::from_isize(c);
        let d_f32: Option<f32> = <f32 as FromPrimitive>::from_isize(d);
        debug_assert_eq!(a_f32, Some(100.0));
        debug_assert_eq!(b_f32, Some(- 100.0));
        debug_assert!(c_f32.is_some());
        debug_assert!(d_f32.is_some());
        let _rug_ed_tests_llm_16_289_llm_16_289_rrrruuuugggg_from_isize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_290_llm_16_290 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u128_with_f32() {
        let _rug_st_tests_llm_16_290_llm_16_290_rrrruuuugggg_test_from_u128_with_f32 = 0;
        let rug_fuzz_0 = 0_u128;
        let rug_fuzz_1 = 42_u128;
        debug_assert_eq!(
            < f32 as FromPrimitive > ::from_u128(rug_fuzz_0), Some(0.0_f32)
        );
        debug_assert_eq!(
            < f32 as FromPrimitive > ::from_u128(rug_fuzz_1), Some(42.0_f32)
        );
        debug_assert_eq!(< f32 as FromPrimitive > ::from_u128(u128::MAX), None);
        let _rug_ed_tests_llm_16_290_llm_16_290_rrrruuuugggg_test_from_u128_with_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_291_llm_16_291 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u16() {
        let _rug_st_tests_llm_16_291_llm_16_291_rrrruuuugggg_test_from_u16 = 0;
        let rug_fuzz_0 = 0;
        let values: Vec<u16> = vec![rug_fuzz_0, 1, u16::MAX];
        for &val in &values {
            let float_val: Option<f32> = <f32 as FromPrimitive>::from_u16(val);
            debug_assert_eq!(float_val, Some(val as f32));
        }
        let big_value: u16 = u16::MAX;
        let float_val: Option<f32> = <f32 as FromPrimitive>::from_u16(big_value);
        debug_assert!(float_val.is_some());
        debug_assert_eq!(float_val, Some(big_value as f32));
        let _rug_ed_tests_llm_16_291_llm_16_291_rrrruuuugggg_test_from_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_292_llm_16_292 {
    use crate::cast::FromPrimitive;
    #[test]
    fn from_u32_test() {
        let _rug_st_tests_llm_16_292_llm_16_292_rrrruuuugggg_from_u32_test = 0;
        let rug_fuzz_0 = 42_u32;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1_u32;
        let rug_fuzz_3 = 24;
        debug_assert_eq!(
            < f32 as FromPrimitive > ::from_u32(rug_fuzz_0), Some(42.0_f32)
        );
        let max_f32_from_u32 = f32::from_bits(u32::MAX);
        debug_assert_eq!(
            < f32 as FromPrimitive > ::from_u32(u32::MAX), Some(max_f32_from_u32)
        );
        debug_assert_eq!(< f32 as FromPrimitive > ::from_u32(rug_fuzz_1), Some(0.0_f32));
        let large_u32 = rug_fuzz_2 << rug_fuzz_3;
        let result = <f32 as FromPrimitive>::from_u32(large_u32);
        debug_assert!(result.is_some());
        debug_assert_eq!(result.unwrap() as u32, large_u32);
        let _rug_ed_tests_llm_16_292_llm_16_292_rrrruuuugggg_from_u32_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_293 {
    use crate::FromPrimitive;
    #[test]
    fn test_from_u64() {
        let _rug_st_tests_llm_16_293_rrrruuuugggg_test_from_u64 = 0;
        let rug_fuzz_0 = 0;
        let values: Vec<u64> = vec![rug_fuzz_0, 42, 1234567890, u64::MAX];
        for &n in &values {
            let result = <f32 as FromPrimitive>::from_u64(n);
            match result {
                Some(value) => {
                    let expected = n as f32;
                    debug_assert!(
                        (value - expected).abs() < f32::EPSILON,
                        "from_u64: {} resulted in {}, expected {}", n, value, expected
                    );
                }
                None => {
                    if (n as f32) as u64 != n {} else {
                        panic!(
                            "from_u64: returned None for a value that should not fail: {}",
                            n
                        );
                    }
                }
            }
        }
        let _rug_ed_tests_llm_16_293_rrrruuuugggg_test_from_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_294_llm_16_294 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u8() {
        let _rug_st_tests_llm_16_294_llm_16_294_rrrruuuugggg_test_from_u8 = 0;
        let rug_fuzz_0 = 0u8;
        let rug_fuzz_1 = 255u8;
        debug_assert_eq!(< f32 as FromPrimitive > ::from_u8(rug_fuzz_0), Some(0f32));
        debug_assert_eq!(< f32 as FromPrimitive > ::from_u8(rug_fuzz_1), Some(255f32));
        let _rug_ed_tests_llm_16_294_llm_16_294_rrrruuuugggg_test_from_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_295_llm_16_295 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_usize_to_f32() {
        let _rug_st_tests_llm_16_295_llm_16_295_rrrruuuugggg_test_from_usize_to_f32 = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 16_777_215usize;
        debug_assert_eq!(
            < f32 as FromPrimitive > ::from_usize(rug_fuzz_0), Some(0.0_f32)
        );
        debug_assert_eq!(
            < f32 as FromPrimitive > ::from_usize(rug_fuzz_1), Some(1.0_f32)
        );
        let max_usize_representable_in_f32 = rug_fuzz_2;
        debug_assert_eq!(
            < f32 as FromPrimitive > ::from_usize(max_usize_representable_in_f32),
            Some(max_usize_representable_in_f32 as f32)
        );
        let _rug_ed_tests_llm_16_295_llm_16_295_rrrruuuugggg_test_from_usize_to_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_296_llm_16_296 {
    use crate::cast::NumCast;
    use crate::ToPrimitive;
    use std::num::Wrapping;
    #[test]
    fn test_cast_from_wrapping() {
        let _rug_st_tests_llm_16_296_llm_16_296_rrrruuuugggg_test_cast_from_wrapping = 0;
        let rug_fuzz_0 = 1i32;
        let rug_fuzz_1 = 1.0f32;
        let rug_fuzz_2 = 1.0f64;
        let rug_fuzz_3 = 1.0;
        let source_value = Wrapping(rug_fuzz_0);
        let result: Option<f32> = <f32 as NumCast>::from(source_value);
        debug_assert_eq!(result, Some(1.0f32));
        let source_value = Wrapping(i32::MAX);
        let result: Option<f32> = <f32 as NumCast>::from(source_value);
        debug_assert_eq!(result, Some(i32::MAX as f32));
        let source_value = Wrapping(i32::MIN);
        let result: Option<f32> = <f32 as NumCast>::from(source_value);
        debug_assert_eq!(result, Some(i32::MIN as f32));
        let source_value = Wrapping(rug_fuzz_1);
        let result: Option<f32> = <f32 as NumCast>::from(source_value);
        debug_assert_eq!(result, Some(1.0f32));
        let source_value = Wrapping(rug_fuzz_2);
        let result: Option<f32> = <f32 as NumCast>::from(source_value);
        debug_assert!(result.unwrap().abs() - rug_fuzz_3 < std::f32::EPSILON);
        let source_value = Wrapping(i64::MAX);
        let result: Option<f32> = <f32 as NumCast>::from(source_value);
        debug_assert!(result.unwrap().is_infinite());
        let source_value = Wrapping(i64::MIN);
        let result: Option<f32> = <f32 as NumCast>::from(source_value);
        debug_assert!(result.unwrap().is_infinite());
        let _rug_ed_tests_llm_16_296_llm_16_296_rrrruuuugggg_test_cast_from_wrapping = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_297_llm_16_297 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_f32_with_f32() {
        let _rug_st_tests_llm_16_297_llm_16_297_rrrruuuugggg_test_to_f32_with_f32 = 0;
        let rug_fuzz_0 = 123.456;
        let value: f32 = rug_fuzz_0;
        debug_assert_eq!(< f32 as ToPrimitive > ::to_f32(& value), Some(value));
        let _rug_ed_tests_llm_16_297_llm_16_297_rrrruuuugggg_test_to_f32_with_f32 = 0;
    }
    #[test]
    fn test_to_f32_with_f64() {
        let _rug_st_tests_llm_16_297_llm_16_297_rrrruuuugggg_test_to_f32_with_f64 = 0;
        let rug_fuzz_0 = 123.456;
        let value: f64 = rug_fuzz_0;
        let expected = value as f32;
        debug_assert_eq!(< f64 as ToPrimitive > ::to_f32(& value), Some(expected));
        let _rug_ed_tests_llm_16_297_llm_16_297_rrrruuuugggg_test_to_f32_with_f64 = 0;
    }
    #[test]
    fn test_to_f32_with_nan() {
        let _rug_st_tests_llm_16_297_llm_16_297_rrrruuuugggg_test_to_f32_with_nan = 0;
        let value: f64 = f64::NAN;
        debug_assert!(< f64 as ToPrimitive > ::to_f32(& value).unwrap().is_nan());
        let _rug_ed_tests_llm_16_297_llm_16_297_rrrruuuugggg_test_to_f32_with_nan = 0;
    }
    #[test]
    fn test_to_f32_with_infinity() {
        let _rug_st_tests_llm_16_297_llm_16_297_rrrruuuugggg_test_to_f32_with_infinity = 0;
        let value: f64 = f64::INFINITY;
        debug_assert_eq!(< f64 as ToPrimitive > ::to_f32(& value), Some(f32::INFINITY));
        let _rug_ed_tests_llm_16_297_llm_16_297_rrrruuuugggg_test_to_f32_with_infinity = 0;
    }
    #[test]
    fn test_to_f32_with_negative_infinity() {
        let _rug_st_tests_llm_16_297_llm_16_297_rrrruuuugggg_test_to_f32_with_negative_infinity = 0;
        let value: f64 = f64::NEG_INFINITY;
        debug_assert_eq!(
            < f64 as ToPrimitive > ::to_f32(& value), Some(f32::NEG_INFINITY)
        );
        let _rug_ed_tests_llm_16_297_llm_16_297_rrrruuuugggg_test_to_f32_with_negative_infinity = 0;
    }
    #[test]
    fn test_to_f32_with_max_value() {
        let _rug_st_tests_llm_16_297_llm_16_297_rrrruuuugggg_test_to_f32_with_max_value = 0;
        let value: f64 = f64::MAX;
        debug_assert_eq!(< f64 as ToPrimitive > ::to_f32(& value), Some(f32::INFINITY));
        let _rug_ed_tests_llm_16_297_llm_16_297_rrrruuuugggg_test_to_f32_with_max_value = 0;
    }
    #[test]
    fn test_to_f32_with_min_value() {
        let _rug_st_tests_llm_16_297_llm_16_297_rrrruuuugggg_test_to_f32_with_min_value = 0;
        let value: f64 = f64::MIN;
        debug_assert_eq!(
            < f64 as ToPrimitive > ::to_f32(& value), Some(f32::NEG_INFINITY)
        );
        let _rug_ed_tests_llm_16_297_llm_16_297_rrrruuuugggg_test_to_f32_with_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_298_llm_16_298 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_f32_to_f64() {
        let _rug_st_tests_llm_16_298_llm_16_298_rrrruuuugggg_test_f32_to_f64 = 0;
        let rug_fuzz_0 = 1234.5678;
        let rug_fuzz_1 = 0.0;
        let rug_fuzz_2 = 0.0;
        let finite_val: f32 = rug_fuzz_0;
        let finite_result = ToPrimitive::to_f64(&finite_val);
        debug_assert_eq!(finite_result, Some(finite_val as f64));
        let nan_val: f32 = f32::NAN;
        let nan_result = ToPrimitive::to_f64(&nan_val);
        debug_assert!(nan_result.unwrap().is_nan());
        let inf_val: f32 = f32::INFINITY;
        let inf_result = ToPrimitive::to_f64(&inf_val);
        debug_assert_eq!(inf_result, Some(f64::INFINITY));
        let neg_inf_val: f32 = f32::NEG_INFINITY;
        let neg_inf_result = ToPrimitive::to_f64(&neg_inf_val);
        debug_assert_eq!(neg_inf_result, Some(f64::NEG_INFINITY));
        let zero_val: f32 = rug_fuzz_1;
        let zero_result = ToPrimitive::to_f64(&zero_val);
        debug_assert_eq!(zero_result, Some(0.0f64));
        let neg_zero_val: f32 = -rug_fuzz_2;
        let neg_zero_result = ToPrimitive::to_f64(&neg_zero_val);
        debug_assert_eq!(neg_zero_result, Some(- 0.0f64));
        let _rug_ed_tests_llm_16_298_llm_16_298_rrrruuuugggg_test_f32_to_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_299 {
    use super::*;
    use crate::*;
    use crate::ToPrimitive;
    #[test]
    fn test_to_i128_within_bounds() {
        let _rug_st_tests_llm_16_299_rrrruuuugggg_test_to_i128_within_bounds = 0;
        let rug_fuzz_0 = 42.0;
        let rug_fuzz_1 = 42.0;
        let f: f32 = rug_fuzz_0;
        debug_assert_eq!(f.to_i128(), Some(42_i128));
        let f: f32 = -rug_fuzz_1;
        debug_assert_eq!(f.to_i128(), Some(- 42_i128));
        let _rug_ed_tests_llm_16_299_rrrruuuugggg_test_to_i128_within_bounds = 0;
    }
    #[test]
    fn test_to_i128_out_of_bounds() {
        let _rug_st_tests_llm_16_299_rrrruuuugggg_test_to_i128_out_of_bounds = 0;
        let rug_fuzz_0 = 1.0;
        let rug_fuzz_1 = 1.0;
        let f: f32 = i128::MAX as f32 + rug_fuzz_0;
        debug_assert_eq!(f.to_i128(), None);
        let f: f32 = i128::MIN as f32 - rug_fuzz_1;
        debug_assert_eq!(f.to_i128(), None);
        let _rug_ed_tests_llm_16_299_rrrruuuugggg_test_to_i128_out_of_bounds = 0;
    }
    #[test]
    fn test_to_i128_edge_cases() {
        let _rug_st_tests_llm_16_299_rrrruuuugggg_test_to_i128_edge_cases = 0;
        let f: f32 = i128::MAX as f32;
        debug_assert_eq!(f.to_i128(), Some(i128::MAX));
        let f: f32 = i128::MIN as f32;
        debug_assert_eq!(f.to_i128(), Some(i128::MIN));
        let _rug_ed_tests_llm_16_299_rrrruuuugggg_test_to_i128_edge_cases = 0;
    }
    #[test]
    fn test_to_i128_precision_loss() {
        let _rug_st_tests_llm_16_299_rrrruuuugggg_test_to_i128_precision_loss = 0;
        let rug_fuzz_0 = 1e20;
        let rug_fuzz_1 = 1e20;
        let f: f32 = rug_fuzz_0;
        debug_assert_eq!(f.to_i128(), None);
        let f: f32 = -rug_fuzz_1;
        debug_assert_eq!(f.to_i128(), None);
        let _rug_ed_tests_llm_16_299_rrrruuuugggg_test_to_i128_precision_loss = 0;
    }
    #[test]
    fn test_to_i128_exact_integers() {
        let _rug_st_tests_llm_16_299_rrrruuuugggg_test_to_i128_exact_integers = 0;
        let rug_fuzz_0 = 1.0;
        let rug_fuzz_1 = 1.0;
        let f: f32 = rug_fuzz_0;
        debug_assert_eq!(f.to_i128(), Some(1_i128));
        let f: f32 = -rug_fuzz_1;
        debug_assert_eq!(f.to_i128(), Some(- 1_i128));
        let _rug_ed_tests_llm_16_299_rrrruuuugggg_test_to_i128_exact_integers = 0;
    }
    #[test]
    fn test_to_i128_with_fractions() {
        let _rug_st_tests_llm_16_299_rrrruuuugggg_test_to_i128_with_fractions = 0;
        let rug_fuzz_0 = 42.1;
        let rug_fuzz_1 = 42.1;
        let f: f32 = rug_fuzz_0;
        debug_assert_eq!(f.to_i128(), Some(42_i128));
        let f: f32 = -rug_fuzz_1;
        debug_assert_eq!(f.to_i128(), Some(- 42_i128));
        let _rug_ed_tests_llm_16_299_rrrruuuugggg_test_to_i128_with_fractions = 0;
    }
    #[test]
    fn test_to_i128_zero() {
        let _rug_st_tests_llm_16_299_rrrruuuugggg_test_to_i128_zero = 0;
        let rug_fuzz_0 = 0.0;
        let f: f32 = rug_fuzz_0;
        debug_assert_eq!(f.to_i128(), Some(0_i128));
        let _rug_ed_tests_llm_16_299_rrrruuuugggg_test_to_i128_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_301_llm_16_301 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_f32_to_i32() {
        let _rug_st_tests_llm_16_301_llm_16_301_rrrruuuugggg_test_f32_to_i32 = 0;
        let rug_fuzz_0 = 0.0f32;
        let rug_fuzz_1 = 1.0f32;
        let rug_fuzz_2 = 1.0f32;
        let rug_fuzz_3 = 1.5f32;
        let rug_fuzz_4 = 1.5f32;
        let rug_fuzz_5 = 1.999999f32;
        let rug_fuzz_6 = 1.999999f32;
        let rug_fuzz_7 = 1.0;
        let rug_fuzz_8 = 1.0;
        let rug_fuzz_9 = 1.1;
        let rug_fuzz_10 = 0.999999;
        let rug_fuzz_11 = 0.999999;
        debug_assert_eq!(rug_fuzz_0.to_i32(), Some(0));
        debug_assert_eq!((- rug_fuzz_1).to_i32(), Some(- 1));
        debug_assert_eq!(rug_fuzz_2.to_i32(), Some(1));
        debug_assert_eq!(rug_fuzz_3.to_i32(), Some(1));
        debug_assert_eq!((- rug_fuzz_4).to_i32(), Some(- 1));
        debug_assert_eq!(rug_fuzz_5.to_i32(), Some(1));
        debug_assert_eq!((- rug_fuzz_6).to_i32(), Some(- 1));
        debug_assert_eq!(f32::MAX.to_i32(), None);
        debug_assert_eq!(f32::MIN.to_i32(), None);
        debug_assert_eq!((i32::MAX as f32).to_i32(), Some(i32::MAX));
        debug_assert_eq!(((i32::MAX as f32) + rug_fuzz_7).to_i32(), None);
        debug_assert_eq!(((i32::MIN as f32) - rug_fuzz_8).to_i32(), None);
        debug_assert_eq!(((i32::MIN as f32) - rug_fuzz_9).to_i32(), None);
        debug_assert_eq!(((i32::MAX as f32) + rug_fuzz_10).to_i32(), Some(i32::MAX));
        debug_assert_eq!(((i32::MIN as f32) - rug_fuzz_11).to_i32(), Some(i32::MIN));
        debug_assert_eq!(f32::NAN.to_i32(), None);
        debug_assert_eq!(f32::INFINITY.to_i32(), None);
        debug_assert_eq!(f32::NEG_INFINITY.to_i32(), None);
        let _rug_ed_tests_llm_16_301_llm_16_301_rrrruuuugggg_test_f32_to_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_302_llm_16_302 {
    use crate::cast::ToPrimitive;
    #[test]
    fn to_i64_within_range() {
        let _rug_st_tests_llm_16_302_llm_16_302_rrrruuuugggg_to_i64_within_range = 0;
        let rug_fuzz_0 = 42.0;
        let f: f32 = rug_fuzz_0;
        debug_assert_eq!(f.to_i64(), Some(42));
        let _rug_ed_tests_llm_16_302_llm_16_302_rrrruuuugggg_to_i64_within_range = 0;
    }
    #[test]
    fn to_i64_below_range() {
        let _rug_st_tests_llm_16_302_llm_16_302_rrrruuuugggg_to_i64_below_range = 0;
        let rug_fuzz_0 = 2.0;
        let f: f32 = (i64::MIN as f32) - rug_fuzz_0;
        debug_assert_eq!(f.to_i64(), None);
        let _rug_ed_tests_llm_16_302_llm_16_302_rrrruuuugggg_to_i64_below_range = 0;
    }
    #[test]
    fn to_i64_above_range() {
        let _rug_st_tests_llm_16_302_llm_16_302_rrrruuuugggg_to_i64_above_range = 0;
        let rug_fuzz_0 = 2.0;
        let f: f32 = (i64::MAX as f32) + rug_fuzz_0;
        debug_assert_eq!(f.to_i64(), None);
        let _rug_ed_tests_llm_16_302_llm_16_302_rrrruuuugggg_to_i64_above_range = 0;
    }
    #[test]
    fn to_i64_just_below_range() {
        let _rug_st_tests_llm_16_302_llm_16_302_rrrruuuugggg_to_i64_just_below_range = 0;
        let rug_fuzz_0 = 0.1;
        let f: f32 = (i64::MIN as f32) - rug_fuzz_0;
        debug_assert_eq!(f.to_i64(), None);
        let _rug_ed_tests_llm_16_302_llm_16_302_rrrruuuugggg_to_i64_just_below_range = 0;
    }
    #[test]
    fn to_i64_just_above_range() {
        let _rug_st_tests_llm_16_302_llm_16_302_rrrruuuugggg_to_i64_just_above_range = 0;
        let rug_fuzz_0 = 0.1;
        let f: f32 = (i64::MAX as f32) + rug_fuzz_0;
        debug_assert_eq!(f.to_i64(), Some(i64::MAX));
        let _rug_ed_tests_llm_16_302_llm_16_302_rrrruuuugggg_to_i64_just_above_range = 0;
    }
    #[test]
    fn to_i64_min_value() {
        let _rug_st_tests_llm_16_302_llm_16_302_rrrruuuugggg_to_i64_min_value = 0;
        let f: f32 = i64::MIN as f32;
        debug_assert_eq!(f.to_i64(), Some(i64::MIN));
        let _rug_ed_tests_llm_16_302_llm_16_302_rrrruuuugggg_to_i64_min_value = 0;
    }
    #[test]
    fn to_i64_max_value() {
        let _rug_st_tests_llm_16_302_llm_16_302_rrrruuuugggg_to_i64_max_value = 0;
        let f: f32 = i64::MAX as f32;
        debug_assert_eq!(f.to_i64(), Some(i64::MAX));
        let _rug_ed_tests_llm_16_302_llm_16_302_rrrruuuugggg_to_i64_max_value = 0;
    }
    #[test]
    fn to_i64_nan() {
        let _rug_st_tests_llm_16_302_llm_16_302_rrrruuuugggg_to_i64_nan = 0;
        let f: f32 = f32::NAN;
        debug_assert_eq!(f.to_i64(), None);
        let _rug_ed_tests_llm_16_302_llm_16_302_rrrruuuugggg_to_i64_nan = 0;
    }
    #[test]
    fn to_i64_infinity() {
        let _rug_st_tests_llm_16_302_llm_16_302_rrrruuuugggg_to_i64_infinity = 0;
        let f: f32 = f32::INFINITY;
        debug_assert_eq!(f.to_i64(), None);
        let _rug_ed_tests_llm_16_302_llm_16_302_rrrruuuugggg_to_i64_infinity = 0;
    }
    #[test]
    fn to_i64_neg_infinity() {
        let _rug_st_tests_llm_16_302_llm_16_302_rrrruuuugggg_to_i64_neg_infinity = 0;
        let f: f32 = f32::NEG_INFINITY;
        debug_assert_eq!(f.to_i64(), None);
        let _rug_ed_tests_llm_16_302_llm_16_302_rrrruuuugggg_to_i64_neg_infinity = 0;
    }
    #[test]
    fn to_i64_zero() {
        let _rug_st_tests_llm_16_302_llm_16_302_rrrruuuugggg_to_i64_zero = 0;
        let rug_fuzz_0 = 0.0;
        let f: f32 = rug_fuzz_0;
        debug_assert_eq!(f.to_i64(), Some(0));
        let _rug_ed_tests_llm_16_302_llm_16_302_rrrruuuugggg_to_i64_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_303_llm_16_303 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_i8_with_f32() {
        let _rug_st_tests_llm_16_303_llm_16_303_rrrruuuugggg_test_to_i8_with_f32 = 0;
        let rug_fuzz_0 = 1.0f32;
        let rug_fuzz_1 = 1.0f32;
        let rug_fuzz_2 = 0.0f32;
        let rug_fuzz_3 = 1.99f32;
        let rug_fuzz_4 = 1.99f32;
        let rug_fuzz_5 = 127.0f32;
        let rug_fuzz_6 = 127.999f32;
        let rug_fuzz_7 = 128.0f32;
        let rug_fuzz_8 = 128.999f32;
        let rug_fuzz_9 = 128.0f32;
        let rug_fuzz_10 = 129.0f32;
        let rug_fuzz_11 = 1e20f32;
        let rug_fuzz_12 = 1e20f32;
        debug_assert_eq!(rug_fuzz_0.to_i8(), Some(1i8));
        debug_assert_eq!((- rug_fuzz_1).to_i8(), Some(- 1i8));
        debug_assert_eq!(rug_fuzz_2.to_i8(), Some(0i8));
        debug_assert_eq!(rug_fuzz_3.to_i8(), Some(1i8));
        debug_assert_eq!((- rug_fuzz_4).to_i8(), Some(- 1i8));
        debug_assert_eq!((rug_fuzz_5).to_i8(), Some(127i8));
        debug_assert_eq!((rug_fuzz_6).to_i8(), Some(127i8));
        debug_assert_eq!((- rug_fuzz_7).to_i8(), Some(- 128i8));
        debug_assert_eq!((- rug_fuzz_8).to_i8(), Some(- 128i8));
        debug_assert_eq!(rug_fuzz_9.to_i8(), None);
        debug_assert_eq!((- rug_fuzz_10).to_i8(), None);
        debug_assert_eq!(rug_fuzz_11.to_i8(), None);
        debug_assert_eq!((- rug_fuzz_12).to_i8(), None);
        debug_assert_eq!(f32::INFINITY.to_i8(), None);
        debug_assert_eq!(f32::NEG_INFINITY.to_i8(), None);
        debug_assert_eq!(f32::NAN.to_i8(), None);
        let _rug_ed_tests_llm_16_303_llm_16_303_rrrruuuugggg_test_to_i8_with_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_304_llm_16_304 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_isize_within_bounds() {
        let _rug_st_tests_llm_16_304_llm_16_304_rrrruuuugggg_test_to_isize_within_bounds = 0;
        let rug_fuzz_0 = 123.0f32;
        let rug_fuzz_1 = 123.0f32;
        let f = rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_isize(& f), Some(123));
        let f = -rug_fuzz_1;
        debug_assert_eq!(ToPrimitive::to_isize(& f), Some(- 123));
        let _rug_ed_tests_llm_16_304_llm_16_304_rrrruuuugggg_test_to_isize_within_bounds = 0;
    }
    #[test]
    fn test_to_isize_outside_bounds() {
        let _rug_st_tests_llm_16_304_llm_16_304_rrrruuuugggg_test_to_isize_outside_bounds = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 2.0;
        let f = (isize::MIN as f32) - rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_isize(& f), None);
        let f = (isize::MAX as f32) + rug_fuzz_1;
        debug_assert_eq!(ToPrimitive::to_isize(& f), None);
        let _rug_ed_tests_llm_16_304_llm_16_304_rrrruuuugggg_test_to_isize_outside_bounds = 0;
    }
    #[test]
    fn test_to_isize_at_edge() {
        let _rug_st_tests_llm_16_304_llm_16_304_rrrruuuugggg_test_to_isize_at_edge = 0;
        let rug_fuzz_0 = 0.5;
        let rug_fuzz_1 = 0.5;
        let f = (isize::MIN as f32) - rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_isize(& f), None);
        let f = (isize::MAX as f32) + rug_fuzz_1;
        debug_assert_eq!(ToPrimitive::to_isize(& f), None);
        let _rug_ed_tests_llm_16_304_llm_16_304_rrrruuuugggg_test_to_isize_at_edge = 0;
    }
    #[test]
    fn test_to_isize_exact_min_max() {
        let _rug_st_tests_llm_16_304_llm_16_304_rrrruuuugggg_test_to_isize_exact_min_max = 0;
        let f = isize::MIN as f32;
        debug_assert_eq!(ToPrimitive::to_isize(& f), Some(isize::MIN));
        let f = isize::MAX as f32;
        debug_assert_eq!(ToPrimitive::to_isize(& f), None);
        let _rug_ed_tests_llm_16_304_llm_16_304_rrrruuuugggg_test_to_isize_exact_min_max = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_305_llm_16_305 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_u128_with_f32() {
        let _rug_st_tests_llm_16_305_llm_16_305_rrrruuuugggg_test_to_u128_with_f32 = 0;
        let rug_fuzz_0 = 0f32;
        let rug_fuzz_1 = 0.99f32;
        let rug_fuzz_2 = 1f32;
        let rug_fuzz_3 = 1.99f32;
        let rug_fuzz_4 = 16777216.0;
        let rug_fuzz_5 = 1f32;
        let rug_fuzz_6 = 1f32;
        debug_assert_eq!((rug_fuzz_0).to_u128(), Some(0u128));
        debug_assert_eq!((rug_fuzz_1).to_u128(), Some(0u128));
        debug_assert_eq!((rug_fuzz_2).to_u128(), Some(1u128));
        debug_assert_eq!((rug_fuzz_3).to_u128(), Some(1u128));
        let large_exact_f32: f32 = rug_fuzz_4;
        debug_assert_eq!((large_exact_f32).to_u128(), Some(16777216u128));
        debug_assert_eq!((u128::MAX as f32).to_u128(), Some(u128::MAX));
        debug_assert_eq!((- rug_fuzz_5).to_u128(), None);
        debug_assert_eq!((u128::MAX as f32 + rug_fuzz_6).to_u128(), None);
        debug_assert_eq!((f32::INFINITY).to_u128(), None);
        debug_assert_eq!((f32::NEG_INFINITY).to_u128(), None);
        debug_assert_eq!((f32::NAN).to_u128(), None);
        let _rug_ed_tests_llm_16_305_llm_16_305_rrrruuuugggg_test_to_u128_with_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_306_llm_16_306 {
    use crate::cast::ToPrimitive;
    #[test]
    fn to_u16_with_positive_value() {
        let _rug_st_tests_llm_16_306_llm_16_306_rrrruuuugggg_to_u16_with_positive_value = 0;
        let rug_fuzz_0 = 42.0f32;
        let value = rug_fuzz_0;
        let result = value.to_u16();
        debug_assert_eq!(result, Some(42u16));
        let _rug_ed_tests_llm_16_306_llm_16_306_rrrruuuugggg_to_u16_with_positive_value = 0;
    }
    #[test]
    fn to_u16_with_negative_value() {
        let _rug_st_tests_llm_16_306_llm_16_306_rrrruuuugggg_to_u16_with_negative_value = 0;
        let rug_fuzz_0 = 42.0f32;
        let value = -rug_fuzz_0;
        let result = value.to_u16();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_306_llm_16_306_rrrruuuugggg_to_u16_with_negative_value = 0;
    }
    #[test]
    fn to_u16_with_value_out_of_range() {
        let _rug_st_tests_llm_16_306_llm_16_306_rrrruuuugggg_to_u16_with_value_out_of_range = 0;
        let rug_fuzz_0 = 10.0;
        let value = u16::MAX as f32 + rug_fuzz_0;
        let result = value.to_u16();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_306_llm_16_306_rrrruuuugggg_to_u16_with_value_out_of_range = 0;
    }
    #[test]
    fn to_u16_with_value_just_within_range() {
        let _rug_st_tests_llm_16_306_llm_16_306_rrrruuuugggg_to_u16_with_value_just_within_range = 0;
        let value = u16::MAX as f32;
        let result = value.to_u16();
        debug_assert_eq!(result, Some(u16::MAX));
        let _rug_ed_tests_llm_16_306_llm_16_306_rrrruuuugggg_to_u16_with_value_just_within_range = 0;
    }
    #[test]
    fn to_u16_with_value_just_out_of_range() {
        let _rug_st_tests_llm_16_306_llm_16_306_rrrruuuugggg_to_u16_with_value_just_out_of_range = 0;
        let rug_fuzz_0 = 1.0;
        let value = u16::MAX as f32 + rug_fuzz_0;
        let result = value.to_u16();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_306_llm_16_306_rrrruuuugggg_to_u16_with_value_just_out_of_range = 0;
    }
    #[test]
    fn to_u16_with_value_just_below_zero() {
        let _rug_st_tests_llm_16_306_llm_16_306_rrrruuuugggg_to_u16_with_value_just_below_zero = 0;
        let rug_fuzz_0 = 0.9999999f32;
        let value = -rug_fuzz_0;
        let result = value.to_u16();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_306_llm_16_306_rrrruuuugggg_to_u16_with_value_just_below_zero = 0;
    }
    #[test]
    fn to_u16_with_zero() {
        let _rug_st_tests_llm_16_306_llm_16_306_rrrruuuugggg_to_u16_with_zero = 0;
        let rug_fuzz_0 = 0.0f32;
        let value = rug_fuzz_0;
        let result = value.to_u16();
        debug_assert_eq!(result, Some(0u16));
        let _rug_ed_tests_llm_16_306_llm_16_306_rrrruuuugggg_to_u16_with_zero = 0;
    }
    #[test]
    fn to_u16_with_value_very_close_to_zero_from_negative() {
        let _rug_st_tests_llm_16_306_llm_16_306_rrrruuuugggg_to_u16_with_value_very_close_to_zero_from_negative = 0;
        let rug_fuzz_0 = 0.0000001f32;
        let value = -rug_fuzz_0;
        let result = value.to_u16();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_306_llm_16_306_rrrruuuugggg_to_u16_with_value_very_close_to_zero_from_negative = 0;
    }
    #[test]
    fn to_u16_with_value_very_close_to_u16_max() {
        let _rug_st_tests_llm_16_306_llm_16_306_rrrruuuugggg_to_u16_with_value_very_close_to_u16_max = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0.9999999f32;
        let value = (u16::MAX - rug_fuzz_0) as f32 + rug_fuzz_1;
        let result = value.to_u16();
        debug_assert_eq!(result, Some(u16::MAX - 1));
        let _rug_ed_tests_llm_16_306_llm_16_306_rrrruuuugggg_to_u16_with_value_very_close_to_u16_max = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_307_llm_16_307 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_u32_with_valid_float() {
        let _rug_st_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_to_u32_with_valid_float = 0;
        let rug_fuzz_0 = 42.0f32;
        let a = rug_fuzz_0;
        let result = a.to_u32();
        debug_assert_eq!(result, Some(42u32));
        let _rug_ed_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_to_u32_with_valid_float = 0;
    }
    #[test]
    fn test_to_u32_with_positive_overflow() {
        let _rug_st_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_to_u32_with_positive_overflow = 0;
        let rug_fuzz_0 = 1000.0;
        let big_float = (u32::MAX as f32) + rug_fuzz_0;
        let result = big_float.to_u32();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_to_u32_with_positive_overflow = 0;
    }
    #[test]
    fn test_to_u32_with_negative_float() {
        let _rug_st_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_to_u32_with_negative_float = 0;
        let rug_fuzz_0 = 42.0f32;
        let negative_float = -rug_fuzz_0;
        let result = negative_float.to_u32();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_to_u32_with_negative_float = 0;
    }
    #[test]
    fn test_to_u32_with_fraction() {
        let _rug_st_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_to_u32_with_fraction = 0;
        let rug_fuzz_0 = 42.7f32;
        let fractional_float = rug_fuzz_0;
        let result = fractional_float.to_u32();
        debug_assert_eq!(result, Some(42u32));
        let _rug_ed_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_to_u32_with_fraction = 0;
    }
    #[test]
    fn test_to_u32_with_zero() {
        let _rug_st_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_to_u32_with_zero = 0;
        let rug_fuzz_0 = 0.0f32;
        let zero_float = rug_fuzz_0;
        let result = zero_float.to_u32();
        debug_assert_eq!(result, Some(0u32));
        let _rug_ed_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_to_u32_with_zero = 0;
    }
    #[test]
    fn test_to_u32_edge_case_max() {
        let _rug_st_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_to_u32_edge_case_max = 0;
        let rug_fuzz_0 = 0.1;
        let max_u32_float = (u32::MAX as f32) - rug_fuzz_0;
        let result = max_u32_float.to_u32();
        debug_assert_eq!(result, Some(u32::MAX));
        let _rug_ed_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_to_u32_edge_case_max = 0;
    }
    #[test]
    fn test_to_u32_edge_case_min() {
        let _rug_st_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_to_u32_edge_case_min = 0;
        let rug_fuzz_0 = 0.9f32;
        let above_min_u32_float = -rug_fuzz_0;
        let result = above_min_u32_float.to_u32();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_to_u32_edge_case_min = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_308 {
    use super::*;
    use crate::*;
    #[test]
    fn to_u64_with_positive_f32() {
        let _rug_st_tests_llm_16_308_rrrruuuugggg_to_u64_with_positive_f32 = 0;
        let rug_fuzz_0 = 42.0;
        let float: f32 = rug_fuzz_0;
        debug_assert_eq!(float.to_u64(), Some(42u64));
        let _rug_ed_tests_llm_16_308_rrrruuuugggg_to_u64_with_positive_f32 = 0;
    }
    #[test]
    fn to_u64_with_negative_f32() {
        let _rug_st_tests_llm_16_308_rrrruuuugggg_to_u64_with_negative_f32 = 0;
        let rug_fuzz_0 = 42.0;
        let float: f32 = -rug_fuzz_0;
        debug_assert_eq!(float.to_u64(), None);
        let _rug_ed_tests_llm_16_308_rrrruuuugggg_to_u64_with_negative_f32 = 0;
    }
    #[test]
    fn to_u64_with_f32_greater_than_u64_max() {
        let _rug_st_tests_llm_16_308_rrrruuuugggg_to_u64_with_f32_greater_than_u64_max = 0;
        let rug_fuzz_0 = 1.0;
        let float: f32 = (u64::MAX as f32) + rug_fuzz_0;
        debug_assert_eq!(float.to_u64(), None);
        let _rug_ed_tests_llm_16_308_rrrruuuugggg_to_u64_with_f32_greater_than_u64_max = 0;
    }
    #[test]
    fn to_u64_with_f32_almost_u64_max() {
        let _rug_st_tests_llm_16_308_rrrruuuugggg_to_u64_with_f32_almost_u64_max = 0;
        let float: f32 = u64::MAX as f32;
        debug_assert!(float.to_u64().is_some());
        let _rug_ed_tests_llm_16_308_rrrruuuugggg_to_u64_with_f32_almost_u64_max = 0;
    }
    #[test]
    fn to_u64_with_f32_just_below_negative_one() {
        let _rug_st_tests_llm_16_308_rrrruuuugggg_to_u64_with_f32_just_below_negative_one = 0;
        let rug_fuzz_0 = 1.0;
        let float: f32 = -rug_fuzz_0 + f32::EPSILON;
        debug_assert_eq!(float.to_u64(), None);
        let _rug_ed_tests_llm_16_308_rrrruuuugggg_to_u64_with_f32_just_below_negative_one = 0;
    }
    #[test]
    fn to_u64_with_f32_just_above_negative_one() {
        let _rug_st_tests_llm_16_308_rrrruuuugggg_to_u64_with_f32_just_above_negative_one = 0;
        let rug_fuzz_0 = 1.0;
        let rug_fuzz_1 = 2.0;
        let float: f32 = -rug_fuzz_0 + rug_fuzz_1 * f32::EPSILON;
        debug_assert!(float.to_u64().is_some());
        let _rug_ed_tests_llm_16_308_rrrruuuugggg_to_u64_with_f32_just_above_negative_one = 0;
    }
    #[test]
    fn to_u64_with_positive_f32_max_value() {
        let _rug_st_tests_llm_16_308_rrrruuuugggg_to_u64_with_positive_f32_max_value = 0;
        let float: f32 = f32::MAX;
        debug_assert_eq!(float.to_u64(), None);
        let _rug_ed_tests_llm_16_308_rrrruuuugggg_to_u64_with_positive_f32_max_value = 0;
    }
    #[test]
    fn to_u64_with_positive_infinity() {
        let _rug_st_tests_llm_16_308_rrrruuuugggg_to_u64_with_positive_infinity = 0;
        let float: f32 = f32::INFINITY;
        debug_assert_eq!(float.to_u64(), None);
        let _rug_ed_tests_llm_16_308_rrrruuuugggg_to_u64_with_positive_infinity = 0;
    }
    #[test]
    fn to_u64_with_negative_infinity() {
        let _rug_st_tests_llm_16_308_rrrruuuugggg_to_u64_with_negative_infinity = 0;
        let float: f32 = f32::NEG_INFINITY;
        debug_assert_eq!(float.to_u64(), None);
        let _rug_ed_tests_llm_16_308_rrrruuuugggg_to_u64_with_negative_infinity = 0;
    }
    #[test]
    fn to_u64_with_nan() {
        let _rug_st_tests_llm_16_308_rrrruuuugggg_to_u64_with_nan = 0;
        let float: f32 = f32::NAN;
        debug_assert_eq!(float.to_u64(), None);
        let _rug_ed_tests_llm_16_308_rrrruuuugggg_to_u64_with_nan = 0;
    }
    #[test]
    fn to_u64_with_f32_that_rounds_to_an_exact_u64() {
        let _rug_st_tests_llm_16_308_rrrruuuugggg_to_u64_with_f32_that_rounds_to_an_exact_u64 = 0;
        let rug_fuzz_0 = 42.99999;
        let float: f32 = rug_fuzz_0;
        debug_assert_eq!(float.to_u64(), Some(42u64));
        let _rug_ed_tests_llm_16_308_rrrruuuugggg_to_u64_with_f32_that_rounds_to_an_exact_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_309_llm_16_309 {
    use crate::ToPrimitive;
    #[test]
    fn test_to_u8_with_positive_float() {
        let _rug_st_tests_llm_16_309_llm_16_309_rrrruuuugggg_test_to_u8_with_positive_float = 0;
        let rug_fuzz_0 = 42.3;
        let value: f32 = rug_fuzz_0;
        debug_assert_eq!(value.to_u8(), Some(42));
        let _rug_ed_tests_llm_16_309_llm_16_309_rrrruuuugggg_test_to_u8_with_positive_float = 0;
    }
    #[test]
    fn test_to_u8_with_max_value() {
        let _rug_st_tests_llm_16_309_llm_16_309_rrrruuuugggg_test_to_u8_with_max_value = 0;
        let value: f32 = u8::MAX as f32;
        debug_assert_eq!(value.to_u8(), Some(u8::MAX));
        let _rug_ed_tests_llm_16_309_llm_16_309_rrrruuuugggg_test_to_u8_with_max_value = 0;
    }
    #[test]
    fn test_to_u8_with_negative_float() {
        let _rug_st_tests_llm_16_309_llm_16_309_rrrruuuugggg_test_to_u8_with_negative_float = 0;
        let rug_fuzz_0 = 1.2;
        let value: f32 = -rug_fuzz_0;
        debug_assert_eq!(value.to_u8(), None);
        let _rug_ed_tests_llm_16_309_llm_16_309_rrrruuuugggg_test_to_u8_with_negative_float = 0;
    }
    #[test]
    fn test_to_u8_with_large_positive_float() {
        let _rug_st_tests_llm_16_309_llm_16_309_rrrruuuugggg_test_to_u8_with_large_positive_float = 0;
        let rug_fuzz_0 = 256.0;
        let value: f32 = rug_fuzz_0;
        debug_assert_eq!(value.to_u8(), None);
        let _rug_ed_tests_llm_16_309_llm_16_309_rrrruuuugggg_test_to_u8_with_large_positive_float = 0;
    }
    #[test]
    fn test_to_u8_with_positive_overflow() {
        let _rug_st_tests_llm_16_309_llm_16_309_rrrruuuugggg_test_to_u8_with_positive_overflow = 0;
        let rug_fuzz_0 = 1.0;
        let value: f32 = (u8::MAX as f32) + rug_fuzz_0;
        debug_assert_eq!(value.to_u8(), None);
        let _rug_ed_tests_llm_16_309_llm_16_309_rrrruuuugggg_test_to_u8_with_positive_overflow = 0;
    }
    #[test]
    fn test_to_u8_with_positive_edge_case() {
        let _rug_st_tests_llm_16_309_llm_16_309_rrrruuuugggg_test_to_u8_with_positive_edge_case = 0;
        let rug_fuzz_0 = 0.1;
        let value: f32 = u8::MAX as f32 - rug_fuzz_0;
        debug_assert_eq!(value.to_u8(), Some(u8::MAX - 1));
        let _rug_ed_tests_llm_16_309_llm_16_309_rrrruuuugggg_test_to_u8_with_positive_edge_case = 0;
    }
    #[test]
    fn test_to_u8_with_negative_edge_case() {
        let _rug_st_tests_llm_16_309_llm_16_309_rrrruuuugggg_test_to_u8_with_negative_edge_case = 0;
        let rug_fuzz_0 = 1.0;
        let value: f32 = -rug_fuzz_0;
        debug_assert_eq!(value.to_u8(), None);
        let _rug_ed_tests_llm_16_309_llm_16_309_rrrruuuugggg_test_to_u8_with_negative_edge_case = 0;
    }
    #[test]
    fn test_to_u8_with_zero() {
        let _rug_st_tests_llm_16_309_llm_16_309_rrrruuuugggg_test_to_u8_with_zero = 0;
        let rug_fuzz_0 = 0.0;
        let value: f32 = rug_fuzz_0;
        debug_assert_eq!(value.to_u8(), Some(0));
        let _rug_ed_tests_llm_16_309_llm_16_309_rrrruuuugggg_test_to_u8_with_zero = 0;
    }
    #[test]
    fn test_to_u8_with_subnormal_value() {
        let _rug_st_tests_llm_16_309_llm_16_309_rrrruuuugggg_test_to_u8_with_subnormal_value = 0;
        let rug_fuzz_0 = 1e-40;
        let value: f32 = rug_fuzz_0;
        debug_assert_eq!(value.to_u8(), Some(0));
        let _rug_ed_tests_llm_16_309_llm_16_309_rrrruuuugggg_test_to_u8_with_subnormal_value = 0;
    }
    #[test]
    fn test_to_u8_with_infinity() {
        let _rug_st_tests_llm_16_309_llm_16_309_rrrruuuugggg_test_to_u8_with_infinity = 0;
        let value: f32 = f32::INFINITY;
        debug_assert_eq!(value.to_u8(), None);
        let _rug_ed_tests_llm_16_309_llm_16_309_rrrruuuugggg_test_to_u8_with_infinity = 0;
    }
    #[test]
    fn test_to_u8_with_nan() {
        let _rug_st_tests_llm_16_309_llm_16_309_rrrruuuugggg_test_to_u8_with_nan = 0;
        let value: f32 = f32::NAN;
        debug_assert_eq!(value.to_u8(), None);
        let _rug_ed_tests_llm_16_309_llm_16_309_rrrruuuugggg_test_to_u8_with_nan = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_436_llm_16_436 {
    use crate::AsPrimitive;
    #[test]
    fn test_as_primitive_f64_to_f32() {
        let _rug_st_tests_llm_16_436_llm_16_436_rrrruuuugggg_test_as_primitive_f64_to_f32 = 0;
        let rug_fuzz_0 = 42.0;
        let rug_fuzz_1 = 42.0f32;
        let value: f64 = rug_fuzz_0;
        let result: f32 = AsPrimitive::<f32>::as_(value);
        let expected: f32 = rug_fuzz_1;
        debug_assert!((result - expected).abs() <= f32::EPSILON);
        let _rug_ed_tests_llm_16_436_llm_16_436_rrrruuuugggg_test_as_primitive_f64_to_f32 = 0;
    }
    #[test]
    fn test_as_primitive_f64_to_f32_large_value() {
        let _rug_st_tests_llm_16_436_llm_16_436_rrrruuuugggg_test_as_primitive_f64_to_f32_large_value = 0;
        let rug_fuzz_0 = 1e10;
        let rug_fuzz_1 = 1e10f32;
        let value: f64 = rug_fuzz_0;
        let result: f32 = AsPrimitive::<f32>::as_(value);
        let expected: f32 = rug_fuzz_1;
        debug_assert!((result - expected).abs() <= f32::EPSILON);
        let _rug_ed_tests_llm_16_436_llm_16_436_rrrruuuugggg_test_as_primitive_f64_to_f32_large_value = 0;
    }
    #[test]
    fn test_as_primitive_f64_to_f32_small_value() {
        let _rug_st_tests_llm_16_436_llm_16_436_rrrruuuugggg_test_as_primitive_f64_to_f32_small_value = 0;
        let rug_fuzz_0 = 1e-10;
        let rug_fuzz_1 = 1e-10f32;
        let value: f64 = rug_fuzz_0;
        let result: f32 = AsPrimitive::<f32>::as_(value);
        let expected: f32 = rug_fuzz_1;
        debug_assert!((result - expected).abs() <= f32::EPSILON);
        let _rug_ed_tests_llm_16_436_llm_16_436_rrrruuuugggg_test_as_primitive_f64_to_f32_small_value = 0;
    }
    #[test]
    fn test_as_primitive_f64_to_f32_edge_case() {
        let _rug_st_tests_llm_16_436_llm_16_436_rrrruuuugggg_test_as_primitive_f64_to_f32_edge_case = 0;
        let value: f64 = f64::MAX;
        let result: f32 = AsPrimitive::<f32>::as_(value);
        debug_assert!(result.is_infinite());
        let _rug_ed_tests_llm_16_436_llm_16_436_rrrruuuugggg_test_as_primitive_f64_to_f32_edge_case = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_437_llm_16_437 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive() {
        let _rug_st_tests_llm_16_437_llm_16_437_rrrruuuugggg_test_as_primitive = 0;
        let rug_fuzz_0 = 42.0;
        let value: f64 = rug_fuzz_0;
        let result: f64 = <f64 as AsPrimitive<f64>>::as_(value);
        debug_assert_eq!(result, 42.0);
        let _rug_ed_tests_llm_16_437_llm_16_437_rrrruuuugggg_test_as_primitive = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_438_llm_16_438 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_f64_to_i128() {
        let _rug_st_tests_llm_16_438_llm_16_438_rrrruuuugggg_test_as_primitive_f64_to_i128 = 0;
        let rug_fuzz_0 = 12345.6;
        let value: f64 = rug_fuzz_0;
        let result: i128 = value.as_();
        debug_assert_eq!(result, 12345i128);
        let _rug_ed_tests_llm_16_438_llm_16_438_rrrruuuugggg_test_as_primitive_f64_to_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_439_llm_16_439 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_f64_to_i16() {
        let _rug_st_tests_llm_16_439_llm_16_439_rrrruuuugggg_test_as_primitive_f64_to_i16 = 0;
        let rug_fuzz_0 = 42.0;
        let rug_fuzz_1 = 42.0;
        let rug_fuzz_2 = 42.99;
        let rug_fuzz_3 = 42.99;
        let rug_fuzz_4 = 32767.0;
        let rug_fuzz_5 = 32768.0;
        let rug_fuzz_6 = 32768.0;
        let rug_fuzz_7 = 32769.0;
        let value: f64 = rug_fuzz_0;
        let result: i16 = AsPrimitive::<i16>::as_(value);
        debug_assert_eq!(result, 42i16, "Casting f64 to i16 failed");
        let value: f64 = f64::MAX;
        let result = AsPrimitive::<i16>::as_(value);
        debug_assert!(
            result <= i16::MAX, "Casting f64::MAX to i16 did not yield i16::MAX"
        );
        let value: f64 = f64::MIN;
        let result = AsPrimitive::<i16>::as_(value);
        debug_assert!(
            result >= i16::MIN, "Casting f64::MIN to i16 did not yield i16::MIN"
        );
        let value: f64 = -rug_fuzz_1;
        let result: i16 = AsPrimitive::<i16>::as_(value);
        debug_assert_eq!(result, - 42i16, "Casting -42.0f64 to i16 failed");
        let value: f64 = rug_fuzz_2;
        let result: i16 = AsPrimitive::<i16>::as_(value);
        debug_assert_eq!(result, 42i16, "Casting 42.99f64 to i16 should yield 42");
        let value: f64 = -rug_fuzz_3;
        let result: i16 = AsPrimitive::<i16>::as_(value);
        debug_assert_eq!(result, - 42i16, "Casting -42.99f64 to i16 should yield -42");
        let value: f64 = rug_fuzz_4;
        let result: i16 = AsPrimitive::<i16>::as_(value);
        debug_assert_eq!(
            result, 32767i16, "Casting 32767.0f64 to i16 should yield 32767"
        );
        let value: f64 = rug_fuzz_5;
        let result = AsPrimitive::<i16>::as_(value);
        debug_assert!(
            result <= i16::MAX, "Casting 32768.0f64 to i16 did not yield i16::MAX"
        );
        let value: f64 = -rug_fuzz_6;
        let result = AsPrimitive::<i16>::as_(value);
        debug_assert!(
            result >= i16::MIN, "Casting -32768.0f64 to i16 did not yield i16::MIN"
        );
        let value: f64 = -rug_fuzz_7;
        let result = AsPrimitive::<i16>::as_(value);
        debug_assert!(
            result >= i16::MIN, "Casting -32769.0f64 to i16 did not yield i16::MIN"
        );
        let _rug_ed_tests_llm_16_439_llm_16_439_rrrruuuugggg_test_as_primitive_f64_to_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_440_llm_16_440 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_f64_as_i32() {
        let _rug_st_tests_llm_16_440_llm_16_440_rrrruuuugggg_test_f64_as_i32 = 0;
        let rug_fuzz_0 = 42.0;
        let rug_fuzz_1 = 42;
        let values: Vec<f64> = vec![
            rug_fuzz_0, 3.14159, - 2.71828, f64::MAX, f64::MIN, 0.0, - 0.0,
            f64::INFINITY, f64::NEG_INFINITY, f64::NAN
        ];
        let expected: Vec<i32> = vec![
            rug_fuzz_1, 3, - 2, i32::MAX, i32::MIN, 0, 0, i32::MAX, i32::MIN, 0
        ];
        let results: Vec<i32> = values
            .iter()
            .map(|&x| <f64 as AsPrimitive<i32>>::as_(x))
            .collect();
        for (&val, (&res, &exp)) in values
            .iter()
            .zip(results.iter().zip(expected.iter()))
            .filter(|(&val, _)| !val.is_nan())
        {
            debug_assert_eq!(res, exp, "failed for value: {}", val);
        }
        let _rug_ed_tests_llm_16_440_llm_16_440_rrrruuuugggg_test_f64_as_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_441_llm_16_441 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_f64_to_i64() {
        let _rug_st_tests_llm_16_441_llm_16_441_rrrruuuugggg_test_as_primitive_f64_to_i64 = 0;
        let rug_fuzz_0 = 42.0;
        let value: f64 = rug_fuzz_0;
        let result: i64 = AsPrimitive::<i64>::as_(value);
        debug_assert_eq!(result, 42i64);
        let _rug_ed_tests_llm_16_441_llm_16_441_rrrruuuugggg_test_as_primitive_f64_to_i64 = 0;
    }
    #[test]
    fn test_as_primitive_f64_to_i64_negative() {
        let _rug_st_tests_llm_16_441_llm_16_441_rrrruuuugggg_test_as_primitive_f64_to_i64_negative = 0;
        let rug_fuzz_0 = 42.0;
        let value: f64 = -rug_fuzz_0;
        let result: i64 = AsPrimitive::<i64>::as_(value);
        debug_assert_eq!(result, - 42i64);
        let _rug_ed_tests_llm_16_441_llm_16_441_rrrruuuugggg_test_as_primitive_f64_to_i64_negative = 0;
    }
    #[test]
    fn test_as_primitive_f64_to_i64_fractional() {
        let _rug_st_tests_llm_16_441_llm_16_441_rrrruuuugggg_test_as_primitive_f64_to_i64_fractional = 0;
        let rug_fuzz_0 = 42.5;
        let value: f64 = rug_fuzz_0;
        let result: i64 = AsPrimitive::<i64>::as_(value);
        debug_assert_eq!(result, 42i64);
        let _rug_ed_tests_llm_16_441_llm_16_441_rrrruuuugggg_test_as_primitive_f64_to_i64_fractional = 0;
    }
    #[test]
    fn test_as_primitive_f64_to_i64_large_number() {
        let _rug_st_tests_llm_16_441_llm_16_441_rrrruuuugggg_test_as_primitive_f64_to_i64_large_number = 0;
        let rug_fuzz_0 = 1e20;
        let rug_fuzz_1 = 2;
        let value: f64 = rug_fuzz_0;
        let result: i64 = AsPrimitive::<i64>::as_(value);
        debug_assert!(result > i64::MAX / rug_fuzz_1);
        let _rug_ed_tests_llm_16_441_llm_16_441_rrrruuuugggg_test_as_primitive_f64_to_i64_large_number = 0;
    }
    #[test]
    fn test_as_primitive_f64_to_i64_zero() {
        let _rug_st_tests_llm_16_441_llm_16_441_rrrruuuugggg_test_as_primitive_f64_to_i64_zero = 0;
        let rug_fuzz_0 = 0.0;
        let value: f64 = rug_fuzz_0;
        let result: i64 = AsPrimitive::<i64>::as_(value);
        debug_assert_eq!(result, 0i64);
        let _rug_ed_tests_llm_16_441_llm_16_441_rrrruuuugggg_test_as_primitive_f64_to_i64_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_442_llm_16_442 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_f64_to_i8() {
        let _rug_st_tests_llm_16_442_llm_16_442_rrrruuuugggg_test_as_primitive_f64_to_i8 = 0;
        let rug_fuzz_0 = 123.456f64;
        let value_f64 = rug_fuzz_0;
        let value_i8: i8 = AsPrimitive::<i8>::as_(value_f64);
        debug_assert_eq!(value_i8, 123i8);
        let _rug_ed_tests_llm_16_442_llm_16_442_rrrruuuugggg_test_as_primitive_f64_to_i8 = 0;
    }
    #[test]
    #[should_panic]
    fn test_as_primitive_f64_to_i8_overflow() {
        let _rug_st_tests_llm_16_442_llm_16_442_rrrruuuugggg_test_as_primitive_f64_to_i8_overflow = 0;
        let rug_fuzz_0 = 256.0f64;
        let value_f64 = rug_fuzz_0;
        let _value_i8: i8 = AsPrimitive::<i8>::as_(value_f64);
        let _rug_ed_tests_llm_16_442_llm_16_442_rrrruuuugggg_test_as_primitive_f64_to_i8_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_443_llm_16_443 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_f64_to_isize() {
        let _rug_st_tests_llm_16_443_llm_16_443_rrrruuuugggg_test_as_primitive_f64_to_isize = 0;
        let rug_fuzz_0 = 42.0;
        let val: f64 = rug_fuzz_0;
        let result: isize = AsPrimitive::<isize>::as_(val);
        debug_assert_eq!(result, 42isize);
        let _rug_ed_tests_llm_16_443_llm_16_443_rrrruuuugggg_test_as_primitive_f64_to_isize = 0;
    }
    #[test]
    fn test_as_primitive_f64_to_isize_negative() {
        let _rug_st_tests_llm_16_443_llm_16_443_rrrruuuugggg_test_as_primitive_f64_to_isize_negative = 0;
        let rug_fuzz_0 = 42.0;
        let val: f64 = -rug_fuzz_0;
        let result: isize = AsPrimitive::<isize>::as_(val);
        debug_assert_eq!(result, - 42isize);
        let _rug_ed_tests_llm_16_443_llm_16_443_rrrruuuugggg_test_as_primitive_f64_to_isize_negative = 0;
    }
    #[test]
    fn test_as_primitive_f64_to_isize_truncation() {
        let _rug_st_tests_llm_16_443_llm_16_443_rrrruuuugggg_test_as_primitive_f64_to_isize_truncation = 0;
        let rug_fuzz_0 = 42.999;
        let val: f64 = rug_fuzz_0;
        let result: isize = AsPrimitive::<isize>::as_(val);
        debug_assert_eq!(result, 42isize);
        let _rug_ed_tests_llm_16_443_llm_16_443_rrrruuuugggg_test_as_primitive_f64_to_isize_truncation = 0;
    }
    #[test]
    #[should_panic(expected = "attempt to cast floating point to integer")]
    fn test_as_primitive_f64_to_isize_overflow() {
        let _rug_st_tests_llm_16_443_llm_16_443_rrrruuuugggg_test_as_primitive_f64_to_isize_overflow = 0;
        let val: f64 = f64::MAX;
        let _result: isize = AsPrimitive::<isize>::as_(val);
        let _rug_ed_tests_llm_16_443_llm_16_443_rrrruuuugggg_test_as_primitive_f64_to_isize_overflow = 0;
    }
    #[test]
    #[should_panic(expected = "attempt to cast floating point to integer")]
    fn test_as_primitive_f64_to_isize_underflow() {
        let _rug_st_tests_llm_16_443_llm_16_443_rrrruuuugggg_test_as_primitive_f64_to_isize_underflow = 0;
        let val: f64 = f64::MIN;
        let _result: isize = AsPrimitive::<isize>::as_(val);
        let _rug_ed_tests_llm_16_443_llm_16_443_rrrruuuugggg_test_as_primitive_f64_to_isize_underflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_444_llm_16_444 {
    use crate::AsPrimitive;
    #[test]
    fn test_as_primitive_f64_to_u128() {
        let _rug_st_tests_llm_16_444_llm_16_444_rrrruuuugggg_test_as_primitive_f64_to_u128 = 0;
        let rug_fuzz_0 = 123.456;
        let value: f64 = rug_fuzz_0;
        let result: u128 = AsPrimitive::<u128>::as_(value);
        debug_assert_eq!(result, 123u128);
        let _rug_ed_tests_llm_16_444_llm_16_444_rrrruuuugggg_test_as_primitive_f64_to_u128 = 0;
    }
    #[test]
    #[should_panic(expected = "attempt to cast to with overflow")]
    fn test_as_primitive_f64_to_u128_overflow() {
        let _rug_st_tests_llm_16_444_llm_16_444_rrrruuuugggg_test_as_primitive_f64_to_u128_overflow = 0;
        let value: f64 = f64::MAX;
        let _result: u128 = AsPrimitive::<u128>::as_(value);
        let _rug_ed_tests_llm_16_444_llm_16_444_rrrruuuugggg_test_as_primitive_f64_to_u128_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_445_llm_16_445 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_f64_to_u16() {
        let _rug_st_tests_llm_16_445_llm_16_445_rrrruuuugggg_test_as_primitive_f64_to_u16 = 0;
        let rug_fuzz_0 = 42.0;
        let value: f64 = rug_fuzz_0;
        let result: u16 = AsPrimitive::<u16>::as_(value);
        debug_assert_eq!(result, 42u16);
        let _rug_ed_tests_llm_16_445_llm_16_445_rrrruuuugggg_test_as_primitive_f64_to_u16 = 0;
    }
    #[test]
    fn test_as_primitive_f64_to_u16_with_truncation() {
        let _rug_st_tests_llm_16_445_llm_16_445_rrrruuuugggg_test_as_primitive_f64_to_u16_with_truncation = 0;
        let rug_fuzz_0 = 42.99;
        let value: f64 = rug_fuzz_0;
        let result: u16 = AsPrimitive::<u16>::as_(value);
        debug_assert_eq!(result, 42u16);
        let _rug_ed_tests_llm_16_445_llm_16_445_rrrruuuugggg_test_as_primitive_f64_to_u16_with_truncation = 0;
    }
    #[test]
    #[should_panic]
    fn test_as_primitive_f64_to_u16_with_overflow() {
        let _rug_st_tests_llm_16_445_llm_16_445_rrrruuuugggg_test_as_primitive_f64_to_u16_with_overflow = 0;
        let value: f64 = f64::MAX;
        let _: u16 = AsPrimitive::<u16>::as_(value);
        let _rug_ed_tests_llm_16_445_llm_16_445_rrrruuuugggg_test_as_primitive_f64_to_u16_with_overflow = 0;
    }
    #[test]
    #[should_panic]
    fn test_as_primitive_f64_to_u16_with_underflow() {
        let _rug_st_tests_llm_16_445_llm_16_445_rrrruuuugggg_test_as_primitive_f64_to_u16_with_underflow = 0;
        let value: f64 = f64::MIN;
        let _: u16 = AsPrimitive::<u16>::as_(value);
        let _rug_ed_tests_llm_16_445_llm_16_445_rrrruuuugggg_test_as_primitive_f64_to_u16_with_underflow = 0;
    }
    #[test]
    fn test_as_primitive_f64_to_u16_zero() {
        let _rug_st_tests_llm_16_445_llm_16_445_rrrruuuugggg_test_as_primitive_f64_to_u16_zero = 0;
        let rug_fuzz_0 = 0.0;
        let value: f64 = rug_fuzz_0;
        let result: u16 = AsPrimitive::<u16>::as_(value);
        debug_assert_eq!(result, 0u16);
        let _rug_ed_tests_llm_16_445_llm_16_445_rrrruuuugggg_test_as_primitive_f64_to_u16_zero = 0;
    }
    #[test]
    fn test_as_primitive_f64_to_u16_negative() {
        let _rug_st_tests_llm_16_445_llm_16_445_rrrruuuugggg_test_as_primitive_f64_to_u16_negative = 0;
        let rug_fuzz_0 = 42.0;
        let rug_fuzz_1 = 0u16;
        let value: f64 = -rug_fuzz_0;
        let result: u16 = AsPrimitive::<u16>::as_(value);
        debug_assert!(result <= rug_fuzz_1);
        let _rug_ed_tests_llm_16_445_llm_16_445_rrrruuuugggg_test_as_primitive_f64_to_u16_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_446 {
    use super::*;
    use crate::*;
    #[test]
    fn test_as_primitive_f64_to_u32() {
        let _rug_st_tests_llm_16_446_rrrruuuugggg_test_as_primitive_f64_to_u32 = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1.0;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 1.5;
        let rug_fuzz_5 = 1;
        let values: [(f64, Option<u32>); 4] = [
            (rug_fuzz_0, Some(rug_fuzz_1)),
            (rug_fuzz_2, Some(rug_fuzz_3)),
            (rug_fuzz_4, Some(rug_fuzz_5)),
            (f64::MAX, None),
        ];
        for &(f, expected) in &values {
            let as_u32: u32 = f.as_();
            match expected {
                Some(val) => debug_assert_eq!(as_u32, val),
                None => debug_assert!(f as u32 > u32::MAX),
            }
        }
        let _rug_ed_tests_llm_16_446_rrrruuuugggg_test_as_primitive_f64_to_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_447 {
    use super::*;
    use crate::*;
    #[test]
    fn test_as_primitive_f64_to_u64() {
        let _rug_st_tests_llm_16_447_rrrruuuugggg_test_as_primitive_f64_to_u64 = 0;
        let rug_fuzz_0 = 42.0;
        let value: f64 = rug_fuzz_0;
        let result = <f64 as cast::AsPrimitive<u64>>::as_(value);
        debug_assert_eq!(result, 42u64);
        let _rug_ed_tests_llm_16_447_rrrruuuugggg_test_as_primitive_f64_to_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_449_llm_16_449 {
    use crate::AsPrimitive;
    #[test]
    fn test_as_primitive_f64_to_usize() {
        let _rug_st_tests_llm_16_449_llm_16_449_rrrruuuugggg_test_as_primitive_f64_to_usize = 0;
        let rug_fuzz_0 = 42.0f64;
        let rug_fuzz_1 = 42.9f64;
        debug_assert_eq!(AsPrimitive:: < usize > ::as_(rug_fuzz_0), 42usize);
        debug_assert_eq!(AsPrimitive:: < usize > ::as_(rug_fuzz_1), 42usize);
        debug_assert_eq!(AsPrimitive:: < usize > ::as_(f64::MAX), usize::MAX);
        debug_assert_eq!(AsPrimitive:: < usize > ::as_(f64::INFINITY), usize::MAX);
        let _rug_ed_tests_llm_16_449_llm_16_449_rrrruuuugggg_test_as_primitive_f64_to_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_450_llm_16_450 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_f32() {
        let _rug_st_tests_llm_16_450_llm_16_450_rrrruuuugggg_test_from_f32 = 0;
        let rug_fuzz_0 = 42.0;
        let float_value: f32 = rug_fuzz_0;
        let converted_value: Option<f64> = <f64 as FromPrimitive>::from_f32(float_value);
        debug_assert_eq!(converted_value, Some(42.0));
        let nan_value: f32 = f32::NAN;
        debug_assert!(< f64 as FromPrimitive > ::from_f32(nan_value).unwrap().is_nan());
        let infinity_value: f32 = f32::INFINITY;
        debug_assert_eq!(
            < f64 as FromPrimitive > ::from_f32(infinity_value), Some(f64::INFINITY)
        );
        let neg_infinity_value: f32 = f32::NEG_INFINITY;
        debug_assert_eq!(
            < f64 as FromPrimitive > ::from_f32(neg_infinity_value),
            Some(f64::NEG_INFINITY)
        );
        let max_f32_value: f32 = f32::MAX;
        debug_assert_eq!(
            < f64 as FromPrimitive > ::from_f32(max_f32_value), Some(max_f32_value as
            f64)
        );
        let min_f32_value: f32 = f32::MIN;
        debug_assert_eq!(
            < f64 as FromPrimitive > ::from_f32(min_f32_value), Some(min_f32_value as
            f64)
        );
        let _rug_ed_tests_llm_16_450_llm_16_450_rrrruuuugggg_test_from_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_451_llm_16_451 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_f64() {
        let _rug_st_tests_llm_16_451_llm_16_451_rrrruuuugggg_test_from_f64 = 0;
        let rug_fuzz_0 = 42.0;
        let float_value: f64 = rug_fuzz_0;
        let converted_value = i32::from_f64(float_value);
        debug_assert_eq!(converted_value, Some(42));
        let overflow_value: f64 = f64::MAX;
        let failed_conversion = i32::from_f64(overflow_value);
        debug_assert_eq!(failed_conversion, None);
        let _rug_ed_tests_llm_16_451_llm_16_451_rrrruuuugggg_test_from_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_452_llm_16_452 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i128() {
        let _rug_st_tests_llm_16_452_llm_16_452_rrrruuuugggg_test_from_i128 = 0;
        let rug_fuzz_0 = 123i128;
        let rug_fuzz_1 = 123i128;
        let val_i128 = rug_fuzz_0;
        let val_f64 = <f64 as FromPrimitive>::from_i128(val_i128);
        debug_assert_eq!(val_f64, Some(123f64));
        let val_i128_neg = -rug_fuzz_1;
        let val_f64_neg = <f64 as FromPrimitive>::from_i128(val_i128_neg);
        debug_assert_eq!(val_f64_neg, Some(- 123f64));
        let val_i128_max = i128::MAX;
        let val_f64_max = <f64 as FromPrimitive>::from_i128(val_i128_max);
        debug_assert!(val_f64_max.is_some());
        let val_i128_min = i128::MIN;
        let val_f64_min = <f64 as FromPrimitive>::from_i128(val_i128_min);
        debug_assert!(val_f64_min.is_some());
        let _rug_ed_tests_llm_16_452_llm_16_452_rrrruuuugggg_test_from_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_453_llm_16_453 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i16() {
        let _rug_st_tests_llm_16_453_llm_16_453_rrrruuuugggg_test_from_i16 = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        debug_assert_eq!(< f64 as FromPrimitive > ::from_i16(rug_fuzz_0), Some(0.0));
        debug_assert_eq!(< f64 as FromPrimitive > ::from_i16(rug_fuzz_1), Some(1.0));
        debug_assert_eq!(< f64 as FromPrimitive > ::from_i16(- rug_fuzz_2), Some(- 1.0));
        debug_assert_eq!(
            < f64 as FromPrimitive > ::from_i16(i16::MAX), Some(i16::MAX as f64)
        );
        debug_assert_eq!(
            < f64 as FromPrimitive > ::from_i16(i16::MIN), Some(i16::MIN as f64)
        );
        let _rug_ed_tests_llm_16_453_llm_16_453_rrrruuuugggg_test_from_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_454_llm_16_454 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i32() {
        assert_eq!(< f64 as FromPrimitive >::from_i32(0), Some(0.0_f64));
        assert_eq!(< f64 as FromPrimitive >::from_i32(42), Some(42.0_f64));
        assert_eq!(< f64 as FromPrimitive >::from_i32(- 42), Some(- 42.0_f64));
        assert_eq!(< f64 as FromPrimitive >::from_i32(i32::MAX), Some(i32::MAX as f64));
        assert_eq!(< f64 as FromPrimitive >::from_i32(i32::MIN), Some(i32::MIN as f64));
        assert_eq!(
            < f64 as FromPrimitive >::from_i32(2147483647), Some(2147483647.0_f64)
        );
        assert_eq!(
            < f64 as FromPrimitive >::from_i32(- 2147483648), Some(- 2147483648.0_f64)
        );
    }
}
#[cfg(test)]
mod tests_llm_16_455_llm_16_455 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i64() {
        let _rug_st_tests_llm_16_455_llm_16_455_rrrruuuugggg_test_from_i64 = 0;
        let rug_fuzz_0 = 42i64;
        let input = rug_fuzz_0;
        let result = <f64 as FromPrimitive>::from_i64(input);
        debug_assert_eq!(result, Some(42.0f64));
        let _rug_ed_tests_llm_16_455_llm_16_455_rrrruuuugggg_test_from_i64 = 0;
    }
    #[test]
    fn test_from_i64_negative() {
        let _rug_st_tests_llm_16_455_llm_16_455_rrrruuuugggg_test_from_i64_negative = 0;
        let rug_fuzz_0 = 42i64;
        let input = -rug_fuzz_0;
        let result = <f64 as FromPrimitive>::from_i64(input);
        debug_assert_eq!(result, Some(- 42.0f64));
        let _rug_ed_tests_llm_16_455_llm_16_455_rrrruuuugggg_test_from_i64_negative = 0;
    }
    #[test]
    fn test_from_i64_zero() {
        let _rug_st_tests_llm_16_455_llm_16_455_rrrruuuugggg_test_from_i64_zero = 0;
        let rug_fuzz_0 = 0i64;
        let input = rug_fuzz_0;
        let result = <f64 as FromPrimitive>::from_i64(input);
        debug_assert_eq!(result, Some(0.0f64));
        let _rug_ed_tests_llm_16_455_llm_16_455_rrrruuuugggg_test_from_i64_zero = 0;
    }
    #[test]
    fn test_from_i64_max() {
        let _rug_st_tests_llm_16_455_llm_16_455_rrrruuuugggg_test_from_i64_max = 0;
        let input = i64::MAX;
        let result = <f64 as FromPrimitive>::from_i64(input);
        debug_assert!(result.is_some());
        let _rug_ed_tests_llm_16_455_llm_16_455_rrrruuuugggg_test_from_i64_max = 0;
    }
    #[test]
    fn test_from_i64_min() {
        let _rug_st_tests_llm_16_455_llm_16_455_rrrruuuugggg_test_from_i64_min = 0;
        let input = i64::MIN;
        let result = <f64 as FromPrimitive>::from_i64(input);
        debug_assert!(result.is_some());
        let _rug_ed_tests_llm_16_455_llm_16_455_rrrruuuugggg_test_from_i64_min = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_456_llm_16_456 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i8() {
        let _rug_st_tests_llm_16_456_llm_16_456_rrrruuuugggg_test_from_i8 = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0.0;
        let test_values: Vec<i8> = vec![rug_fuzz_0, 1, 127, - 1, - 128];
        let expected_values: Vec<Option<f64>> = vec![
            Some(rug_fuzz_1), Some(1.0), Some(127.0), Some(- 1.0), Some(- 128.0)
        ];
        for (test_val, expected_val) in test_values.iter().zip(expected_values.iter()) {
            let result = <f64 as FromPrimitive>::from_i8(*test_val);
            debug_assert_eq!(result, * expected_val, "from_i8: for input {}", test_val);
        }
        let _rug_ed_tests_llm_16_456_llm_16_456_rrrruuuugggg_test_from_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_457_llm_16_457 {
    use super::*;
    use crate::*;
    #[test]
    fn test_from_isize_within_bounds() {
        let _rug_st_tests_llm_16_457_llm_16_457_rrrruuuugggg_test_from_isize_within_bounds = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0.0;
        let values: [(isize, Option<f64>); 3] = [
            (isize::MIN, Some(isize::MIN as f64)),
            (rug_fuzz_0, Some(rug_fuzz_1)),
            (isize::MAX, Some(isize::MAX as f64)),
        ];
        for &(input, expected) in &values {
            let result = f64::from_isize(input);
            debug_assert_eq!(result, expected);
        }
        let _rug_ed_tests_llm_16_457_llm_16_457_rrrruuuugggg_test_from_isize_within_bounds = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_458_llm_16_458 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u128() {
        let _rug_st_tests_llm_16_458_llm_16_458_rrrruuuugggg_test_from_u128 = 0;
        let rug_fuzz_0 = 0_u128;
        let rug_fuzz_1 = 0.0_f64;
        let rug_fuzz_2 = 1_u128;
        let rug_fuzz_3 = 1.0_f64;
        let values = [
            (rug_fuzz_0, Some(rug_fuzz_1)),
            (rug_fuzz_2, Some(rug_fuzz_3)),
            (u128::from(u64::MAX), Some(u64::MAX as f64)),
        ];
        for (input, expected) in values.iter() {
            debug_assert_eq!(f64::from_u128(* input), * expected);
        }
        let _rug_ed_tests_llm_16_458_llm_16_458_rrrruuuugggg_test_from_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_459_llm_16_459 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u16() {
        let _rug_st_tests_llm_16_459_llm_16_459_rrrruuuugggg_test_from_u16 = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        debug_assert_eq!(< f64 as FromPrimitive > ::from_u16(rug_fuzz_0), Some(0.0f64));
        debug_assert_eq!(< f64 as FromPrimitive > ::from_u16(rug_fuzz_1), Some(1.0f64));
        debug_assert_eq!(
            < f64 as FromPrimitive > ::from_u16(u16::MAX), Some(u16::MAX as f64)
        );
        let _rug_ed_tests_llm_16_459_llm_16_459_rrrruuuugggg_test_from_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_460_llm_16_460 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u32_with_f64() {
        let _rug_st_tests_llm_16_460_llm_16_460_rrrruuuugggg_test_from_u32_with_f64 = 0;
        let rug_fuzz_0 = 0_u32;
        let rug_fuzz_1 = 1_u32;
        let rug_fuzz_2 = 42_u32;
        debug_assert_eq!(< f64 as FromPrimitive > ::from_u32(rug_fuzz_0), Some(0.0_f64));
        debug_assert_eq!(< f64 as FromPrimitive > ::from_u32(rug_fuzz_1), Some(1.0_f64));
        debug_assert_eq!(
            < f64 as FromPrimitive > ::from_u32(rug_fuzz_2), Some(42.0_f64)
        );
        debug_assert_eq!(
            < f64 as FromPrimitive > ::from_u32(u32::MAX), Some(u32::MAX as f64)
        );
        let _rug_ed_tests_llm_16_460_llm_16_460_rrrruuuugggg_test_from_u32_with_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_461_llm_16_461 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u64() {
        let _rug_st_tests_llm_16_461_llm_16_461_rrrruuuugggg_test_from_u64 = 0;
        let rug_fuzz_0 = 12345;
        let rug_fuzz_1 = 12345.0;
        let value_u64: u64 = rug_fuzz_0;
        let expected_f64: f64 = rug_fuzz_1;
        debug_assert_eq!(
            < f64 as FromPrimitive > ::from_u64(value_u64), Some(expected_f64)
        );
        let value_u64_large: u64 = u64::MAX;
        let result = <f64 as FromPrimitive>::from_u64(value_u64_large);
        if let Some(value_f64) = result {
            debug_assert!(value_f64.is_finite());
        } else {
            debug_assert!(
                value_u64_large as f64 == f64::INFINITY || value_u64_large as f64 ==
                f64::NAN
            );
        }
        let _rug_ed_tests_llm_16_461_llm_16_461_rrrruuuugggg_test_from_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_462_llm_16_462 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u8() {
        let _rug_st_tests_llm_16_462_llm_16_462_rrrruuuugggg_test_from_u8 = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 255;
        let rug_fuzz_3 = 100;
        debug_assert_eq!(< f64 as FromPrimitive > ::from_u8(rug_fuzz_0), Some(0.0));
        debug_assert_eq!(< f64 as FromPrimitive > ::from_u8(rug_fuzz_1), Some(1.0));
        debug_assert_eq!(< f64 as FromPrimitive > ::from_u8(rug_fuzz_2), Some(255.0));
        debug_assert_eq!(< f64 as FromPrimitive > ::from_u8(rug_fuzz_3), Some(100.0));
        let _rug_ed_tests_llm_16_462_llm_16_462_rrrruuuugggg_test_from_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_463_llm_16_463 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_usize() {
        let _rug_st_tests_llm_16_463_llm_16_463_rrrruuuugggg_test_from_usize = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 42;
        debug_assert_eq!(< f64 as FromPrimitive > ::from_usize(rug_fuzz_0), Some(0.0));
        debug_assert_eq!(< f64 as FromPrimitive > ::from_usize(rug_fuzz_1), Some(42.0));
        debug_assert_eq!(
            < f64 as FromPrimitive > ::from_usize(usize::MAX), Some(usize::MAX as f64)
        );
        let _rug_ed_tests_llm_16_463_llm_16_463_rrrruuuugggg_test_from_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_464_llm_16_464 {
    use crate::cast::NumCast;
    use crate::cast::ToPrimitive;
    use crate::Num;
    use std::num::Wrapping;
    #[test]
    fn numcast_from_wrapping_to_f64() {
        let _rug_st_tests_llm_16_464_llm_16_464_rrrruuuugggg_numcast_from_wrapping_to_f64 = 0;
        let rug_fuzz_0 = 42u8;
        let rug_fuzz_1 = 42u16;
        let rug_fuzz_2 = 42u32;
        let rug_fuzz_3 = 42u64;
        let rug_fuzz_4 = 42usize;
        let rug_fuzz_5 = 42i8;
        let rug_fuzz_6 = 42i16;
        let rug_fuzz_7 = 42i32;
        let rug_fuzz_8 = 42i64;
        let rug_fuzz_9 = 42isize;
        let rug_fuzz_10 = 42.0f32;
        debug_assert_eq!(< f64 as NumCast > ::from(Wrapping(rug_fuzz_0)), Some(42f64));
        debug_assert_eq!(< f64 as NumCast > ::from(Wrapping(rug_fuzz_1)), Some(42f64));
        debug_assert_eq!(< f64 as NumCast > ::from(Wrapping(rug_fuzz_2)), Some(42f64));
        debug_assert_eq!(< f64 as NumCast > ::from(Wrapping(rug_fuzz_3)), Some(42f64));
        debug_assert_eq!(< f64 as NumCast > ::from(Wrapping(rug_fuzz_4)), Some(42f64));
        debug_assert_eq!(< f64 as NumCast > ::from(Wrapping(rug_fuzz_5)), Some(42f64));
        debug_assert_eq!(< f64 as NumCast > ::from(Wrapping(rug_fuzz_6)), Some(42f64));
        debug_assert_eq!(< f64 as NumCast > ::from(Wrapping(rug_fuzz_7)), Some(42f64));
        debug_assert_eq!(< f64 as NumCast > ::from(Wrapping(rug_fuzz_8)), Some(42f64));
        debug_assert_eq!(< f64 as NumCast > ::from(Wrapping(rug_fuzz_9)), Some(42f64));
        debug_assert_eq!(< f64 as NumCast > ::from(Wrapping(rug_fuzz_10)), Some(42f64));
        debug_assert_eq!(
            < f64 as NumCast > ::from(Wrapping(u128::MAX)), Some(f64::INFINITY)
        );
        debug_assert_eq!(
            < f64 as NumCast > ::from(Wrapping(i128::MAX)), Some(f64::INFINITY)
        );
        debug_assert_eq!(
            < f64 as NumCast > ::from(Wrapping(i128::MIN)), Some(f64::NEG_INFINITY)
        );
        let _rug_ed_tests_llm_16_464_llm_16_464_rrrruuuugggg_numcast_from_wrapping_to_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_465_llm_16_465 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_f32_with_finite_f64() {
        let _rug_st_tests_llm_16_465_llm_16_465_rrrruuuugggg_test_to_f32_with_finite_f64 = 0;
        let rug_fuzz_0 = 1234.5678_f64;
        let rug_fuzz_1 = 1234.5678_f32;
        let finite_f64 = rug_fuzz_0;
        let expected_f32 = Some(rug_fuzz_1);
        debug_assert_eq!(finite_f64.to_f32(), expected_f32);
        let _rug_ed_tests_llm_16_465_llm_16_465_rrrruuuugggg_test_to_f32_with_finite_f64 = 0;
    }
    #[test]
    fn test_to_f32_with_infinity() {
        let _rug_st_tests_llm_16_465_llm_16_465_rrrruuuugggg_test_to_f32_with_infinity = 0;
        let infinity_f64 = f64::INFINITY;
        debug_assert_eq!(infinity_f64.to_f32(), Some(f32::INFINITY));
        let _rug_ed_tests_llm_16_465_llm_16_465_rrrruuuugggg_test_to_f32_with_infinity = 0;
    }
    #[test]
    fn test_to_f32_with_negative_infinity() {
        let _rug_st_tests_llm_16_465_llm_16_465_rrrruuuugggg_test_to_f32_with_negative_infinity = 0;
        let neg_infinity_f64 = f64::NEG_INFINITY;
        debug_assert_eq!(neg_infinity_f64.to_f32(), Some(f32::NEG_INFINITY));
        let _rug_ed_tests_llm_16_465_llm_16_465_rrrruuuugggg_test_to_f32_with_negative_infinity = 0;
    }
    #[test]
    fn test_to_f32_with_nan() {
        let _rug_st_tests_llm_16_465_llm_16_465_rrrruuuugggg_test_to_f32_with_nan = 0;
        let nan_f64 = f64::NAN;
        debug_assert!(nan_f64.to_f32().unwrap().is_nan());
        let _rug_ed_tests_llm_16_465_llm_16_465_rrrruuuugggg_test_to_f32_with_nan = 0;
    }
    #[test]
    fn test_to_f32_with_max_value() {
        let _rug_st_tests_llm_16_465_llm_16_465_rrrruuuugggg_test_to_f32_with_max_value = 0;
        let max_f64 = f64::MAX;
        debug_assert_eq!(max_f64.to_f32(), Some(f32::INFINITY));
        let _rug_ed_tests_llm_16_465_llm_16_465_rrrruuuugggg_test_to_f32_with_max_value = 0;
    }
    #[test]
    fn test_to_f32_with_min_value() {
        let _rug_st_tests_llm_16_465_llm_16_465_rrrruuuugggg_test_to_f32_with_min_value = 0;
        let min_f64 = f64::MIN;
        debug_assert_eq!(min_f64.to_f32(), Some(f32::NEG_INFINITY));
        let _rug_ed_tests_llm_16_465_llm_16_465_rrrruuuugggg_test_to_f32_with_min_value = 0;
    }
    #[test]
    fn test_to_f32_with_zero() {
        let _rug_st_tests_llm_16_465_llm_16_465_rrrruuuugggg_test_to_f32_with_zero = 0;
        let rug_fuzz_0 = 0.0_f64;
        let zero_f64 = rug_fuzz_0;
        debug_assert_eq!(zero_f64.to_f32(), Some(0.0_f32));
        let _rug_ed_tests_llm_16_465_llm_16_465_rrrruuuugggg_test_to_f32_with_zero = 0;
    }
    #[test]
    fn test_to_f32_with_negative_zero() {
        let _rug_st_tests_llm_16_465_llm_16_465_rrrruuuugggg_test_to_f32_with_negative_zero = 0;
        let rug_fuzz_0 = 0.0_f64;
        let neg_zero_f64 = -rug_fuzz_0;
        debug_assert_eq!(neg_zero_f64.to_f32(), Some(- 0.0_f32));
        let _rug_ed_tests_llm_16_465_llm_16_465_rrrruuuugggg_test_to_f32_with_negative_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_466_llm_16_466 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_f64_with_f64() {
        let _rug_st_tests_llm_16_466_llm_16_466_rrrruuuugggg_test_to_f64_with_f64 = 0;
        let rug_fuzz_0 = 123.456f64;
        let num = rug_fuzz_0;
        let result = <f64 as ToPrimitive>::to_f64(&num);
        debug_assert_eq!(result, Some(123.456f64));
        let _rug_ed_tests_llm_16_466_llm_16_466_rrrruuuugggg_test_to_f64_with_f64 = 0;
    }
    #[test]
    fn test_to_f64_with_nan() {
        let _rug_st_tests_llm_16_466_llm_16_466_rrrruuuugggg_test_to_f64_with_nan = 0;
        let num = f64::NAN;
        let result = <f64 as ToPrimitive>::to_f64(&num);
        debug_assert!(result.unwrap().is_nan());
        let _rug_ed_tests_llm_16_466_llm_16_466_rrrruuuugggg_test_to_f64_with_nan = 0;
    }
    #[test]
    fn test_to_f64_with_infinity() {
        let _rug_st_tests_llm_16_466_llm_16_466_rrrruuuugggg_test_to_f64_with_infinity = 0;
        let num = f64::INFINITY;
        let result = <f64 as ToPrimitive>::to_f64(&num);
        debug_assert_eq!(result, Some(f64::INFINITY));
        let _rug_ed_tests_llm_16_466_llm_16_466_rrrruuuugggg_test_to_f64_with_infinity = 0;
    }
    #[test]
    fn test_to_f64_with_negative_infinity() {
        let _rug_st_tests_llm_16_466_llm_16_466_rrrruuuugggg_test_to_f64_with_negative_infinity = 0;
        let num = f64::NEG_INFINITY;
        let result = <f64 as ToPrimitive>::to_f64(&num);
        debug_assert_eq!(result, Some(f64::NEG_INFINITY));
        let _rug_ed_tests_llm_16_466_llm_16_466_rrrruuuugggg_test_to_f64_with_negative_infinity = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_467_llm_16_467 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_i128_within_bounds() {
        let _rug_st_tests_llm_16_467_llm_16_467_rrrruuuugggg_test_to_i128_within_bounds = 0;
        let rug_fuzz_0 = 1234.0;
        let f: f64 = rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_i128(& f), Some(1234_i128));
        let _rug_ed_tests_llm_16_467_llm_16_467_rrrruuuugggg_test_to_i128_within_bounds = 0;
    }
    #[test]
    fn test_to_i128_below_bounds() {
        let _rug_st_tests_llm_16_467_llm_16_467_rrrruuuugggg_test_to_i128_below_bounds = 0;
        let rug_fuzz_0 = 2.0;
        let f: f64 = (i128::MIN as f64) - rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_i128(& f), None);
        let _rug_ed_tests_llm_16_467_llm_16_467_rrrruuuugggg_test_to_i128_below_bounds = 0;
    }
    #[test]
    fn test_to_i128_above_bounds() {
        let _rug_st_tests_llm_16_467_llm_16_467_rrrruuuugggg_test_to_i128_above_bounds = 0;
        let rug_fuzz_0 = 2.0;
        let f: f64 = (i128::MAX as f64) + rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_i128(& f), None);
        let _rug_ed_tests_llm_16_467_llm_16_467_rrrruuuugggg_test_to_i128_above_bounds = 0;
    }
    #[test]
    fn test_to_i128_at_min_bound() {
        let _rug_st_tests_llm_16_467_llm_16_467_rrrruuuugggg_test_to_i128_at_min_bound = 0;
        let f: f64 = i128::MIN as f64;
        debug_assert_eq!(ToPrimitive::to_i128(& f), Some(i128::MIN));
        let _rug_ed_tests_llm_16_467_llm_16_467_rrrruuuugggg_test_to_i128_at_min_bound = 0;
    }
    #[test]
    fn test_to_i128_just_below_max_bound() {
        let _rug_st_tests_llm_16_467_llm_16_467_rrrruuuugggg_test_to_i128_just_below_max_bound = 0;
        let rug_fuzz_0 = 0.1;
        let f: f64 = i128::MAX as f64 - rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_i128(& f), Some(i128::MAX));
        let _rug_ed_tests_llm_16_467_llm_16_467_rrrruuuugggg_test_to_i128_just_below_max_bound = 0;
    }
    #[test]
    fn test_to_i128_at_max_bound() {
        let _rug_st_tests_llm_16_467_llm_16_467_rrrruuuugggg_test_to_i128_at_max_bound = 0;
        let f: f64 = i128::MAX as f64;
        debug_assert_eq!(ToPrimitive::to_i128(& f), Some(i128::MAX));
        let _rug_ed_tests_llm_16_467_llm_16_467_rrrruuuugggg_test_to_i128_at_max_bound = 0;
    }
    #[test]
    fn test_to_i128_negative() {
        let _rug_st_tests_llm_16_467_llm_16_467_rrrruuuugggg_test_to_i128_negative = 0;
        let rug_fuzz_0 = 1234.0;
        let f: f64 = -rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_i128(& f), Some(- 1234_i128));
        let _rug_ed_tests_llm_16_467_llm_16_467_rrrruuuugggg_test_to_i128_negative = 0;
    }
    #[test]
    fn test_to_i128_fractional() {
        let _rug_st_tests_llm_16_467_llm_16_467_rrrruuuugggg_test_to_i128_fractional = 0;
        let rug_fuzz_0 = 1234.5678;
        let f: f64 = rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_i128(& f), Some(1234_i128));
        let _rug_ed_tests_llm_16_467_llm_16_467_rrrruuuugggg_test_to_i128_fractional = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_468_llm_16_468 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_i16_with_f64() {
        let _rug_st_tests_llm_16_468_llm_16_468_rrrruuuugggg_test_to_i16_with_f64 = 0;
        let rug_fuzz_0 = 0.0f64;
        let rug_fuzz_1 = 1.0f64;
        let rug_fuzz_2 = 1.0f64;
        let rug_fuzz_3 = 1.0;
        let rug_fuzz_4 = 1.0;
        let rug_fuzz_5 = 0.1;
        let rug_fuzz_6 = 0.1;
        let rug_fuzz_7 = 0.99999999999999;
        let rug_fuzz_8 = 0.99999999999999;
        let rug_fuzz_9 = 32767.1f64;
        let rug_fuzz_10 = 32768.1f64;
        let rug_fuzz_11 = 32767.9f64;
        let rug_fuzz_12 = 32768.9f64;
        let rug_fuzz_13 = 32768.0f64;
        let rug_fuzz_14 = 32769.0f64;
        debug_assert_eq!(rug_fuzz_0.to_i16(), Some(0i16));
        debug_assert_eq!(rug_fuzz_1.to_i16(), Some(1i16));
        debug_assert_eq!((- rug_fuzz_2).to_i16(), Some(- 1i16));
        debug_assert_eq!((i16::MAX as f64).to_i16(), Some(i16::MAX));
        debug_assert_eq!((i16::MIN as f64).to_i16(), Some(i16::MIN));
        debug_assert_eq!((i16::MAX as f64 + rug_fuzz_3).to_i16(), None);
        debug_assert_eq!((i16::MIN as f64 - rug_fuzz_4).to_i16(), None);
        debug_assert_eq!(f64::NAN.to_i16(), None);
        debug_assert_eq!(f64::INFINITY.to_i16(), None);
        debug_assert_eq!(f64::NEG_INFINITY.to_i16(), None);
        debug_assert_eq!(((i16::MAX as f64) + rug_fuzz_5).to_i16(), None);
        debug_assert_eq!(((i16::MIN as f64) - rug_fuzz_6).to_i16(), None);
        debug_assert_eq!(((i16::MAX as f64) + rug_fuzz_7).to_i16(), Some(i16::MAX));
        debug_assert_eq!(((i16::MIN as f64) - rug_fuzz_8).to_i16(), Some(i16::MIN));
        debug_assert_eq!(rug_fuzz_9.to_i16(), Some(32767i16));
        debug_assert_eq!((- rug_fuzz_10).to_i16(), Some(- 32768i16));
        debug_assert_eq!(rug_fuzz_11.to_i16(), Some(32767i16));
        debug_assert_eq!((- rug_fuzz_12).to_i16(), Some(- 32768i16));
        debug_assert_eq!(rug_fuzz_13.to_i16(), None);
        debug_assert_eq!((- rug_fuzz_14).to_i16(), None);
        let _rug_ed_tests_llm_16_468_llm_16_468_rrrruuuugggg_test_to_i16_with_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_469_llm_16_469 {
    use crate::cast::ToPrimitive;
    #[test]
    fn to_i32_with_in_range_float() {
        let _rug_st_tests_llm_16_469_llm_16_469_rrrruuuugggg_to_i32_with_in_range_float = 0;
        let rug_fuzz_0 = 42.0;
        let value: f64 = rug_fuzz_0;
        debug_assert_eq!(value.to_i32(), Some(42_i32));
        let _rug_ed_tests_llm_16_469_llm_16_469_rrrruuuugggg_to_i32_with_in_range_float = 0;
    }
    #[test]
    fn to_i32_with_out_of_range_float() {
        let _rug_st_tests_llm_16_469_llm_16_469_rrrruuuugggg_to_i32_with_out_of_range_float = 0;
        let rug_fuzz_0 = 10.0;
        let value: f64 = f64::from(i32::MAX) + rug_fuzz_0;
        debug_assert_eq!(value.to_i32(), None);
        let _rug_ed_tests_llm_16_469_llm_16_469_rrrruuuugggg_to_i32_with_out_of_range_float = 0;
    }
    #[test]
    fn to_i32_with_small_out_of_range_float() {
        let _rug_st_tests_llm_16_469_llm_16_469_rrrruuuugggg_to_i32_with_small_out_of_range_float = 0;
        let rug_fuzz_0 = 10.0;
        let value: f64 = f64::from(i32::MIN) - rug_fuzz_0;
        debug_assert_eq!(value.to_i32(), None);
        let _rug_ed_tests_llm_16_469_llm_16_469_rrrruuuugggg_to_i32_with_small_out_of_range_float = 0;
    }
    #[test]
    fn to_i32_with_large_float() {
        let _rug_st_tests_llm_16_469_llm_16_469_rrrruuuugggg_to_i32_with_large_float = 0;
        let rug_fuzz_0 = 1.0e20;
        let value: f64 = rug_fuzz_0;
        debug_assert_eq!(value.to_i32(), None);
        let _rug_ed_tests_llm_16_469_llm_16_469_rrrruuuugggg_to_i32_with_large_float = 0;
    }
    #[test]
    fn to_i32_with_nans() {
        let _rug_st_tests_llm_16_469_llm_16_469_rrrruuuugggg_to_i32_with_nans = 0;
        let value: f64 = f64::NAN;
        debug_assert_eq!(value.to_i32(), None);
        let _rug_ed_tests_llm_16_469_llm_16_469_rrrruuuugggg_to_i32_with_nans = 0;
    }
    #[test]
    fn to_i32_with_infinities() {
        let _rug_st_tests_llm_16_469_llm_16_469_rrrruuuugggg_to_i32_with_infinities = 0;
        let pos_inf: f64 = f64::INFINITY;
        let neg_inf: f64 = f64::NEG_INFINITY;
        debug_assert_eq!(pos_inf.to_i32(), None);
        debug_assert_eq!(neg_inf.to_i32(), None);
        let _rug_ed_tests_llm_16_469_llm_16_469_rrrruuuugggg_to_i32_with_infinities = 0;
    }
    #[test]
    fn to_i32_with_rounding_down() {
        let _rug_st_tests_llm_16_469_llm_16_469_rrrruuuugggg_to_i32_with_rounding_down = 0;
        let rug_fuzz_0 = 42.99;
        let value: f64 = rug_fuzz_0;
        debug_assert_eq!(value.to_i32(), Some(42_i32));
        let _rug_ed_tests_llm_16_469_llm_16_469_rrrruuuugggg_to_i32_with_rounding_down = 0;
    }
    #[test]
    fn to_i32_with_rounding_up() {
        let _rug_st_tests_llm_16_469_llm_16_469_rrrruuuugggg_to_i32_with_rounding_up = 0;
        let rug_fuzz_0 = 42.99;
        let value: f64 = -rug_fuzz_0;
        debug_assert_eq!(value.to_i32(), Some(- 42_i32));
        let _rug_ed_tests_llm_16_469_llm_16_469_rrrruuuugggg_to_i32_with_rounding_up = 0;
    }
    #[test]
    fn to_i32_with_zero() {
        let _rug_st_tests_llm_16_469_llm_16_469_rrrruuuugggg_to_i32_with_zero = 0;
        let rug_fuzz_0 = 0.0;
        let value: f64 = rug_fuzz_0;
        debug_assert_eq!(value.to_i32(), Some(0_i32));
        let _rug_ed_tests_llm_16_469_llm_16_469_rrrruuuugggg_to_i32_with_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_470_llm_16_470 {
    use crate::cast::ToPrimitive;
    #[test]
    fn to_i64_within_bounds() {
        let _rug_st_tests_llm_16_470_llm_16_470_rrrruuuugggg_to_i64_within_bounds = 0;
        let rug_fuzz_0 = 0_i64;
        debug_assert_eq!(Some(rug_fuzz_0), 0f64.to_i64());
        debug_assert_eq!(Some(i64::MAX), (i64::MAX as f64).to_i64());
        debug_assert_eq!(Some(i64::MIN), (i64::MIN as f64).to_i64());
        let _rug_ed_tests_llm_16_470_llm_16_470_rrrruuuugggg_to_i64_within_bounds = 0;
    }
    #[test]
    fn to_i64_out_of_bounds() {
        let _rug_st_tests_llm_16_470_llm_16_470_rrrruuuugggg_to_i64_out_of_bounds = 0;
        debug_assert_eq!(None, (i64::MAX as f64 + 1.0).to_i64());
        debug_assert_eq!(None, (i64::MIN as f64 - 1.0).to_i64());
        let _rug_ed_tests_llm_16_470_llm_16_470_rrrruuuugggg_to_i64_out_of_bounds = 0;
    }
    #[test]
    fn to_i64_edge_cases() {
        let _rug_st_tests_llm_16_470_llm_16_470_rrrruuuugggg_to_i64_edge_cases = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 1;
        debug_assert_eq!(
            Some((i64::MAX - rug_fuzz_0) as i64), ((i64::MAX - 1) as f64).to_i64()
        );
        debug_assert_eq!(
            Some((i64::MIN + rug_fuzz_1) as i64), ((i64::MIN + 1) as f64).to_i64()
        );
        debug_assert_eq!(None, (i64::MAX as f64 + 0.5).to_i64());
        debug_assert_eq!(None, (i64::MIN as f64 - 0.5).to_i64());
        let _rug_ed_tests_llm_16_470_llm_16_470_rrrruuuugggg_to_i64_edge_cases = 0;
    }
    #[test]
    fn to_i64_nan_infinity() {
        let _rug_st_tests_llm_16_470_llm_16_470_rrrruuuugggg_to_i64_nan_infinity = 0;
        debug_assert_eq!(None, f64::NAN.to_i64());
        debug_assert_eq!(None, f64::INFINITY.to_i64());
        debug_assert_eq!(None, f64::NEG_INFINITY.to_i64());
        let _rug_ed_tests_llm_16_470_llm_16_470_rrrruuuugggg_to_i64_nan_infinity = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_471_llm_16_471 {
    use crate::cast::ToPrimitive;
    #[test]
    fn to_i8_in_range() {
        let _rug_st_tests_llm_16_471_llm_16_471_rrrruuuugggg_to_i8_in_range = 0;
        let rug_fuzz_0 = 127f64;
        let rug_fuzz_1 = 128f64;
        let num = rug_fuzz_0;
        debug_assert_eq!(num.to_i8(), Some(127i8));
        let num = -rug_fuzz_1;
        debug_assert_eq!(num.to_i8(), Some(- 128i8));
        let _rug_ed_tests_llm_16_471_llm_16_471_rrrruuuugggg_to_i8_in_range = 0;
    }
    #[test]
    fn to_i8_out_of_range() {
        let _rug_st_tests_llm_16_471_llm_16_471_rrrruuuugggg_to_i8_out_of_range = 0;
        let rug_fuzz_0 = 128f64;
        let rug_fuzz_1 = 129f64;
        let num = rug_fuzz_0;
        debug_assert_eq!(num.to_i8(), None);
        let num = -rug_fuzz_1;
        debug_assert_eq!(num.to_i8(), None);
        let _rug_ed_tests_llm_16_471_llm_16_471_rrrruuuugggg_to_i8_out_of_range = 0;
    }
    #[test]
    fn to_i8_edge_cases() {
        let _rug_st_tests_llm_16_471_llm_16_471_rrrruuuugggg_to_i8_edge_cases = 0;
        let rug_fuzz_0 = 127.999f64;
        let rug_fuzz_1 = 128.999f64;
        let num = rug_fuzz_0;
        debug_assert_eq!(num.to_i8(), Some(127i8));
        let num = -rug_fuzz_1;
        debug_assert_eq!(num.to_i8(), Some(- 128i8));
        let _rug_ed_tests_llm_16_471_llm_16_471_rrrruuuugggg_to_i8_edge_cases = 0;
    }
    #[test]
    fn to_i8_non_integer() {
        let _rug_st_tests_llm_16_471_llm_16_471_rrrruuuugggg_to_i8_non_integer = 0;
        let rug_fuzz_0 = 0.1f64;
        let rug_fuzz_1 = 0.1f64;
        let rug_fuzz_2 = 127.5f64;
        let rug_fuzz_3 = 128.5f64;
        let num = rug_fuzz_0;
        debug_assert_eq!(num.to_i8(), Some(0i8));
        let num = -rug_fuzz_1;
        debug_assert_eq!(num.to_i8(), Some(0i8));
        let num = rug_fuzz_2;
        debug_assert_eq!(num.to_i8(), Some(127i8));
        let num = -rug_fuzz_3;
        debug_assert_eq!(num.to_i8(), Some(- 128i8));
        let _rug_ed_tests_llm_16_471_llm_16_471_rrrruuuugggg_to_i8_non_integer = 0;
    }
    #[test]
    fn to_i8_exact_limits() {
        let _rug_st_tests_llm_16_471_llm_16_471_rrrruuuugggg_to_i8_exact_limits = 0;
        let rug_fuzz_0 = 1.0;
        let rug_fuzz_1 = 1.0;
        let num = (i8::MIN as f64) - rug_fuzz_0;
        debug_assert_eq!(num.to_i8(), None);
        let num = (i8::MAX as f64) + rug_fuzz_1;
        debug_assert_eq!(num.to_i8(), None);
        let _rug_ed_tests_llm_16_471_llm_16_471_rrrruuuugggg_to_i8_exact_limits = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_472_llm_16_472 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_isize_within_bounds() {
        let _rug_st_tests_llm_16_472_llm_16_472_rrrruuuugggg_test_to_isize_within_bounds = 0;
        let rug_fuzz_0 = 42.0f64;
        let rug_fuzz_1 = 42_isize;
        let num = rug_fuzz_0;
        let expected = Some(rug_fuzz_1);
        let result = num.to_isize();
        debug_assert_eq!(expected, result);
        let _rug_ed_tests_llm_16_472_llm_16_472_rrrruuuugggg_test_to_isize_within_bounds = 0;
    }
    #[test]
    fn test_to_isize_below_bounds() {
        let _rug_st_tests_llm_16_472_llm_16_472_rrrruuuugggg_test_to_isize_below_bounds = 0;
        let rug_fuzz_0 = 1.0;
        let num = (isize::MIN as f64) - rug_fuzz_0;
        let expected = None;
        let result = num.to_isize();
        debug_assert_eq!(expected, result);
        let _rug_ed_tests_llm_16_472_llm_16_472_rrrruuuugggg_test_to_isize_below_bounds = 0;
    }
    #[test]
    fn test_to_isize_above_bounds() {
        let _rug_st_tests_llm_16_472_llm_16_472_rrrruuuugggg_test_to_isize_above_bounds = 0;
        let rug_fuzz_0 = 1.0;
        let num = (isize::MAX as f64) + rug_fuzz_0;
        let expected = None;
        let result = num.to_isize();
        debug_assert_eq!(expected, result);
        let _rug_ed_tests_llm_16_472_llm_16_472_rrrruuuugggg_test_to_isize_above_bounds = 0;
    }
    #[test]
    fn test_to_isize_min_val() {
        let _rug_st_tests_llm_16_472_llm_16_472_rrrruuuugggg_test_to_isize_min_val = 0;
        let num = isize::MIN as f64;
        let expected = Some(isize::MIN);
        let result = num.to_isize();
        debug_assert_eq!(expected, result);
        let _rug_ed_tests_llm_16_472_llm_16_472_rrrruuuugggg_test_to_isize_min_val = 0;
    }
    #[test]
    fn test_to_isize_max_val() {
        let _rug_st_tests_llm_16_472_llm_16_472_rrrruuuugggg_test_to_isize_max_val = 0;
        let num = isize::MAX as f64;
        let expected = Some(isize::MAX);
        let result = num.to_isize();
        debug_assert_eq!(expected, result);
        let _rug_ed_tests_llm_16_472_llm_16_472_rrrruuuugggg_test_to_isize_max_val = 0;
    }
    #[test]
    fn test_to_isize_nan() {
        let _rug_st_tests_llm_16_472_llm_16_472_rrrruuuugggg_test_to_isize_nan = 0;
        let num = f64::NAN;
        let expected = None;
        let result = num.to_isize();
        debug_assert_eq!(expected, result);
        let _rug_ed_tests_llm_16_472_llm_16_472_rrrruuuugggg_test_to_isize_nan = 0;
    }
    #[test]
    fn test_to_isize_infinity() {
        let _rug_st_tests_llm_16_472_llm_16_472_rrrruuuugggg_test_to_isize_infinity = 0;
        let num = f64::INFINITY;
        let expected = None;
        let result = num.to_isize();
        debug_assert_eq!(expected, result);
        let _rug_ed_tests_llm_16_472_llm_16_472_rrrruuuugggg_test_to_isize_infinity = 0;
    }
    #[test]
    fn test_to_isize_negative_infinity() {
        let _rug_st_tests_llm_16_472_llm_16_472_rrrruuuugggg_test_to_isize_negative_infinity = 0;
        let num = f64::NEG_INFINITY;
        let expected = None;
        let result = num.to_isize();
        debug_assert_eq!(expected, result);
        let _rug_ed_tests_llm_16_472_llm_16_472_rrrruuuugggg_test_to_isize_negative_infinity = 0;
    }
    #[test]
    fn test_to_isize_fraction() {
        let _rug_st_tests_llm_16_472_llm_16_472_rrrruuuugggg_test_to_isize_fraction = 0;
        let rug_fuzz_0 = 42.5f64;
        let rug_fuzz_1 = 42_isize;
        let num = rug_fuzz_0;
        let expected = Some(rug_fuzz_1);
        let result = num.to_isize();
        debug_assert_eq!(expected, result);
        let _rug_ed_tests_llm_16_472_llm_16_472_rrrruuuugggg_test_to_isize_fraction = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_473_llm_16_473 {
    use super::*;
    use crate::*;
    #[test]
    fn test_to_u128_with_f64_within_range() {
        let _rug_st_tests_llm_16_473_llm_16_473_rrrruuuugggg_test_to_u128_with_f64_within_range = 0;
        let rug_fuzz_0 = 42.0;
        let value: f64 = rug_fuzz_0;
        debug_assert_eq!(< f64 as ToPrimitive > ::to_u128(& value), Some(42_u128));
        let _rug_ed_tests_llm_16_473_llm_16_473_rrrruuuugggg_test_to_u128_with_f64_within_range = 0;
    }
    #[test]
    fn test_to_u128_with_f64_below_range() {
        let _rug_st_tests_llm_16_473_llm_16_473_rrrruuuugggg_test_to_u128_with_f64_below_range = 0;
        let rug_fuzz_0 = 1.0;
        let value: f64 = -rug_fuzz_0;
        debug_assert_eq!(< f64 as ToPrimitive > ::to_u128(& value), None);
        let _rug_ed_tests_llm_16_473_llm_16_473_rrrruuuugggg_test_to_u128_with_f64_below_range = 0;
    }
    #[test]
    fn test_to_u128_with_f64_above_range() {
        let _rug_st_tests_llm_16_473_llm_16_473_rrrruuuugggg_test_to_u128_with_f64_above_range = 0;
        let rug_fuzz_0 = 1.0;
        let value: f64 = u128::MAX as f64 + rug_fuzz_0;
        debug_assert_eq!(< f64 as ToPrimitive > ::to_u128(& value), None);
        let _rug_ed_tests_llm_16_473_llm_16_473_rrrruuuugggg_test_to_u128_with_f64_above_range = 0;
    }
    #[test]
    fn test_to_u128_with_f64_at_upper_bound() {
        let _rug_st_tests_llm_16_473_llm_16_473_rrrruuuugggg_test_to_u128_with_f64_at_upper_bound = 0;
        let value: f64 = u128::MAX as f64;
        let result = <f64 as ToPrimitive>::to_u128(&value);
        debug_assert!(result.is_some() && result.unwrap() <= u128::MAX);
        let _rug_ed_tests_llm_16_473_llm_16_473_rrrruuuugggg_test_to_u128_with_f64_at_upper_bound = 0;
    }
    #[test]
    fn test_to_u128_with_f64_at_lower_bound() {
        let _rug_st_tests_llm_16_473_llm_16_473_rrrruuuugggg_test_to_u128_with_f64_at_lower_bound = 0;
        let rug_fuzz_0 = 0.0;
        let value: f64 = rug_fuzz_0;
        debug_assert_eq!(< f64 as ToPrimitive > ::to_u128(& value), Some(0));
        let _rug_ed_tests_llm_16_473_llm_16_473_rrrruuuugggg_test_to_u128_with_f64_at_lower_bound = 0;
    }
    #[test]
    fn test_to_u128_with_f64_with_fraction() {
        let _rug_st_tests_llm_16_473_llm_16_473_rrrruuuugggg_test_to_u128_with_f64_with_fraction = 0;
        let rug_fuzz_0 = 42.99999999999999;
        let value: f64 = rug_fuzz_0;
        debug_assert_eq!(< f64 as ToPrimitive > ::to_u128(& value), Some(42));
        let _rug_ed_tests_llm_16_473_llm_16_473_rrrruuuugggg_test_to_u128_with_f64_with_fraction = 0;
    }
    #[test]
    fn test_to_u128_with_f64_with_negative_fraction() {
        let _rug_st_tests_llm_16_473_llm_16_473_rrrruuuugggg_test_to_u128_with_f64_with_negative_fraction = 0;
        let rug_fuzz_0 = 0.99999999999999;
        let value: f64 = -rug_fuzz_0;
        debug_assert_eq!(< f64 as ToPrimitive > ::to_u128(& value), None);
        let _rug_ed_tests_llm_16_473_llm_16_473_rrrruuuugggg_test_to_u128_with_f64_with_negative_fraction = 0;
    }
    #[test]
    fn test_to_u128_with_f64_max_value() {
        let _rug_st_tests_llm_16_473_llm_16_473_rrrruuuugggg_test_to_u128_with_f64_max_value = 0;
        let value: f64 = u128::MAX as f64;
        debug_assert_eq!(< f64 as ToPrimitive > ::to_u128(& value), Some(u128::MAX));
        let _rug_ed_tests_llm_16_473_llm_16_473_rrrruuuugggg_test_to_u128_with_f64_max_value = 0;
    }
    #[test]
    fn test_to_u128_with_f64_infinity() {
        let _rug_st_tests_llm_16_473_llm_16_473_rrrruuuugggg_test_to_u128_with_f64_infinity = 0;
        debug_assert_eq!(< f64 as ToPrimitive > ::to_u128(& f64::INFINITY), None);
        let _rug_ed_tests_llm_16_473_llm_16_473_rrrruuuugggg_test_to_u128_with_f64_infinity = 0;
    }
    #[test]
    fn test_to_u128_with_f64_negative_infinity() {
        let _rug_st_tests_llm_16_473_llm_16_473_rrrruuuugggg_test_to_u128_with_f64_negative_infinity = 0;
        debug_assert_eq!(< f64 as ToPrimitive > ::to_u128(& f64::NEG_INFINITY), None);
        let _rug_ed_tests_llm_16_473_llm_16_473_rrrruuuugggg_test_to_u128_with_f64_negative_infinity = 0;
    }
    #[test]
    fn test_to_u128_with_f64_nan() {
        let _rug_st_tests_llm_16_473_llm_16_473_rrrruuuugggg_test_to_u128_with_f64_nan = 0;
        debug_assert_eq!(< f64 as ToPrimitive > ::to_u128(& f64::NAN), None);
        let _rug_ed_tests_llm_16_473_llm_16_473_rrrruuuugggg_test_to_u128_with_f64_nan = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_474_llm_16_474 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_f64_to_u16_within_bounds() {
        let _rug_st_tests_llm_16_474_llm_16_474_rrrruuuugggg_test_f64_to_u16_within_bounds = 0;
        let rug_fuzz_0 = 42.0f64;
        let num_within_bounds = rug_fuzz_0;
        debug_assert_eq!(num_within_bounds.to_u16(), Some(42u16));
        let _rug_ed_tests_llm_16_474_llm_16_474_rrrruuuugggg_test_f64_to_u16_within_bounds = 0;
    }
    #[test]
    fn test_f64_to_u16_below_lower_bound() {
        let _rug_st_tests_llm_16_474_llm_16_474_rrrruuuugggg_test_f64_to_u16_below_lower_bound = 0;
        let rug_fuzz_0 = 1.0f64;
        let num_below_lower_bound = -rug_fuzz_0;
        debug_assert_eq!(num_below_lower_bound.to_u16(), None);
        let _rug_ed_tests_llm_16_474_llm_16_474_rrrruuuugggg_test_f64_to_u16_below_lower_bound = 0;
    }
    #[test]
    fn test_f64_to_u16_above_upper_bound() {
        let _rug_st_tests_llm_16_474_llm_16_474_rrrruuuugggg_test_f64_to_u16_above_upper_bound = 0;
        let rug_fuzz_0 = 1.0;
        let num_above_upper_bound = (u16::MAX as f64) + rug_fuzz_0;
        debug_assert_eq!(num_above_upper_bound.to_u16(), None);
        let _rug_ed_tests_llm_16_474_llm_16_474_rrrruuuugggg_test_f64_to_u16_above_upper_bound = 0;
    }
    #[test]
    fn test_f64_to_u16_at_upper_bound() {
        let _rug_st_tests_llm_16_474_llm_16_474_rrrruuuugggg_test_f64_to_u16_at_upper_bound = 0;
        let num_at_upper_bound = u16::MAX as f64;
        debug_assert_eq!(num_at_upper_bound.to_u16(), Some(u16::MAX));
        let _rug_ed_tests_llm_16_474_llm_16_474_rrrruuuugggg_test_f64_to_u16_at_upper_bound = 0;
    }
    #[test]
    fn test_f64_to_u16_lower_bound_edge() {
        let _rug_st_tests_llm_16_474_llm_16_474_rrrruuuugggg_test_f64_to_u16_lower_bound_edge = 0;
        let rug_fuzz_0 = 0.0f64;
        let num_at_lower_edge = rug_fuzz_0;
        debug_assert_eq!(num_at_lower_edge.to_u16(), Some(0u16));
        let _rug_ed_tests_llm_16_474_llm_16_474_rrrruuuugggg_test_f64_to_u16_lower_bound_edge = 0;
    }
    #[test]
    fn test_f64_to_u16_precision_loss() {
        let _rug_st_tests_llm_16_474_llm_16_474_rrrruuuugggg_test_f64_to_u16_precision_loss = 0;
        let rug_fuzz_0 = 42.99999999999999f64;
        let num_with_precision_loss = rug_fuzz_0;
        debug_assert_eq!(num_with_precision_loss.to_u16(), Some(42u16));
        let _rug_ed_tests_llm_16_474_llm_16_474_rrrruuuugggg_test_f64_to_u16_precision_loss = 0;
    }
    #[test]
    fn test_f64_to_u16_infinity() {
        let _rug_st_tests_llm_16_474_llm_16_474_rrrruuuugggg_test_f64_to_u16_infinity = 0;
        let positive_infinity = f64::INFINITY;
        debug_assert_eq!(positive_infinity.to_u16(), None);
        let negative_infinity = f64::NEG_INFINITY;
        debug_assert_eq!(negative_infinity.to_u16(), None);
        let _rug_ed_tests_llm_16_474_llm_16_474_rrrruuuugggg_test_f64_to_u16_infinity = 0;
    }
    #[test]
    fn test_f64_to_u16_nan() {
        let _rug_st_tests_llm_16_474_llm_16_474_rrrruuuugggg_test_f64_to_u16_nan = 0;
        let nan = f64::NAN;
        debug_assert_eq!(nan.to_u16(), None);
        let _rug_ed_tests_llm_16_474_llm_16_474_rrrruuuugggg_test_f64_to_u16_nan = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_475_llm_16_475 {
    use crate::ToPrimitive;
    #[test]
    fn test_to_u32_with_f64() {
        let _rug_st_tests_llm_16_475_llm_16_475_rrrruuuugggg_test_to_u32_with_f64 = 0;
        let rug_fuzz_0 = 0f64;
        let rug_fuzz_1 = 1f64;
        let rug_fuzz_2 = 1.999f64;
        let rug_fuzz_3 = 1.0;
        let rug_fuzz_4 = 1000.0;
        let rug_fuzz_5 = 1.0f64;
        let rug_fuzz_6 = 1.1f64;
        let rug_fuzz_7 = 0.1f64;
        let rug_fuzz_8 = 0.1;
        let rug_fuzz_9 = 1.0;
        debug_assert_eq!(rug_fuzz_0.to_u32(), Some(0u32));
        debug_assert_eq!(rug_fuzz_1.to_u32(), Some(1u32));
        debug_assert_eq!(rug_fuzz_2.to_u32(), Some(1u32));
        debug_assert_eq!((u32::MAX as f64).to_u32(), Some(u32::MAX));
        debug_assert!((u32::MAX as f64 + rug_fuzz_3).to_u32().is_none());
        debug_assert!((u32::MAX as f64 + rug_fuzz_4).to_u32().is_none());
        debug_assert!((- rug_fuzz_5).to_u32().is_none());
        debug_assert!((- rug_fuzz_6).to_u32().is_none());
        debug_assert!((- rug_fuzz_7).to_u32().is_none());
        debug_assert!(((u32::MAX as f64) + rug_fuzz_8).to_u32().is_none());
        debug_assert_eq!(((u32::MAX as f64) - rug_fuzz_9).to_u32(), Some(u32::MAX - 1));
        let _rug_ed_tests_llm_16_475_llm_16_475_rrrruuuugggg_test_to_u32_with_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_476_llm_16_476 {
    use crate::ToPrimitive;
    #[test]
    fn to_u64_with_positive_float() {
        let _rug_st_tests_llm_16_476_llm_16_476_rrrruuuugggg_to_u64_with_positive_float = 0;
        let rug_fuzz_0 = 42.0f64;
        let f = rug_fuzz_0;
        let result = f.to_u64();
        debug_assert_eq!(result, Some(42u64));
        let _rug_ed_tests_llm_16_476_llm_16_476_rrrruuuugggg_to_u64_with_positive_float = 0;
    }
    #[test]
    fn to_u64_with_negative_float() {
        let _rug_st_tests_llm_16_476_llm_16_476_rrrruuuugggg_to_u64_with_negative_float = 0;
        let rug_fuzz_0 = 42.0f64;
        let f = -rug_fuzz_0;
        let result = f.to_u64();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_476_llm_16_476_rrrruuuugggg_to_u64_with_negative_float = 0;
    }
    #[test]
    fn to_u64_with_large_float() {
        let _rug_st_tests_llm_16_476_llm_16_476_rrrruuuugggg_to_u64_with_large_float = 0;
        let f = u64::MAX as f64;
        let result = f.to_u64();
        debug_assert_eq!(result, Some(u64::MAX));
        let _rug_ed_tests_llm_16_476_llm_16_476_rrrruuuugggg_to_u64_with_large_float = 0;
    }
    #[test]
    fn to_u64_with_large_float_out_of_range() {
        let _rug_st_tests_llm_16_476_llm_16_476_rrrruuuugggg_to_u64_with_large_float_out_of_range = 0;
        let rug_fuzz_0 = 1.0;
        let f = (u64::MAX as f64) + rug_fuzz_0;
        let result = f.to_u64();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_476_llm_16_476_rrrruuuugggg_to_u64_with_large_float_out_of_range = 0;
    }
    #[test]
    fn to_u64_with_max_float() {
        let _rug_st_tests_llm_16_476_llm_16_476_rrrruuuugggg_to_u64_with_max_float = 0;
        let rug_fuzz_0 = 0.9999999999999999;
        let f = (u64::MAX as f64) + rug_fuzz_0;
        let result = f.to_u64();
        debug_assert_eq!(result, Some(u64::MAX));
        let _rug_ed_tests_llm_16_476_llm_16_476_rrrruuuugggg_to_u64_with_max_float = 0;
    }
    #[test]
    fn to_u64_with_min_positive_subnormal() {
        let _rug_st_tests_llm_16_476_llm_16_476_rrrruuuugggg_to_u64_with_min_positive_subnormal = 0;
        let rug_fuzz_0 = 5e-324f64;
        let f = rug_fuzz_0;
        let result = f.to_u64();
        debug_assert_eq!(result, Some(0u64));
        let _rug_ed_tests_llm_16_476_llm_16_476_rrrruuuugggg_to_u64_with_min_positive_subnormal = 0;
    }
    #[test]
    fn to_u64_with_zero() {
        let _rug_st_tests_llm_16_476_llm_16_476_rrrruuuugggg_to_u64_with_zero = 0;
        let rug_fuzz_0 = 0.0f64;
        let f = rug_fuzz_0;
        let result = f.to_u64();
        debug_assert_eq!(result, Some(0u64));
        let _rug_ed_tests_llm_16_476_llm_16_476_rrrruuuugggg_to_u64_with_zero = 0;
    }
    #[test]
    fn to_u64_with_negative_subnormal() {
        let _rug_st_tests_llm_16_476_llm_16_476_rrrruuuugggg_to_u64_with_negative_subnormal = 0;
        let rug_fuzz_0 = 5e-324f64;
        let f = -rug_fuzz_0;
        let result = f.to_u64();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_476_llm_16_476_rrrruuuugggg_to_u64_with_negative_subnormal = 0;
    }
    #[test]
    fn to_u64_with_nan() {
        let _rug_st_tests_llm_16_476_llm_16_476_rrrruuuugggg_to_u64_with_nan = 0;
        let f = f64::NAN;
        let result = f.to_u64();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_476_llm_16_476_rrrruuuugggg_to_u64_with_nan = 0;
    }
    #[test]
    fn to_u64_with_infinity() {
        let _rug_st_tests_llm_16_476_llm_16_476_rrrruuuugggg_to_u64_with_infinity = 0;
        let f = f64::INFINITY;
        let result = f.to_u64();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_476_llm_16_476_rrrruuuugggg_to_u64_with_infinity = 0;
    }
    #[test]
    fn to_u64_with_neg_infinity() {
        let _rug_st_tests_llm_16_476_llm_16_476_rrrruuuugggg_to_u64_with_neg_infinity = 0;
        let f = f64::NEG_INFINITY;
        let result = f.to_u64();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_476_llm_16_476_rrrruuuugggg_to_u64_with_neg_infinity = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_477_llm_16_477 {
    use crate::cast::ToPrimitive;
    #[test]
    fn f64_to_u8() {
        let _rug_st_tests_llm_16_477_llm_16_477_rrrruuuugggg_f64_to_u8 = 0;
        let rug_fuzz_0 = 0.0f64;
        let rug_fuzz_1 = 0.999f64;
        let rug_fuzz_2 = 1.0f64;
        let rug_fuzz_3 = 1.999f64;
        let rug_fuzz_4 = 255.0f64;
        let rug_fuzz_5 = 255.999f64;
        let rug_fuzz_6 = 1.0f64;
        let rug_fuzz_7 = 0.999f64;
        let rug_fuzz_8 = 256.0f64;
        debug_assert_eq!(rug_fuzz_0.to_u8(), Some(0));
        debug_assert_eq!(rug_fuzz_1.to_u8(), Some(0));
        debug_assert_eq!(rug_fuzz_2.to_u8(), Some(1));
        debug_assert_eq!(rug_fuzz_3.to_u8(), Some(1));
        debug_assert_eq!(rug_fuzz_4.to_u8(), Some(255));
        debug_assert_eq!(rug_fuzz_5.to_u8(), Some(255));
        debug_assert_eq!((- rug_fuzz_6).to_u8(), None);
        debug_assert_eq!((- rug_fuzz_7).to_u8(), None);
        debug_assert_eq!(rug_fuzz_8.to_u8(), None);
        debug_assert_eq!(f64::MAX.to_u8(), None);
        debug_assert_eq!(f64::INFINITY.to_u8(), None);
        debug_assert_eq!(f64::NAN.to_u8(), None);
        let _rug_ed_tests_llm_16_477_llm_16_477_rrrruuuugggg_f64_to_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_478_llm_16_478 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_usize() {
        let _rug_st_tests_llm_16_478_llm_16_478_rrrruuuugggg_test_to_usize = 0;
        let rug_fuzz_0 = 0.0f64;
        let rug_fuzz_1 = 1.0f64;
        let rug_fuzz_2 = 1.5f64;
        let rug_fuzz_3 = 1.0f64;
        let rug_fuzz_4 = 1.0;
        let rug_fuzz_5 = 1.0;
        debug_assert_eq!(rug_fuzz_0.to_usize(), Some(0usize));
        debug_assert_eq!(rug_fuzz_1.to_usize(), Some(1usize));
        debug_assert_eq!(rug_fuzz_2.to_usize(), Some(1usize));
        debug_assert_eq!((- rug_fuzz_3).to_usize(), None);
        debug_assert_eq!(f64::MAX.to_usize(), None);
        debug_assert_eq!((usize::MAX as f64).to_usize(), Some(usize::MAX));
        debug_assert_eq!((usize::MAX as f64 + rug_fuzz_4).to_usize(), None);
        debug_assert_eq!(
            (usize::MAX as f64 - rug_fuzz_5).to_usize(), Some(usize::MAX - 1)
        );
        let _rug_ed_tests_llm_16_478_llm_16_478_rrrruuuugggg_test_to_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_602_llm_16_602 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_i128_as_f32() {
        let _rug_st_tests_llm_16_602_llm_16_602_rrrruuuugggg_test_i128_as_f32 = 0;
        let rug_fuzz_0 = 123456789012345678901234567890_i128;
        let rug_fuzz_1 = 123456789012345678901234567890_i128;
        let x: i128 = rug_fuzz_0;
        let y: f32 = <i128 as AsPrimitive<f32>>::as_(x);
        let expected: f32 = rug_fuzz_1 as f32;
        debug_assert_eq!(
            y, expected, "Casting i128 to f32 did not produce the expected result."
        );
        let _rug_ed_tests_llm_16_602_llm_16_602_rrrruuuugggg_test_i128_as_f32 = 0;
    }
    #[test]
    fn test_i128_as_f32_bounds() {
        let _rug_st_tests_llm_16_602_llm_16_602_rrrruuuugggg_test_i128_as_f32_bounds = 0;
        let max_i128: i128 = i128::MAX;
        let min_i128: i128 = i128::MIN;
        let max_as_f32: f32 = <i128 as AsPrimitive<f32>>::as_(max_i128);
        let min_as_f32: f32 = <i128 as AsPrimitive<f32>>::as_(min_i128);
        debug_assert!(
            max_as_f32.is_finite(),
            "Casting i128::MAX to f32 did not produce a finite value."
        );
        debug_assert!(
            min_as_f32.is_finite(),
            "Casting i128::MIN to f32 did not produce a finite value."
        );
        let _rug_ed_tests_llm_16_602_llm_16_602_rrrruuuugggg_test_i128_as_f32_bounds = 0;
    }
    #[test]
    fn test_i128_as_f32_precision() {
        let _rug_st_tests_llm_16_602_llm_16_602_rrrruuuugggg_test_i128_as_f32_precision = 0;
        let rug_fuzz_0 = 12345_i128;
        let rug_fuzz_1 = 12345_i128;
        let x: i128 = rug_fuzz_0;
        let y: f32 = <i128 as AsPrimitive<f32>>::as_(x);
        let expected: f32 = rug_fuzz_1 as f32;
        debug_assert_eq!(
            y, expected,
            "Casting i128 to f32 did not maintain precision for small values."
        );
        let _rug_ed_tests_llm_16_602_llm_16_602_rrrruuuugggg_test_i128_as_f32_precision = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_603_llm_16_603 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_i128_to_f64() {
        let _rug_st_tests_llm_16_603_llm_16_603_rrrruuuugggg_test_as_primitive_i128_to_f64 = 0;
        let rug_fuzz_0 = 123456789123456789123456789123456789;
        let rug_fuzz_1 = 1e-5;
        let value: i128 = rug_fuzz_0;
        let result: f64 = <i128 as AsPrimitive<f64>>::as_(value);
        let expected: f64 = value as f64;
        let acceptable_error = rug_fuzz_1 * expected;
        debug_assert!((result - expected).abs() < acceptable_error);
        let _rug_ed_tests_llm_16_603_llm_16_603_rrrruuuugggg_test_as_primitive_i128_to_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_604_llm_16_604 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_i128_to_i128() {
        let _rug_st_tests_llm_16_604_llm_16_604_rrrruuuugggg_test_as_primitive_i128_to_i128 = 0;
        let rug_fuzz_0 = 1234567890123456789i128;
        let value: i128 = rug_fuzz_0;
        let result: i128 = AsPrimitive::<i128>::as_(value);
        debug_assert_eq!(value, result);
        let _rug_ed_tests_llm_16_604_llm_16_604_rrrruuuugggg_test_as_primitive_i128_to_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_605_llm_16_605 {
    use crate::cast::AsPrimitive;
    #[test]
    fn as_i128_to_i16() {
        let _rug_st_tests_llm_16_605_llm_16_605_rrrruuuugggg_as_i128_to_i16 = 0;
        let rug_fuzz_0 = 0i128;
        let rug_fuzz_1 = 123456789i128;
        let value: i128 = i16::MAX as i128;
        let result: i16 = value.as_();
        debug_assert_eq!(result, i16::MAX);
        let value: i128 = i16::MIN as i128;
        let result: i16 = value.as_();
        debug_assert_eq!(result, i16::MIN);
        let value: i128 = rug_fuzz_0;
        let result: i16 = value.as_();
        debug_assert_eq!(result, 0);
        let value: i128 = rug_fuzz_1;
        let result: i16 = value.as_();
        debug_assert_eq!(result, 123456789i128 as i16);
        let _rug_ed_tests_llm_16_605_llm_16_605_rrrruuuugggg_as_i128_to_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_606_llm_16_606 {
    use crate::AsPrimitive;
    #[test]
    fn i128_as_i32() {
        let _rug_st_tests_llm_16_606_llm_16_606_rrrruuuugggg_i128_as_i32 = 0;
        let rug_fuzz_0 = 42;
        let value: i128 = i32::MAX as i128;
        let result: i32 = AsPrimitive::<i32>::as_(value);
        debug_assert_eq!(result, i32::MAX);
        let value: i128 = i32::MIN as i128;
        let result: i32 = AsPrimitive::<i32>::as_(value);
        debug_assert_eq!(result, i32::MIN);
        let value: i128 = rug_fuzz_0;
        let result: i32 = AsPrimitive::<i32>::as_(value);
        debug_assert_eq!(result, 42);
        let _rug_ed_tests_llm_16_606_llm_16_606_rrrruuuugggg_i128_as_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_607_llm_16_607 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_i128_to_i64() {
        let _rug_st_tests_llm_16_607_llm_16_607_rrrruuuugggg_test_as_primitive_i128_to_i64 = 0;
        let rug_fuzz_0 = 123456789012345678901234567890_i128;
        let value_i128: i128 = i64::MAX as i128;
        let value_i64: i64 = AsPrimitive::<i64>::as_(value_i128);
        debug_assert_eq!(value_i64, i64::MAX);
        let value_i128: i128 = i64::MIN as i128;
        let value_i64: i64 = AsPrimitive::<i64>::as_(value_i128);
        debug_assert_eq!(value_i64, i64::MIN);
        let value_i128: i128 = rug_fuzz_0;
        let value_i64: i64 = AsPrimitive::<i64>::as_(value_i128);
        debug_assert_eq!(value_i64, 123456789012345678901234567890_i128 as i64);
        let _rug_ed_tests_llm_16_607_llm_16_607_rrrruuuugggg_test_as_primitive_i128_to_i64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_608_llm_16_608 {
    use crate::cast::AsPrimitive;
    #[test]
    fn i128_as_i8() {
        let _rug_st_tests_llm_16_608_llm_16_608_rrrruuuugggg_i128_as_i8 = 0;
        let rug_fuzz_0 = 127;
        let rug_fuzz_1 = 128;
        let rug_fuzz_2 = 129;
        let x: i128 = rug_fuzz_0;
        let y: i8 = x.as_();
        debug_assert_eq!(y, 127i8);
        let x: i128 = rug_fuzz_1;
        let y: i8 = x.as_();
        debug_assert_eq!(y, - 128i8);
        let x: i128 = -rug_fuzz_2;
        let y: i8 = x.as_();
        debug_assert_eq!(y, 127i8);
        let _rug_ed_tests_llm_16_608_llm_16_608_rrrruuuugggg_i128_as_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_610_llm_16_610 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_i128_to_u128() {
        let _rug_st_tests_llm_16_610_llm_16_610_rrrruuuugggg_test_as_i128_to_u128 = 0;
        let rug_fuzz_0 = 42;
        let a: i128 = rug_fuzz_0;
        let b: u128 = AsPrimitive::<u128>::as_(a);
        debug_assert_eq!(b, 42u128);
        let c: i128 = i128::min_value();
        let result = std::panic::catch_unwind(|| AsPrimitive::<u128>::as_(c));
        debug_assert!(result.is_err(), "Casting negative i128 to u128 should panic");
        let _rug_ed_tests_llm_16_610_llm_16_610_rrrruuuugggg_test_as_i128_to_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_612 {
    use super::*;
    use crate::*;
    #[test]
    fn test_as_primitive_i128_to_u32() {
        let _rug_st_tests_llm_16_612_rrrruuuugggg_test_as_primitive_i128_to_u32 = 0;
        let rug_fuzz_0 = 0i128;
        let rug_fuzz_1 = 0u32;
        let values: Vec<i128> = vec![
            rug_fuzz_0, 1i128, i32::MAX as i128, i64::MAX as i128, u32::MAX as i128
        ];
        let expected: Vec<u32> = vec![
            rug_fuzz_1, 1u32, i32::MAX as u32, i64::MAX as u32, u32::MAX
        ];
        let results: Vec<u32> = values.iter().map(|&val| val.as_()).collect();
        debug_assert_eq!(results, expected);
        let _rug_ed_tests_llm_16_612_rrrruuuugggg_test_as_primitive_i128_to_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_613 {
    use super::*;
    use crate::*;
    use crate::cast::AsPrimitive;
    #[test]
    fn i128_to_u64_cast() {
        let _rug_st_tests_llm_16_613_rrrruuuugggg_i128_to_u64_cast = 0;
        let rug_fuzz_0 = 123;
        let val_i128: i128 = rug_fuzz_0;
        let val_u64: u64 = AsPrimitive::<u64>::as_(val_i128);
        debug_assert_eq!(val_u64, 123u64);
        let _rug_ed_tests_llm_16_613_rrrruuuugggg_i128_to_u64_cast = 0;
    }
    #[test]
    #[should_panic(expected = "attempt to cast to with overflow")]
    fn i128_to_u64_cast_overflow() {
        let _rug_st_tests_llm_16_613_rrrruuuugggg_i128_to_u64_cast_overflow = 0;
        let val_i128: i128 = i128::max_value();
        let _: u64 = AsPrimitive::<u64>::as_(val_i128);
        let _rug_ed_tests_llm_16_613_rrrruuuugggg_i128_to_u64_cast_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_614_llm_16_614 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_i128_to_u8() {
        let _rug_st_tests_llm_16_614_llm_16_614_rrrruuuugggg_test_as_primitive_i128_to_u8 = 0;
        let rug_fuzz_0 = 100;
        let rug_fuzz_1 = 100;
        let val_i128: i128 = rug_fuzz_0;
        let val_u8: u8 = val_i128.as_();
        debug_assert_eq!(val_u8, 100u8);
        let val_i128: i128 = -rug_fuzz_1;
        let val_u8: u8 = val_i128.as_();
        debug_assert_eq!(val_u8, 156u8);
        let val_i128: i128 = i128::MAX;
        let val_u8: u8 = val_i128.as_();
        debug_assert_eq!(val_u8, 255u8);
        let _rug_ed_tests_llm_16_614_llm_16_614_rrrruuuugggg_test_as_primitive_i128_to_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_615_llm_16_615 {
    use crate::AsPrimitive;
    #[test]
    fn test_as_primitive_i128_to_usize() {
        let _rug_st_tests_llm_16_615_llm_16_615_rrrruuuugggg_test_as_primitive_i128_to_usize = 0;
        let rug_fuzz_0 = 42;
        let value: i128 = rug_fuzz_0;
        let result = <i128 as AsPrimitive<usize>>::as_(value);
        debug_assert_eq!(result, 42_usize);
        let _rug_ed_tests_llm_16_615_llm_16_615_rrrruuuugggg_test_as_primitive_i128_to_usize = 0;
    }
    #[test]
    #[should_panic]
    fn test_as_primitive_i128_to_usize_overflow() {
        let _rug_st_tests_llm_16_615_llm_16_615_rrrruuuugggg_test_as_primitive_i128_to_usize_overflow = 0;
        let value: i128 = i128::MAX;
        let _result = <i128 as AsPrimitive<usize>>::as_(value);
        let _rug_ed_tests_llm_16_615_llm_16_615_rrrruuuugggg_test_as_primitive_i128_to_usize_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_616_llm_16_616 {
    use super::*;
    use crate::*;
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_f32_to_i128() {
        let _rug_st_tests_llm_16_616_llm_16_616_rrrruuuugggg_test_from_f32_to_i128 = 0;
        let rug_fuzz_0 = 0f32;
        let rug_fuzz_1 = 123.0f32;
        let rug_fuzz_2 = 123.0f32;
        debug_assert_eq!(< i128 as FromPrimitive > ::from_f32(rug_fuzz_0), Some(0i128));
        debug_assert_eq!(
            < i128 as FromPrimitive > ::from_f32(rug_fuzz_1), Some(123i128)
        );
        debug_assert_eq!(
            < i128 as FromPrimitive > ::from_f32(- rug_fuzz_2), Some(- 123i128)
        );
        debug_assert!(< i128 as FromPrimitive > ::from_f32(f32::MAX).is_none());
        debug_assert!(< i128 as FromPrimitive > ::from_f32(f32::MIN).is_none());
        debug_assert!(< i128 as FromPrimitive > ::from_f32(f32::NAN).is_none());
        debug_assert!(< i128 as FromPrimitive > ::from_f32(f32::INFINITY).is_none());
        debug_assert!(< i128 as FromPrimitive > ::from_f32(f32::NEG_INFINITY).is_none());
        let _rug_ed_tests_llm_16_616_llm_16_616_rrrruuuugggg_test_from_f32_to_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_617_llm_16_617 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_f64_to_i128() {
        let _rug_st_tests_llm_16_617_llm_16_617_rrrruuuugggg_test_from_f64_to_i128 = 0;
        let rug_fuzz_0 = 0.0_f64;
        let rug_fuzz_1 = 123.0_f64;
        let rug_fuzz_2 = 123.0_f64;
        debug_assert_eq!(< i128 as FromPrimitive > ::from_f64(rug_fuzz_0), Some(0_i128));
        debug_assert_eq!(
            < i128 as FromPrimitive > ::from_f64(rug_fuzz_1), Some(123_i128)
        );
        debug_assert_eq!(
            < i128 as FromPrimitive > ::from_f64(- rug_fuzz_2), Some(- 123_i128)
        );
        debug_assert_eq!(< i128 as FromPrimitive > ::from_f64(f64::MAX), None);
        debug_assert_eq!(< i128 as FromPrimitive > ::from_f64(f64::MIN), None);
        debug_assert_eq!(< i128 as FromPrimitive > ::from_f64(f64::INFINITY), None);
        debug_assert_eq!(< i128 as FromPrimitive > ::from_f64(f64::NEG_INFINITY), None);
        debug_assert_eq!(< i128 as FromPrimitive > ::from_f64(f64::NAN), None);
        let _rug_ed_tests_llm_16_617_llm_16_617_rrrruuuugggg_test_from_f64_to_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_618_llm_16_618 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i128() {
        let _rug_st_tests_llm_16_618_llm_16_618_rrrruuuugggg_test_from_i128 = 0;
        let rug_fuzz_0 = 0_i128;
        let rug_fuzz_1 = 42_i128;
        debug_assert_eq!(
            < i128 as FromPrimitive > ::from_i128(rug_fuzz_0), Some(0_i128)
        );
        debug_assert_eq!(
            < i128 as FromPrimitive > ::from_i128(rug_fuzz_1), Some(42_i128)
        );
        let _rug_ed_tests_llm_16_618_llm_16_618_rrrruuuugggg_test_from_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_619_llm_16_619 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i16() {
        let _rug_st_tests_llm_16_619_llm_16_619_rrrruuuugggg_test_from_i16 = 0;
        let rug_fuzz_0 = 0i16;
        let rug_fuzz_1 = 1i16;
        debug_assert_eq!(< i128 as FromPrimitive > ::from_i16(rug_fuzz_0), Some(0i128));
        debug_assert_eq!(
            < i128 as FromPrimitive > ::from_i16(- rug_fuzz_1), Some(- 1i128)
        );
        debug_assert_eq!(
            < i128 as FromPrimitive > ::from_i16(i16::MAX), Some(i16::MAX as i128)
        );
        debug_assert_eq!(
            < i128 as FromPrimitive > ::from_i16(i16::MIN), Some(i16::MIN as i128)
        );
        let _rug_ed_tests_llm_16_619_llm_16_619_rrrruuuugggg_test_from_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_620_llm_16_620 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i32_with_128_bit_integer() {
        let _rug_st_tests_llm_16_620_llm_16_620_rrrruuuugggg_test_from_i32_with_128_bit_integer = 0;
        let value_i32: i32 = i32::MAX;
        let converted = <i128 as FromPrimitive>::from_i32(value_i32);
        debug_assert_eq!(converted, Some(i128::from(value_i32)));
        let _rug_ed_tests_llm_16_620_llm_16_620_rrrruuuugggg_test_from_i32_with_128_bit_integer = 0;
    }
    #[test]
    fn test_from_i32_with_min_value() {
        let _rug_st_tests_llm_16_620_llm_16_620_rrrruuuugggg_test_from_i32_with_min_value = 0;
        let value_i32: i32 = i32::MIN;
        let converted = <i128 as FromPrimitive>::from_i32(value_i32);
        debug_assert_eq!(converted, Some(i128::from(value_i32)));
        let _rug_ed_tests_llm_16_620_llm_16_620_rrrruuuugggg_test_from_i32_with_min_value = 0;
    }
    #[test]
    fn test_from_i32_with_zero() {
        let _rug_st_tests_llm_16_620_llm_16_620_rrrruuuugggg_test_from_i32_with_zero = 0;
        let rug_fuzz_0 = 0;
        let value_i32: i32 = rug_fuzz_0;
        let converted = <i128 as FromPrimitive>::from_i32(value_i32);
        debug_assert_eq!(converted, Some(i128::from(value_i32)));
        let _rug_ed_tests_llm_16_620_llm_16_620_rrrruuuugggg_test_from_i32_with_zero = 0;
    }
    #[test]
    fn test_from_i32_with_positive_value() {
        let _rug_st_tests_llm_16_620_llm_16_620_rrrruuuugggg_test_from_i32_with_positive_value = 0;
        let rug_fuzz_0 = 123;
        let value_i32: i32 = rug_fuzz_0;
        let converted = <i128 as FromPrimitive>::from_i32(value_i32);
        debug_assert_eq!(converted, Some(i128::from(value_i32)));
        let _rug_ed_tests_llm_16_620_llm_16_620_rrrruuuugggg_test_from_i32_with_positive_value = 0;
    }
    #[test]
    fn test_from_i32_with_negative_value() {
        let _rug_st_tests_llm_16_620_llm_16_620_rrrruuuugggg_test_from_i32_with_negative_value = 0;
        let rug_fuzz_0 = 123;
        let value_i32: i32 = -rug_fuzz_0;
        let converted = <i128 as FromPrimitive>::from_i32(value_i32);
        debug_assert_eq!(converted, Some(i128::from(value_i32)));
        let _rug_ed_tests_llm_16_620_llm_16_620_rrrruuuugggg_test_from_i32_with_negative_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_621_llm_16_621 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i64() {
        let _rug_st_tests_llm_16_621_llm_16_621_rrrruuuugggg_test_from_i64 = 0;
        let rug_fuzz_0 = 0_i64;
        let rug_fuzz_1 = 0_i128;
        let rug_fuzz_2 = 1_i64;
        let rug_fuzz_3 = 1_i128;
        let rug_fuzz_4 = 1_i64;
        let rug_fuzz_5 = 1_i128;
        let test_cases = [
            (rug_fuzz_0, Some(rug_fuzz_1)),
            (rug_fuzz_2, Some(rug_fuzz_3)),
            (-rug_fuzz_4, Some(-rug_fuzz_5)),
            (i64::MAX, Some(i64::MAX as i128)),
            (i64::MIN, Some(i64::MIN as i128)),
        ];
        for &(input, expected) in test_cases.iter() {
            let result = i128::from_i64(input);
            debug_assert_eq!(result, expected, "from_i64({}) failed", input);
        }
        let _rug_ed_tests_llm_16_621_llm_16_621_rrrruuuugggg_test_from_i64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_622_llm_16_622 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i8() {
        assert_eq!(< i128 as FromPrimitive >::from_i8(0), Some(0i128));
        assert_eq!(< i128 as FromPrimitive >::from_i8(- 1), Some(- 1i128));
        assert_eq!(< i128 as FromPrimitive >::from_i8(127), Some(127i128));
        assert_eq!(< i128 as FromPrimitive >::from_i8(- 128), Some(- 128i128));
    }
}
#[cfg(test)]
mod tests_llm_16_623_llm_16_623 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_isize() {
        let _rug_st_tests_llm_16_623_llm_16_623_rrrruuuugggg_test_from_isize = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        debug_assert_eq!(
            < i128 as FromPrimitive > ::from_isize(rug_fuzz_0), Some(0i128)
        );
        debug_assert_eq!(
            < i128 as FromPrimitive > ::from_isize(- rug_fuzz_1), Some(- 1i128)
        );
        debug_assert_eq!(
            < i128 as FromPrimitive > ::from_isize(isize::MAX), Some(isize::MAX as i128)
        );
        debug_assert_eq!(
            < i128 as FromPrimitive > ::from_isize(isize::MIN), Some(isize::MIN as i128)
        );
        let _rug_ed_tests_llm_16_623_llm_16_623_rrrruuuugggg_test_from_isize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_624_llm_16_624 {
    use super::*;
    use crate::*;
    #[test]
    fn test_from_u128() {
        let _rug_st_tests_llm_16_624_llm_16_624_rrrruuuugggg_test_from_u128 = 0;
        let rug_fuzz_0 = 0_u128;
        let rug_fuzz_1 = 1;
        debug_assert_eq!(
            < i128 as FromPrimitive > ::from_u128(rug_fuzz_0), Some(0_i128)
        );
        debug_assert_eq!(
            < i128 as FromPrimitive > ::from_u128(u128::MAX), Some(i128::MAX)
        );
        debug_assert_eq!(
            < i128 as FromPrimitive > ::from_u128((i128::MAX as u128) + rug_fuzz_1), None
        );
        let _rug_ed_tests_llm_16_624_llm_16_624_rrrruuuugggg_test_from_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_625_llm_16_625 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u16() {
        let _rug_st_tests_llm_16_625_llm_16_625_rrrruuuugggg_test_from_u16 = 0;
        let rug_fuzz_0 = 0_u16;
        debug_assert_eq!(< i128 as FromPrimitive > ::from_u16(rug_fuzz_0), Some(0_i128));
        debug_assert_eq!(
            < i128 as FromPrimitive > ::from_u16(u16::MAX), Some(u16::MAX as i128)
        );
        let _rug_ed_tests_llm_16_625_llm_16_625_rrrruuuugggg_test_from_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_626_llm_16_626 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u32() {
        let _rug_st_tests_llm_16_626_llm_16_626_rrrruuuugggg_test_from_u32 = 0;
        let rug_fuzz_0 = 0_u32;
        let rug_fuzz_1 = 1_u32;
        let rug_fuzz_2 = 1;
        debug_assert_eq!(< i128 as FromPrimitive > ::from_u32(rug_fuzz_0), Some(0_i128));
        debug_assert_eq!(< i128 as FromPrimitive > ::from_u32(rug_fuzz_1), Some(1_i128));
        debug_assert_eq!(
            < i128 as FromPrimitive > ::from_u32(u32::MAX), Some(u32::MAX as i128)
        );
        debug_assert_eq!(
            < i128 as FromPrimitive > ::from_u32(u32::MAX - rug_fuzz_2), Some((u32::MAX -
            1) as i128)
        );
        let _rug_ed_tests_llm_16_626_llm_16_626_rrrruuuugggg_test_from_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_627_llm_16_627 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u64_with_i128() {
        let _rug_st_tests_llm_16_627_llm_16_627_rrrruuuugggg_test_from_u64_with_i128 = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let values: Vec<u64> = vec![rug_fuzz_0, 1, 1234567890, u64::MAX];
        for &val in &values {
            let result = <i128 as FromPrimitive>::from_u64(val);
            debug_assert_eq!(result, Some(val as i128));
        }
        debug_assert_eq!(< i128 as FromPrimitive > ::from_u64(rug_fuzz_1), Some(0i128));
        debug_assert_eq!(
            < i128 as FromPrimitive > ::from_u64(u64::MAX), Some(u64::MAX as i128)
        );
        let _rug_ed_tests_llm_16_627_llm_16_627_rrrruuuugggg_test_from_u64_with_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_628_llm_16_628 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u8() {
        let _rug_st_tests_llm_16_628_llm_16_628_rrrruuuugggg_test_from_u8 = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 123;
        let zero_u8: u8 = rug_fuzz_0;
        let max_u8: u8 = u8::MAX;
        if let Some(zero_i128) = i128::from_u8(zero_u8) {
            debug_assert_eq!(zero_i128, 0i128);
        } else {
            panic!("Failed to convert u8 to i128 for value 0");
        }
        if let Some(max_u8_i128) = i128::from_u8(max_u8) {
            debug_assert_eq!(max_u8_i128, u8::MAX as i128);
        } else {
            panic!("Failed to convert u8 to i128 for max value of u8");
        }
        let value_u8: u8 = rug_fuzz_1;
        if let Some(value_i128) = i128::from_u8(value_u8) {
            debug_assert_eq!(value_i128, 123i128);
        } else {
            panic!("Failed to convert u8 to i128 for value 123");
        }
        let _rug_ed_tests_llm_16_628_llm_16_628_rrrruuuugggg_test_from_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_629_llm_16_629 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_usize() {
        let _rug_st_tests_llm_16_629_llm_16_629_rrrruuuugggg_test_from_usize = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        debug_assert_eq!(
            < i128 as FromPrimitive > ::from_usize(rug_fuzz_0), Some(0i128)
        );
        debug_assert_eq!(
            < i128 as FromPrimitive > ::from_usize(usize::MAX), Some(usize::MAX as i128)
        );
        debug_assert_eq!(
            < i128 as FromPrimitive > ::from_usize(rug_fuzz_1), Some(1i128)
        );
        let _rug_ed_tests_llm_16_629_llm_16_629_rrrruuuugggg_test_from_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_630_llm_16_630 {
    use crate::cast::NumCast;
    use crate::ToPrimitive;
    use std::num::Wrapping;
    #[test]
    fn test_cast_from_wrapping() {
        let _rug_st_tests_llm_16_630_llm_16_630_rrrruuuugggg_test_cast_from_wrapping = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42;
        let num_i128: i128 = rug_fuzz_0;
        let num_wrapping = Wrapping(num_i128);
        let result = <i128 as NumCast>::from(num_wrapping);
        debug_assert_eq!(result, Some(num_i128));
        let num_i128: i128 = i128::max_value();
        let num_wrapping = Wrapping(num_i128);
        let result = <i128 as NumCast>::from(num_wrapping);
        debug_assert_eq!(result, Some(i128::max_value()));
        let num_i128: i128 = i128::min_value();
        let num_wrapping = Wrapping(num_i128);
        let result = <i128 as NumCast>::from(num_wrapping);
        debug_assert_eq!(result, Some(i128::min_value()));
        let num_u128: u128 = rug_fuzz_1;
        let num_wrapping = Wrapping(num_u128);
        let result = <i128 as NumCast>::from(num_wrapping);
        debug_assert_eq!(result, Some(num_u128 as i128));
        let num_u128: u128 = u128::max_value();
        let num_wrapping = Wrapping(num_u128);
        let result = <i128 as NumCast>::from(num_wrapping);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_630_llm_16_630_rrrruuuugggg_test_cast_from_wrapping = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_631_llm_16_631 {
    use crate::cast::ToPrimitive;
    #[test]
    fn i128_to_f32() {
        let _rug_st_tests_llm_16_631_llm_16_631_rrrruuuugggg_i128_to_f32 = 0;
        let rug_fuzz_0 = 0i128;
        let rug_fuzz_1 = 1i128;
        let rug_fuzz_2 = 1i128;
        debug_assert_eq!((rug_fuzz_0).to_f32(), Some(0.0f32));
        debug_assert_eq!((rug_fuzz_1).to_f32(), Some(1.0f32));
        debug_assert_eq!((- rug_fuzz_2).to_f32(), Some(- 1.0f32));
        debug_assert_eq!((i128::MAX).to_f32(), Some(i128::MAX as f32));
        debug_assert_eq!((i128::MIN).to_f32(), Some(i128::MIN as f32));
        debug_assert!((i128::MAX).to_f32().unwrap().is_infinite());
        debug_assert!((i128::MIN).to_f32().unwrap().is_infinite());
        let _rug_ed_tests_llm_16_631_llm_16_631_rrrruuuugggg_i128_to_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_632 {
    use super::*;
    use crate::*;
    #[test]
    fn test_i128_to_f64() {
        let _rug_st_tests_llm_16_632_rrrruuuugggg_test_i128_to_f64 = 0;
        let rug_fuzz_0 = 0i128;
        let rug_fuzz_1 = 1i128;
        let rug_fuzz_2 = 1i128;
        debug_assert_eq!((rug_fuzz_0).to_f64(), Some(0f64));
        debug_assert_eq!((rug_fuzz_1).to_f64(), Some(1f64));
        debug_assert_eq!((- rug_fuzz_2).to_f64(), Some(- 1f64));
        debug_assert_eq!((i128::MAX).to_f64(), Some(i128::MAX as f64));
        debug_assert_eq!((i128::MIN).to_f64(), Some(i128::MIN as f64));
        let _rug_ed_tests_llm_16_632_rrrruuuugggg_test_i128_to_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_634_llm_16_634 {
    use crate::ToPrimitive;
    #[test]
    fn to_i16_with_i128() {
        let _rug_st_tests_llm_16_634_llm_16_634_rrrruuuugggg_to_i16_with_i128 = 0;
        let rug_fuzz_0 = 0i128;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        debug_assert_eq!((rug_fuzz_0).to_i16(), Some(0i16));
        debug_assert_eq!((i16::MIN as i128).to_i16(), Some(i16::MIN));
        debug_assert_eq!((i16::MAX as i128).to_i16(), Some(i16::MAX));
        debug_assert_eq!(((i16::MAX as i128) + rug_fuzz_1).to_i16(), None);
        debug_assert_eq!(((i16::MIN as i128) - rug_fuzz_2).to_i16(), None);
        debug_assert_eq!((i128::MAX).to_i16(), None);
        debug_assert_eq!((i128::MIN).to_i16(), None);
        let _rug_ed_tests_llm_16_634_llm_16_634_rrrruuuugggg_to_i16_with_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_635_llm_16_635 {
    use crate::cast::ToPrimitive;
    #[test]
    fn to_i32_with_i128_within_bounds() {
        let _rug_st_tests_llm_16_635_llm_16_635_rrrruuuugggg_to_i32_with_i128_within_bounds = 0;
        let rug_fuzz_0 = 42i128;
        let within_bounds = rug_fuzz_0;
        debug_assert_eq!(within_bounds.to_i32(), Some(42i32));
        let _rug_ed_tests_llm_16_635_llm_16_635_rrrruuuugggg_to_i32_with_i128_within_bounds = 0;
    }
    #[test]
    fn to_i32_with_i128_below_bounds() {
        let _rug_st_tests_llm_16_635_llm_16_635_rrrruuuugggg_to_i32_with_i128_below_bounds = 0;
        let below_bounds = i128::MIN;
        debug_assert_eq!(below_bounds.to_i32(), None);
        let _rug_ed_tests_llm_16_635_llm_16_635_rrrruuuugggg_to_i32_with_i128_below_bounds = 0;
    }
    #[test]
    fn to_i32_with_i128_above_bounds() {
        let _rug_st_tests_llm_16_635_llm_16_635_rrrruuuugggg_to_i32_with_i128_above_bounds = 0;
        let above_bounds = i128::MAX;
        debug_assert_eq!(above_bounds.to_i32(), None);
        let _rug_ed_tests_llm_16_635_llm_16_635_rrrruuuugggg_to_i32_with_i128_above_bounds = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_636 {
    use super::*;
    use crate::*;
    #[test]
    fn to_i64_with_i128_within_i64_bounds() {
        let _rug_st_tests_llm_16_636_rrrruuuugggg_to_i64_with_i128_within_i64_bounds = 0;
        let value_within_bounds: i128 = i64::MAX as i128;
        debug_assert_eq!(value_within_bounds.to_i64(), Some(i64::MAX));
        let _rug_ed_tests_llm_16_636_rrrruuuugggg_to_i64_with_i128_within_i64_bounds = 0;
    }
    #[test]
    fn to_i64_with_i128_exceeding_i64_bounds() {
        let _rug_st_tests_llm_16_636_rrrruuuugggg_to_i64_with_i128_exceeding_i64_bounds = 0;
        let rug_fuzz_0 = 1;
        let value_exceeding_bounds: i128 = (i64::MAX as i128) + rug_fuzz_0;
        debug_assert_eq!(value_exceeding_bounds.to_i64(), None);
        let _rug_ed_tests_llm_16_636_rrrruuuugggg_to_i64_with_i128_exceeding_i64_bounds = 0;
    }
    #[test]
    fn to_i64_with_i128_within_negative_i64_bounds() {
        let _rug_st_tests_llm_16_636_rrrruuuugggg_to_i64_with_i128_within_negative_i64_bounds = 0;
        let value_within_bounds: i128 = i64::MIN as i128;
        debug_assert_eq!(value_within_bounds.to_i64(), Some(i64::MIN));
        let _rug_ed_tests_llm_16_636_rrrruuuugggg_to_i64_with_i128_within_negative_i64_bounds = 0;
    }
    #[test]
    fn to_i64_with_i128_exceeding_negative_i64_bounds() {
        let _rug_st_tests_llm_16_636_rrrruuuugggg_to_i64_with_i128_exceeding_negative_i64_bounds = 0;
        let rug_fuzz_0 = 1;
        let value_exceeding_bounds: i128 = (i64::MIN as i128) - rug_fuzz_0;
        debug_assert_eq!(value_exceeding_bounds.to_i64(), None);
        let _rug_ed_tests_llm_16_636_rrrruuuugggg_to_i64_with_i128_exceeding_negative_i64_bounds = 0;
    }
    #[test]
    fn to_i64_with_small_i128() {
        let _rug_st_tests_llm_16_636_rrrruuuugggg_to_i64_with_small_i128 = 0;
        let rug_fuzz_0 = 1;
        let value: i128 = rug_fuzz_0;
        debug_assert_eq!(value.to_i64(), Some(1));
        let _rug_ed_tests_llm_16_636_rrrruuuugggg_to_i64_with_small_i128 = 0;
    }
    #[test]
    fn to_i64_with_zero_i128() {
        let _rug_st_tests_llm_16_636_rrrruuuugggg_to_i64_with_zero_i128 = 0;
        let rug_fuzz_0 = 0;
        let value: i128 = rug_fuzz_0;
        debug_assert_eq!(value.to_i64(), Some(0));
        let _rug_ed_tests_llm_16_636_rrrruuuugggg_to_i64_with_zero_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_637 {
    use crate::ToPrimitive;
    #[test]
    fn test_to_i8_with_i128() {
        let _rug_st_tests_llm_16_637_rrrruuuugggg_test_to_i8_with_i128 = 0;
        let rug_fuzz_0 = 0_i128;
        let rug_fuzz_1 = 127_i128;
        let rug_fuzz_2 = 128_i128;
        let rug_fuzz_3 = 128_i128;
        let rug_fuzz_4 = 129_i128;
        let rug_fuzz_5 = 100_i128;
        let rug_fuzz_6 = 100_i128;
        debug_assert_eq!(rug_fuzz_0.to_i8(), Some(0_i8));
        debug_assert_eq!(rug_fuzz_1.to_i8(), Some(127_i8));
        debug_assert_eq!((- rug_fuzz_2).to_i8(), Some(- 128_i8));
        debug_assert_eq!(rug_fuzz_3.to_i8(), None);
        debug_assert_eq!((- rug_fuzz_4).to_i8(), None);
        debug_assert_eq!(i128::MAX.to_i8(), None);
        debug_assert_eq!(i128::MIN.to_i8(), None);
        debug_assert_eq!(rug_fuzz_5.to_i8(), Some(100_i8));
        debug_assert_eq!((- rug_fuzz_6).to_i8(), Some(- 100_i8));
        let _rug_ed_tests_llm_16_637_rrrruuuugggg_test_to_i8_with_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_638_llm_16_638 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_isize_within_bounds() {
        let _rug_st_tests_llm_16_638_llm_16_638_rrrruuuugggg_test_to_isize_within_bounds = 0;
        let rug_fuzz_0 = 0i128;
        let rug_fuzz_1 = 1i128;
        let rug_fuzz_2 = 1i128;
        let min_isize: i128 = i128::from(isize::MIN as i128);
        let max_isize: i128 = i128::from(isize::MAX as i128);
        debug_assert_eq!(min_isize.to_isize(), Some(isize::MIN));
        debug_assert_eq!(max_isize.to_isize(), Some(isize::MAX));
        debug_assert_eq!(rug_fuzz_0.to_isize(), Some(0));
        debug_assert_eq!(rug_fuzz_1.to_isize(), Some(1));
        debug_assert_eq!((- rug_fuzz_2).to_isize(), Some(- 1));
        let _rug_ed_tests_llm_16_638_llm_16_638_rrrruuuugggg_test_to_isize_within_bounds = 0;
    }
    #[test]
    fn test_to_isize_out_of_bounds() {
        let _rug_st_tests_llm_16_638_llm_16_638_rrrruuuugggg_test_to_isize_out_of_bounds = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 1;
        let above_max: i128 = (isize::MAX as i128) + rug_fuzz_0;
        let below_min: i128 = (isize::MIN as i128) - rug_fuzz_1;
        debug_assert_eq!(above_max.to_isize(), None);
        debug_assert_eq!(below_min.to_isize(), None);
        let _rug_ed_tests_llm_16_638_llm_16_638_rrrruuuugggg_test_to_isize_out_of_bounds = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_639_llm_16_639 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_u128_with_i128() {
        let _rug_st_tests_llm_16_639_llm_16_639_rrrruuuugggg_test_to_u128_with_i128 = 0;
        let rug_fuzz_0 = 123i128;
        let rug_fuzz_1 = 0i128;
        let rug_fuzz_2 = 123i128;
        let rug_fuzz_3 = 0;
        let positive = rug_fuzz_0;
        debug_assert_eq!(positive.to_u128(), Some(123u128));
        let zero = rug_fuzz_1;
        debug_assert_eq!(zero.to_u128(), Some(0u128));
        let negative = -rug_fuzz_2;
        debug_assert_eq!(negative.to_u128(), None);
        let max_i128 = i128::MAX;
        debug_assert_eq!(max_i128.to_u128(), Some(i128::MAX as u128));
        let min_i128 = i128::MIN;
        debug_assert_eq!(min_i128.to_u128(), None);
        let max_u128_as_i128 = u128::MAX as i128;
        if max_u128_as_i128 >= rug_fuzz_3 {
            debug_assert_eq!(max_u128_as_i128.to_u128(), Some(u128::MAX));
        } else {
            debug_assert_eq!(max_u128_as_i128.to_u128(), None);
        }
        let _rug_ed_tests_llm_16_639_llm_16_639_rrrruuuugggg_test_to_u128_with_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_640 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_u16_with_i128() {
        let _rug_st_tests_llm_16_640_rrrruuuugggg_test_to_u16_with_i128 = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let min_value = i128::MIN;
        let max_value = i128::MAX;
        let zero: i128 = rug_fuzz_0;
        let positive_within_u16: i128 = u16::MAX.into();
        let negative_one: i128 = -rug_fuzz_1;
        debug_assert_eq!(min_value.to_u16(), None);
        debug_assert_eq!(max_value.to_u16(), None);
        debug_assert_eq!(zero.to_u16(), Some(0));
        debug_assert_eq!(positive_within_u16.to_u16(), Some(u16::MAX));
        debug_assert_eq!(negative_one.to_u16(), None);
        let _rug_ed_tests_llm_16_640_rrrruuuugggg_test_to_u16_with_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_641_llm_16_641 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_i128_to_u32() {
        let _rug_st_tests_llm_16_641_llm_16_641_rrrruuuugggg_test_i128_to_u32 = 0;
        let rug_fuzz_0 = 123i128;
        let rug_fuzz_1 = 1i128;
        let rug_fuzz_2 = 0i128;
        let small_pos_i128 = rug_fuzz_0;
        let big_pos_i128 = i128::MAX;
        let small_neg_i128 = -rug_fuzz_1;
        let zero_i128 = rug_fuzz_2;
        debug_assert_eq!(small_pos_i128.to_u32(), Some(123u32));
        debug_assert_eq!(big_pos_i128.to_u32(), None);
        debug_assert_eq!(small_neg_i128.to_u32(), None);
        debug_assert_eq!(zero_i128.to_u32(), Some(0u32));
        let _rug_ed_tests_llm_16_641_llm_16_641_rrrruuuugggg_test_i128_to_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_642_llm_16_642 {
    use crate::cast::ToPrimitive;
    #[test]
    fn to_u64_with_positive_i128() {
        let _rug_st_tests_llm_16_642_llm_16_642_rrrruuuugggg_to_u64_with_positive_i128 = 0;
        let rug_fuzz_0 = 123;
        let value: i128 = rug_fuzz_0;
        let result = ToPrimitive::to_u64(&value);
        debug_assert_eq!(result, Some(123u64));
        let _rug_ed_tests_llm_16_642_llm_16_642_rrrruuuugggg_to_u64_with_positive_i128 = 0;
    }
    #[test]
    fn to_u64_with_negative_i128() {
        let _rug_st_tests_llm_16_642_llm_16_642_rrrruuuugggg_to_u64_with_negative_i128 = 0;
        let rug_fuzz_0 = 123;
        let value: i128 = -rug_fuzz_0;
        let result = ToPrimitive::to_u64(&value);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_642_llm_16_642_rrrruuuugggg_to_u64_with_negative_i128 = 0;
    }
    #[test]
    fn to_u64_with_i128_exceeding_u64() {
        let _rug_st_tests_llm_16_642_llm_16_642_rrrruuuugggg_to_u64_with_i128_exceeding_u64 = 0;
        let rug_fuzz_0 = 1;
        let value: i128 = u64::MAX as i128 + rug_fuzz_0;
        let result = ToPrimitive::to_u64(&value);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_642_llm_16_642_rrrruuuugggg_to_u64_with_i128_exceeding_u64 = 0;
    }
    #[test]
    fn to_u64_with_i128_within_u64_range() {
        let _rug_st_tests_llm_16_642_llm_16_642_rrrruuuugggg_to_u64_with_i128_within_u64_range = 0;
        let value: i128 = u64::MAX as i128;
        let result = ToPrimitive::to_u64(&value);
        debug_assert_eq!(result, Some(u64::MAX));
        let _rug_ed_tests_llm_16_642_llm_16_642_rrrruuuugggg_to_u64_with_i128_within_u64_range = 0;
    }
    #[test]
    fn to_u64_with_i128_at_edge_of_negative() {
        let _rug_st_tests_llm_16_642_llm_16_642_rrrruuuugggg_to_u64_with_i128_at_edge_of_negative = 0;
        let rug_fuzz_0 = 0;
        let value: i128 = rug_fuzz_0;
        let result = ToPrimitive::to_u64(&value);
        debug_assert_eq!(result, Some(0u64));
        let _rug_ed_tests_llm_16_642_llm_16_642_rrrruuuugggg_to_u64_with_i128_at_edge_of_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_643_llm_16_643 {
    use super::*;
    use crate::*;
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_u8_with_i128() {
        let _rug_st_tests_llm_16_643_llm_16_643_rrrruuuugggg_test_to_u8_with_i128 = 0;
        let rug_fuzz_0 = 100;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let x: i128 = rug_fuzz_0;
        debug_assert_eq!(x.to_u8(), Some(100_u8));
        let x: i128 = u8::MAX as i128;
        debug_assert_eq!(x.to_u8(), Some(u8::MAX));
        let x: i128 = -rug_fuzz_1;
        debug_assert_eq!(x.to_u8(), None);
        let x: i128 = (u8::MAX as i128) + rug_fuzz_2;
        debug_assert_eq!(x.to_u8(), None);
        let _rug_ed_tests_llm_16_643_llm_16_643_rrrruuuugggg_test_to_u8_with_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_712_llm_16_712 {
    use crate::AsPrimitive;
    #[test]
    fn test_as_primitive_i16_to_f32() {
        let _rug_st_tests_llm_16_712_llm_16_712_rrrruuuugggg_test_as_primitive_i16_to_f32 = 0;
        let rug_fuzz_0 = 42;
        let value: i16 = rug_fuzz_0;
        let result: f32 = AsPrimitive::<f32>::as_(value);
        debug_assert_eq!(result, 42f32);
        let _rug_ed_tests_llm_16_712_llm_16_712_rrrruuuugggg_test_as_primitive_i16_to_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_713_llm_16_713 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_i16_to_f64() {
        let _rug_st_tests_llm_16_713_llm_16_713_rrrruuuugggg_test_as_i16_to_f64 = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42.0;
        let val: i16 = rug_fuzz_0;
        let result: f64 = val.as_();
        let expected: f64 = rug_fuzz_1;
        debug_assert_eq!(
            result, expected, "Casting i16 to f64 did not match expected value"
        );
        let _rug_ed_tests_llm_16_713_llm_16_713_rrrruuuugggg_test_as_i16_to_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_714_llm_16_714 {
    use crate::AsPrimitive;
    #[test]
    fn i16_as_i128() {
        let _rug_st_tests_llm_16_714_llm_16_714_rrrruuuugggg_i16_as_i128 = 0;
        let rug_fuzz_0 = 123;
        let value: i16 = rug_fuzz_0;
        let result: i128 = AsPrimitive::<i128>::as_(value);
        debug_assert_eq!(result, 123i128);
        let _rug_ed_tests_llm_16_714_llm_16_714_rrrruuuugggg_i16_as_i128 = 0;
    }
    #[test]
    fn negative_i16_as_i128() {
        let _rug_st_tests_llm_16_714_llm_16_714_rrrruuuugggg_negative_i16_as_i128 = 0;
        let rug_fuzz_0 = 123;
        let value: i16 = -rug_fuzz_0;
        let result: i128 = AsPrimitive::<i128>::as_(value);
        debug_assert_eq!(result, - 123i128);
        let _rug_ed_tests_llm_16_714_llm_16_714_rrrruuuugggg_negative_i16_as_i128 = 0;
    }
    #[test]
    fn i16_as_i128_max_value() {
        let _rug_st_tests_llm_16_714_llm_16_714_rrrruuuugggg_i16_as_i128_max_value = 0;
        let value: i16 = i16::MAX;
        let result: i128 = AsPrimitive::<i128>::as_(value);
        debug_assert_eq!(result, i16::MAX as i128);
        let _rug_ed_tests_llm_16_714_llm_16_714_rrrruuuugggg_i16_as_i128_max_value = 0;
    }
    #[test]
    fn i16_as_i128_min_value() {
        let _rug_st_tests_llm_16_714_llm_16_714_rrrruuuugggg_i16_as_i128_min_value = 0;
        let value: i16 = i16::MIN;
        let result: i128 = AsPrimitive::<i128>::as_(value);
        debug_assert_eq!(result, i16::MIN as i128);
        let _rug_ed_tests_llm_16_714_llm_16_714_rrrruuuugggg_i16_as_i128_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_715_llm_16_715 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_i16_to_i16() {
        let _rug_st_tests_llm_16_715_llm_16_715_rrrruuuugggg_test_as_primitive_i16_to_i16 = 0;
        let rug_fuzz_0 = 42;
        let value: i16 = rug_fuzz_0;
        let result: i16 = <i16 as AsPrimitive<i16>>::as_(value);
        debug_assert_eq!(result, 42);
        let _rug_ed_tests_llm_16_715_llm_16_715_rrrruuuugggg_test_as_primitive_i16_to_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_716_llm_16_716 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_i16_to_i32() {
        let _rug_st_tests_llm_16_716_llm_16_716_rrrruuuugggg_test_as_primitive_i16_to_i32 = 0;
        let rug_fuzz_0 = 123;
        let value: i16 = rug_fuzz_0;
        let result: i32 = AsPrimitive::<i32>::as_(value);
        debug_assert_eq!(result, 123i32);
        let _rug_ed_tests_llm_16_716_llm_16_716_rrrruuuugggg_test_as_primitive_i16_to_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_717_llm_16_717 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_i16_to_i64() {
        let _rug_st_tests_llm_16_717_llm_16_717_rrrruuuugggg_test_as_primitive_i16_to_i64 = 0;
        let rug_fuzz_0 = 123;
        let value: i16 = rug_fuzz_0;
        let result: i64 = AsPrimitive::<i64>::as_(value);
        debug_assert_eq!(result, 123i64);
        let _rug_ed_tests_llm_16_717_llm_16_717_rrrruuuugggg_test_as_primitive_i16_to_i64 = 0;
    }
    #[test]
    fn test_as_primitive_i16_to_i64_negative() {
        let _rug_st_tests_llm_16_717_llm_16_717_rrrruuuugggg_test_as_primitive_i16_to_i64_negative = 0;
        let rug_fuzz_0 = 123;
        let value: i16 = -rug_fuzz_0;
        let result: i64 = AsPrimitive::<i64>::as_(value);
        debug_assert_eq!(result, - 123i64);
        let _rug_ed_tests_llm_16_717_llm_16_717_rrrruuuugggg_test_as_primitive_i16_to_i64_negative = 0;
    }
    #[test]
    fn test_as_primitive_i16_to_i64_zero() {
        let _rug_st_tests_llm_16_717_llm_16_717_rrrruuuugggg_test_as_primitive_i16_to_i64_zero = 0;
        let rug_fuzz_0 = 0;
        let value: i16 = rug_fuzz_0;
        let result: i64 = AsPrimitive::<i64>::as_(value);
        debug_assert_eq!(result, 0i64);
        let _rug_ed_tests_llm_16_717_llm_16_717_rrrruuuugggg_test_as_primitive_i16_to_i64_zero = 0;
    }
    #[test]
    fn test_as_primitive_i16_to_i64_max() {
        let _rug_st_tests_llm_16_717_llm_16_717_rrrruuuugggg_test_as_primitive_i16_to_i64_max = 0;
        let value: i16 = i16::MAX;
        let result: i64 = AsPrimitive::<i64>::as_(value);
        debug_assert_eq!(result, i16::MAX as i64);
        let _rug_ed_tests_llm_16_717_llm_16_717_rrrruuuugggg_test_as_primitive_i16_to_i64_max = 0;
    }
    #[test]
    fn test_as_primitive_i16_to_i64_min() {
        let _rug_st_tests_llm_16_717_llm_16_717_rrrruuuugggg_test_as_primitive_i16_to_i64_min = 0;
        let value: i16 = i16::MIN;
        let result: i64 = AsPrimitive::<i64>::as_(value);
        debug_assert_eq!(result, i16::MIN as i64);
        let _rug_ed_tests_llm_16_717_llm_16_717_rrrruuuugggg_test_as_primitive_i16_to_i64_min = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_718_llm_16_718 {
    use crate::cast::AsPrimitive;
    #[test]
    fn i16_as_i8_cast() {
        let _rug_st_tests_llm_16_718_llm_16_718_rrrruuuugggg_i16_as_i8_cast = 0;
        let rug_fuzz_0 = 100;
        let rug_fuzz_1 = 100;
        let rug_fuzz_2 = 300;
        let val_i16: i16 = rug_fuzz_0;
        let val_i8: i8 = AsPrimitive::<i8>::as_(val_i16);
        debug_assert_eq!(val_i8, 100i8);
        let val_i16_negative: i16 = -rug_fuzz_1;
        let val_i8_negative: i8 = AsPrimitive::<i8>::as_(val_i16_negative);
        debug_assert_eq!(val_i8_negative, - 100i8);
        let val_i16_overflow: i16 = rug_fuzz_2;
        let val_i8_overflow: i8 = AsPrimitive::<i8>::as_(val_i16_overflow);
        let _rug_ed_tests_llm_16_718_llm_16_718_rrrruuuugggg_i16_as_i8_cast = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_719_llm_16_719 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive() {
        let _rug_st_tests_llm_16_719_llm_16_719_rrrruuuugggg_test_as_primitive = 0;
        let rug_fuzz_0 = 42;
        let value: i16 = rug_fuzz_0;
        let result: isize = value.as_();
        debug_assert_eq!(result, 42isize);
        let _rug_ed_tests_llm_16_719_llm_16_719_rrrruuuugggg_test_as_primitive = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_720_llm_16_720 {
    use crate::cast::AsPrimitive;
    #[test]
    fn i16_as_u128() {
        let _rug_st_tests_llm_16_720_llm_16_720_rrrruuuugggg_i16_as_u128 = 0;
        let rug_fuzz_0 = 123;
        let value: i16 = rug_fuzz_0;
        let result: u128 = AsPrimitive::<u128>::as_(value);
        debug_assert_eq!(result, 123u128);
        let _rug_ed_tests_llm_16_720_llm_16_720_rrrruuuugggg_i16_as_u128 = 0;
    }
    #[test]
    #[should_panic(expected = "attempt to cast negative integer to unsigned integer")]
    fn negative_i16_as_u128() {
        let _rug_st_tests_llm_16_720_llm_16_720_rrrruuuugggg_negative_i16_as_u128 = 0;
        let rug_fuzz_0 = 123;
        let value: i16 = -rug_fuzz_0;
        let result: u128 = AsPrimitive::<u128>::as_(value);
        debug_assert_eq!(result, u128::MAX - 122);
        let _rug_ed_tests_llm_16_720_llm_16_720_rrrruuuugggg_negative_i16_as_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_721_llm_16_721 {
    use crate::cast::AsPrimitive;
    #[test]
    fn i16_as_u16() {
        let _rug_st_tests_llm_16_721_llm_16_721_rrrruuuugggg_i16_as_u16 = 0;
        let rug_fuzz_0 = 42;
        let value_i16: i16 = rug_fuzz_0;
        let value_u16: u16 = AsPrimitive::<u16>::as_(value_i16);
        debug_assert_eq!(value_u16, 42u16);
        let _rug_ed_tests_llm_16_721_llm_16_721_rrrruuuugggg_i16_as_u16 = 0;
    }
    #[test]
    #[should_panic(expected = "attempt to cast to with overflow")]
    fn i16_as_u16_overflow_negative() {
        let _rug_st_tests_llm_16_721_llm_16_721_rrrruuuugggg_i16_as_u16_overflow_negative = 0;
        let rug_fuzz_0 = 1;
        let value_i16: i16 = -rug_fuzz_0;
        let _value_u16: u16 = AsPrimitive::<u16>::as_(value_i16);
        let _rug_ed_tests_llm_16_721_llm_16_721_rrrruuuugggg_i16_as_u16_overflow_negative = 0;
    }
    #[test]
    fn i16_as_u16_edge_cases() {
        let _rug_st_tests_llm_16_721_llm_16_721_rrrruuuugggg_i16_as_u16_edge_cases = 0;
        let min_i16_as_u16: u16 = AsPrimitive::<u16>::as_(i16::MIN);
        debug_assert_eq!(min_i16_as_u16, 0u16);
        let max_i16_as_u16: u16 = AsPrimitive::<u16>::as_(i16::MAX);
        debug_assert_eq!(max_i16_as_u16, i16::MAX as u16);
        let _rug_ed_tests_llm_16_721_llm_16_721_rrrruuuugggg_i16_as_u16_edge_cases = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_722_llm_16_722 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_i16_to_u32() {
        let _rug_st_tests_llm_16_722_llm_16_722_rrrruuuugggg_test_as_primitive_i16_to_u32 = 0;
        let rug_fuzz_0 = 123;
        let rug_fuzz_1 = 123;
        let value_i16: i16 = rug_fuzz_0;
        let value_u32: u32 = AsPrimitive::<u32>::as_(value_i16);
        debug_assert_eq!(value_u32, 123u32);
        let negative_i16: i16 = -rug_fuzz_1;
        let casted_negative: u32 = AsPrimitive::<u32>::as_(negative_i16);
        debug_assert_eq!(casted_negative, (negative_i16 as i16 as u32));
        let _rug_ed_tests_llm_16_722_llm_16_722_rrrruuuugggg_test_as_primitive_i16_to_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_723_llm_16_723 {
    use crate::cast::AsPrimitive;
    #[test]
    fn i16_as_u64() {
        let _rug_st_tests_llm_16_723_llm_16_723_rrrruuuugggg_i16_as_u64 = 0;
        let rug_fuzz_0 = 42;
        let x: i16 = rug_fuzz_0;
        let y: u64 = AsPrimitive::<u64>::as_(x);
        debug_assert_eq!(y, 42u64);
        let _rug_ed_tests_llm_16_723_llm_16_723_rrrruuuugggg_i16_as_u64 = 0;
    }
    #[test]
    #[should_panic(expected = "attempt to cast to with overflow")]
    fn i16_as_u64_negative() {
        let _rug_st_tests_llm_16_723_llm_16_723_rrrruuuugggg_i16_as_u64_negative = 0;
        let rug_fuzz_0 = 42;
        let x: i16 = -rug_fuzz_0;
        let y: u64 = AsPrimitive::<u64>::as_(x);
        debug_assert_eq!(y, 0u64);
        let _rug_ed_tests_llm_16_723_llm_16_723_rrrruuuugggg_i16_as_u64_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_724_llm_16_724 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_i16_to_u8() {
        let _rug_st_tests_llm_16_724_llm_16_724_rrrruuuugggg_test_as_primitive_i16_to_u8 = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 0;
        let value_i16: i16 = rug_fuzz_0;
        let value_u8: u8 = AsPrimitive::<u8>::as_(value_i16);
        debug_assert_eq!(value_u8, 42u8);
        let max_value_i16: i16 = i16::MAX;
        debug_assert!(max_value_i16 > u8::MAX.into());
        let max_value_u8: u8 = AsPrimitive::<u8>::as_(max_value_i16);
        debug_assert_eq!(max_value_u8, u8::MAX);
        let min_value_i16: i16 = i16::MIN;
        debug_assert!(min_value_i16 < rug_fuzz_1);
        let min_value_u8: u8 = AsPrimitive::<u8>::as_(min_value_i16);
        debug_assert_eq!(min_value_u8, 0);
        let _rug_ed_tests_llm_16_724_llm_16_724_rrrruuuugggg_test_as_primitive_i16_to_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_725_llm_16_725 {
    use super::*;
    use crate::*;
    #[test]
    fn test_as_primitive_i16_to_usize() {
        let _rug_st_tests_llm_16_725_llm_16_725_rrrruuuugggg_test_as_primitive_i16_to_usize = 0;
        let rug_fuzz_0 = 42;
        let value_i16: i16 = rug_fuzz_0;
        let result: usize = value_i16.as_();
        debug_assert_eq!(result, 42usize);
        let _rug_ed_tests_llm_16_725_llm_16_725_rrrruuuugggg_test_as_primitive_i16_to_usize = 0;
    }
    #[test]
    #[should_panic]
    fn test_as_primitive_i16_to_usize_negative() {
        let _rug_st_tests_llm_16_725_llm_16_725_rrrruuuugggg_test_as_primitive_i16_to_usize_negative = 0;
        let rug_fuzz_0 = 42;
        let value_i16: i16 = -rug_fuzz_0;
        let _result: usize = value_i16.as_();
        let _rug_ed_tests_llm_16_725_llm_16_725_rrrruuuugggg_test_as_primitive_i16_to_usize_negative = 0;
    }
    #[test]
    fn test_as_primitive_i16_to_usize_max_value() {
        let _rug_st_tests_llm_16_725_llm_16_725_rrrruuuugggg_test_as_primitive_i16_to_usize_max_value = 0;
        let value_i16: i16 = i16::MAX;
        let result: usize = value_i16.as_();
        debug_assert_eq!(result, i16::MAX as usize);
        let _rug_ed_tests_llm_16_725_llm_16_725_rrrruuuugggg_test_as_primitive_i16_to_usize_max_value = 0;
    }
    #[test]
    fn test_as_primitive_i16_to_usize_min_value() {
        let _rug_st_tests_llm_16_725_llm_16_725_rrrruuuugggg_test_as_primitive_i16_to_usize_min_value = 0;
        let value_i16: i16 = i16::MIN;
        let result: usize = value_i16.as_();
        debug_assert_eq!(result, value_i16.wrapping_abs() as usize);
        let _rug_ed_tests_llm_16_725_llm_16_725_rrrruuuugggg_test_as_primitive_i16_to_usize_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_727_llm_16_727 {
    use crate::cast::FromPrimitive;
    #[test]
    fn from_f64_to_i16_conversion_test() {
        let _rug_st_tests_llm_16_727_llm_16_727_rrrruuuugggg_from_f64_to_i16_conversion_test = 0;
        let rug_fuzz_0 = 42.0;
        let rug_fuzz_1 = 42.7;
        let rug_fuzz_2 = 42.7;
        let rug_fuzz_3 = 32768.0;
        let rug_fuzz_4 = 32769.0;
        let rug_fuzz_5 = 32767.0;
        let rug_fuzz_6 = 32768.0;
        debug_assert_eq!(< i16 as FromPrimitive > ::from_f64(rug_fuzz_0), Some(42i16));
        debug_assert_eq!(< i16 as FromPrimitive > ::from_f64(rug_fuzz_1), Some(42i16));
        debug_assert_eq!(
            < i16 as FromPrimitive > ::from_f64(- rug_fuzz_2), Some(- 42i16)
        );
        debug_assert_eq!(< i16 as FromPrimitive > ::from_f64(f64::MAX), None);
        debug_assert_eq!(< i16 as FromPrimitive > ::from_f64(rug_fuzz_3), None);
        debug_assert_eq!(< i16 as FromPrimitive > ::from_f64(f64::MIN), None);
        debug_assert_eq!(< i16 as FromPrimitive > ::from_f64(- rug_fuzz_4), None);
        debug_assert_eq!(
            < i16 as FromPrimitive > ::from_f64(rug_fuzz_5), Some(32767i16)
        );
        debug_assert_eq!(
            < i16 as FromPrimitive > ::from_f64(- rug_fuzz_6), Some(- 32768i16)
        );
        let _rug_ed_tests_llm_16_727_llm_16_727_rrrruuuugggg_from_f64_to_i16_conversion_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_728_llm_16_728 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i128_to_i16() {
        let _rug_st_tests_llm_16_728_llm_16_728_rrrruuuugggg_test_from_i128_to_i16 = 0;
        let rug_fuzz_0 = 0i128;
        let rug_fuzz_1 = 32767i128;
        let rug_fuzz_2 = 32768i128;
        let rug_fuzz_3 = 32768i128;
        let rug_fuzz_4 = 32769i128;
        let rug_fuzz_5 = 32766i128;
        let rug_fuzz_6 = 32767i128;
        debug_assert_eq!(< i16 as FromPrimitive > ::from_i128(rug_fuzz_0), Some(0i16));
        debug_assert_eq!(
            < i16 as FromPrimitive > ::from_i128(rug_fuzz_1), Some(32767i16)
        );
        debug_assert_eq!(
            < i16 as FromPrimitive > ::from_i128(- rug_fuzz_2), Some(- 32768i16)
        );
        debug_assert_eq!(< i16 as FromPrimitive > ::from_i128(rug_fuzz_3), None);
        debug_assert_eq!(< i16 as FromPrimitive > ::from_i128(- rug_fuzz_4), None);
        debug_assert_eq!(
            < i16 as FromPrimitive > ::from_i128(rug_fuzz_5), Some(32766i16)
        );
        debug_assert_eq!(
            < i16 as FromPrimitive > ::from_i128(- rug_fuzz_6), Some(- 32767i16)
        );
        let _rug_ed_tests_llm_16_728_llm_16_728_rrrruuuugggg_test_from_i128_to_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_729_llm_16_729 {
    use crate::FromPrimitive;
    #[test]
    fn test_from_i16() {
        let _rug_st_tests_llm_16_729_llm_16_729_rrrruuuugggg_test_from_i16 = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        debug_assert_eq!(< i16 as FromPrimitive > ::from_i16(rug_fuzz_0), Some(0i16));
        debug_assert_eq!(
            < i16 as FromPrimitive > ::from_i16(- rug_fuzz_1), Some(- 1i16)
        );
        debug_assert_eq!(< i16 as FromPrimitive > ::from_i16(i16::MAX), Some(i16::MAX));
        debug_assert_eq!(< i16 as FromPrimitive > ::from_i16(i16::MIN), Some(i16::MIN));
        let _rug_ed_tests_llm_16_729_llm_16_729_rrrruuuugggg_test_from_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_730_llm_16_730 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i32() {
        let _rug_st_tests_llm_16_730_llm_16_730_rrrruuuugggg_test_from_i32 = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 32767;
        let rug_fuzz_4 = 32768;
        let rug_fuzz_5 = 32768;
        let rug_fuzz_6 = 32769;
        debug_assert_eq!(< i16 as FromPrimitive > ::from_i32(rug_fuzz_0), Some(0i16));
        debug_assert_eq!(< i16 as FromPrimitive > ::from_i32(i32::MAX), None);
        debug_assert_eq!(< i16 as FromPrimitive > ::from_i32(i32::MIN), None);
        debug_assert_eq!(< i16 as FromPrimitive > ::from_i32(rug_fuzz_1), Some(1i16));
        debug_assert_eq!(
            < i16 as FromPrimitive > ::from_i32(- rug_fuzz_2), Some(- 1i16)
        );
        debug_assert_eq!(
            < i16 as FromPrimitive > ::from_i32(rug_fuzz_3), Some(32767i16)
        );
        debug_assert_eq!(
            < i16 as FromPrimitive > ::from_i32(- rug_fuzz_4), Some(- 32768i16)
        );
        debug_assert_eq!(< i16 as FromPrimitive > ::from_i32(rug_fuzz_5), None);
        debug_assert_eq!(< i16 as FromPrimitive > ::from_i32(- rug_fuzz_6), None);
        let _rug_ed_tests_llm_16_730_llm_16_730_rrrruuuugggg_test_from_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_731_llm_16_731 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_i16_from_i64_within_range() {
        let _rug_st_tests_llm_16_731_llm_16_731_rrrruuuugggg_test_i16_from_i64_within_range = 0;
        let rug_fuzz_0 = 32_767;
        let value_i64: i64 = rug_fuzz_0;
        let result = <i16 as FromPrimitive>::from_i64(value_i64);
        debug_assert_eq!(result, Some(32_767i16));
        let _rug_ed_tests_llm_16_731_llm_16_731_rrrruuuugggg_test_i16_from_i64_within_range = 0;
    }
    #[test]
    fn test_i16_from_i64_below_range() {
        let _rug_st_tests_llm_16_731_llm_16_731_rrrruuuugggg_test_i16_from_i64_below_range = 0;
        let rug_fuzz_0 = 32_768;
        let value_i64: i64 = -rug_fuzz_0;
        let result = <i16 as FromPrimitive>::from_i64(value_i64);
        debug_assert_eq!(result, Some(- 32_768i16));
        let _rug_ed_tests_llm_16_731_llm_16_731_rrrruuuugggg_test_i16_from_i64_below_range = 0;
    }
    #[test]
    fn test_i16_from_i64_above_range() {
        let _rug_st_tests_llm_16_731_llm_16_731_rrrruuuugggg_test_i16_from_i64_above_range = 0;
        let rug_fuzz_0 = 32_768;
        let value_i64: i64 = rug_fuzz_0;
        let result = <i16 as FromPrimitive>::from_i64(value_i64);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_731_llm_16_731_rrrruuuugggg_test_i16_from_i64_above_range = 0;
    }
    #[test]
    fn test_i16_from_i64_way_below_range() {
        let _rug_st_tests_llm_16_731_llm_16_731_rrrruuuugggg_test_i16_from_i64_way_below_range = 0;
        let value_i64: i64 = i64::MIN;
        let result = <i16 as FromPrimitive>::from_i64(value_i64);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_731_llm_16_731_rrrruuuugggg_test_i16_from_i64_way_below_range = 0;
    }
    #[test]
    fn test_i16_from_i64_way_above_range() {
        let _rug_st_tests_llm_16_731_llm_16_731_rrrruuuugggg_test_i16_from_i64_way_above_range = 0;
        let value_i64: i64 = i64::MAX;
        let result = <i16 as FromPrimitive>::from_i64(value_i64);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_731_llm_16_731_rrrruuuugggg_test_i16_from_i64_way_above_range = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_732_llm_16_732 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i8_to_i16() {
        assert_eq!(< i16 as FromPrimitive >::from_i8(0), Some(0i16));
        assert_eq!(< i16 as FromPrimitive >::from_i8(- 1), Some(- 1i16));
        assert_eq!(< i16 as FromPrimitive >::from_i8(127), Some(127i16));
        assert_eq!(< i16 as FromPrimitive >::from_i8(- 128), Some(- 128i16));
    }
}
#[cfg(test)]
mod tests_llm_16_733 {
    use super::*;
    use crate::*;
    #[test]
    fn from_isize_for_i16() {
        let _rug_st_tests_llm_16_733_rrrruuuugggg_from_isize_for_i16 = 0;
        let rug_fuzz_0 = 0isize;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        debug_assert_eq!(
            < i16 as cast::FromPrimitive > ::from_isize(rug_fuzz_0), Some(0i16)
        );
        debug_assert_eq!(
            < i16 as cast::FromPrimitive > ::from_isize(i16::MAX as isize),
            Some(i16::MAX)
        );
        debug_assert_eq!(
            < i16 as cast::FromPrimitive > ::from_isize(i16::MIN as isize),
            Some(i16::MIN)
        );
        debug_assert_eq!(
            < i16 as cast::FromPrimitive > ::from_isize((i16::MAX as isize) +
            rug_fuzz_1), None
        );
        debug_assert_eq!(
            < i16 as cast::FromPrimitive > ::from_isize((i16::MIN as isize) -
            rug_fuzz_2), None
        );
        let _rug_ed_tests_llm_16_733_rrrruuuugggg_from_isize_for_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_734_llm_16_734 {
    use crate::FromPrimitive;
    #[test]
    fn test_from_u128() {
        let _rug_st_tests_llm_16_734_llm_16_734_rrrruuuugggg_test_from_u128 = 0;
        let rug_fuzz_0 = 32767u128;
        let rug_fuzz_1 = 32768u128;
        let rug_fuzz_2 = 0u128;
        let rug_fuzz_3 = 1u128;
        debug_assert_eq!(
            < i16 as FromPrimitive > ::from_u128(rug_fuzz_0), Some(32767i16)
        );
        debug_assert_eq!(< i16 as FromPrimitive > ::from_u128(rug_fuzz_1), None);
        debug_assert_eq!(< i16 as FromPrimitive > ::from_u128(u128::MAX), None);
        debug_assert_eq!(< i16 as FromPrimitive > ::from_u128(rug_fuzz_2), Some(0i16));
        debug_assert_eq!(< i16 as FromPrimitive > ::from_u128(rug_fuzz_3), Some(1i16));
        debug_assert_eq!(
            < i16 as FromPrimitive > ::from_u128(u128::from(i16::MAX as u128)),
            Some(i16::MAX)
        );
        let _rug_ed_tests_llm_16_734_llm_16_734_rrrruuuugggg_test_from_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_735_llm_16_735 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u16_to_i16() {
        let _rug_st_tests_llm_16_735_llm_16_735_rrrruuuugggg_test_from_u16_to_i16 = 0;
        let rug_fuzz_0 = 0_u16;
        let rug_fuzz_1 = 32767_u16;
        let rug_fuzz_2 = 32768_u16;
        debug_assert_eq!(< i16 as FromPrimitive > ::from_u16(rug_fuzz_0), Some(0_i16));
        debug_assert_eq!(
            < i16 as FromPrimitive > ::from_u16(rug_fuzz_1), Some(32767_i16)
        );
        debug_assert_eq!(< i16 as FromPrimitive > ::from_u16(rug_fuzz_2), None);
        debug_assert_eq!(< i16 as FromPrimitive > ::from_u16(u16::MAX), None);
        let _rug_ed_tests_llm_16_735_llm_16_735_rrrruuuugggg_test_from_u16_to_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_736_llm_16_736 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u32_with_in_range_value() {
        let _rug_st_tests_llm_16_736_llm_16_736_rrrruuuugggg_test_from_u32_with_in_range_value = 0;
        let rug_fuzz_0 = 32;
        let value: u32 = rug_fuzz_0;
        let result = <i16 as FromPrimitive>::from_u32(value);
        debug_assert_eq!(result, Some(32i16));
        let _rug_ed_tests_llm_16_736_llm_16_736_rrrruuuugggg_test_from_u32_with_in_range_value = 0;
    }
    #[test]
    fn test_from_u32_with_out_of_range_value() {
        let _rug_st_tests_llm_16_736_llm_16_736_rrrruuuugggg_test_from_u32_with_out_of_range_value = 0;
        let value: u32 = u32::MAX;
        let result = <i16 as FromPrimitive>::from_u32(value);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_736_llm_16_736_rrrruuuugggg_test_from_u32_with_out_of_range_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_737_llm_16_737 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u64() {
        let _rug_st_tests_llm_16_737_llm_16_737_rrrruuuugggg_test_from_u64 = 0;
        let rug_fuzz_0 = 0u64;
        let rug_fuzz_1 = 32767u64;
        let rug_fuzz_2 = 32768u64;
        debug_assert_eq!(< i16 as FromPrimitive > ::from_u64(rug_fuzz_0), Some(0i16));
        debug_assert_eq!(
            < i16 as FromPrimitive > ::from_u64(rug_fuzz_1), Some(32767i16)
        );
        debug_assert_eq!(< i16 as FromPrimitive > ::from_u64(rug_fuzz_2), None);
        debug_assert_eq!(< i16 as FromPrimitive > ::from_u64(u64::MAX), None);
        let _rug_ed_tests_llm_16_737_llm_16_737_rrrruuuugggg_test_from_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_738_llm_16_738 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u8() {
        let _rug_st_tests_llm_16_738_llm_16_738_rrrruuuugggg_test_from_u8 = 0;
        let rug_fuzz_0 = 100;
        let value_within_range: u8 = rug_fuzz_0;
        debug_assert_eq!(
            < i16 as FromPrimitive > ::from_u8(value_within_range), Some(100i16)
        );
        let max_u8_value: u8 = u8::MAX;
        debug_assert_eq!(< i16 as FromPrimitive > ::from_u8(max_u8_value), Some(255i16));
        let _rug_ed_tests_llm_16_738_llm_16_738_rrrruuuugggg_test_from_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_739_llm_16_739 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_usize() {
        let _rug_st_tests_llm_16_739_llm_16_739_rrrruuuugggg_test_from_usize = 0;
        let rug_fuzz_0 = 156;
        let value_in_range: usize = rug_fuzz_0;
        debug_assert_eq!(
            < i16 as FromPrimitive > ::from_usize(value_in_range), Some(156i16)
        );
        let value_out_of_range: usize = usize::max_value();
        debug_assert_eq!(
            < i16 as FromPrimitive > ::from_usize(value_out_of_range), None
        );
        let _rug_ed_tests_llm_16_739_llm_16_739_rrrruuuugggg_test_from_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_740_llm_16_740 {
    use crate::cast::{NumCast, ToPrimitive};
    use std::num::Wrapping;
    #[test]
    fn test_from_wrapping_to_i16() {
        let _rug_st_tests_llm_16_740_llm_16_740_rrrruuuugggg_test_from_wrapping_to_i16 = 0;
        let rug_fuzz_0 = 42i16;
        let rug_fuzz_1 = 42i8;
        let rug_fuzz_2 = 42u8;
        debug_assert_eq!(< i16 as NumCast > ::from(Wrapping(rug_fuzz_0)), Some(42i16));
        debug_assert_eq!(< i16 as NumCast > ::from(Wrapping(rug_fuzz_1)), Some(42i16));
        debug_assert_eq!(< i16 as NumCast > ::from(Wrapping(rug_fuzz_2)), Some(42i16));
        let _rug_ed_tests_llm_16_740_llm_16_740_rrrruuuugggg_test_from_wrapping_to_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_741_llm_16_741 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_f32() {
        let _rug_st_tests_llm_16_741_llm_16_741_rrrruuuugggg_test_to_f32 = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42;
        let value: i16 = rug_fuzz_0;
        let result = ToPrimitive::to_f32(&value);
        debug_assert_eq!(result, Some(42.0_f32));
        let value: i16 = -rug_fuzz_1;
        let result = ToPrimitive::to_f32(&value);
        debug_assert_eq!(result, Some(- 42.0_f32));
        let value: i16 = i16::MAX;
        let result = ToPrimitive::to_f32(&value);
        debug_assert_eq!(result, Some(i16::MAX as f32));
        let value: i16 = i16::MIN;
        let result = ToPrimitive::to_f32(&value);
        debug_assert_eq!(result, Some(i16::MIN as f32));
        let _rug_ed_tests_llm_16_741_llm_16_741_rrrruuuugggg_test_to_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_742_llm_16_742 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_f64() {
        let _rug_st_tests_llm_16_742_llm_16_742_rrrruuuugggg_test_to_f64 = 0;
        let rug_fuzz_0 = 0i16;
        let rug_fuzz_1 = 1i16;
        let rug_fuzz_2 = 1i16;
        debug_assert_eq!(ToPrimitive::to_f64(& rug_fuzz_0), Some(0.0_f64));
        debug_assert_eq!(ToPrimitive::to_f64(& rug_fuzz_1), Some(1.0_f64));
        debug_assert_eq!(ToPrimitive::to_f64(& - rug_fuzz_2), Some(- 1.0_f64));
        debug_assert_eq!(ToPrimitive::to_f64(& i16::MAX), Some(i16::MAX as f64));
        debug_assert_eq!(ToPrimitive::to_f64(& i16::MIN), Some(i16::MIN as f64));
        let _rug_ed_tests_llm_16_742_llm_16_742_rrrruuuugggg_test_to_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_743 {
    use crate::ToPrimitive;
    #[test]
    fn test_to_i128() {
        let _rug_st_tests_llm_16_743_rrrruuuugggg_test_to_i128 = 0;
        let rug_fuzz_0 = 0i16;
        let rug_fuzz_1 = 123i16;
        let rug_fuzz_2 = 123i16;
        debug_assert_eq!(i16::MIN.to_i128(), Some(i128::from(i16::MIN)));
        debug_assert_eq!(i16::MAX.to_i128(), Some(i128::from(i16::MAX)));
        debug_assert_eq!(rug_fuzz_0.to_i128(), Some(i128::from(0i16)));
        debug_assert_eq!(rug_fuzz_1.to_i128(), Some(i128::from(123i16)));
        debug_assert_eq!((- rug_fuzz_2).to_i128(), Some(i128::from(- 123i16)));
        let _rug_ed_tests_llm_16_743_rrrruuuugggg_test_to_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_744_llm_16_744 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_i16_within_bounds() {
        let _rug_st_tests_llm_16_744_llm_16_744_rrrruuuugggg_test_to_i16_within_bounds = 0;
        let rug_fuzz_0 = 10i16;
        let rug_fuzz_1 = 0i16;
        let rug_fuzz_2 = 10i16;
        debug_assert_eq!(rug_fuzz_0.to_i16(), Some(10i16));
        debug_assert_eq!(rug_fuzz_1.to_i16(), Some(0i16));
        debug_assert_eq!((- rug_fuzz_2).to_i16(), Some(- 10i16));
        debug_assert_eq!(i16::MAX.to_i16(), Some(i16::MAX));
        debug_assert_eq!(i16::MIN.to_i16(), Some(i16::MIN));
        let _rug_ed_tests_llm_16_744_llm_16_744_rrrruuuugggg_test_to_i16_within_bounds = 0;
    }
    #[test]
    fn test_to_i16_out_of_bounds() {
        let _rug_st_tests_llm_16_744_llm_16_744_rrrruuuugggg_test_to_i16_out_of_bounds = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 1;
        debug_assert_eq!(i16::MAX.to_i16(), Some(i16::MAX));
        debug_assert_eq!((i16::MAX as i32 + rug_fuzz_0).to_i16(), None);
        debug_assert_eq!((i16::MIN as i32 - rug_fuzz_1).to_i16(), None);
        let _rug_ed_tests_llm_16_744_llm_16_744_rrrruuuugggg_test_to_i16_out_of_bounds = 0;
    }
    #[test]
    fn test_to_i16_with_different_types() {
        let _rug_st_tests_llm_16_744_llm_16_744_rrrruuuugggg_test_to_i16_with_different_types = 0;
        let rug_fuzz_0 = 10u8;
        let rug_fuzz_1 = 255u8;
        let rug_fuzz_2 = 10u16;
        let rug_fuzz_3 = 10u32;
        let rug_fuzz_4 = 10u64;
        let rug_fuzz_5 = 10usize;
        let rug_fuzz_6 = 10i8;
        let rug_fuzz_7 = 10i32;
        let rug_fuzz_8 = 10i64;
        let rug_fuzz_9 = 10isize;
        let rug_fuzz_10 = 10;
        let rug_fuzz_11 = 1i32;
        let rug_fuzz_12 = 1i64;
        debug_assert_eq!(rug_fuzz_0.to_i16(), Some(10i16));
        debug_assert_eq!(rug_fuzz_1.to_i16(), Some(255i16));
        debug_assert_eq!(rug_fuzz_2.to_i16(), Some(10i16));
        debug_assert_eq!(rug_fuzz_3.to_i16(), Some(10i16));
        debug_assert_eq!(rug_fuzz_4.to_i16(), Some(10i16));
        debug_assert_eq!(rug_fuzz_5.to_i16(), Some(10i16));
        debug_assert_eq!(rug_fuzz_6.to_i16(), Some(10i16));
        debug_assert_eq!(rug_fuzz_7.to_i16(), Some(10i16));
        debug_assert_eq!(rug_fuzz_8.to_i16(), Some(10i16));
        debug_assert_eq!(rug_fuzz_9.to_i16(), Some(10i16));
        debug_assert_eq!((- rug_fuzz_10).to_i16(), Some(- 10i16));
        debug_assert_eq!((u32::MAX).to_i16(), None);
        debug_assert_eq!((u64::MAX).to_i16(), None);
        debug_assert_eq!((i64::MAX).to_i16(), None);
        debug_assert_eq!((- rug_fuzz_11).to_i16(), Some(- 1i16));
        debug_assert_eq!((- rug_fuzz_12).to_i16(), Some(- 1i16));
        let _rug_ed_tests_llm_16_744_llm_16_744_rrrruuuugggg_test_to_i16_with_different_types = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_745 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_i16_to_i32() {
        let _rug_st_tests_llm_16_745_rrrruuuugggg_test_i16_to_i32 = 0;
        let rug_fuzz_0 = 0_i16;
        let rug_fuzz_1 = 1_i16;
        debug_assert_eq!(< i16 as ToPrimitive > ::to_i32(& rug_fuzz_0), Some(0_i32));
        debug_assert_eq!(< i16 as ToPrimitive > ::to_i32(& - rug_fuzz_1), Some(- 1_i32));
        debug_assert_eq!(
            < i16 as ToPrimitive > ::to_i32(& i16::MAX), Some(i32::from(i16::MAX))
        );
        debug_assert_eq!(
            < i16 as ToPrimitive > ::to_i32(& i16::MIN), Some(i32::from(i16::MIN))
        );
        let _rug_ed_tests_llm_16_745_rrrruuuugggg_test_i16_to_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_746_llm_16_746 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_i16_to_i64() {
        let _rug_st_tests_llm_16_746_llm_16_746_rrrruuuugggg_test_i16_to_i64 = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 123;
        let small_value: i16 = i16::MIN;
        let small_value_converted = small_value.to_i64();
        debug_assert_eq!(small_value_converted, Some(i64::from(i16::MIN)));
        let large_value: i16 = i16::MAX;
        let large_value_converted = large_value.to_i64();
        debug_assert_eq!(large_value_converted, Some(i64::from(i16::MAX)));
        let min_i16: i16 = i16::MIN;
        let min_i16_converted = min_i16.to_i64();
        debug_assert_eq!(min_i16_converted, Some(i64::from(i16::MIN)));
        let max_i16: i16 = i16::MAX;
        let max_i16_converted = max_i16.to_i64();
        debug_assert_eq!(max_i16_converted, Some(i64::from(i16::MAX)));
        let specific_value: i16 = rug_fuzz_0;
        let specific_value_converted = specific_value.to_i64();
        debug_assert_eq!(specific_value_converted, Some(42i64));
        let negative_value: i16 = -rug_fuzz_1;
        let negative_value_converted = negative_value.to_i64();
        debug_assert_eq!(negative_value_converted, Some(- 123i64));
        let _rug_ed_tests_llm_16_746_llm_16_746_rrrruuuugggg_test_i16_to_i64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_747_llm_16_747 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_i8_success() {
        let _rug_st_tests_llm_16_747_llm_16_747_rrrruuuugggg_test_to_i8_success = 0;
        let rug_fuzz_0 = 100;
        let value: i16 = rug_fuzz_0;
        let result = value.to_i8();
        debug_assert_eq!(result, Some(100i8));
        let _rug_ed_tests_llm_16_747_llm_16_747_rrrruuuugggg_test_to_i8_success = 0;
    }
    #[test]
    fn test_to_i8_negative_success() {
        let _rug_st_tests_llm_16_747_llm_16_747_rrrruuuugggg_test_to_i8_negative_success = 0;
        let rug_fuzz_0 = 100;
        let value: i16 = -rug_fuzz_0;
        let result = value.to_i8();
        debug_assert_eq!(result, Some(- 100i8));
        let _rug_ed_tests_llm_16_747_llm_16_747_rrrruuuugggg_test_to_i8_negative_success = 0;
    }
    #[test]
    fn test_to_i8_overflow() {
        let _rug_st_tests_llm_16_747_llm_16_747_rrrruuuugggg_test_to_i8_overflow = 0;
        let rug_fuzz_0 = 1000;
        let value: i16 = rug_fuzz_0;
        let result = value.to_i8();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_747_llm_16_747_rrrruuuugggg_test_to_i8_overflow = 0;
    }
    #[test]
    fn test_to_i8_underflow() {
        let _rug_st_tests_llm_16_747_llm_16_747_rrrruuuugggg_test_to_i8_underflow = 0;
        let rug_fuzz_0 = 1000;
        let value: i16 = -rug_fuzz_0;
        let result = value.to_i8();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_747_llm_16_747_rrrruuuugggg_test_to_i8_underflow = 0;
    }
    #[test]
    fn test_to_i8_max_boundary() {
        let _rug_st_tests_llm_16_747_llm_16_747_rrrruuuugggg_test_to_i8_max_boundary = 0;
        let value: i16 = i8::MAX as i16;
        let result = value.to_i8();
        debug_assert_eq!(result, Some(i8::MAX));
        let _rug_ed_tests_llm_16_747_llm_16_747_rrrruuuugggg_test_to_i8_max_boundary = 0;
    }
    #[test]
    fn test_to_i8_min_boundary() {
        let _rug_st_tests_llm_16_747_llm_16_747_rrrruuuugggg_test_to_i8_min_boundary = 0;
        let value: i16 = i8::MIN as i16;
        let result = value.to_i8();
        debug_assert_eq!(result, Some(i8::MIN));
        let _rug_ed_tests_llm_16_747_llm_16_747_rrrruuuugggg_test_to_i8_min_boundary = 0;
    }
    #[test]
    fn test_to_i8_just_above_max_boundary() {
        let _rug_st_tests_llm_16_747_llm_16_747_rrrruuuugggg_test_to_i8_just_above_max_boundary = 0;
        let rug_fuzz_0 = 1;
        let value: i16 = (i8::MAX as i16) + rug_fuzz_0;
        let result = value.to_i8();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_747_llm_16_747_rrrruuuugggg_test_to_i8_just_above_max_boundary = 0;
    }
    #[test]
    fn test_to_i8_just_below_min_boundary() {
        let _rug_st_tests_llm_16_747_llm_16_747_rrrruuuugggg_test_to_i8_just_below_min_boundary = 0;
        let rug_fuzz_0 = 1;
        let value: i16 = (i8::MIN as i16) - rug_fuzz_0;
        let result = value.to_i8();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_747_llm_16_747_rrrruuuugggg_test_to_i8_just_below_min_boundary = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_748_llm_16_748 {
    use crate::cast::ToPrimitive;
    #[test]
    fn to_isize_with_i16() {
        let _rug_st_tests_llm_16_748_llm_16_748_rrrruuuugggg_to_isize_with_i16 = 0;
        let rug_fuzz_0 = 123i16;
        let rug_fuzz_1 = 123i16;
        let positive_within_bounds = rug_fuzz_0;
        let negative_within_bounds = -rug_fuzz_1;
        debug_assert_eq!(positive_within_bounds.to_isize(), Some(123isize));
        debug_assert_eq!(negative_within_bounds.to_isize(), Some(- 123isize));
        let max_i16_as_isize = i16::MAX;
        let min_i16_as_isize = i16::MIN;
        debug_assert_eq!(max_i16_as_isize.to_isize(), Some(i16::MAX as isize));
        debug_assert_eq!(min_i16_as_isize.to_isize(), Some(i16::MIN as isize));
        let _rug_ed_tests_llm_16_748_llm_16_748_rrrruuuugggg_to_isize_with_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_749 {
    use super::*;
    use crate::*;
    #[test]
    fn test_i16_to_u128() {
        let _rug_st_tests_llm_16_749_rrrruuuugggg_test_i16_to_u128 = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        debug_assert_eq!(
            < i16 as cast::ToPrimitive > ::to_u128(& rug_fuzz_0), Some(0u128)
        );
        debug_assert_eq!(
            < i16 as cast::ToPrimitive > ::to_u128(& rug_fuzz_1), Some(1u128)
        );
        debug_assert_eq!(< i16 as cast::ToPrimitive > ::to_u128(& - rug_fuzz_2), None);
        debug_assert_eq!(
            < i16 as cast::ToPrimitive > ::to_u128(& i16::MAX), Some(32767u128)
        );
        debug_assert_eq!(< i16 as cast::ToPrimitive > ::to_u128(& i16::MIN), None);
        let _rug_ed_tests_llm_16_749_rrrruuuugggg_test_i16_to_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_750_llm_16_750 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_u16_with_positive_i16() {
        let _rug_st_tests_llm_16_750_llm_16_750_rrrruuuugggg_test_to_u16_with_positive_i16 = 0;
        let rug_fuzz_0 = 100;
        let num: i16 = rug_fuzz_0;
        debug_assert_eq!(num.to_u16(), Some(100u16));
        let _rug_ed_tests_llm_16_750_llm_16_750_rrrruuuugggg_test_to_u16_with_positive_i16 = 0;
    }
    #[test]
    fn test_to_u16_with_negative_i16() {
        let _rug_st_tests_llm_16_750_llm_16_750_rrrruuuugggg_test_to_u16_with_negative_i16 = 0;
        let rug_fuzz_0 = 100;
        let num: i16 = -rug_fuzz_0;
        debug_assert_eq!(num.to_u16(), None);
        let _rug_ed_tests_llm_16_750_llm_16_750_rrrruuuugggg_test_to_u16_with_negative_i16 = 0;
    }
    #[test]
    fn test_to_u16_with_i16_max() {
        let _rug_st_tests_llm_16_750_llm_16_750_rrrruuuugggg_test_to_u16_with_i16_max = 0;
        let num: i16 = i16::MAX;
        debug_assert_eq!(num.to_u16(), Some(32767u16));
        let _rug_ed_tests_llm_16_750_llm_16_750_rrrruuuugggg_test_to_u16_with_i16_max = 0;
    }
    #[test]
    fn test_to_u16_with_i16_min() {
        let _rug_st_tests_llm_16_750_llm_16_750_rrrruuuugggg_test_to_u16_with_i16_min = 0;
        let num: i16 = i16::MIN;
        debug_assert_eq!(num.to_u16(), None);
        let _rug_ed_tests_llm_16_750_llm_16_750_rrrruuuugggg_test_to_u16_with_i16_min = 0;
    }
    #[test]
    fn test_to_u16_with_zero() {
        let _rug_st_tests_llm_16_750_llm_16_750_rrrruuuugggg_test_to_u16_with_zero = 0;
        let rug_fuzz_0 = 0;
        let num: i16 = rug_fuzz_0;
        debug_assert_eq!(num.to_u16(), Some(0u16));
        let _rug_ed_tests_llm_16_750_llm_16_750_rrrruuuugggg_test_to_u16_with_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_751_llm_16_751 {
    use crate::cast::ToPrimitive;
    #[test]
    fn to_u32_with_positive_i16() {
        let _rug_st_tests_llm_16_751_llm_16_751_rrrruuuugggg_to_u32_with_positive_i16 = 0;
        let rug_fuzz_0 = 123;
        let value: i16 = rug_fuzz_0;
        let result = value.to_u32();
        debug_assert_eq!(result, Some(123_u32));
        let _rug_ed_tests_llm_16_751_llm_16_751_rrrruuuugggg_to_u32_with_positive_i16 = 0;
    }
    #[test]
    fn to_u32_with_negative_i16() {
        let _rug_st_tests_llm_16_751_llm_16_751_rrrruuuugggg_to_u32_with_negative_i16 = 0;
        let rug_fuzz_0 = 123;
        let value: i16 = -rug_fuzz_0;
        let result = value.to_u32();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_751_llm_16_751_rrrruuuugggg_to_u32_with_negative_i16 = 0;
    }
    #[test]
    fn to_u32_with_i16_max() {
        let _rug_st_tests_llm_16_751_llm_16_751_rrrruuuugggg_to_u32_with_i16_max = 0;
        let value: i16 = i16::MAX;
        let result = value.to_u32();
        debug_assert_eq!(result, Some(i16::MAX as u32));
        let _rug_ed_tests_llm_16_751_llm_16_751_rrrruuuugggg_to_u32_with_i16_max = 0;
    }
    #[test]
    fn to_u32_with_i16_min() {
        let _rug_st_tests_llm_16_751_llm_16_751_rrrruuuugggg_to_u32_with_i16_min = 0;
        let value: i16 = i16::MIN;
        let result = value.to_u32();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_751_llm_16_751_rrrruuuugggg_to_u32_with_i16_min = 0;
    }
    #[test]
    fn to_u32_with_zero_i16() {
        let _rug_st_tests_llm_16_751_llm_16_751_rrrruuuugggg_to_u32_with_zero_i16 = 0;
        let rug_fuzz_0 = 0;
        let value: i16 = rug_fuzz_0;
        let result = value.to_u32();
        debug_assert_eq!(result, Some(0_u32));
        let _rug_ed_tests_llm_16_751_llm_16_751_rrrruuuugggg_to_u32_with_zero_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_752_llm_16_752 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_u64_with_positive_i16() {
        let _rug_st_tests_llm_16_752_llm_16_752_rrrruuuugggg_test_to_u64_with_positive_i16 = 0;
        let rug_fuzz_0 = 123;
        let value: i16 = rug_fuzz_0;
        debug_assert_eq!(value.to_u64(), Some(123u64));
        let _rug_ed_tests_llm_16_752_llm_16_752_rrrruuuugggg_test_to_u64_with_positive_i16 = 0;
    }
    #[test]
    fn test_to_u64_with_max_i16() {
        let _rug_st_tests_llm_16_752_llm_16_752_rrrruuuugggg_test_to_u64_with_max_i16 = 0;
        let value: i16 = i16::MAX;
        debug_assert_eq!(value.to_u64(), Some(i16::MAX as u64));
        let _rug_ed_tests_llm_16_752_llm_16_752_rrrruuuugggg_test_to_u64_with_max_i16 = 0;
    }
    #[test]
    fn test_to_u64_with_min_i16() {
        let _rug_st_tests_llm_16_752_llm_16_752_rrrruuuugggg_test_to_u64_with_min_i16 = 0;
        let value: i16 = i16::MIN;
        debug_assert_eq!(value.to_u64(), None);
        let _rug_ed_tests_llm_16_752_llm_16_752_rrrruuuugggg_test_to_u64_with_min_i16 = 0;
    }
    #[test]
    fn test_to_u64_with_zero() {
        let _rug_st_tests_llm_16_752_llm_16_752_rrrruuuugggg_test_to_u64_with_zero = 0;
        let rug_fuzz_0 = 0;
        let value: i16 = rug_fuzz_0;
        debug_assert_eq!(value.to_u64(), Some(0u64));
        let _rug_ed_tests_llm_16_752_llm_16_752_rrrruuuugggg_test_to_u64_with_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_753_llm_16_753 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_u8_with_i16() {
        let _rug_st_tests_llm_16_753_llm_16_753_rrrruuuugggg_test_to_u8_with_i16 = 0;
        let rug_fuzz_0 = 0_i16;
        let rug_fuzz_1 = 1_i16;
        let rug_fuzz_2 = 127_i16;
        let rug_fuzz_3 = 128_i16;
        let rug_fuzz_4 = 255_i16;
        let rug_fuzz_5 = 256_i16;
        let rug_fuzz_6 = 1_i16;
        debug_assert_eq!(rug_fuzz_0.to_u8(), Some(0_u8));
        debug_assert_eq!(rug_fuzz_1.to_u8(), Some(1_u8));
        debug_assert_eq!(rug_fuzz_2.to_u8(), Some(127_u8));
        debug_assert_eq!(rug_fuzz_3.to_u8(), Some(128_u8));
        debug_assert_eq!(rug_fuzz_4.to_u8(), Some(255_u8));
        debug_assert_eq!(rug_fuzz_5.to_u8(), None);
        debug_assert_eq!((- rug_fuzz_6).to_u8(), None);
        debug_assert_eq!(i16::MAX.to_u8(), None);
        debug_assert_eq!(i16::MIN.to_u8(), None);
        let _rug_ed_tests_llm_16_753_llm_16_753_rrrruuuugggg_test_to_u8_with_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_754_llm_16_754 {
    use crate::cast::ToPrimitive;
    #[test]
    fn i16_to_usize() {
        let _rug_st_tests_llm_16_754_llm_16_754_rrrruuuugggg_i16_to_usize = 0;
        let rug_fuzz_0 = 0i16;
        let rug_fuzz_1 = 1i16;
        let rug_fuzz_2 = 1i16;
        debug_assert_eq!(rug_fuzz_0.to_usize(), Some(0usize));
        debug_assert_eq!(rug_fuzz_1.to_usize(), Some(1usize));
        debug_assert_eq!(i16::MAX.to_usize(), Some(i16::MAX as usize));
        debug_assert_eq!((- rug_fuzz_2).to_usize(), None);
        debug_assert_eq!(i16::MIN.to_usize(), None);
        let _rug_ed_tests_llm_16_754_llm_16_754_rrrruuuugggg_i16_to_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_822_llm_16_822 {
    use crate::AsPrimitive;
    #[test]
    fn test_as_primitive_i32_to_f32() {
        let _rug_st_tests_llm_16_822_llm_16_822_rrrruuuugggg_test_as_primitive_i32_to_f32 = 0;
        let rug_fuzz_0 = 100;
        let value: i32 = rug_fuzz_0;
        let result: f32 = value.as_();
        debug_assert_eq!(result, 100f32);
        let _rug_ed_tests_llm_16_822_llm_16_822_rrrruuuugggg_test_as_primitive_i32_to_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_823_llm_16_823 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_i32_to_f64() {
        let _rug_st_tests_llm_16_823_llm_16_823_rrrruuuugggg_test_as_primitive_i32_to_f64 = 0;
        let rug_fuzz_0 = 42;
        let value: i32 = rug_fuzz_0;
        let result: f64 = value.as_();
        debug_assert_eq!(result, 42f64);
        let _rug_ed_tests_llm_16_823_llm_16_823_rrrruuuugggg_test_as_primitive_i32_to_f64 = 0;
    }
    #[test]
    fn test_as_primitive_i32_to_f64_negative() {
        let _rug_st_tests_llm_16_823_llm_16_823_rrrruuuugggg_test_as_primitive_i32_to_f64_negative = 0;
        let rug_fuzz_0 = 42;
        let value: i32 = -rug_fuzz_0;
        let result: f64 = value.as_();
        debug_assert_eq!(result, - 42f64);
        let _rug_ed_tests_llm_16_823_llm_16_823_rrrruuuugggg_test_as_primitive_i32_to_f64_negative = 0;
    }
    #[test]
    fn test_as_primitive_i32_to_f64_zero() {
        let _rug_st_tests_llm_16_823_llm_16_823_rrrruuuugggg_test_as_primitive_i32_to_f64_zero = 0;
        let rug_fuzz_0 = 0;
        let value: i32 = rug_fuzz_0;
        let result: f64 = value.as_();
        debug_assert_eq!(result, 0f64);
        let _rug_ed_tests_llm_16_823_llm_16_823_rrrruuuugggg_test_as_primitive_i32_to_f64_zero = 0;
    }
    #[test]
    fn test_as_primitive_i32_to_f64_max() {
        let _rug_st_tests_llm_16_823_llm_16_823_rrrruuuugggg_test_as_primitive_i32_to_f64_max = 0;
        let value: i32 = i32::MAX;
        let result: f64 = value.as_();
        debug_assert_eq!(result, i32::MAX as f64);
        let _rug_ed_tests_llm_16_823_llm_16_823_rrrruuuugggg_test_as_primitive_i32_to_f64_max = 0;
    }
    #[test]
    fn test_as_primitive_i32_to_f64_min() {
        let _rug_st_tests_llm_16_823_llm_16_823_rrrruuuugggg_test_as_primitive_i32_to_f64_min = 0;
        let value: i32 = i32::MIN;
        let result: f64 = value.as_();
        debug_assert_eq!(result, i32::MIN as f64);
        let _rug_ed_tests_llm_16_823_llm_16_823_rrrruuuugggg_test_as_primitive_i32_to_f64_min = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_824_llm_16_824 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_i32_to_i128() {
        let _rug_st_tests_llm_16_824_llm_16_824_rrrruuuugggg_test_as_primitive_i32_to_i128 = 0;
        let rug_fuzz_0 = 42;
        let value: i32 = rug_fuzz_0;
        let result: i128 = AsPrimitive::<i128>::as_(value);
        debug_assert_eq!(result, 42i128);
        let _rug_ed_tests_llm_16_824_llm_16_824_rrrruuuugggg_test_as_primitive_i32_to_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_825_llm_16_825 {
    use crate::AsPrimitive;
    #[test]
    fn test_as_i32_to_i16() {
        let _rug_st_tests_llm_16_825_llm_16_825_rrrruuuugggg_test_as_i32_to_i16 = 0;
        let rug_fuzz_0 = 32767;
        let rug_fuzz_1 = 32768;
        let rug_fuzz_2 = 32768;
        let rug_fuzz_3 = 32769;
        let val_i32: i32 = rug_fuzz_0;
        let val_i16: i16 = val_i32.as_();
        debug_assert_eq!(val_i16, 32767i16);
        let val_i32_negative: i32 = -rug_fuzz_1;
        let val_i16_negative: i16 = val_i32_negative.as_();
        debug_assert_eq!(val_i16_negative, - 32768i16);
        let val_i32_overflow: i32 = rug_fuzz_2;
        #[cfg(debug_assertions)]
        {
            let val_i16_overflow: i16 = val_i32_overflow.as_();
            debug_assert_eq!(val_i16_overflow, - 32768i16);
        }
        let val_i32_underflow: i32 = -rug_fuzz_3;
        #[cfg(debug_assertions)]
        {
            let val_i16_underflow: i16 = val_i32_underflow.as_();
            debug_assert_eq!(val_i16_underflow, 32767i16);
        }
        let _rug_ed_tests_llm_16_825_llm_16_825_rrrruuuugggg_test_as_i32_to_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_826_llm_16_826 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_i32_to_i32() {
        let _rug_st_tests_llm_16_826_llm_16_826_rrrruuuugggg_test_as_primitive_i32_to_i32 = 0;
        let rug_fuzz_0 = 42;
        let value: i32 = rug_fuzz_0;
        let result: i32 = AsPrimitive::<i32>::as_(value);
        debug_assert_eq!(result, 42i32);
        let _rug_ed_tests_llm_16_826_llm_16_826_rrrruuuugggg_test_as_primitive_i32_to_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_827_llm_16_827 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_i32_to_i64() {
        let _rug_st_tests_llm_16_827_llm_16_827_rrrruuuugggg_test_as_i32_to_i64 = 0;
        let rug_fuzz_0 = 10;
        let value: i32 = rug_fuzz_0;
        let result: i64 = AsPrimitive::<i64>::as_(value);
        debug_assert_eq!(result, 10i64);
        let _rug_ed_tests_llm_16_827_llm_16_827_rrrruuuugggg_test_as_i32_to_i64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_828_llm_16_828 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_i32_to_i8() {
        let _rug_st_tests_llm_16_828_llm_16_828_rrrruuuugggg_test_as_primitive_i32_to_i8 = 0;
        let rug_fuzz_0 = 100;
        let rug_fuzz_1 = 100;
        let rug_fuzz_2 = 1000;
        let rug_fuzz_3 = 1000;
        let value: i32 = rug_fuzz_0;
        let result: i8 = AsPrimitive::<i8>::as_(value);
        debug_assert_eq!(result, 100i8);
        let value: i32 = -rug_fuzz_1;
        let result: i8 = AsPrimitive::<i8>::as_(value);
        debug_assert_eq!(result, - 100i8);
        let value: i32 = rug_fuzz_2;
        let result: i8 = AsPrimitive::<i8>::as_(value);
        debug_assert_eq!(result, - 24i8);
        let value: i32 = -rug_fuzz_3;
        let result: i8 = AsPrimitive::<i8>::as_(value);
        debug_assert_eq!(result, 24i8);
        let _rug_ed_tests_llm_16_828_llm_16_828_rrrruuuugggg_test_as_primitive_i32_to_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_829_llm_16_829 {
    use crate::AsPrimitive;
    #[test]
    fn test_as_primitive_i32_to_isize() {
        let _rug_st_tests_llm_16_829_llm_16_829_rrrruuuugggg_test_as_primitive_i32_to_isize = 0;
        let rug_fuzz_0 = 1234;
        let value_i32: i32 = rug_fuzz_0;
        let value_isize: isize = AsPrimitive::<isize>::as_(value_i32);
        debug_assert_eq!(value_isize, 1234isize);
        let _rug_ed_tests_llm_16_829_llm_16_829_rrrruuuugggg_test_as_primitive_i32_to_isize = 0;
    }
    #[test]
    fn test_as_primitive_i32_to_isize_negative() {
        let _rug_st_tests_llm_16_829_llm_16_829_rrrruuuugggg_test_as_primitive_i32_to_isize_negative = 0;
        let rug_fuzz_0 = 1234;
        let value_i32: i32 = -rug_fuzz_0;
        let value_isize: isize = AsPrimitive::<isize>::as_(value_i32);
        debug_assert_eq!(value_isize, - 1234isize);
        let _rug_ed_tests_llm_16_829_llm_16_829_rrrruuuugggg_test_as_primitive_i32_to_isize_negative = 0;
    }
    #[test]
    fn test_as_primitive_i32_to_isize_zero() {
        let _rug_st_tests_llm_16_829_llm_16_829_rrrruuuugggg_test_as_primitive_i32_to_isize_zero = 0;
        let rug_fuzz_0 = 0;
        let value_i32: i32 = rug_fuzz_0;
        let value_isize: isize = AsPrimitive::<isize>::as_(value_i32);
        debug_assert_eq!(value_isize, 0isize);
        let _rug_ed_tests_llm_16_829_llm_16_829_rrrruuuugggg_test_as_primitive_i32_to_isize_zero = 0;
    }
    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_as_primitive_i32_to_isize_overflow() {
        let _rug_st_tests_llm_16_829_llm_16_829_rrrruuuugggg_test_as_primitive_i32_to_isize_overflow = 0;
        let value_i32: i32 = i32::MAX;
        let _value_isize: isize = AsPrimitive::<isize>::as_(value_i32);
        let _rug_ed_tests_llm_16_829_llm_16_829_rrrruuuugggg_test_as_primitive_i32_to_isize_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_830_llm_16_830 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_i32_to_u128() {
        let _rug_st_tests_llm_16_830_llm_16_830_rrrruuuugggg_test_as_i32_to_u128 = 0;
        let rug_fuzz_0 = 42;
        let x: i32 = rug_fuzz_0;
        let y: u128 = AsPrimitive::<u128>::as_(x);
        debug_assert_eq!(y, 42u128);
        let _rug_ed_tests_llm_16_830_llm_16_830_rrrruuuugggg_test_as_i32_to_u128 = 0;
    }
    #[test]
    fn test_as_i32_to_u128_no_overflow() {
        let _rug_st_tests_llm_16_830_llm_16_830_rrrruuuugggg_test_as_i32_to_u128_no_overflow = 0;
        let x: i32 = i32::MAX;
        let y: u128 = AsPrimitive::<u128>::as_(x);
        debug_assert_eq!(y, i32::MAX as u128);
        let _rug_ed_tests_llm_16_830_llm_16_830_rrrruuuugggg_test_as_i32_to_u128_no_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_832_llm_16_832 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_i32_to_u32() {
        let _rug_st_tests_llm_16_832_llm_16_832_rrrruuuugggg_test_as_primitive_i32_to_u32 = 0;
        let rug_fuzz_0 = 42;
        let value_i32: i32 = rug_fuzz_0;
        let value_u32: u32 = AsPrimitive::<u32>::as_(value_i32);
        debug_assert_eq!(value_u32, 42u32);
        let _rug_ed_tests_llm_16_832_llm_16_832_rrrruuuugggg_test_as_primitive_i32_to_u32 = 0;
    }
    #[test]
    fn test_as_primitive_i32_to_u32_negative() {
        let _rug_st_tests_llm_16_832_llm_16_832_rrrruuuugggg_test_as_primitive_i32_to_u32_negative = 0;
        let rug_fuzz_0 = 42;
        let value_i32: i32 = -rug_fuzz_0;
        let value_u32: u32 = AsPrimitive::<u32>::as_(value_i32);
        debug_assert_eq!(value_u32, value_i32.wrapping_abs() as u32);
        let _rug_ed_tests_llm_16_832_llm_16_832_rrrruuuugggg_test_as_primitive_i32_to_u32_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_833_llm_16_833 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_i32_to_u64() {
        let _rug_st_tests_llm_16_833_llm_16_833_rrrruuuugggg_test_as_primitive_i32_to_u64 = 0;
        let rug_fuzz_0 = 12345;
        let val: i32 = rug_fuzz_0;
        let result: u64 = AsPrimitive::<u64>::as_(val);
        debug_assert_eq!(result, 12345u64);
        let _rug_ed_tests_llm_16_833_llm_16_833_rrrruuuugggg_test_as_primitive_i32_to_u64 = 0;
    }
    #[test]
    fn test_as_primitive_i32_to_u64_negative() {
        let _rug_st_tests_llm_16_833_llm_16_833_rrrruuuugggg_test_as_primitive_i32_to_u64_negative = 0;
        let rug_fuzz_0 = 12345;
        let val: i32 = -rug_fuzz_0;
        let _rug_ed_tests_llm_16_833_llm_16_833_rrrruuuugggg_test_as_primitive_i32_to_u64_negative = 0;
    }
    #[test]
    fn test_as_primitive_i32_to_u64_max() {
        let _rug_st_tests_llm_16_833_llm_16_833_rrrruuuugggg_test_as_primitive_i32_to_u64_max = 0;
        let val: i32 = i32::MAX;
        let result: u64 = AsPrimitive::<u64>::as_(val);
        debug_assert_eq!(result, i32::MAX as u64);
        let _rug_ed_tests_llm_16_833_llm_16_833_rrrruuuugggg_test_as_primitive_i32_to_u64_max = 0;
    }
    #[test]
    #[should_panic(expected = "attempt to cast to with overflow")]
    fn test_as_primitive_i32_to_u64_min() {
        let _rug_st_tests_llm_16_833_llm_16_833_rrrruuuugggg_test_as_primitive_i32_to_u64_min = 0;
        let val: i32 = i32::MIN;
        let result: u64 = AsPrimitive::<u64>::as_(val);
        debug_assert_eq!(result, i32::MIN as u64);
        let _rug_ed_tests_llm_16_833_llm_16_833_rrrruuuugggg_test_as_primitive_i32_to_u64_min = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_835_llm_16_835 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_i32_to_usize() {
        let _rug_st_tests_llm_16_835_llm_16_835_rrrruuuugggg_test_as_i32_to_usize = 0;
        let rug_fuzz_0 = 42;
        let value: i32 = rug_fuzz_0;
        let result: usize = AsPrimitive::<usize>::as_(value);
        debug_assert_eq!(result, 42usize);
        let _rug_ed_tests_llm_16_835_llm_16_835_rrrruuuugggg_test_as_i32_to_usize = 0;
    }
    #[test]
    #[should_panic(expected = "attempt to cast to usize with overflow")]
    fn test_as_negative_i32_to_usize() {
        let _rug_st_tests_llm_16_835_llm_16_835_rrrruuuugggg_test_as_negative_i32_to_usize = 0;
        let rug_fuzz_0 = 42;
        let value: i32 = -rug_fuzz_0;
        let _: usize = AsPrimitive::<usize>::as_(value);
        let _rug_ed_tests_llm_16_835_llm_16_835_rrrruuuugggg_test_as_negative_i32_to_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_836_llm_16_836 {
    use super::*;
    use crate::*;
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_f32() {
        let _rug_st_tests_llm_16_836_llm_16_836_rrrruuuugggg_test_from_f32 = 0;
        let rug_fuzz_0 = 42.0_f32;
        let rug_fuzz_1 = 42.999_f32;
        let rug_fuzz_2 = 42.0_f32;
        let rug_fuzz_3 = 2.1e10_f32;
        let rug_fuzz_4 = 2.1e10_f32;
        let rug_fuzz_5 = 0.0_f32;
        debug_assert_eq!(< i32 as FromPrimitive > ::from_f32(rug_fuzz_0), Some(42));
        debug_assert_eq!(< i32 as FromPrimitive > ::from_f32(rug_fuzz_1), Some(42));
        debug_assert_eq!(< i32 as FromPrimitive > ::from_f32(- rug_fuzz_2), Some(- 42));
        debug_assert_eq!(< i32 as FromPrimitive > ::from_f32(- rug_fuzz_3), None);
        debug_assert_eq!(< i32 as FromPrimitive > ::from_f32(rug_fuzz_4), None);
        debug_assert_eq!(< i32 as FromPrimitive > ::from_f32(rug_fuzz_5), Some(0));
        debug_assert_eq!(< i32 as FromPrimitive > ::from_f32(f32::INFINITY), None);
        debug_assert_eq!(< i32 as FromPrimitive > ::from_f32(f32::NEG_INFINITY), None);
        debug_assert!(< i32 as FromPrimitive > ::from_f32(f32::NAN).is_none());
        let _rug_ed_tests_llm_16_836_llm_16_836_rrrruuuugggg_test_from_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_837_llm_16_837 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_f64_to_i32() {
        let _rug_st_tests_llm_16_837_llm_16_837_rrrruuuugggg_test_from_f64_to_i32 = 0;
        let rug_fuzz_0 = 42.0;
        let rug_fuzz_1 = 42.999;
        let rug_fuzz_2 = 42.0;
        debug_assert_eq!(< i32 as FromPrimitive > ::from_f64(rug_fuzz_0), Some(42));
        debug_assert_eq!(< i32 as FromPrimitive > ::from_f64(rug_fuzz_1), Some(42));
        debug_assert_eq!(< i32 as FromPrimitive > ::from_f64(- rug_fuzz_2), Some(- 42));
        debug_assert_eq!(< i32 as FromPrimitive > ::from_f64(f64::NAN), None);
        debug_assert_eq!(< i32 as FromPrimitive > ::from_f64(f64::INFINITY), None);
        debug_assert_eq!(< i32 as FromPrimitive > ::from_f64(f64::NEG_INFINITY), None);
        debug_assert_eq!(< i32 as FromPrimitive > ::from_f64(f64::MAX), None);
        debug_assert_eq!(< i32 as FromPrimitive > ::from_f64(f64::MIN), None);
        let _rug_ed_tests_llm_16_837_llm_16_837_rrrruuuugggg_test_from_f64_to_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_838_llm_16_838 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i128() {
        let _rug_st_tests_llm_16_838_llm_16_838_rrrruuuugggg_test_from_i128 = 0;
        let rug_fuzz_0 = 0i128;
        let rug_fuzz_1 = 2147483647i128;
        let rug_fuzz_2 = 2147483648i128;
        let rug_fuzz_3 = 2147483648i128;
        let rug_fuzz_4 = 2147483649i128;
        debug_assert_eq!(< i32 as FromPrimitive > ::from_i128(rug_fuzz_0), Some(0i32));
        debug_assert_eq!(
            < i32 as FromPrimitive > ::from_i128(rug_fuzz_1), Some(2147483647i32)
        );
        debug_assert_eq!(
            < i32 as FromPrimitive > ::from_i128(- rug_fuzz_2), Some(- 2147483648i32)
        );
        debug_assert_eq!(< i32 as FromPrimitive > ::from_i128(rug_fuzz_3), None);
        debug_assert_eq!(< i32 as FromPrimitive > ::from_i128(- rug_fuzz_4), None);
        let _rug_ed_tests_llm_16_838_llm_16_838_rrrruuuugggg_test_from_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_839_llm_16_839 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i16() {
        let _rug_st_tests_llm_16_839_llm_16_839_rrrruuuugggg_test_from_i16 = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        debug_assert_eq!(< i32 as FromPrimitive > ::from_i16(rug_fuzz_0), Some(0i32));
        debug_assert_eq!(
            < i32 as FromPrimitive > ::from_i16(- rug_fuzz_1), Some(- 1i32)
        );
        debug_assert_eq!(
            < i32 as FromPrimitive > ::from_i16(i16::MAX), Some(i16::MAX as i32)
        );
        debug_assert_eq!(
            < i32 as FromPrimitive > ::from_i16(i16::MIN), Some(i16::MIN as i32)
        );
        let _rug_ed_tests_llm_16_839_llm_16_839_rrrruuuugggg_test_from_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_840_llm_16_840 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i32() {
        let _rug_st_tests_llm_16_840_llm_16_840_rrrruuuugggg_test_from_i32 = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 1;
        debug_assert_eq!(< u32 as FromPrimitive > ::from_i32(rug_fuzz_0), Some(0u32));
        debug_assert_eq!(< u32 as FromPrimitive > ::from_i32(- rug_fuzz_1), None);
        debug_assert_eq!(
            < u32 as FromPrimitive > ::from_i32(i32::MAX), Some(i32::MAX as u32)
        );
        debug_assert_eq!(< i64 as FromPrimitive > ::from_i32(rug_fuzz_2), Some(0i64));
        debug_assert_eq!(
            < i64 as FromPrimitive > ::from_i32(- rug_fuzz_3), Some(- 1i64)
        );
        debug_assert_eq!(
            < i64 as FromPrimitive > ::from_i32(i32::MAX), Some(i32::MAX as i64)
        );
        debug_assert_eq!(< f32 as FromPrimitive > ::from_i32(rug_fuzz_4), Some(0.0f32));
        debug_assert_eq!(
            < f32 as FromPrimitive > ::from_i32(- rug_fuzz_5), Some(- 1.0f32)
        );
        debug_assert_eq!(
            < f32 as FromPrimitive > ::from_i32(i32::MAX), Some(i32::MAX as f32)
        );
        let _rug_ed_tests_llm_16_840_llm_16_840_rrrruuuugggg_test_from_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_841_llm_16_841 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i64() {
        let _rug_st_tests_llm_16_841_llm_16_841_rrrruuuugggg_test_from_i64 = 0;
        let rug_fuzz_0 = 0i64;
        let rug_fuzz_1 = 1i64;
        let rug_fuzz_2 = 1i64;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 1;
        debug_assert_eq!(< i32 as FromPrimitive > ::from_i64(rug_fuzz_0), Some(0i32));
        debug_assert_eq!(< i32 as FromPrimitive > ::from_i64(i64::MAX), None);
        debug_assert_eq!(< i32 as FromPrimitive > ::from_i64(i64::MIN), None);
        debug_assert_eq!(< i32 as FromPrimitive > ::from_i64(rug_fuzz_1), Some(1i32));
        debug_assert_eq!(
            < i32 as FromPrimitive > ::from_i64(- rug_fuzz_2), Some(- 1i32)
        );
        debug_assert_eq!(
            < i32 as FromPrimitive > ::from_i64(i32::MAX as i64), Some(i32::MAX)
        );
        debug_assert_eq!(
            < i32 as FromPrimitive > ::from_i64((i32::MIN as i64) - rug_fuzz_3), None
        );
        debug_assert_eq!(
            < i32 as FromPrimitive > ::from_i64((i32::MAX as i64) + rug_fuzz_4), None
        );
        let _rug_ed_tests_llm_16_841_llm_16_841_rrrruuuugggg_test_from_i64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_842_llm_16_842 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i8_positive() {
        let _rug_st_tests_llm_16_842_llm_16_842_rrrruuuugggg_test_from_i8_positive = 0;
        let rug_fuzz_0 = 42;
        let value: i8 = rug_fuzz_0;
        let result: Option<i32> = <i32 as FromPrimitive>::from_i8(value);
        debug_assert_eq!(result, Some(42i32));
        let _rug_ed_tests_llm_16_842_llm_16_842_rrrruuuugggg_test_from_i8_positive = 0;
    }
    #[test]
    fn test_from_i8_negative() {
        let _rug_st_tests_llm_16_842_llm_16_842_rrrruuuugggg_test_from_i8_negative = 0;
        let rug_fuzz_0 = 42;
        let value: i8 = -rug_fuzz_0;
        let result: Option<i32> = <i32 as FromPrimitive>::from_i8(value);
        debug_assert_eq!(result, Some(- 42i32));
        let _rug_ed_tests_llm_16_842_llm_16_842_rrrruuuugggg_test_from_i8_negative = 0;
    }
    #[test]
    fn test_from_i8_zero() {
        let _rug_st_tests_llm_16_842_llm_16_842_rrrruuuugggg_test_from_i8_zero = 0;
        let rug_fuzz_0 = 0;
        let value: i8 = rug_fuzz_0;
        let result: Option<i32> = <i32 as FromPrimitive>::from_i8(value);
        debug_assert_eq!(result, Some(0i32));
        let _rug_ed_tests_llm_16_842_llm_16_842_rrrruuuugggg_test_from_i8_zero = 0;
    }
    #[test]
    fn test_from_i8_min() {
        let _rug_st_tests_llm_16_842_llm_16_842_rrrruuuugggg_test_from_i8_min = 0;
        let value: i8 = i8::MIN;
        let result: Option<i32> = <i32 as FromPrimitive>::from_i8(value);
        debug_assert_eq!(result, Some(i32::from(i8::MIN)));
        let _rug_ed_tests_llm_16_842_llm_16_842_rrrruuuugggg_test_from_i8_min = 0;
    }
    #[test]
    fn test_from_i8_max() {
        let _rug_st_tests_llm_16_842_llm_16_842_rrrruuuugggg_test_from_i8_max = 0;
        let value: i8 = i8::MAX;
        let result: Option<i32> = <i32 as FromPrimitive>::from_i8(value);
        debug_assert_eq!(result, Some(i32::from(i8::MAX)));
        let _rug_ed_tests_llm_16_842_llm_16_842_rrrruuuugggg_test_from_i8_max = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_843_llm_16_843 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_isize_within_bounds() {
        let _rug_st_tests_llm_16_843_llm_16_843_rrrruuuugggg_test_from_isize_within_bounds = 0;
        let rug_fuzz_0 = 42;
        let num: Option<i32> = <i32 as FromPrimitive>::from_isize(rug_fuzz_0);
        debug_assert_eq!(num, Some(42));
        let _rug_ed_tests_llm_16_843_llm_16_843_rrrruuuugggg_test_from_isize_within_bounds = 0;
    }
    #[test]
    fn test_from_isize_below_bounds() {
        let _rug_st_tests_llm_16_843_llm_16_843_rrrruuuugggg_test_from_isize_below_bounds = 0;
        let num: Option<i32> = <i32 as FromPrimitive>::from_isize(isize::min_value());
        let expected = if isize::min_value() as i64 >= i32::min_value() as i64 {
            Some(isize::min_value() as i32)
        } else {
            None
        };
        debug_assert_eq!(num, expected);
        let _rug_ed_tests_llm_16_843_llm_16_843_rrrruuuugggg_test_from_isize_below_bounds = 0;
    }
    #[test]
    fn test_from_isize_above_bounds() {
        let _rug_st_tests_llm_16_843_llm_16_843_rrrruuuugggg_test_from_isize_above_bounds = 0;
        let num: Option<i32> = <i32 as FromPrimitive>::from_isize(isize::max_value());
        let expected = if isize::max_value() as i64 <= i32::max_value() as i64 {
            Some(isize::max_value() as i32)
        } else {
            None
        };
        debug_assert_eq!(num, expected);
        let _rug_ed_tests_llm_16_843_llm_16_843_rrrruuuugggg_test_from_isize_above_bounds = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_844_llm_16_844 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u128() {
        let _rug_st_tests_llm_16_844_llm_16_844_rrrruuuugggg_test_from_u128 = 0;
        let rug_fuzz_0 = 0_u128;
        let rug_fuzz_1 = 2147483647_u128;
        let rug_fuzz_2 = 2147483648_u128;
        debug_assert_eq!(< i32 as FromPrimitive > ::from_u128(rug_fuzz_0), Some(0_i32));
        debug_assert_eq!(
            < i32 as FromPrimitive > ::from_u128(rug_fuzz_1), Some(2147483647_i32)
        );
        debug_assert_eq!(< i32 as FromPrimitive > ::from_u128(rug_fuzz_2), None);
        debug_assert_eq!(< i32 as FromPrimitive > ::from_u128(u128::MAX), None);
        let _rug_ed_tests_llm_16_844_llm_16_844_rrrruuuugggg_test_from_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_845_llm_16_845 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u16() {
        let _rug_st_tests_llm_16_845_llm_16_845_rrrruuuugggg_test_from_u16 = 0;
        let rug_fuzz_0 = 0u16;
        let rug_fuzz_1 = 100u16;
        debug_assert_eq!(< i32 as FromPrimitive > ::from_u16(rug_fuzz_0), Some(0i32));
        debug_assert_eq!(< i32 as FromPrimitive > ::from_u16(rug_fuzz_1), Some(100i32));
        debug_assert_eq!(< i32 as FromPrimitive > ::from_u16(u16::MAX), Some(65535i32));
        let _rug_ed_tests_llm_16_845_llm_16_845_rrrruuuugggg_test_from_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_846_llm_16_846 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u32() {
        let _rug_st_tests_llm_16_846_llm_16_846_rrrruuuugggg_test_from_u32 = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        debug_assert_eq!(< i32 as FromPrimitive > ::from_u32(rug_fuzz_0), Some(0i32));
        debug_assert_eq!(< i32 as FromPrimitive > ::from_u32(u32::MAX), None);
        debug_assert_eq!(< i32 as FromPrimitive > ::from_u32(rug_fuzz_1), Some(1i32));
        debug_assert_eq!(
            < i32 as FromPrimitive > ::from_u32(i32::MAX as u32), Some(i32::MAX)
        );
        debug_assert_eq!(
            < i32 as FromPrimitive > ::from_u32(i32::MAX.wrapping_add(rug_fuzz_2) as
            u32), None
        );
        let _rug_ed_tests_llm_16_846_llm_16_846_rrrruuuugggg_test_from_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_847_llm_16_847 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u64() {
        let _rug_st_tests_llm_16_847_llm_16_847_rrrruuuugggg_test_from_u64 = 0;
        let rug_fuzz_0 = 0_u64;
        let rug_fuzz_1 = 2_u64;
        let rug_fuzz_2 = 31;
        let rug_fuzz_3 = 2_u64;
        let rug_fuzz_4 = 31;
        let rug_fuzz_5 = 1;
        let rug_fuzz_6 = 2_u64;
        let rug_fuzz_7 = 31;
        let rug_fuzz_8 = 1;
        debug_assert_eq!(< i32 as FromPrimitive > ::from_u64(rug_fuzz_0), Some(0_i32));
        debug_assert_eq!(
            < i32 as FromPrimitive > ::from_u64(rug_fuzz_1.pow(rug_fuzz_2)), Some(2_i32
            .pow(31))
        );
        debug_assert_eq!(
            < i32 as FromPrimitive > ::from_u64(rug_fuzz_3.pow(rug_fuzz_4) - rug_fuzz_5),
            Some((2_i32.pow(31) - 1))
        );
        debug_assert_eq!(
            < i32 as FromPrimitive > ::from_u64(rug_fuzz_6.pow(rug_fuzz_7) + rug_fuzz_8),
            None
        );
        debug_assert_eq!(< i32 as FromPrimitive > ::from_u64(u64::MAX), None);
        let _rug_ed_tests_llm_16_847_llm_16_847_rrrruuuugggg_test_from_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_848_llm_16_848 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u8() {
        let _rug_st_tests_llm_16_848_llm_16_848_rrrruuuugggg_test_from_u8 = 0;
        let rug_fuzz_0 = 0u8;
        let rug_fuzz_1 = 255u8;
        let rug_fuzz_2 = 100u8;
        debug_assert_eq!(< i32 as FromPrimitive > ::from_u8(rug_fuzz_0), Some(0i32));
        debug_assert_eq!(< i32 as FromPrimitive > ::from_u8(rug_fuzz_1), Some(255i32));
        debug_assert_eq!(< i32 as FromPrimitive > ::from_u8(rug_fuzz_2), Some(100i32));
        let _rug_ed_tests_llm_16_848_llm_16_848_rrrruuuugggg_test_from_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_849_llm_16_849 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_usize() {
        let _rug_st_tests_llm_16_849_llm_16_849_rrrruuuugggg_test_from_usize = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 123;
        let value_within_bounds: usize = i32::MAX as usize;
        debug_assert_eq!(
            < i32 as FromPrimitive > ::from_usize(value_within_bounds), Some(i32::MAX)
        );
        let value_out_of_bounds: usize = (i32::MAX as usize).wrapping_add(rug_fuzz_0);
        debug_assert_eq!(
            < i32 as FromPrimitive > ::from_usize(value_out_of_bounds), None
        );
        debug_assert_eq!(< i32 as FromPrimitive > ::from_usize(rug_fuzz_1), Some(0));
        let typical_value: usize = rug_fuzz_2;
        debug_assert_eq!(
            < i32 as FromPrimitive > ::from_usize(typical_value), Some(123)
        );
        let _rug_ed_tests_llm_16_849_llm_16_849_rrrruuuugggg_test_from_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_850_llm_16_850 {
    use crate::NumCast;
    use std::num::Wrapping;
    #[test]
    fn test_wrapping_cast_to_i32() {
        let _rug_st_tests_llm_16_850_llm_16_850_rrrruuuugggg_test_wrapping_cast_to_i32 = 0;
        let rug_fuzz_0 = 42i64;
        let w_i64 = Wrapping(rug_fuzz_0);
        let w_i32 = <i32 as NumCast>::from(w_i64);
        debug_assert_eq!(w_i32, Some(42i32));
        let w_i64 = Wrapping(i64::MAX);
        let w_i32 = <i32 as NumCast>::from(w_i64);
        debug_assert_eq!(w_i32, None);
        let w_i64 = Wrapping(i64::MIN);
        let w_i32 = <i32 as NumCast>::from(w_i64);
        debug_assert_eq!(w_i32, None);
        let _rug_ed_tests_llm_16_850_llm_16_850_rrrruuuugggg_test_wrapping_cast_to_i32 = 0;
    }
    #[test]
    fn test_wrapping_cast_to_i32_from_usize() {
        let _rug_st_tests_llm_16_850_llm_16_850_rrrruuuugggg_test_wrapping_cast_to_i32_from_usize = 0;
        let rug_fuzz_0 = 4;
        let rug_fuzz_1 = 8;
        let w_usize = Wrapping(usize::MAX);
        let w_i32 = <i32 as NumCast>::from(w_usize);
        match std::mem::size_of::<usize>() {
            rug_fuzz_0 => debug_assert_eq!(w_i32, Some(i32::MAX)),
            rug_fuzz_1 => debug_assert_eq!(w_i32, None),
            _ => unreachable!(),
        };
        let _rug_ed_tests_llm_16_850_llm_16_850_rrrruuuugggg_test_wrapping_cast_to_i32_from_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_851 {
    use super::*;
    use crate::*;
    #[test]
    fn test_to_f32() {
        let _rug_st_tests_llm_16_851_rrrruuuugggg_test_to_f32 = 0;
        let rug_fuzz_0 = 123i32;
        let value_i32 = rug_fuzz_0;
        let value_f32 = value_i32.to_f32();
        debug_assert_eq!(value_f32, Some(123f32));
        let _rug_ed_tests_llm_16_851_rrrruuuugggg_test_to_f32 = 0;
    }
    #[test]
    fn test_to_f32_negative() {
        let _rug_st_tests_llm_16_851_rrrruuuugggg_test_to_f32_negative = 0;
        let rug_fuzz_0 = 123i32;
        let value_i32 = -rug_fuzz_0;
        let value_f32 = value_i32.to_f32();
        debug_assert_eq!(value_f32, Some(- 123f32));
        let _rug_ed_tests_llm_16_851_rrrruuuugggg_test_to_f32_negative = 0;
    }
    #[test]
    fn test_to_f32_zero() {
        let _rug_st_tests_llm_16_851_rrrruuuugggg_test_to_f32_zero = 0;
        let rug_fuzz_0 = 0i32;
        let value_i32 = rug_fuzz_0;
        let value_f32 = value_i32.to_f32();
        debug_assert_eq!(value_f32, Some(0f32));
        let _rug_ed_tests_llm_16_851_rrrruuuugggg_test_to_f32_zero = 0;
    }
    #[test]
    fn test_to_f32_max() {
        let _rug_st_tests_llm_16_851_rrrruuuugggg_test_to_f32_max = 0;
        let value_i32 = i32::MAX;
        let value_f32 = value_i32.to_f32();
        debug_assert_eq!(value_f32, Some(i32::MAX as f32));
        let _rug_ed_tests_llm_16_851_rrrruuuugggg_test_to_f32_max = 0;
    }
    #[test]
    fn test_to_f32_min() {
        let _rug_st_tests_llm_16_851_rrrruuuugggg_test_to_f32_min = 0;
        let value_i32 = i32::MIN;
        let value_f32 = value_i32.to_f32();
        debug_assert_eq!(value_f32, Some(i32::MIN as f32));
        let _rug_ed_tests_llm_16_851_rrrruuuugggg_test_to_f32_min = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_852_llm_16_852 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_f64() {
        let _rug_st_tests_llm_16_852_llm_16_852_rrrruuuugggg_test_to_f64 = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = 0;
        let val: i32 = rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_f64(& val), Some(42.0_f64));
        let val: i32 = -rug_fuzz_1;
        debug_assert_eq!(ToPrimitive::to_f64(& val), Some(- 42.0_f64));
        let val: i32 = rug_fuzz_2;
        debug_assert_eq!(ToPrimitive::to_f64(& val), Some(0.0_f64));
        let _rug_ed_tests_llm_16_852_llm_16_852_rrrruuuugggg_test_to_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_853_llm_16_853 {
    use crate::cast::ToPrimitive;
    #[test]
    fn i32_to_i128_within_bounds() {
        let _rug_st_tests_llm_16_853_llm_16_853_rrrruuuugggg_i32_to_i128_within_bounds = 0;
        let rug_fuzz_0 = 0;
        debug_assert_eq!(ToPrimitive::to_i128(& rug_fuzz_0), Some(0i128));
        debug_assert_eq!(ToPrimitive::to_i128(& i32::MAX), Some(i32::MAX as i128));
        debug_assert_eq!(ToPrimitive::to_i128(& i32::MIN), Some(i32::MIN as i128));
        let _rug_ed_tests_llm_16_853_llm_16_853_rrrruuuugggg_i32_to_i128_within_bounds = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_854_llm_16_854 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_i16_with_i32_in_range() {
        let _rug_st_tests_llm_16_854_llm_16_854_rrrruuuugggg_test_to_i16_with_i32_in_range = 0;
        let rug_fuzz_0 = 32767;
        let value: i32 = rug_fuzz_0;
        let result = value.to_i16();
        debug_assert_eq!(result, Some(32767i16));
        let _rug_ed_tests_llm_16_854_llm_16_854_rrrruuuugggg_test_to_i16_with_i32_in_range = 0;
    }
    #[test]
    fn test_to_i16_with_i32_at_upper_bound() {
        let _rug_st_tests_llm_16_854_llm_16_854_rrrruuuugggg_test_to_i16_with_i32_at_upper_bound = 0;
        let value: i32 = i16::MAX as i32;
        let result = value.to_i16();
        debug_assert_eq!(result, Some(i16::MAX));
        let _rug_ed_tests_llm_16_854_llm_16_854_rrrruuuugggg_test_to_i16_with_i32_at_upper_bound = 0;
    }
    #[test]
    fn test_to_i16_with_i32_at_lower_bound() {
        let _rug_st_tests_llm_16_854_llm_16_854_rrrruuuugggg_test_to_i16_with_i32_at_lower_bound = 0;
        let value: i32 = i16::MIN as i32;
        let result = value.to_i16();
        debug_assert_eq!(result, Some(i16::MIN));
        let _rug_ed_tests_llm_16_854_llm_16_854_rrrruuuugggg_test_to_i16_with_i32_at_lower_bound = 0;
    }
    #[test]
    fn test_to_i16_with_i32_above_upper_bound() {
        let _rug_st_tests_llm_16_854_llm_16_854_rrrruuuugggg_test_to_i16_with_i32_above_upper_bound = 0;
        let rug_fuzz_0 = 1;
        let value: i32 = (i16::MAX as i32) + rug_fuzz_0;
        let result = value.to_i16();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_854_llm_16_854_rrrruuuugggg_test_to_i16_with_i32_above_upper_bound = 0;
    }
    #[test]
    fn test_to_i16_with_i32_below_lower_bound() {
        let _rug_st_tests_llm_16_854_llm_16_854_rrrruuuugggg_test_to_i16_with_i32_below_lower_bound = 0;
        let rug_fuzz_0 = 1;
        let value: i32 = (i16::MIN as i32) - rug_fuzz_0;
        let result = value.to_i16();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_854_llm_16_854_rrrruuuugggg_test_to_i16_with_i32_below_lower_bound = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_855 {
    use crate::cast::ToPrimitive;
    #[test]
    fn to_i32_with_i32() {
        let _rug_st_tests_llm_16_855_rrrruuuugggg_to_i32_with_i32 = 0;
        let rug_fuzz_0 = 5;
        let x: i32 = rug_fuzz_0;
        debug_assert_eq!(< i32 as ToPrimitive > ::to_i32(& x), Some(x));
        let _rug_ed_tests_llm_16_855_rrrruuuugggg_to_i32_with_i32 = 0;
    }
    #[test]
    fn to_i32_with_i64_in_bounds() {
        let _rug_st_tests_llm_16_855_rrrruuuugggg_to_i32_with_i64_in_bounds = 0;
        let x: i64 = i32::MAX as i64;
        debug_assert_eq!(< i64 as ToPrimitive > ::to_i32(& x), Some(x as i32));
        let _rug_ed_tests_llm_16_855_rrrruuuugggg_to_i32_with_i64_in_bounds = 0;
    }
    #[test]
    fn to_i32_with_i64_out_of_bounds() {
        let _rug_st_tests_llm_16_855_rrrruuuugggg_to_i32_with_i64_out_of_bounds = 0;
        let x: i64 = i64::MAX;
        debug_assert_eq!(< i64 as ToPrimitive > ::to_i32(& x), None);
        let _rug_ed_tests_llm_16_855_rrrruuuugggg_to_i32_with_i64_out_of_bounds = 0;
    }
    #[test]
    fn to_i32_with_u64_in_bounds() {
        let _rug_st_tests_llm_16_855_rrrruuuugggg_to_i32_with_u64_in_bounds = 0;
        let x: u64 = i32::MAX as u64;
        debug_assert_eq!(< u64 as ToPrimitive > ::to_i32(& x), Some(x as i32));
        let _rug_ed_tests_llm_16_855_rrrruuuugggg_to_i32_with_u64_in_bounds = 0;
    }
    #[test]
    fn to_i32_with_u64_out_of_bounds() {
        let _rug_st_tests_llm_16_855_rrrruuuugggg_to_i32_with_u64_out_of_bounds = 0;
        let x: u64 = u64::MAX;
        debug_assert_eq!(< u64 as ToPrimitive > ::to_i32(& x), None);
        let _rug_ed_tests_llm_16_855_rrrruuuugggg_to_i32_with_u64_out_of_bounds = 0;
    }
    #[test]
    fn to_i32_with_f64_in_bounds() {
        let _rug_st_tests_llm_16_855_rrrruuuugggg_to_i32_with_f64_in_bounds = 0;
        let rug_fuzz_0 = 5.0;
        let x: f64 = rug_fuzz_0;
        debug_assert_eq!(< f64 as ToPrimitive > ::to_i32(& x), Some(x as i32));
        let _rug_ed_tests_llm_16_855_rrrruuuugggg_to_i32_with_f64_in_bounds = 0;
    }
    #[test]
    fn to_i32_with_f64_out_of_bounds() {
        let _rug_st_tests_llm_16_855_rrrruuuugggg_to_i32_with_f64_out_of_bounds = 0;
        let x: f64 = f64::MAX;
        debug_assert_eq!(< f64 as ToPrimitive > ::to_i32(& x), None);
        let _rug_ed_tests_llm_16_855_rrrruuuugggg_to_i32_with_f64_out_of_bounds = 0;
    }
    #[test]
    fn to_i32_with_f64_negative_in_bounds() {
        let _rug_st_tests_llm_16_855_rrrruuuugggg_to_i32_with_f64_negative_in_bounds = 0;
        let rug_fuzz_0 = 5.0;
        let x: f64 = -rug_fuzz_0;
        debug_assert_eq!(< f64 as ToPrimitive > ::to_i32(& x), Some(x as i32));
        let _rug_ed_tests_llm_16_855_rrrruuuugggg_to_i32_with_f64_negative_in_bounds = 0;
    }
    #[test]
    fn to_i32_with_f64_negative_out_of_bounds() {
        let _rug_st_tests_llm_16_855_rrrruuuugggg_to_i32_with_f64_negative_out_of_bounds = 0;
        let x: f64 = -f64::MAX;
        debug_assert_eq!(< f64 as ToPrimitive > ::to_i32(& x), None);
        let _rug_ed_tests_llm_16_855_rrrruuuugggg_to_i32_with_f64_negative_out_of_bounds = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_856_llm_16_856 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_i64_with_i32() {
        let _rug_st_tests_llm_16_856_llm_16_856_rrrruuuugggg_test_to_i64_with_i32 = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 123;
        let rug_fuzz_2 = 123;
        let min_i32 = i32::MIN;
        let max_i32 = i32::MAX;
        let min_i32_as_i64: Option<i64> = min_i32.to_i64();
        let max_i32_as_i64: Option<i64> = max_i32.to_i64();
        debug_assert_eq!(min_i32_as_i64, Some(i32::MIN as i64));
        debug_assert_eq!(max_i32_as_i64, Some(i32::MAX as i64));
        let zero_i32: i32 = rug_fuzz_0;
        let zero_i32_as_i64: Option<i64> = zero_i32.to_i64();
        debug_assert_eq!(zero_i32_as_i64, Some(0));
        let positive_i32: i32 = rug_fuzz_1;
        let positive_i32_as_i64: Option<i64> = positive_i32.to_i64();
        debug_assert_eq!(positive_i32_as_i64, Some(123));
        let negative_i32: i32 = -rug_fuzz_2;
        let negative_i32_as_i64: Option<i64> = negative_i32.to_i64();
        debug_assert_eq!(negative_i32_as_i64, Some(- 123));
        let _rug_ed_tests_llm_16_856_llm_16_856_rrrruuuugggg_test_to_i64_with_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_857_llm_16_857 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_i8_with_in_range_value() {
        let _rug_st_tests_llm_16_857_llm_16_857_rrrruuuugggg_test_to_i8_with_in_range_value = 0;
        let rug_fuzz_0 = 100;
        let x: i32 = rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_i8(& x), Some(100i8));
        let _rug_ed_tests_llm_16_857_llm_16_857_rrrruuuugggg_test_to_i8_with_in_range_value = 0;
    }
    #[test]
    fn test_to_i8_with_value_too_large() {
        let _rug_st_tests_llm_16_857_llm_16_857_rrrruuuugggg_test_to_i8_with_value_too_large = 0;
        let rug_fuzz_0 = 1000;
        let x: i32 = rug_fuzz_0;
        debug_assert!(ToPrimitive::to_i8(& x).is_none());
        let _rug_ed_tests_llm_16_857_llm_16_857_rrrruuuugggg_test_to_i8_with_value_too_large = 0;
    }
    #[test]
    fn test_to_i8_with_value_too_small() {
        let _rug_st_tests_llm_16_857_llm_16_857_rrrruuuugggg_test_to_i8_with_value_too_small = 0;
        let rug_fuzz_0 = 1000;
        let x: i32 = -rug_fuzz_0;
        debug_assert!(ToPrimitive::to_i8(& x).is_none());
        let _rug_ed_tests_llm_16_857_llm_16_857_rrrruuuugggg_test_to_i8_with_value_too_small = 0;
    }
    #[test]
    fn test_to_i8_with_max_value() {
        let _rug_st_tests_llm_16_857_llm_16_857_rrrruuuugggg_test_to_i8_with_max_value = 0;
        let x: i32 = i8::MAX as i32;
        debug_assert_eq!(ToPrimitive::to_i8(& x), Some(i8::MAX));
        let _rug_ed_tests_llm_16_857_llm_16_857_rrrruuuugggg_test_to_i8_with_max_value = 0;
    }
    #[test]
    fn test_to_i8_with_min_value() {
        let _rug_st_tests_llm_16_857_llm_16_857_rrrruuuugggg_test_to_i8_with_min_value = 0;
        let x: i32 = i8::MIN as i32;
        debug_assert_eq!(ToPrimitive::to_i8(& x), Some(i8::MIN));
        let _rug_ed_tests_llm_16_857_llm_16_857_rrrruuuugggg_test_to_i8_with_min_value = 0;
    }
    #[test]
    fn test_to_i8_with_zero() {
        let _rug_st_tests_llm_16_857_llm_16_857_rrrruuuugggg_test_to_i8_with_zero = 0;
        let rug_fuzz_0 = 0;
        let x: i32 = rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_i8(& x), Some(0i8));
        let _rug_ed_tests_llm_16_857_llm_16_857_rrrruuuugggg_test_to_i8_with_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_860_llm_16_860 {
    use crate::ToPrimitive;
    #[test]
    fn test_to_u16_within_range() {
        let _rug_st_tests_llm_16_860_llm_16_860_rrrruuuugggg_test_to_u16_within_range = 0;
        let rug_fuzz_0 = 100;
        let a: i32 = rug_fuzz_0;
        debug_assert_eq!(a.to_u16(), Some(100u16));
        let _rug_ed_tests_llm_16_860_llm_16_860_rrrruuuugggg_test_to_u16_within_range = 0;
    }
    #[test]
    fn test_to_u16_negative_number() {
        let _rug_st_tests_llm_16_860_llm_16_860_rrrruuuugggg_test_to_u16_negative_number = 0;
        let rug_fuzz_0 = 100;
        let b: i32 = -rug_fuzz_0;
        debug_assert_eq!(b.to_u16(), None);
        let _rug_ed_tests_llm_16_860_llm_16_860_rrrruuuugggg_test_to_u16_negative_number = 0;
    }
    #[test]
    fn test_to_u16_at_max() {
        let _rug_st_tests_llm_16_860_llm_16_860_rrrruuuugggg_test_to_u16_at_max = 0;
        let c: i32 = u16::MAX as i32;
        debug_assert_eq!(c.to_u16(), Some(u16::MAX));
        let _rug_ed_tests_llm_16_860_llm_16_860_rrrruuuugggg_test_to_u16_at_max = 0;
    }
    #[test]
    fn test_to_u16_beyond_max() {
        let _rug_st_tests_llm_16_860_llm_16_860_rrrruuuugggg_test_to_u16_beyond_max = 0;
        let rug_fuzz_0 = 1;
        let d: i32 = (u16::MAX as i32) + rug_fuzz_0;
        debug_assert_eq!(d.to_u16(), None);
        let _rug_ed_tests_llm_16_860_llm_16_860_rrrruuuugggg_test_to_u16_beyond_max = 0;
    }
    #[test]
    fn test_to_u16_on_zero() {
        let _rug_st_tests_llm_16_860_llm_16_860_rrrruuuugggg_test_to_u16_on_zero = 0;
        let rug_fuzz_0 = 0;
        let e: i32 = rug_fuzz_0;
        debug_assert_eq!(e.to_u16(), Some(0u16));
        let _rug_ed_tests_llm_16_860_llm_16_860_rrrruuuugggg_test_to_u16_on_zero = 0;
    }
    #[test]
    fn test_to_u16_on_max_i32() {
        let _rug_st_tests_llm_16_860_llm_16_860_rrrruuuugggg_test_to_u16_on_max_i32 = 0;
        let f: i32 = i32::MAX;
        debug_assert_eq!(f.to_u16(), None);
        let _rug_ed_tests_llm_16_860_llm_16_860_rrrruuuugggg_test_to_u16_on_max_i32 = 0;
    }
    #[test]
    fn test_to_u16_on_min_i32() {
        let _rug_st_tests_llm_16_860_llm_16_860_rrrruuuugggg_test_to_u16_on_min_i32 = 0;
        let g: i32 = i32::MIN;
        debug_assert_eq!(g.to_u16(), None);
        let _rug_ed_tests_llm_16_860_llm_16_860_rrrruuuugggg_test_to_u16_on_min_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_861_llm_16_861 {
    use crate::ToPrimitive;
    #[test]
    fn test_to_u32_with_positive_i32() {
        let _rug_st_tests_llm_16_861_llm_16_861_rrrruuuugggg_test_to_u32_with_positive_i32 = 0;
        let rug_fuzz_0 = 123;
        let value: i32 = rug_fuzz_0;
        debug_assert_eq!(value.to_u32(), Some(123_u32));
        let _rug_ed_tests_llm_16_861_llm_16_861_rrrruuuugggg_test_to_u32_with_positive_i32 = 0;
    }
    #[test]
    fn test_to_u32_with_negative_i32() {
        let _rug_st_tests_llm_16_861_llm_16_861_rrrruuuugggg_test_to_u32_with_negative_i32 = 0;
        let rug_fuzz_0 = 123;
        let value: i32 = -rug_fuzz_0;
        debug_assert_eq!(value.to_u32(), None);
        let _rug_ed_tests_llm_16_861_llm_16_861_rrrruuuugggg_test_to_u32_with_negative_i32 = 0;
    }
    #[test]
    fn test_to_u32_with_i32_max() {
        let _rug_st_tests_llm_16_861_llm_16_861_rrrruuuugggg_test_to_u32_with_i32_max = 0;
        let value: i32 = i32::MAX;
        debug_assert_eq!(value.to_u32(), Some(i32::MAX as u32));
        let _rug_ed_tests_llm_16_861_llm_16_861_rrrruuuugggg_test_to_u32_with_i32_max = 0;
    }
    #[test]
    fn test_to_u32_with_i32_min() {
        let _rug_st_tests_llm_16_861_llm_16_861_rrrruuuugggg_test_to_u32_with_i32_min = 0;
        let value: i32 = i32::MIN;
        debug_assert_eq!(value.to_u32(), None);
        let _rug_ed_tests_llm_16_861_llm_16_861_rrrruuuugggg_test_to_u32_with_i32_min = 0;
    }
    #[test]
    fn test_to_u32_with_zero_i32() {
        let _rug_st_tests_llm_16_861_llm_16_861_rrrruuuugggg_test_to_u32_with_zero_i32 = 0;
        let rug_fuzz_0 = 0;
        let value: i32 = rug_fuzz_0;
        debug_assert_eq!(value.to_u32(), Some(0_u32));
        let _rug_ed_tests_llm_16_861_llm_16_861_rrrruuuugggg_test_to_u32_with_zero_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_862_llm_16_862 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_u64_positive_i32() {
        let _rug_st_tests_llm_16_862_llm_16_862_rrrruuuugggg_test_to_u64_positive_i32 = 0;
        let rug_fuzz_0 = 12345;
        let value: i32 = rug_fuzz_0;
        let result = value.to_u64();
        debug_assert_eq!(result, Some(12345u64));
        let _rug_ed_tests_llm_16_862_llm_16_862_rrrruuuugggg_test_to_u64_positive_i32 = 0;
    }
    #[test]
    fn test_to_u64_negative_i32() {
        let _rug_st_tests_llm_16_862_llm_16_862_rrrruuuugggg_test_to_u64_negative_i32 = 0;
        let rug_fuzz_0 = 1;
        let value: i32 = -rug_fuzz_0;
        let result = value.to_u64();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_862_llm_16_862_rrrruuuugggg_test_to_u64_negative_i32 = 0;
    }
    #[test]
    fn test_to_u64_i32_max() {
        let _rug_st_tests_llm_16_862_llm_16_862_rrrruuuugggg_test_to_u64_i32_max = 0;
        let value: i32 = i32::MAX;
        let result = value.to_u64();
        debug_assert_eq!(result, Some(i32::MAX as u64));
        let _rug_ed_tests_llm_16_862_llm_16_862_rrrruuuugggg_test_to_u64_i32_max = 0;
    }
    #[test]
    fn test_to_u64_i32_min() {
        let _rug_st_tests_llm_16_862_llm_16_862_rrrruuuugggg_test_to_u64_i32_min = 0;
        let value: i32 = i32::MIN;
        let result = value.to_u64();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_862_llm_16_862_rrrruuuugggg_test_to_u64_i32_min = 0;
    }
    #[test]
    fn test_to_u64_zero() {
        let _rug_st_tests_llm_16_862_llm_16_862_rrrruuuugggg_test_to_u64_zero = 0;
        let rug_fuzz_0 = 0;
        let value: i32 = rug_fuzz_0;
        let result = value.to_u64();
        debug_assert_eq!(result, Some(0u64));
        let _rug_ed_tests_llm_16_862_llm_16_862_rrrruuuugggg_test_to_u64_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_863_llm_16_863 {
    use super::*;
    use crate::*;
    #[test]
    fn test_to_u8_with_i32() {
        let _rug_st_tests_llm_16_863_llm_16_863_rrrruuuugggg_test_to_u8_with_i32 = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 255;
        let rug_fuzz_3 = 256;
        let rug_fuzz_4 = 1;
        debug_assert_eq!(< i32 as ToPrimitive > ::to_u8(& rug_fuzz_0), Some(0u8));
        debug_assert_eq!(< i32 as ToPrimitive > ::to_u8(& rug_fuzz_1), Some(1u8));
        debug_assert_eq!(< i32 as ToPrimitive > ::to_u8(& rug_fuzz_2), Some(255u8));
        debug_assert_eq!(< i32 as ToPrimitive > ::to_u8(& rug_fuzz_3), None);
        debug_assert_eq!(< i32 as ToPrimitive > ::to_u8(& - rug_fuzz_4), None);
        debug_assert_eq!(< i32 as ToPrimitive > ::to_u8(& i32::MIN), None);
        debug_assert_eq!(< i32 as ToPrimitive > ::to_u8(& i32::MAX), None);
        let _rug_ed_tests_llm_16_863_llm_16_863_rrrruuuugggg_test_to_u8_with_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_864 {
    use crate::ToPrimitive;
    #[test]
    fn test_to_usize_with_positive_i32() {
        let _rug_st_tests_llm_16_864_rrrruuuugggg_test_to_usize_with_positive_i32 = 0;
        let rug_fuzz_0 = 42;
        let value: i32 = rug_fuzz_0;
        debug_assert_eq!(value.to_usize(), Some(42_usize));
        let _rug_ed_tests_llm_16_864_rrrruuuugggg_test_to_usize_with_positive_i32 = 0;
    }
    #[test]
    fn test_to_usize_with_negative_i32() {
        let _rug_st_tests_llm_16_864_rrrruuuugggg_test_to_usize_with_negative_i32 = 0;
        let rug_fuzz_0 = 42;
        let value: i32 = -rug_fuzz_0;
        debug_assert_eq!(value.to_usize(), None);
        let _rug_ed_tests_llm_16_864_rrrruuuugggg_test_to_usize_with_negative_i32 = 0;
    }
    #[test]
    fn test_to_usize_with_i32_max() {
        let _rug_st_tests_llm_16_864_rrrruuuugggg_test_to_usize_with_i32_max = 0;
        let value: i32 = i32::MAX;
        debug_assert_eq!(value.to_usize(), Some(i32::MAX as usize));
        let _rug_ed_tests_llm_16_864_rrrruuuugggg_test_to_usize_with_i32_max = 0;
    }
    #[test]
    fn test_to_usize_with_i32_min() {
        let _rug_st_tests_llm_16_864_rrrruuuugggg_test_to_usize_with_i32_min = 0;
        let value: i32 = i32::MIN;
        debug_assert_eq!(value.to_usize(), None);
        let _rug_ed_tests_llm_16_864_rrrruuuugggg_test_to_usize_with_i32_min = 0;
    }
    #[test]
    fn test_to_usize_with_zero_i32() {
        let _rug_st_tests_llm_16_864_rrrruuuugggg_test_to_usize_with_zero_i32 = 0;
        let rug_fuzz_0 = 0;
        let value: i32 = rug_fuzz_0;
        debug_assert_eq!(value.to_usize(), Some(0_usize));
        let _rug_ed_tests_llm_16_864_rrrruuuugggg_test_to_usize_with_zero_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_932_llm_16_932 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_i64_to_f32() {
        let _rug_st_tests_llm_16_932_llm_16_932_rrrruuuugggg_test_as_primitive_i64_to_f32 = 0;
        let rug_fuzz_0 = 42;
        let value: i64 = rug_fuzz_0;
        let result: f32 = AsPrimitive::<f32>::as_(value);
        debug_assert_eq!(result, 42.0_f32);
        let _rug_ed_tests_llm_16_932_llm_16_932_rrrruuuugggg_test_as_primitive_i64_to_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_933_llm_16_933 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_i64_to_f64() {
        let _rug_st_tests_llm_16_933_llm_16_933_rrrruuuugggg_test_as_primitive_i64_to_f64 = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42.0;
        let rug_fuzz_2 = 42;
        let rug_fuzz_3 = 42.0;
        let value: i64 = rug_fuzz_0;
        let result: f64 = AsPrimitive::<f64>::as_(value);
        let expected: f64 = rug_fuzz_1;
        debug_assert_eq!(result, expected);
        let value: i64 = -rug_fuzz_2;
        let result: f64 = AsPrimitive::<f64>::as_(value);
        let expected: f64 = -rug_fuzz_3;
        debug_assert_eq!(result, expected);
        let value: i64 = i64::MAX;
        let result: f64 = AsPrimitive::<f64>::as_(value);
        let expected: f64 = i64::MAX as f64;
        debug_assert_eq!(result, expected);
        let value: i64 = i64::MIN;
        let result: f64 = AsPrimitive::<f64>::as_(value);
        let expected: f64 = i64::MIN as f64;
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_933_llm_16_933_rrrruuuugggg_test_as_primitive_i64_to_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_934_llm_16_934 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_i64_to_i128() {
        let _rug_st_tests_llm_16_934_llm_16_934_rrrruuuugggg_test_as_primitive_i64_to_i128 = 0;
        let x: i64 = i64::MAX;
        let y: i128 = AsPrimitive::<i128>::as_(x);
        debug_assert_eq!(y, i64::MAX as i128);
        let _rug_ed_tests_llm_16_934_llm_16_934_rrrruuuugggg_test_as_primitive_i64_to_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_935_llm_16_935 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_i64_to_i16() {
        let _rug_st_tests_llm_16_935_llm_16_935_rrrruuuugggg_test_as_i64_to_i16 = 0;
        let rug_fuzz_0 = 0x1234_5678_9ABC_DEF0;
        let rug_fuzz_1 = 0i64;
        let x: i64 = rug_fuzz_0;
        let y: i16 = AsPrimitive::<i16>::as_(x);
        debug_assert_eq!(y, 0xF0i16);
        let x: i64 = i16::MAX as i64;
        let y: i16 = AsPrimitive::<i16>::as_(x);
        debug_assert_eq!(y, i16::MAX);
        let x: i64 = i16::MIN as i64;
        let y: i16 = AsPrimitive::<i16>::as_(x);
        debug_assert_eq!(y, i16::MIN);
        let x: i64 = rug_fuzz_1;
        let y: i16 = AsPrimitive::<i16>::as_(x);
        debug_assert_eq!(y, 0i16);
        let _rug_ed_tests_llm_16_935_llm_16_935_rrrruuuugggg_test_as_i64_to_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_936_llm_16_936 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_i64_to_i32() {
        let _rug_st_tests_llm_16_936_llm_16_936_rrrruuuugggg_test_as_i64_to_i32 = 0;
        let rug_fuzz_0 = 1234567890;
        let x: i64 = rug_fuzz_0;
        let y: i32 = <i64 as AsPrimitive<i32>>::as_(x);
        debug_assert_eq!(y, 1234567890i32);
        let _rug_ed_tests_llm_16_936_llm_16_936_rrrruuuugggg_test_as_i64_to_i32 = 0;
    }
    #[test]
    fn test_as_i64_to_i32_overflow() {
        let _rug_st_tests_llm_16_936_llm_16_936_rrrruuuugggg_test_as_i64_to_i32_overflow = 0;
        let x: i64 = i64::MAX;
        let y: i32 = <i64 as AsPrimitive<i32>>::as_(x);
        debug_assert_eq!(y, (i64::MAX as i32));
        let _rug_ed_tests_llm_16_936_llm_16_936_rrrruuuugggg_test_as_i64_to_i32_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_937_llm_16_937 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_i64_to_i64() {
        let _rug_st_tests_llm_16_937_llm_16_937_rrrruuuugggg_test_as_primitive_i64_to_i64 = 0;
        let rug_fuzz_0 = 1234567890;
        let value: i64 = rug_fuzz_0;
        let result: i64 = AsPrimitive::<i64>::as_(value);
        debug_assert_eq!(result, 1234567890i64);
        let _rug_ed_tests_llm_16_937_llm_16_937_rrrruuuugggg_test_as_primitive_i64_to_i64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_938_llm_16_938 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_i64_as_i8() {
        let _rug_st_tests_llm_16_938_llm_16_938_rrrruuuugggg_test_i64_as_i8 = 0;
        let rug_fuzz_0 = 120;
        let rug_fuzz_1 = 120;
        let rug_fuzz_2 = 130;
        let rug_fuzz_3 = 126;
        let rug_fuzz_4 = 130;
        let rug_fuzz_5 = 126;
        let val_i64: i64 = rug_fuzz_0;
        let expected_i8: i8 = rug_fuzz_1;
        let result_i8: i8 = val_i64.as_();
        debug_assert_eq!(
            result_i8, expected_i8, "Casting i64 to i8 failed at value {}", val_i64
        );
        let val_i64: i64 = rug_fuzz_2;
        let expected_i8: i8 = -rug_fuzz_3;
        let result_i8: i8 = val_i64.as_();
        debug_assert_eq!(
            result_i8, expected_i8, "Casting i64 to i8 should wrap around at value {}",
            val_i64
        );
        let val_i64: i64 = -rug_fuzz_4;
        let expected_i8: i8 = rug_fuzz_5;
        let result_i8: i8 = val_i64.as_();
        debug_assert_eq!(
            result_i8, expected_i8, "Casting i64 to i8 should wrap around at value {}",
            val_i64
        );
        let _rug_ed_tests_llm_16_938_llm_16_938_rrrruuuugggg_test_i64_as_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_939 {
    use crate::AsPrimitive;
    #[test]
    fn i64_to_isize_casting() {
        let _rug_st_tests_llm_16_939_rrrruuuugggg_i64_to_isize_casting = 0;
        let val: i64 = i64::MAX;
        let casted: isize = AsPrimitive::<isize>::as_(val);
        #[cfg(target_pointer_width = "64")] debug_assert_eq!(casted, i64::MAX as isize);
        #[cfg(target_pointer_width = "32")] debug_assert_eq!(casted, isize::MAX);
        let _rug_ed_tests_llm_16_939_rrrruuuugggg_i64_to_isize_casting = 0;
    }
    #[test]
    fn negative_i64_to_isize_casting() {
        let _rug_st_tests_llm_16_939_rrrruuuugggg_negative_i64_to_isize_casting = 0;
        let val: i64 = i64::MIN;
        let casted: isize = AsPrimitive::<isize>::as_(val);
        #[cfg(target_pointer_width = "64")] debug_assert_eq!(casted, i64::MIN as isize);
        #[cfg(target_pointer_width = "32")] debug_assert_eq!(casted, isize::MIN);
        let _rug_ed_tests_llm_16_939_rrrruuuugggg_negative_i64_to_isize_casting = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_941_llm_16_941 {
    use crate::AsPrimitive;
    #[test]
    fn i64_as_u16() {
        let _rug_st_tests_llm_16_941_llm_16_941_rrrruuuugggg_i64_as_u16 = 0;
        let rug_fuzz_0 = 256;
        let rug_fuzz_1 = 65_535;
        let rug_fuzz_2 = 65_536;
        let rug_fuzz_3 = 1;
        let num: i64 = rug_fuzz_0;
        let result: u16 = num.as_();
        debug_assert_eq!(result, 256u16);
        let num: i64 = rug_fuzz_1;
        let result: u16 = num.as_();
        debug_assert_eq!(result, 65_535u16);
        let num: i64 = rug_fuzz_2;
        let result: u16 = num.as_();
        debug_assert_eq!(result, 0u16);
        let num: i64 = -rug_fuzz_3;
        let result: u16 = num.as_();
        debug_assert_eq!(result, u16::MAX);
        let _rug_ed_tests_llm_16_941_llm_16_941_rrrruuuugggg_i64_as_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_942_llm_16_942 {
    use crate::AsPrimitive;
    #[test]
    fn test_as_primitive_i64_to_u32() {
        let _rug_st_tests_llm_16_942_llm_16_942_rrrruuuugggg_test_as_primitive_i64_to_u32 = 0;
        let rug_fuzz_0 = 123;
        let rug_fuzz_1 = 123;
        let rug_fuzz_2 = 0;
        let value: i64 = rug_fuzz_0;
        let result = <i64 as AsPrimitive<u32>>::as_(value);
        debug_assert_eq!(result, 123u32);
        let value: i64 = i64::MAX;
        let result = <i64 as AsPrimitive<u32>>::as_(value);
        debug_assert_eq!(result as i64, i64::MAX as u32 as i64);
        let negative_value: i64 = -rug_fuzz_1;
        let result = <i64 as AsPrimitive<u32>>::as_(negative_value);
        debug_assert!(
            result > rug_fuzz_2, "casting negative i64 to u32 didn't panic as expected"
        );
        let _rug_ed_tests_llm_16_942_llm_16_942_rrrruuuugggg_test_as_primitive_i64_to_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_943_llm_16_943 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_i64_to_u64() {
        let _rug_st_tests_llm_16_943_llm_16_943_rrrruuuugggg_test_as_primitive_i64_to_u64 = 0;
        let rug_fuzz_0 = 123;
        let rug_fuzz_1 = 1;
        let value: i64 = rug_fuzz_0;
        let result: u64 = value.as_();
        debug_assert_eq!(result, 123u64);
        let negative_value: i64 = -rug_fuzz_1;
        let result_negative: u64 = negative_value.as_();
        debug_assert_eq!(result_negative, negative_value as u64);
        let max_i64_value: i64 = i64::MAX;
        let result_max: u64 = max_i64_value.as_();
        debug_assert_eq!(result_max, i64::MAX as u64);
        let _rug_ed_tests_llm_16_943_llm_16_943_rrrruuuugggg_test_as_primitive_i64_to_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_944 {
    use crate::AsPrimitive;
    #[test]
    fn test_as_primitive_i64_to_u8() {
        let _rug_st_tests_llm_16_944_rrrruuuugggg_test_as_primitive_i64_to_u8 = 0;
        let rug_fuzz_0 = 123;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 256;
        let num_i64: i64 = rug_fuzz_0;
        let num_u8: u8 = AsPrimitive::<u8>::as_(num_i64);
        debug_assert_eq!(num_u8, 123u8);
        let num_i64: i64 = -rug_fuzz_1;
        let num_u8: u8 = AsPrimitive::<u8>::as_(num_i64);
        debug_assert_eq!(num_u8, 255u8);
        let num_i64: i64 = rug_fuzz_2;
        let num_u8: u8 = AsPrimitive::<u8>::as_(num_i64);
        debug_assert_eq!(num_u8, 0u8);
        let _rug_ed_tests_llm_16_944_rrrruuuugggg_test_as_primitive_i64_to_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_945_llm_16_945 {
    use crate::AsPrimitive;
    #[test]
    fn test_as_primitive_i64_to_usize() {
        let val_i64: i64 = 42;
        let val_usize: usize = AsPrimitive::<usize>::as_(val_i64);
        assert_eq!(val_usize, 42_usize);
    }
    #[test]
    fn test_as_primitive_i64_to_usize_negative() {
        let val_i64: i64 = -42;
        assert!(cfg!(target_pointer_width = "64"), "Test only valid on 64-bit targets.");
        let val_usize: usize = AsPrimitive::<usize>::as_(val_i64);
        assert_eq!(val_usize, (- 42i64) as usize);
    }
    #[test]
    #[should_panic(expected = "attempt to cast to usize with overflow")]
    fn test_as_primitive_i64_to_usize_overflow() {
        let val_i64: i64 = i64::max_value();
        let _val_usize: usize = AsPrimitive::<usize>::as_(val_i64);
    }
}
#[cfg(test)]
mod tests_llm_16_946_llm_16_946 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_f32() {
        let _rug_st_tests_llm_16_946_llm_16_946_rrrruuuugggg_test_from_f32 = 0;
        let rug_fuzz_0 = 1.0_f32;
        let rug_fuzz_1 = 1.5_f32;
        let rug_fuzz_2 = 1.5_f32;
        debug_assert_eq!(< i64 as FromPrimitive > ::from_f32(rug_fuzz_0), Some(1_i64));
        debug_assert_eq!(< i64 as FromPrimitive > ::from_f32(rug_fuzz_1), Some(1_i64));
        debug_assert_eq!(
            < i64 as FromPrimitive > ::from_f32(- rug_fuzz_2), Some(- 1_i64)
        );
        debug_assert_eq!(< i64 as FromPrimitive > ::from_f32(f32::MAX), None);
        debug_assert_eq!(< i64 as FromPrimitive > ::from_f32(f32::MIN), None);
        debug_assert_eq!(< i64 as FromPrimitive > ::from_f32(f32::NAN), None);
        debug_assert_eq!(< i64 as FromPrimitive > ::from_f32(f32::INFINITY), None);
        debug_assert_eq!(< i64 as FromPrimitive > ::from_f32(f32::NEG_INFINITY), None);
        let _rug_ed_tests_llm_16_946_llm_16_946_rrrruuuugggg_test_from_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_947_llm_16_947 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_f64() {
        let _rug_st_tests_llm_16_947_llm_16_947_rrrruuuugggg_test_from_f64 = 0;
        let rug_fuzz_0 = 42.0;
        let rug_fuzz_1 = 42.0;
        let rug_fuzz_2 = 0.0;
        let rug_fuzz_3 = 42.123;
        let rug_fuzz_4 = 5e-324;
        debug_assert_eq!(< i64 as FromPrimitive > ::from_f64(rug_fuzz_0), Some(42i64));
        debug_assert_eq!(
            < i64 as FromPrimitive > ::from_f64(- rug_fuzz_1), Some(- 42i64)
        );
        debug_assert_eq!(< i64 as FromPrimitive > ::from_f64(rug_fuzz_2), Some(0i64));
        debug_assert_eq!(< i64 as FromPrimitive > ::from_f64(rug_fuzz_3), Some(42i64));
        debug_assert_eq!(< i64 as FromPrimitive > ::from_f64(f64::MAX), None);
        debug_assert_eq!(< i64 as FromPrimitive > ::from_f64(f64::MIN), None);
        debug_assert_eq!(< i64 as FromPrimitive > ::from_f64(f64::NAN), None);
        debug_assert_eq!(< i64 as FromPrimitive > ::from_f64(f64::INFINITY), None);
        debug_assert_eq!(< i64 as FromPrimitive > ::from_f64(f64::NEG_INFINITY), None);
        debug_assert_eq!(< i64 as FromPrimitive > ::from_f64(rug_fuzz_4), Some(0i64));
        let _rug_ed_tests_llm_16_947_llm_16_947_rrrruuuugggg_test_from_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_948 {
    use super::*;
    use crate::*;
    #[test]
    fn test_from_i128() {
        let _rug_st_tests_llm_16_948_rrrruuuugggg_test_from_i128 = 0;
        let rug_fuzz_0 = 0_i128;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        debug_assert_eq!(
            < i64 as cast::FromPrimitive > ::from_i128(rug_fuzz_0), Some(0_i64)
        );
        debug_assert_eq!(
            < i64 as cast::FromPrimitive > ::from_i128(i64::MAX as i128), Some(i64::MAX)
        );
        debug_assert_eq!(
            < i64 as cast::FromPrimitive > ::from_i128(i64::MIN as i128), Some(i64::MIN)
        );
        debug_assert_eq!(
            < i64 as cast::FromPrimitive > ::from_i128((i64::MAX as i128) + rug_fuzz_1),
            None
        );
        debug_assert_eq!(
            < i64 as cast::FromPrimitive > ::from_i128((i64::MIN as i128) - rug_fuzz_2),
            None
        );
        let _rug_ed_tests_llm_16_948_rrrruuuugggg_test_from_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_949 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i16() {
        let _rug_st_tests_llm_16_949_rrrruuuugggg_test_from_i16 = 0;
        let rug_fuzz_0 = 0_i16;
        let rug_fuzz_1 = 1_i16;
        debug_assert_eq!(< i64 as FromPrimitive > ::from_i16(rug_fuzz_0), Some(0_i64));
        debug_assert_eq!(
            < i64 as FromPrimitive > ::from_i16(- rug_fuzz_1), Some(- 1_i64)
        );
        debug_assert_eq!(
            < i64 as FromPrimitive > ::from_i16(i16::MAX), Some(i64::from(i16::MAX))
        );
        let _rug_ed_tests_llm_16_949_rrrruuuugggg_test_from_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_950_llm_16_950 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i32() {
        let _rug_st_tests_llm_16_950_llm_16_950_rrrruuuugggg_test_from_i32 = 0;
        let rug_fuzz_0 = 0i32;
        let rug_fuzz_1 = 1i32;
        debug_assert_eq!(< i64 as FromPrimitive > ::from_i32(rug_fuzz_0), Some(0i64));
        debug_assert_eq!(
            < i64 as FromPrimitive > ::from_i32(- rug_fuzz_1), Some(- 1i64)
        );
        debug_assert_eq!(
            < i64 as FromPrimitive > ::from_i32(i32::MAX), Some(i64::from(i32::MAX))
        );
        debug_assert_eq!(
            < i64 as FromPrimitive > ::from_i32(i32::MIN), Some(i64::from(i32::MIN))
        );
        let _rug_ed_tests_llm_16_950_llm_16_950_rrrruuuugggg_test_from_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_951_llm_16_951 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i64() {
        let _rug_st_tests_llm_16_951_llm_16_951_rrrruuuugggg_test_from_i64 = 0;
        let rug_fuzz_0 = 0_i64;
        debug_assert_eq!(< i64 as FromPrimitive > ::from_i64(rug_fuzz_0), Some(0_i64));
        debug_assert_eq!(< i64 as FromPrimitive > ::from_i64(i64::MIN), Some(i64::MIN));
        debug_assert_eq!(< i64 as FromPrimitive > ::from_i64(i64::MAX), Some(i64::MAX));
        let _rug_ed_tests_llm_16_951_llm_16_951_rrrruuuugggg_test_from_i64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_952_llm_16_952 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i8() {
        let _rug_st_tests_llm_16_952_llm_16_952_rrrruuuugggg_test_from_i8 = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        debug_assert_eq!(< i64 as FromPrimitive > ::from_i8(rug_fuzz_0), Some(0i64));
        debug_assert_eq!(< i64 as FromPrimitive > ::from_i8(- rug_fuzz_1), Some(- 1i64));
        debug_assert_eq!(
            < i64 as FromPrimitive > ::from_i8(i8::MAX), Some(i64::from(i8::MAX))
        );
        let _rug_ed_tests_llm_16_952_llm_16_952_rrrruuuugggg_test_from_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_953_llm_16_953 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_isize() {
        let _rug_st_tests_llm_16_953_llm_16_953_rrrruuuugggg_test_from_isize = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        debug_assert_eq!(< i64 as FromPrimitive > ::from_isize(rug_fuzz_0), Some(0i64));
        debug_assert_eq!(< i64 as FromPrimitive > ::from_isize(rug_fuzz_1), Some(1i64));
        debug_assert_eq!(
            < i64 as FromPrimitive > ::from_isize(- rug_fuzz_2), Some(- 1i64)
        );
        debug_assert_eq!(
            < i64 as FromPrimitive > ::from_isize(isize::max_value()),
            Some(isize::max_value() as i64)
        );
        let _rug_ed_tests_llm_16_953_llm_16_953_rrrruuuugggg_test_from_isize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_954_llm_16_954 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u128_within_range() {
        let _rug_st_tests_llm_16_954_llm_16_954_rrrruuuugggg_test_from_u128_within_range = 0;
        let value: u128 = i64::MAX as u128;
        let result = <i64 as FromPrimitive>::from_u128(value);
        debug_assert_eq!(result, Some(i64::MAX));
        let _rug_ed_tests_llm_16_954_llm_16_954_rrrruuuugggg_test_from_u128_within_range = 0;
    }
    #[test]
    fn test_from_u128_below_range() {
        let _rug_st_tests_llm_16_954_llm_16_954_rrrruuuugggg_test_from_u128_below_range = 0;
        let rug_fuzz_0 = 0;
        let value: u128 = rug_fuzz_0;
        let result = <i64 as FromPrimitive>::from_u128(value);
        debug_assert_eq!(result, Some(0));
        let _rug_ed_tests_llm_16_954_llm_16_954_rrrruuuugggg_test_from_u128_below_range = 0;
    }
    #[test]
    fn test_from_u128_above_range() {
        let _rug_st_tests_llm_16_954_llm_16_954_rrrruuuugggg_test_from_u128_above_range = 0;
        let rug_fuzz_0 = 1;
        let value: u128 = (i64::MAX as u128) + rug_fuzz_0;
        let result = <i64 as FromPrimitive>::from_u128(value);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_954_llm_16_954_rrrruuuugggg_test_from_u128_above_range = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_955_llm_16_955 {
    use crate::cast::FromPrimitive;
    #[test]
    fn from_u16_test() {
        let _rug_st_tests_llm_16_955_llm_16_955_rrrruuuugggg_from_u16_test = 0;
        let rug_fuzz_0 = 0_u16;
        let rug_fuzz_1 = 1_u16;
        debug_assert_eq!(< i64 as FromPrimitive > ::from_u16(rug_fuzz_0), Some(0_i64));
        debug_assert_eq!(< i64 as FromPrimitive > ::from_u16(rug_fuzz_1), Some(1_i64));
        debug_assert_eq!(
            < i64 as FromPrimitive > ::from_u16(u16::MAX), Some(u16::MAX as i64)
        );
        let _rug_ed_tests_llm_16_955_llm_16_955_rrrruuuugggg_from_u16_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_956_llm_16_956 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u32() {
        let _rug_st_tests_llm_16_956_llm_16_956_rrrruuuugggg_test_from_u32 = 0;
        let rug_fuzz_0 = 0_u32;
        let rug_fuzz_1 = 123456_u32;
        let rug_fuzz_2 = 1;
        debug_assert_eq!(< i64 as FromPrimitive > ::from_u32(rug_fuzz_0), Some(0_i64));
        debug_assert_eq!(
            < i64 as FromPrimitive > ::from_u32(u32::MAX), Some(u32::MAX as i64)
        );
        debug_assert_eq!(
            < i64 as FromPrimitive > ::from_u32(rug_fuzz_1), Some(123456_i64)
        );
        debug_assert_eq!(
            < i64 as FromPrimitive > ::from_u32((i64::MAX as u32)
            .wrapping_add(rug_fuzz_2)), None
        );
        let _rug_ed_tests_llm_16_956_llm_16_956_rrrruuuugggg_test_from_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_957_llm_16_957 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u64() {
        let _rug_st_tests_llm_16_957_llm_16_957_rrrruuuugggg_test_from_u64 = 0;
        let rug_fuzz_0 = 0_u64;
        let rug_fuzz_1 = 123_u64;
        let rug_fuzz_2 = 1;
        debug_assert_eq!(< i64 as FromPrimitive > ::from_u64(rug_fuzz_0), Some(0_i64));
        debug_assert_eq!(< i64 as FromPrimitive > ::from_u64(u64::MAX), None);
        debug_assert_eq!(< i64 as FromPrimitive > ::from_u64(rug_fuzz_1), Some(123_i64));
        debug_assert_eq!(
            < i64 as FromPrimitive > ::from_u64(i64::MAX as u64), Some(i64::MAX)
        );
        debug_assert_eq!(
            < i64 as FromPrimitive > ::from_u64((i64::MAX as u64)
            .wrapping_add(rug_fuzz_2)), None
        );
        let _rug_ed_tests_llm_16_957_llm_16_957_rrrruuuugggg_test_from_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_958_llm_16_958 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u8() {
        let _rug_st_tests_llm_16_958_llm_16_958_rrrruuuugggg_test_from_u8 = 0;
        let rug_fuzz_0 = 0u8;
        let rug_fuzz_1 = 1u8;
        debug_assert_eq!(< i64 as FromPrimitive > ::from_u8(rug_fuzz_0), Some(0i64));
        debug_assert_eq!(< i64 as FromPrimitive > ::from_u8(rug_fuzz_1), Some(1i64));
        debug_assert_eq!(
            < i64 as FromPrimitive > ::from_u8(u8::MAX), Some(i64::from(u8::MAX))
        );
        let _rug_ed_tests_llm_16_958_llm_16_958_rrrruuuugggg_test_from_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_959 {
    use crate::FromPrimitive;
    #[test]
    fn test_from_usize() {
        let _rug_st_tests_llm_16_959_rrrruuuugggg_test_from_usize = 0;
        let rug_fuzz_0 = 0;
        debug_assert_eq!(< i64 as FromPrimitive > ::from_usize(rug_fuzz_0), Some(0i64));
        debug_assert_eq!(
            < i64 as FromPrimitive > ::from_usize(usize::MAX), Some(usize::MAX as i64)
        );
        let _rug_ed_tests_llm_16_959_rrrruuuugggg_test_from_usize = 0;
    }
    #[test]
    #[should_panic(expected = "attempt to cast to overflowed")]
    fn test_from_usize_overflow() {
        let _rug_st_tests_llm_16_959_rrrruuuugggg_test_from_usize_overflow = 0;
        let _ = <i64 as FromPrimitive>::from_usize(usize::MAX).unwrap();
        let _rug_ed_tests_llm_16_959_rrrruuuugggg_test_from_usize_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_960_llm_16_960 {
    use crate::{NumCast, ToPrimitive};
    use std::num::Wrapping;
    #[test]
    fn test_numcast_from_wrapping_to_i64() {
        let _rug_st_tests_llm_16_960_llm_16_960_rrrruuuugggg_test_numcast_from_wrapping_to_i64 = 0;
        let rug_fuzz_0 = 42i8;
        let rug_fuzz_1 = 42i16;
        let rug_fuzz_2 = 42i32;
        let rug_fuzz_3 = 42i64;
        let rug_fuzz_4 = 42i128;
        let rug_fuzz_5 = 42u8;
        let rug_fuzz_6 = 42u16;
        let rug_fuzz_7 = 42u32;
        let rug_fuzz_8 = 42u64;
        let rug_fuzz_9 = 42u128;
        let rug_fuzz_10 = 42.0f32;
        let rug_fuzz_11 = 42.0f64;
        let rug_fuzz_12 = 42i8;
        let rug_fuzz_13 = 42i16;
        let rug_fuzz_14 = 42i32;
        let rug_fuzz_15 = 42i64;
        let rug_fuzz_16 = 42i128;
        debug_assert_eq!(< i64 as NumCast > ::from(Wrapping(rug_fuzz_0)), Some(42i64));
        debug_assert_eq!(< i64 as NumCast > ::from(Wrapping(rug_fuzz_1)), Some(42i64));
        debug_assert_eq!(< i64 as NumCast > ::from(Wrapping(rug_fuzz_2)), Some(42i64));
        debug_assert_eq!(< i64 as NumCast > ::from(Wrapping(rug_fuzz_3)), Some(42i64));
        debug_assert_eq!(< i64 as NumCast > ::from(Wrapping(rug_fuzz_4)), Some(42i64));
        debug_assert_eq!(< i64 as NumCast > ::from(Wrapping(rug_fuzz_5)), Some(42i64));
        debug_assert_eq!(< i64 as NumCast > ::from(Wrapping(rug_fuzz_6)), Some(42i64));
        debug_assert_eq!(< i64 as NumCast > ::from(Wrapping(rug_fuzz_7)), Some(42i64));
        debug_assert_eq!(< i64 as NumCast > ::from(Wrapping(rug_fuzz_8)), Some(42i64));
        debug_assert_eq!(< i64 as NumCast > ::from(Wrapping(rug_fuzz_9)), Some(42i64));
        debug_assert_eq!(< i64 as NumCast > ::from(Wrapping(rug_fuzz_10)), Some(42i64));
        debug_assert_eq!(< i64 as NumCast > ::from(Wrapping(rug_fuzz_11)), Some(42i64));
        debug_assert_eq!(
            < i64 as NumCast > ::from(Wrapping(- rug_fuzz_12)), Some(- 42i64)
        );
        debug_assert_eq!(
            < i64 as NumCast > ::from(Wrapping(- rug_fuzz_13)), Some(- 42i64)
        );
        debug_assert_eq!(
            < i64 as NumCast > ::from(Wrapping(- rug_fuzz_14)), Some(- 42i64)
        );
        debug_assert_eq!(
            < i64 as NumCast > ::from(Wrapping(- rug_fuzz_15)), Some(- 42i64)
        );
        debug_assert_eq!(
            < i64 as NumCast > ::from(Wrapping(- rug_fuzz_16)), Some(- 42i64)
        );
        debug_assert_eq!(
            < i64 as NumCast > ::from(Wrapping(i32::MAX)), Some(i32::MAX as i64)
        );
        debug_assert_eq!(
            < i64 as NumCast > ::from(Wrapping(i32::MIN)), Some(i32::MIN as i64)
        );
        debug_assert_eq!(< i64 as NumCast > ::from(Wrapping(u64::MAX)), None);
        let _rug_ed_tests_llm_16_960_llm_16_960_rrrruuuugggg_test_numcast_from_wrapping_to_i64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_961 {
    use super::*;
    use crate::*;
    #[test]
    fn i64_to_f32_exact() {
        let _rug_st_tests_llm_16_961_rrrruuuugggg_i64_to_f32_exact = 0;
        let rug_fuzz_0 = 0i64;
        let rug_fuzz_1 = 1i64;
        let rug_fuzz_2 = 1i64;
        debug_assert_eq!(
            < i64 as cast::ToPrimitive > ::to_f32(& rug_fuzz_0), Some(0.0f32)
        );
        debug_assert_eq!(
            < i64 as cast::ToPrimitive > ::to_f32(& rug_fuzz_1), Some(1.0f32)
        );
        debug_assert_eq!(
            < i64 as cast::ToPrimitive > ::to_f32(& - rug_fuzz_2), Some(- 1.0f32)
        );
        let _rug_ed_tests_llm_16_961_rrrruuuugggg_i64_to_f32_exact = 0;
    }
    #[test]
    fn i64_to_f32_large_numbers() {
        let _rug_st_tests_llm_16_961_rrrruuuugggg_i64_to_f32_large_numbers = 0;
        debug_assert_eq!(
            < i64 as cast::ToPrimitive > ::to_f32(& i64::MAX), Some(i64::MAX as f32)
        );
        debug_assert_eq!(
            < i64 as cast::ToPrimitive > ::to_f32(& i64::MIN), Some(i64::MIN as f32)
        );
        let _rug_ed_tests_llm_16_961_rrrruuuugggg_i64_to_f32_large_numbers = 0;
    }
    #[test]
    fn i64_to_f32_precision_loss() {
        let _rug_st_tests_llm_16_961_rrrruuuugggg_i64_to_f32_precision_loss = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 52;
        let rug_fuzz_2 = 0.0f32;
        let large_num: i64 = rug_fuzz_0 << rug_fuzz_1;
        let f32_result = <i64 as cast::ToPrimitive>::to_f32(&large_num).unwrap();
        debug_assert!(
            (large_num as f32 - f32_result).abs() > rug_fuzz_2,
            "Precision loss is expected for large i64 values when cast to f32"
        );
        let _rug_ed_tests_llm_16_961_rrrruuuugggg_i64_to_f32_precision_loss = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_963_llm_16_963 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_i128_within_bounds() {
        let _rug_st_tests_llm_16_963_llm_16_963_rrrruuuugggg_test_to_i128_within_bounds = 0;
        let value: i64 = i64::MAX;
        let result = ToPrimitive::to_i128(&value);
        debug_assert_eq!(result, Some(i128::from(i64::MAX)));
        let _rug_ed_tests_llm_16_963_llm_16_963_rrrruuuugggg_test_to_i128_within_bounds = 0;
    }
    #[test]
    fn test_to_i128_out_of_bounds_negative() {
        let _rug_st_tests_llm_16_963_llm_16_963_rrrruuuugggg_test_to_i128_out_of_bounds_negative = 0;
        let value: i64 = i64::MIN;
        let result = ToPrimitive::to_i128(&value);
        debug_assert_eq!(result, Some(i128::from(i64::MIN)));
        let _rug_ed_tests_llm_16_963_llm_16_963_rrrruuuugggg_test_to_i128_out_of_bounds_negative = 0;
    }
    #[test]
    fn test_to_i128_out_of_bounds_positive() {
        let _rug_st_tests_llm_16_963_llm_16_963_rrrruuuugggg_test_to_i128_out_of_bounds_positive = 0;
        let value: i64 = i64::MAX;
        let result = ToPrimitive::to_i128(&value);
        debug_assert_eq!(result, Some(i128::from(i64::MAX)));
        let _rug_ed_tests_llm_16_963_llm_16_963_rrrruuuugggg_test_to_i128_out_of_bounds_positive = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_964_llm_16_964 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_i16_in_range() {
        let _rug_st_tests_llm_16_964_llm_16_964_rrrruuuugggg_test_to_i16_in_range = 0;
        let rug_fuzz_0 = 0i64;
        let rug_fuzz_1 = 1i64;
        let rug_fuzz_2 = 1i64;
        debug_assert_eq!((rug_fuzz_0).to_i16(), Some(0i16));
        debug_assert_eq!((rug_fuzz_1).to_i16(), Some(1i16));
        debug_assert_eq!((- rug_fuzz_2).to_i16(), Some(- 1i16));
        debug_assert_eq!((i16::MAX as i64).to_i16(), Some(i16::MAX));
        debug_assert_eq!((i16::MIN as i64).to_i16(), Some(i16::MIN));
        let _rug_ed_tests_llm_16_964_llm_16_964_rrrruuuugggg_test_to_i16_in_range = 0;
    }
    #[test]
    fn test_to_i16_out_of_range() {
        let _rug_st_tests_llm_16_964_llm_16_964_rrrruuuugggg_test_to_i16_out_of_range = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 1;
        debug_assert_eq!((i16::MAX as i64 + rug_fuzz_0).to_i16(), None);
        debug_assert_eq!((i16::MIN as i64 - rug_fuzz_1).to_i16(), None);
        let _rug_ed_tests_llm_16_964_llm_16_964_rrrruuuugggg_test_to_i16_out_of_range = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_965_llm_16_965 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_i32_with_i64_within_bounds() {
        let _rug_st_tests_llm_16_965_llm_16_965_rrrruuuugggg_test_to_i32_with_i64_within_bounds = 0;
        let rug_fuzz_0 = 0i64;
        debug_assert_eq!(ToPrimitive::to_i32(& rug_fuzz_0), Some(0i32));
        debug_assert_eq!(ToPrimitive::to_i32(& (i32::MAX as i64)), Some(i32::MAX));
        debug_assert_eq!(ToPrimitive::to_i32(& (i32::MIN as i64)), Some(i32::MIN));
        let _rug_ed_tests_llm_16_965_llm_16_965_rrrruuuugggg_test_to_i32_with_i64_within_bounds = 0;
    }
    #[test]
    fn test_to_i32_with_i64_out_of_bounds() {
        let _rug_st_tests_llm_16_965_llm_16_965_rrrruuuugggg_test_to_i32_with_i64_out_of_bounds = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 1;
        debug_assert_eq!(ToPrimitive::to_i32(& (i32::MAX as i64 + rug_fuzz_0)), None);
        debug_assert_eq!(ToPrimitive::to_i32(& (i32::MIN as i64 - rug_fuzz_1)), None);
        let _rug_ed_tests_llm_16_965_llm_16_965_rrrruuuugggg_test_to_i32_with_i64_out_of_bounds = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_966_llm_16_966 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_i64_with_i64() {
        let _rug_st_tests_llm_16_966_llm_16_966_rrrruuuugggg_test_to_i64_with_i64 = 0;
        let rug_fuzz_0 = 42;
        let value: i64 = rug_fuzz_0;
        debug_assert_eq!(value.to_i64(), Some(42));
        let _rug_ed_tests_llm_16_966_llm_16_966_rrrruuuugggg_test_to_i64_with_i64 = 0;
    }
    #[test]
    fn test_to_i64_with_i32() {
        let _rug_st_tests_llm_16_966_llm_16_966_rrrruuuugggg_test_to_i64_with_i32 = 0;
        let rug_fuzz_0 = 42;
        let value: i32 = rug_fuzz_0;
        debug_assert_eq!(value.to_i64(), Some(42i64));
        let _rug_ed_tests_llm_16_966_llm_16_966_rrrruuuugggg_test_to_i64_with_i32 = 0;
    }
    #[test]
    fn test_to_i64_with_u64_within_range() {
        let _rug_st_tests_llm_16_966_llm_16_966_rrrruuuugggg_test_to_i64_with_u64_within_range = 0;
        let value: u64 = i64::MAX as u64;
        debug_assert_eq!(value.to_i64(), Some(i64::MAX));
        let _rug_ed_tests_llm_16_966_llm_16_966_rrrruuuugggg_test_to_i64_with_u64_within_range = 0;
    }
    #[test]
    fn test_to_i64_with_u64_out_of_range() {
        let _rug_st_tests_llm_16_966_llm_16_966_rrrruuuugggg_test_to_i64_with_u64_out_of_range = 0;
        let rug_fuzz_0 = 1;
        let value: u64 = (i64::MAX as u64) + rug_fuzz_0;
        debug_assert_eq!(value.to_i64(), None);
        let _rug_ed_tests_llm_16_966_llm_16_966_rrrruuuugggg_test_to_i64_with_u64_out_of_range = 0;
    }
    #[test]
    fn test_to_i64_with_u32() {
        let _rug_st_tests_llm_16_966_llm_16_966_rrrruuuugggg_test_to_i64_with_u32 = 0;
        let rug_fuzz_0 = 42;
        let value: u32 = rug_fuzz_0;
        debug_assert_eq!(value.to_i64(), Some(42i64));
        let _rug_ed_tests_llm_16_966_llm_16_966_rrrruuuugggg_test_to_i64_with_u32 = 0;
    }
    #[test]
    fn test_to_i64_with_i8() {
        let _rug_st_tests_llm_16_966_llm_16_966_rrrruuuugggg_test_to_i64_with_i8 = 0;
        let rug_fuzz_0 = 42;
        let value: i8 = rug_fuzz_0;
        debug_assert_eq!(value.to_i64(), Some(42i64));
        let _rug_ed_tests_llm_16_966_llm_16_966_rrrruuuugggg_test_to_i64_with_i8 = 0;
    }
    #[test]
    fn test_to_i64_with_f64_within_range() {
        let _rug_st_tests_llm_16_966_llm_16_966_rrrruuuugggg_test_to_i64_with_f64_within_range = 0;
        let rug_fuzz_0 = 42.0;
        let value: f64 = rug_fuzz_0;
        debug_assert_eq!(value.to_i64(), Some(42i64));
        let _rug_ed_tests_llm_16_966_llm_16_966_rrrruuuugggg_test_to_i64_with_f64_within_range = 0;
    }
    #[test]
    fn test_to_i64_with_f64_out_of_range() {
        let _rug_st_tests_llm_16_966_llm_16_966_rrrruuuugggg_test_to_i64_with_f64_out_of_range = 0;
        let rug_fuzz_0 = 2.0;
        let value: f64 = (i64::MAX as f64) * rug_fuzz_0;
        debug_assert_eq!(value.to_i64(), None);
        let _rug_ed_tests_llm_16_966_llm_16_966_rrrruuuugggg_test_to_i64_with_f64_out_of_range = 0;
    }
    #[test]
    fn test_to_i64_with_f32_within_range() {
        let _rug_st_tests_llm_16_966_llm_16_966_rrrruuuugggg_test_to_i64_with_f32_within_range = 0;
        let rug_fuzz_0 = 42.0;
        let value: f32 = rug_fuzz_0;
        debug_assert_eq!(value.to_i64(), Some(42i64));
        let _rug_ed_tests_llm_16_966_llm_16_966_rrrruuuugggg_test_to_i64_with_f32_within_range = 0;
    }
    #[test]
    fn test_to_i64_with_f32_out_of_range() {
        let _rug_st_tests_llm_16_966_llm_16_966_rrrruuuugggg_test_to_i64_with_f32_out_of_range = 0;
        let rug_fuzz_0 = 2.0;
        let value: f32 = (i64::MAX as f32) * rug_fuzz_0;
        debug_assert_eq!(value.to_i64(), None);
        let _rug_ed_tests_llm_16_966_llm_16_966_rrrruuuugggg_test_to_i64_with_f32_out_of_range = 0;
    }
    #[test]
    fn test_to_i64_with_negative() {
        let _rug_st_tests_llm_16_966_llm_16_966_rrrruuuugggg_test_to_i64_with_negative = 0;
        let rug_fuzz_0 = 42;
        let value: i32 = -rug_fuzz_0;
        debug_assert_eq!(value.to_i64(), Some(- 42i64));
        let _rug_ed_tests_llm_16_966_llm_16_966_rrrruuuugggg_test_to_i64_with_negative = 0;
    }
    #[test]
    fn test_to_i64_max_value() {
        let _rug_st_tests_llm_16_966_llm_16_966_rrrruuuugggg_test_to_i64_max_value = 0;
        let value = i64::MAX;
        debug_assert_eq!(value.to_i64(), Some(i64::MAX));
        let _rug_ed_tests_llm_16_966_llm_16_966_rrrruuuugggg_test_to_i64_max_value = 0;
    }
    #[test]
    fn test_to_i64_min_value() {
        let _rug_st_tests_llm_16_966_llm_16_966_rrrruuuugggg_test_to_i64_min_value = 0;
        let value = i64::MIN;
        debug_assert_eq!(value.to_i64(), Some(i64::MIN));
        let _rug_ed_tests_llm_16_966_llm_16_966_rrrruuuugggg_test_to_i64_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_967_llm_16_967 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_i8_with_i64() {
        let _rug_st_tests_llm_16_967_llm_16_967_rrrruuuugggg_test_to_i8_with_i64 = 0;
        let rug_fuzz_0 = 0i64;
        let rug_fuzz_1 = 127i64;
        let rug_fuzz_2 = 128i64;
        let rug_fuzz_3 = 128i64;
        let rug_fuzz_4 = 129i64;
        debug_assert_eq!(rug_fuzz_0.to_i8(), Some(0i8));
        debug_assert_eq!(rug_fuzz_1.to_i8(), Some(127i8));
        debug_assert_eq!((- rug_fuzz_2).to_i8(), Some(- 128i8));
        debug_assert_eq!(rug_fuzz_3.to_i8(), None);
        debug_assert_eq!((- rug_fuzz_4).to_i8(), None);
        debug_assert_eq!(i64::MAX.to_i8(), None);
        debug_assert_eq!(i64::MIN.to_i8(), None);
        let _rug_ed_tests_llm_16_967_llm_16_967_rrrruuuugggg_test_to_i8_with_i64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_969 {
    use super::*;
    use crate::*;
    #[test]
    fn test_to_u128_positive() {
        let _rug_st_tests_llm_16_969_rrrruuuugggg_test_to_u128_positive = 0;
        let rug_fuzz_0 = 123;
        let value: i64 = rug_fuzz_0;
        let result = <i64 as cast::ToPrimitive>::to_u128(&value);
        debug_assert_eq!(result, Some(123_u128));
        let _rug_ed_tests_llm_16_969_rrrruuuugggg_test_to_u128_positive = 0;
    }
    #[test]
    fn test_to_u128_zero() {
        let _rug_st_tests_llm_16_969_rrrruuuugggg_test_to_u128_zero = 0;
        let rug_fuzz_0 = 0;
        let value: i64 = rug_fuzz_0;
        let result = <i64 as cast::ToPrimitive>::to_u128(&value);
        debug_assert_eq!(result, Some(0_u128));
        let _rug_ed_tests_llm_16_969_rrrruuuugggg_test_to_u128_zero = 0;
    }
    #[test]
    fn test_to_u128_negative() {
        let _rug_st_tests_llm_16_969_rrrruuuugggg_test_to_u128_negative = 0;
        let rug_fuzz_0 = 123;
        let value: i64 = -rug_fuzz_0;
        let result = <i64 as cast::ToPrimitive>::to_u128(&value);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_969_rrrruuuugggg_test_to_u128_negative = 0;
    }
    #[test]
    fn test_to_u128_max() {
        let _rug_st_tests_llm_16_969_rrrruuuugggg_test_to_u128_max = 0;
        let value: i64 = i64::MAX;
        let result = <i64 as cast::ToPrimitive>::to_u128(&value);
        debug_assert_eq!(result, Some(i64::MAX as u128));
        let _rug_ed_tests_llm_16_969_rrrruuuugggg_test_to_u128_max = 0;
    }
    #[test]
    fn test_to_u128_min() {
        let _rug_st_tests_llm_16_969_rrrruuuugggg_test_to_u128_min = 0;
        let value: i64 = i64::MIN;
        let result = <i64 as cast::ToPrimitive>::to_u128(&value);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_969_rrrruuuugggg_test_to_u128_min = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_970_llm_16_970 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_u16_with_positive_i64_within_range() {
        let _rug_st_tests_llm_16_970_llm_16_970_rrrruuuugggg_test_to_u16_with_positive_i64_within_range = 0;
        let rug_fuzz_0 = 42;
        let value: i64 = rug_fuzz_0;
        let result = value.to_u16();
        debug_assert_eq!(result, Some(42u16));
        let _rug_ed_tests_llm_16_970_llm_16_970_rrrruuuugggg_test_to_u16_with_positive_i64_within_range = 0;
    }
    #[test]
    fn test_to_u16_with_negative_i64() {
        let _rug_st_tests_llm_16_970_llm_16_970_rrrruuuugggg_test_to_u16_with_negative_i64 = 0;
        let rug_fuzz_0 = 1;
        let value: i64 = -rug_fuzz_0;
        let result = value.to_u16();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_970_llm_16_970_rrrruuuugggg_test_to_u16_with_negative_i64 = 0;
    }
    #[test]
    fn test_to_u16_with_positive_i64_out_of_range() {
        let _rug_st_tests_llm_16_970_llm_16_970_rrrruuuugggg_test_to_u16_with_positive_i64_out_of_range = 0;
        let value: i64 = i64::MAX;
        let result = value.to_u16();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_970_llm_16_970_rrrruuuugggg_test_to_u16_with_positive_i64_out_of_range = 0;
    }
    #[test]
    fn test_to_u16_with_zero_i64() {
        let _rug_st_tests_llm_16_970_llm_16_970_rrrruuuugggg_test_to_u16_with_zero_i64 = 0;
        let rug_fuzz_0 = 0;
        let value: i64 = rug_fuzz_0;
        let result = value.to_u16();
        debug_assert_eq!(result, Some(0u16));
        let _rug_ed_tests_llm_16_970_llm_16_970_rrrruuuugggg_test_to_u16_with_zero_i64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_971_llm_16_971 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_u32_with_in_range_value() {
        let _rug_st_tests_llm_16_971_llm_16_971_rrrruuuugggg_test_to_u32_with_in_range_value = 0;
        let rug_fuzz_0 = 12345678;
        let value: i64 = rug_fuzz_0;
        let result = value.to_u32();
        debug_assert_eq!(result, Some(12345678u32));
        let _rug_ed_tests_llm_16_971_llm_16_971_rrrruuuugggg_test_to_u32_with_in_range_value = 0;
    }
    #[test]
    fn test_to_u32_with_negative_value() {
        let _rug_st_tests_llm_16_971_llm_16_971_rrrruuuugggg_test_to_u32_with_negative_value = 0;
        let rug_fuzz_0 = 12345678;
        let value: i64 = -rug_fuzz_0;
        let result = value.to_u32();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_971_llm_16_971_rrrruuuugggg_test_to_u32_with_negative_value = 0;
    }
    #[test]
    fn test_to_u32_with_out_of_range_value() {
        let _rug_st_tests_llm_16_971_llm_16_971_rrrruuuugggg_test_to_u32_with_out_of_range_value = 0;
        let value: i64 = i64::MAX;
        let result = value.to_u32();
        debug_assert!(result.is_none() || result == Some(i64::MAX as u32));
        let _rug_ed_tests_llm_16_971_llm_16_971_rrrruuuugggg_test_to_u32_with_out_of_range_value = 0;
    }
    #[test]
    fn test_to_u32_with_zero() {
        let _rug_st_tests_llm_16_971_llm_16_971_rrrruuuugggg_test_to_u32_with_zero = 0;
        let rug_fuzz_0 = 0;
        let value: i64 = rug_fuzz_0;
        let result = value.to_u32();
        debug_assert_eq!(result, Some(0u32));
        let _rug_ed_tests_llm_16_971_llm_16_971_rrrruuuugggg_test_to_u32_with_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_972_llm_16_972 {
    use crate::ToPrimitive;
    #[test]
    fn test_to_u64() {
        let _rug_st_tests_llm_16_972_llm_16_972_rrrruuuugggg_test_to_u64 = 0;
        let rug_fuzz_0 = 0i64;
        let rug_fuzz_1 = 1i64;
        let rug_fuzz_2 = 1i64;
        debug_assert_eq!(rug_fuzz_0.to_u64(), Some(0u64));
        debug_assert_eq!(rug_fuzz_1.to_u64(), Some(1u64));
        debug_assert_eq!(i64::MAX.to_u64(), Some(i64::MAX as u64));
        debug_assert_eq!((- rug_fuzz_2).to_u64(), None);
        let _rug_ed_tests_llm_16_972_llm_16_972_rrrruuuugggg_test_to_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_973_llm_16_973 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_u8_in_range() {
        let _rug_st_tests_llm_16_973_llm_16_973_rrrruuuugggg_test_to_u8_in_range = 0;
        let rug_fuzz_0 = 100i64;
        debug_assert_eq!((rug_fuzz_0).to_u8(), Some(100u8));
        let _rug_ed_tests_llm_16_973_llm_16_973_rrrruuuugggg_test_to_u8_in_range = 0;
    }
    #[test]
    fn test_to_u8_zero() {
        let _rug_st_tests_llm_16_973_llm_16_973_rrrruuuugggg_test_to_u8_zero = 0;
        let rug_fuzz_0 = 0i64;
        debug_assert_eq!((rug_fuzz_0).to_u8(), Some(0u8));
        let _rug_ed_tests_llm_16_973_llm_16_973_rrrruuuugggg_test_to_u8_zero = 0;
    }
    #[test]
    fn test_to_u8_at_max_bound() {
        let _rug_st_tests_llm_16_973_llm_16_973_rrrruuuugggg_test_to_u8_at_max_bound = 0;
        debug_assert_eq!((u8::MAX as i64).to_u8(), Some(u8::MAX));
        let _rug_ed_tests_llm_16_973_llm_16_973_rrrruuuugggg_test_to_u8_at_max_bound = 0;
    }
    #[test]
    fn test_to_u8_above_max_bound() {
        let _rug_st_tests_llm_16_973_llm_16_973_rrrruuuugggg_test_to_u8_above_max_bound = 0;
        let rug_fuzz_0 = 1;
        debug_assert_eq!(((u8::MAX as i64) + rug_fuzz_0).to_u8(), None);
        let _rug_ed_tests_llm_16_973_llm_16_973_rrrruuuugggg_test_to_u8_above_max_bound = 0;
    }
    #[test]
    fn test_to_u8_negative() {
        let _rug_st_tests_llm_16_973_llm_16_973_rrrruuuugggg_test_to_u8_negative = 0;
        let rug_fuzz_0 = 1i64;
        debug_assert_eq!((- rug_fuzz_0).to_u8(), None);
        let _rug_ed_tests_llm_16_973_llm_16_973_rrrruuuugggg_test_to_u8_negative = 0;
    }
    #[test]
    fn test_to_u8_well_below_zero() {
        let _rug_st_tests_llm_16_973_llm_16_973_rrrruuuugggg_test_to_u8_well_below_zero = 0;
        debug_assert_eq!((i64::MIN).to_u8(), None);
        let _rug_ed_tests_llm_16_973_llm_16_973_rrrruuuugggg_test_to_u8_well_below_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_974_llm_16_974 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_usize_with_positive_i64() {
        let _rug_st_tests_llm_16_974_llm_16_974_rrrruuuugggg_test_to_usize_with_positive_i64 = 0;
        let rug_fuzz_0 = 42;
        let value: i64 = rug_fuzz_0;
        let result = value.to_usize();
        debug_assert_eq!(result, Some(42));
        let _rug_ed_tests_llm_16_974_llm_16_974_rrrruuuugggg_test_to_usize_with_positive_i64 = 0;
    }
    #[test]
    fn test_to_usize_with_negative_i64() {
        let _rug_st_tests_llm_16_974_llm_16_974_rrrruuuugggg_test_to_usize_with_negative_i64 = 0;
        let rug_fuzz_0 = 42;
        let value: i64 = -rug_fuzz_0;
        let result = value.to_usize();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_974_llm_16_974_rrrruuuugggg_test_to_usize_with_negative_i64 = 0;
    }
    #[test]
    fn test_to_usize_with_i64_max() {
        let _rug_st_tests_llm_16_974_llm_16_974_rrrruuuugggg_test_to_usize_with_i64_max = 0;
        let value: i64 = i64::MAX;
        let result = value.to_usize();
        #[cfg(target_pointer_width = "64")]
        debug_assert_eq!(result, Some(i64::MAX as usize));
        #[cfg(not(target_pointer_width = "64"))] debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_974_llm_16_974_rrrruuuugggg_test_to_usize_with_i64_max = 0;
    }
    #[test]
    fn test_to_usize_with_i64_min() {
        let _rug_st_tests_llm_16_974_llm_16_974_rrrruuuugggg_test_to_usize_with_i64_min = 0;
        let value: i64 = i64::MIN;
        let result = value.to_usize();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_974_llm_16_974_rrrruuuugggg_test_to_usize_with_i64_min = 0;
    }
    #[test]
    #[cfg(target_pointer_width = "64")]
    fn test_to_usize_with_i64_max_overflow() {
        let _rug_st_tests_llm_16_974_llm_16_974_rrrruuuugggg_test_to_usize_with_i64_max_overflow = 0;
        let value: i64 = i64::MAX;
        let result = value.to_usize();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_974_llm_16_974_rrrruuuugggg_test_to_usize_with_i64_max_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1042_llm_16_1042 {
    use super::*;
    use crate::*;
    use crate::cast::AsPrimitive;
    #[test]
    fn i8_as_f32() {
        let _rug_st_tests_llm_16_1042_llm_16_1042_rrrruuuugggg_i8_as_f32 = 0;
        let rug_fuzz_0 = 42;
        let a: i8 = rug_fuzz_0;
        let b: f32 = AsPrimitive::<f32>::as_(a);
        let c: f32 = a as f32;
        debug_assert_eq!(b, c);
        let _rug_ed_tests_llm_16_1042_llm_16_1042_rrrruuuugggg_i8_as_f32 = 0;
    }
    #[test]
    fn i8_as_f32_negative() {
        let _rug_st_tests_llm_16_1042_llm_16_1042_rrrruuuugggg_i8_as_f32_negative = 0;
        let rug_fuzz_0 = 42;
        let a: i8 = -rug_fuzz_0;
        let b: f32 = AsPrimitive::<f32>::as_(a);
        let c: f32 = a as f32;
        debug_assert_eq!(b, c);
        let _rug_ed_tests_llm_16_1042_llm_16_1042_rrrruuuugggg_i8_as_f32_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1043_llm_16_1043 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_i8_as_f64() {
        let _rug_st_tests_llm_16_1043_llm_16_1043_rrrruuuugggg_test_i8_as_f64 = 0;
        let rug_fuzz_0 = 42;
        let x: i8 = rug_fuzz_0;
        let y: f64 = AsPrimitive::<f64>::as_(x);
        debug_assert_eq!(y, 42.0_f64);
        let _rug_ed_tests_llm_16_1043_llm_16_1043_rrrruuuugggg_test_i8_as_f64 = 0;
    }
    #[test]
    fn test_i8_as_f64_negative() {
        let _rug_st_tests_llm_16_1043_llm_16_1043_rrrruuuugggg_test_i8_as_f64_negative = 0;
        let rug_fuzz_0 = 42;
        let x: i8 = -rug_fuzz_0;
        let y: f64 = AsPrimitive::<f64>::as_(x);
        debug_assert_eq!(y, - 42.0_f64);
        let _rug_ed_tests_llm_16_1043_llm_16_1043_rrrruuuugggg_test_i8_as_f64_negative = 0;
    }
    #[test]
    fn test_i8_as_f64_min_value() {
        let _rug_st_tests_llm_16_1043_llm_16_1043_rrrruuuugggg_test_i8_as_f64_min_value = 0;
        let x: i8 = i8::MIN;
        let y: f64 = AsPrimitive::<f64>::as_(x);
        debug_assert_eq!(y, i8::MIN as f64);
        let _rug_ed_tests_llm_16_1043_llm_16_1043_rrrruuuugggg_test_i8_as_f64_min_value = 0;
    }
    #[test]
    fn test_i8_as_f64_max_value() {
        let _rug_st_tests_llm_16_1043_llm_16_1043_rrrruuuugggg_test_i8_as_f64_max_value = 0;
        let x: i8 = i8::MAX;
        let y: f64 = AsPrimitive::<f64>::as_(x);
        debug_assert_eq!(y, i8::MAX as f64);
        let _rug_ed_tests_llm_16_1043_llm_16_1043_rrrruuuugggg_test_i8_as_f64_max_value = 0;
    }
    #[test]
    fn test_i8_as_f64_zero() {
        let _rug_st_tests_llm_16_1043_llm_16_1043_rrrruuuugggg_test_i8_as_f64_zero = 0;
        let rug_fuzz_0 = 0;
        let x: i8 = rug_fuzz_0;
        let y: f64 = AsPrimitive::<f64>::as_(x);
        debug_assert_eq!(y, 0.0_f64);
        let _rug_ed_tests_llm_16_1043_llm_16_1043_rrrruuuugggg_test_i8_as_f64_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1044_llm_16_1044 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_i8_to_i128() {
        let _rug_st_tests_llm_16_1044_llm_16_1044_rrrruuuugggg_test_as_primitive_i8_to_i128 = 0;
        let rug_fuzz_0 = 42;
        let value: i8 = rug_fuzz_0;
        let result: i128 = AsPrimitive::<i128>::as_(value);
        debug_assert_eq!(result, 42i128);
        let _rug_ed_tests_llm_16_1044_llm_16_1044_rrrruuuugggg_test_as_primitive_i8_to_i128 = 0;
    }
    #[test]
    fn test_as_primitive_i8_to_i128_negative() {
        let _rug_st_tests_llm_16_1044_llm_16_1044_rrrruuuugggg_test_as_primitive_i8_to_i128_negative = 0;
        let rug_fuzz_0 = 42;
        let value: i8 = -rug_fuzz_0;
        let result: i128 = AsPrimitive::<i128>::as_(value);
        debug_assert_eq!(result, - 42i128);
        let _rug_ed_tests_llm_16_1044_llm_16_1044_rrrruuuugggg_test_as_primitive_i8_to_i128_negative = 0;
    }
    #[test]
    fn test_as_primitive_i8_to_i128_min() {
        let _rug_st_tests_llm_16_1044_llm_16_1044_rrrruuuugggg_test_as_primitive_i8_to_i128_min = 0;
        let value: i8 = i8::MIN;
        let result: i128 = AsPrimitive::<i128>::as_(value);
        debug_assert_eq!(result, i8::MIN as i128);
        let _rug_ed_tests_llm_16_1044_llm_16_1044_rrrruuuugggg_test_as_primitive_i8_to_i128_min = 0;
    }
    #[test]
    fn test_as_primitive_i8_to_i128_max() {
        let _rug_st_tests_llm_16_1044_llm_16_1044_rrrruuuugggg_test_as_primitive_i8_to_i128_max = 0;
        let value: i8 = i8::MAX;
        let result: i128 = AsPrimitive::<i128>::as_(value);
        debug_assert_eq!(result, i8::MAX as i128);
        let _rug_ed_tests_llm_16_1044_llm_16_1044_rrrruuuugggg_test_as_primitive_i8_to_i128_max = 0;
    }
    #[test]
    fn test_as_primitive_i8_to_i128_zero() {
        let _rug_st_tests_llm_16_1044_llm_16_1044_rrrruuuugggg_test_as_primitive_i8_to_i128_zero = 0;
        let rug_fuzz_0 = 0;
        let value: i8 = rug_fuzz_0;
        let result: i128 = AsPrimitive::<i128>::as_(value);
        debug_assert_eq!(result, 0i128);
        let _rug_ed_tests_llm_16_1044_llm_16_1044_rrrruuuugggg_test_as_primitive_i8_to_i128_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1045_llm_16_1045 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_i8_as_i16() {
        let _rug_st_tests_llm_16_1045_llm_16_1045_rrrruuuugggg_test_i8_as_i16 = 0;
        let rug_fuzz_0 = 100;
        let value: i8 = rug_fuzz_0;
        let casted_value: i16 = value.as_();
        debug_assert_eq!(casted_value, 100i16);
        let _rug_ed_tests_llm_16_1045_llm_16_1045_rrrruuuugggg_test_i8_as_i16 = 0;
    }
    #[test]
    fn test_i8_as_i16_negative() {
        let _rug_st_tests_llm_16_1045_llm_16_1045_rrrruuuugggg_test_i8_as_i16_negative = 0;
        let rug_fuzz_0 = 100;
        let value: i8 = -rug_fuzz_0;
        let casted_value: i16 = value.as_();
        debug_assert_eq!(casted_value, - 100i16);
        let _rug_ed_tests_llm_16_1045_llm_16_1045_rrrruuuugggg_test_i8_as_i16_negative = 0;
    }
    #[test]
    fn test_i8_as_i16_zero() {
        let _rug_st_tests_llm_16_1045_llm_16_1045_rrrruuuugggg_test_i8_as_i16_zero = 0;
        let rug_fuzz_0 = 0;
        let value: i8 = rug_fuzz_0;
        let casted_value: i16 = value.as_();
        debug_assert_eq!(casted_value, 0i16);
        let _rug_ed_tests_llm_16_1045_llm_16_1045_rrrruuuugggg_test_i8_as_i16_zero = 0;
    }
    #[test]
    fn test_i8_as_i16_max() {
        let _rug_st_tests_llm_16_1045_llm_16_1045_rrrruuuugggg_test_i8_as_i16_max = 0;
        let value: i8 = i8::MAX;
        let casted_value: i16 = value.as_();
        debug_assert_eq!(casted_value, i8::MAX as i16);
        let _rug_ed_tests_llm_16_1045_llm_16_1045_rrrruuuugggg_test_i8_as_i16_max = 0;
    }
    #[test]
    fn test_i8_as_i16_min() {
        let _rug_st_tests_llm_16_1045_llm_16_1045_rrrruuuugggg_test_i8_as_i16_min = 0;
        let value: i8 = i8::MIN;
        let casted_value: i16 = value.as_();
        debug_assert_eq!(casted_value, i8::MIN as i16);
        let _rug_ed_tests_llm_16_1045_llm_16_1045_rrrruuuugggg_test_i8_as_i16_min = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1046_llm_16_1046 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_i8_to_i32() {
        let _rug_st_tests_llm_16_1046_llm_16_1046_rrrruuuugggg_test_as_primitive_i8_to_i32 = 0;
        let rug_fuzz_0 = 8;
        let value: i8 = rug_fuzz_0;
        let result: i32 = value.as_();
        debug_assert_eq!(result, 8i32);
        let _rug_ed_tests_llm_16_1046_llm_16_1046_rrrruuuugggg_test_as_primitive_i8_to_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1047_llm_16_1047 {
    use crate::AsPrimitive;
    #[test]
    fn test_as_primitive_i8_to_i64() {
        let _rug_st_tests_llm_16_1047_llm_16_1047_rrrruuuugggg_test_as_primitive_i8_to_i64 = 0;
        let rug_fuzz_0 = 123;
        let value_i8: i8 = rug_fuzz_0;
        let value_i64: i64 = value_i8.as_();
        debug_assert_eq!(value_i64, 123i64);
        let _rug_ed_tests_llm_16_1047_llm_16_1047_rrrruuuugggg_test_as_primitive_i8_to_i64 = 0;
    }
    #[test]
    fn test_as_primitive_i8_to_i64_negative() {
        let _rug_st_tests_llm_16_1047_llm_16_1047_rrrruuuugggg_test_as_primitive_i8_to_i64_negative = 0;
        let rug_fuzz_0 = 123;
        let value_i8: i8 = -rug_fuzz_0;
        let value_i64: i64 = value_i8.as_();
        debug_assert_eq!(value_i64, - 123i64);
        let _rug_ed_tests_llm_16_1047_llm_16_1047_rrrruuuugggg_test_as_primitive_i8_to_i64_negative = 0;
    }
    #[test]
    fn test_as_primitive_i8_to_i64_min() {
        let _rug_st_tests_llm_16_1047_llm_16_1047_rrrruuuugggg_test_as_primitive_i8_to_i64_min = 0;
        let value_i8: i8 = i8::MIN;
        let value_i64: i64 = value_i8.as_();
        debug_assert_eq!(value_i64, i64::from(i8::MIN));
        let _rug_ed_tests_llm_16_1047_llm_16_1047_rrrruuuugggg_test_as_primitive_i8_to_i64_min = 0;
    }
    #[test]
    fn test_as_primitive_i8_to_i64_max() {
        let _rug_st_tests_llm_16_1047_llm_16_1047_rrrruuuugggg_test_as_primitive_i8_to_i64_max = 0;
        let value_i8: i8 = i8::MAX;
        let value_i64: i64 = value_i8.as_();
        debug_assert_eq!(value_i64, i64::from(i8::MAX));
        let _rug_ed_tests_llm_16_1047_llm_16_1047_rrrruuuugggg_test_as_primitive_i8_to_i64_max = 0;
    }
    #[test]
    fn test_as_primitive_i8_to_i64_zero() {
        let _rug_st_tests_llm_16_1047_llm_16_1047_rrrruuuugggg_test_as_primitive_i8_to_i64_zero = 0;
        let rug_fuzz_0 = 0;
        let value_i8: i8 = rug_fuzz_0;
        let value_i64: i64 = value_i8.as_();
        debug_assert_eq!(value_i64, 0i64);
        let _rug_ed_tests_llm_16_1047_llm_16_1047_rrrruuuugggg_test_as_primitive_i8_to_i64_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1048_llm_16_1048 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_i8_to_i8() {
        let _rug_st_tests_llm_16_1048_llm_16_1048_rrrruuuugggg_test_as_primitive_i8_to_i8 = 0;
        let rug_fuzz_0 = 42;
        let value: i8 = rug_fuzz_0;
        let result: i8 = AsPrimitive::<i8>::as_(value);
        debug_assert_eq!(result, 42i8);
        let _rug_ed_tests_llm_16_1048_llm_16_1048_rrrruuuugggg_test_as_primitive_i8_to_i8 = 0;
    }
    #[test]
    fn test_as_primitive_i8_to_i8_negative() {
        let _rug_st_tests_llm_16_1048_llm_16_1048_rrrruuuugggg_test_as_primitive_i8_to_i8_negative = 0;
        let rug_fuzz_0 = 42;
        let value: i8 = -rug_fuzz_0;
        let result: i8 = AsPrimitive::<i8>::as_(value);
        debug_assert_eq!(result, - 42i8);
        let _rug_ed_tests_llm_16_1048_llm_16_1048_rrrruuuugggg_test_as_primitive_i8_to_i8_negative = 0;
    }
    #[test]
    fn test_as_primitive_i8_to_i8_zero() {
        let _rug_st_tests_llm_16_1048_llm_16_1048_rrrruuuugggg_test_as_primitive_i8_to_i8_zero = 0;
        let rug_fuzz_0 = 0;
        let value: i8 = rug_fuzz_0;
        let result: i8 = AsPrimitive::<i8>::as_(value);
        debug_assert_eq!(result, 0i8);
        let _rug_ed_tests_llm_16_1048_llm_16_1048_rrrruuuugggg_test_as_primitive_i8_to_i8_zero = 0;
    }
    #[test]
    fn test_as_primitive_i8_to_i8_max() {
        let _rug_st_tests_llm_16_1048_llm_16_1048_rrrruuuugggg_test_as_primitive_i8_to_i8_max = 0;
        let value: i8 = i8::MAX;
        let result: i8 = AsPrimitive::<i8>::as_(value);
        debug_assert_eq!(result, i8::MAX);
        let _rug_ed_tests_llm_16_1048_llm_16_1048_rrrruuuugggg_test_as_primitive_i8_to_i8_max = 0;
    }
    #[test]
    fn test_as_primitive_i8_to_i8_min() {
        let _rug_st_tests_llm_16_1048_llm_16_1048_rrrruuuugggg_test_as_primitive_i8_to_i8_min = 0;
        let value: i8 = i8::MIN;
        let result: i8 = AsPrimitive::<i8>::as_(value);
        debug_assert_eq!(result, i8::MIN);
        let _rug_ed_tests_llm_16_1048_llm_16_1048_rrrruuuugggg_test_as_primitive_i8_to_i8_min = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1049_llm_16_1049 {
    use crate::cast::AsPrimitive;
    #[test]
    fn i8_as_isize() {
        let _rug_st_tests_llm_16_1049_llm_16_1049_rrrruuuugggg_i8_as_isize = 0;
        let rug_fuzz_0 = 42;
        let val: i8 = rug_fuzz_0;
        let result = AsPrimitive::<isize>::as_(val);
        debug_assert_eq!(result, 42isize);
        let _rug_ed_tests_llm_16_1049_llm_16_1049_rrrruuuugggg_i8_as_isize = 0;
    }
    #[test]
    fn i8_as_isize_negative() {
        let _rug_st_tests_llm_16_1049_llm_16_1049_rrrruuuugggg_i8_as_isize_negative = 0;
        let rug_fuzz_0 = 42;
        let val: i8 = -rug_fuzz_0;
        let result = AsPrimitive::<isize>::as_(val);
        debug_assert_eq!(result, - 42isize);
        let _rug_ed_tests_llm_16_1049_llm_16_1049_rrrruuuugggg_i8_as_isize_negative = 0;
    }
    #[test]
    fn i8_as_isize_max() {
        let _rug_st_tests_llm_16_1049_llm_16_1049_rrrruuuugggg_i8_as_isize_max = 0;
        let val: i8 = i8::MAX;
        let result = AsPrimitive::<isize>::as_(val);
        debug_assert_eq!(result, i8::MAX as isize);
        let _rug_ed_tests_llm_16_1049_llm_16_1049_rrrruuuugggg_i8_as_isize_max = 0;
    }
    #[test]
    fn i8_as_isize_min() {
        let _rug_st_tests_llm_16_1049_llm_16_1049_rrrruuuugggg_i8_as_isize_min = 0;
        let val: i8 = i8::MIN;
        let result = AsPrimitive::<isize>::as_(val);
        debug_assert_eq!(result, i8::MIN as isize);
        let _rug_ed_tests_llm_16_1049_llm_16_1049_rrrruuuugggg_i8_as_isize_min = 0;
    }
    #[test]
    fn i8_as_isize_zero() {
        let _rug_st_tests_llm_16_1049_llm_16_1049_rrrruuuugggg_i8_as_isize_zero = 0;
        let rug_fuzz_0 = 0;
        let val: i8 = rug_fuzz_0;
        let result = AsPrimitive::<isize>::as_(val);
        debug_assert_eq!(result, 0isize);
        let _rug_ed_tests_llm_16_1049_llm_16_1049_rrrruuuugggg_i8_as_isize_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1050_llm_16_1050 {
    use super::*;
    use crate::*;
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_i8_to_u128() {
        let _rug_st_tests_llm_16_1050_llm_16_1050_rrrruuuugggg_test_as_primitive_i8_to_u128 = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let val_i8: i8 = i8::MAX;
        let val_as_u128: u128 = AsPrimitive::<u128>::as_(val_i8);
        debug_assert_eq!(val_as_u128, i8::MAX as u128);
        let val_i8: i8 = i8::MIN;
        let val_as_u128: u128 = AsPrimitive::<u128>::as_(val_i8);
        debug_assert_eq!(val_as_u128, i8::MIN as u128);
        let val_i8: i8 = rug_fuzz_0;
        let val_as_u128: u128 = AsPrimitive::<u128>::as_(val_i8);
        debug_assert_eq!(val_as_u128, 0u128);
        let val_i8: i8 = -rug_fuzz_1;
        let val_as_u128: u128 = AsPrimitive::<u128>::as_(val_i8);
        debug_assert_eq!(val_as_u128, u128::MAX);
        let val_i8: i8 = rug_fuzz_2;
        let val_as_u128: u128 = AsPrimitive::<u128>::as_(val_i8);
        debug_assert_eq!(val_as_u128, 1u128);
        let _rug_ed_tests_llm_16_1050_llm_16_1050_rrrruuuugggg_test_as_primitive_i8_to_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1051_llm_16_1051 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_i8_as_u16() {
        let _rug_st_tests_llm_16_1051_llm_16_1051_rrrruuuugggg_test_i8_as_u16 = 0;
        let rug_fuzz_0 = 42;
        let value: i8 = rug_fuzz_0;
        let as_u16: u16 = AsPrimitive::<u16>::as_(value);
        debug_assert_eq!(as_u16, 42u16);
        let _rug_ed_tests_llm_16_1051_llm_16_1051_rrrruuuugggg_test_i8_as_u16 = 0;
    }
    #[test]
    fn test_i8_as_u16_negative() {
        let _rug_st_tests_llm_16_1051_llm_16_1051_rrrruuuugggg_test_i8_as_u16_negative = 0;
        let rug_fuzz_0 = 42;
        let value: i8 = -rug_fuzz_0;
        let as_u16: u16 = AsPrimitive::<u16>::as_(value);
        debug_assert_eq!(as_u16, 214u16);
        let _rug_ed_tests_llm_16_1051_llm_16_1051_rrrruuuugggg_test_i8_as_u16_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1052_llm_16_1052 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_i8_as_u32() {
        let _rug_st_tests_llm_16_1052_llm_16_1052_rrrruuuugggg_test_i8_as_u32 = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 127;
        let a: i8 = -rug_fuzz_0;
        let b: u32 = <i8 as AsPrimitive<u32>>::as_(a);
        debug_assert_eq!(b, u32::MAX);
        let c: i8 = rug_fuzz_1;
        let d: u32 = <i8 as AsPrimitive<u32>>::as_(c);
        debug_assert_eq!(d, 0);
        let e: i8 = rug_fuzz_2;
        let f: u32 = <i8 as AsPrimitive<u32>>::as_(e);
        debug_assert_eq!(f, 127);
        let _rug_ed_tests_llm_16_1052_llm_16_1052_rrrruuuugggg_test_i8_as_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1053_llm_16_1053 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_i8_to_u64() {
        let _rug_st_tests_llm_16_1053_llm_16_1053_rrrruuuugggg_test_as_primitive_i8_to_u64 = 0;
        let rug_fuzz_0 = 42;
        let value_i8: i8 = rug_fuzz_0;
        let value_u64: u64 = AsPrimitive::<u64>::as_(value_i8);
        debug_assert_eq!(value_u64, 42u64);
        let _rug_ed_tests_llm_16_1053_llm_16_1053_rrrruuuugggg_test_as_primitive_i8_to_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1054_llm_16_1054 {
    use crate::cast::AsPrimitive;
    #[test]
    fn i8_as_u8() {
        let _rug_st_tests_llm_16_1054_llm_16_1054_rrrruuuugggg_i8_as_u8 = 0;
        let rug_fuzz_0 = 1i8;
        let val_i8 = -rug_fuzz_0;
        let val_u8: u8 = AsPrimitive::as_(val_i8);
        debug_assert_eq!(val_u8, 0xFFu8);
        let _rug_ed_tests_llm_16_1054_llm_16_1054_rrrruuuugggg_i8_as_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1055_llm_16_1055 {
    use crate::cast::AsPrimitive;
    #[test]
    fn i8_as_usize() {
        let _rug_st_tests_llm_16_1055_llm_16_1055_rrrruuuugggg_i8_as_usize = 0;
        let rug_fuzz_0 = 7;
        let value: i8 = rug_fuzz_0;
        let result: usize = AsPrimitive::<usize>::as_(value);
        debug_assert_eq!(result, value as usize);
        let _rug_ed_tests_llm_16_1055_llm_16_1055_rrrruuuugggg_i8_as_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1056_llm_16_1056 {
    use crate::FromPrimitive;
    #[test]
    fn test_from_f32_to_i8() {
        let _rug_st_tests_llm_16_1056_llm_16_1056_rrrruuuugggg_test_from_f32_to_i8 = 0;
        let rug_fuzz_0 = 0.0_f32;
        let rug_fuzz_1 = 127.0_f32;
        let rug_fuzz_2 = 128.0_f32;
        let rug_fuzz_3 = 127.999_f32;
        let rug_fuzz_4 = 128.999_f32;
        let rug_fuzz_5 = 128.0_f32;
        let rug_fuzz_6 = 129.0_f32;
        debug_assert_eq!(< i8 as FromPrimitive > ::from_f32(rug_fuzz_0), Some(0i8));
        debug_assert_eq!(< i8 as FromPrimitive > ::from_f32(rug_fuzz_1), Some(127i8));
        debug_assert_eq!(
            < i8 as FromPrimitive > ::from_f32(- rug_fuzz_2), Some(- 128i8)
        );
        debug_assert_eq!(< i8 as FromPrimitive > ::from_f32(rug_fuzz_3), Some(127i8));
        debug_assert_eq!(
            < i8 as FromPrimitive > ::from_f32(- rug_fuzz_4), Some(- 128i8)
        );
        debug_assert_eq!(< i8 as FromPrimitive > ::from_f32(rug_fuzz_5), None);
        debug_assert_eq!(< i8 as FromPrimitive > ::from_f32(- rug_fuzz_6), None);
        debug_assert_eq!(< i8 as FromPrimitive > ::from_f32(f32::NAN), None);
        debug_assert_eq!(< i8 as FromPrimitive > ::from_f32(f32::INFINITY), None);
        debug_assert_eq!(< i8 as FromPrimitive > ::from_f32(f32::NEG_INFINITY), None);
        let _rug_ed_tests_llm_16_1056_llm_16_1056_rrrruuuugggg_test_from_f32_to_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1057_llm_16_1057 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_f64_to_i8() {
        let _rug_st_tests_llm_16_1057_llm_16_1057_rrrruuuugggg_test_from_f64_to_i8 = 0;
        let rug_fuzz_0 = 42.0;
        let rug_fuzz_1 = 127.0;
        let rug_fuzz_2 = 128.0;
        let rug_fuzz_3 = 128.0;
        let rug_fuzz_4 = 129.0;
        let rug_fuzz_5 = 0.0;
        let rug_fuzz_6 = 0.0;
        let rug_fuzz_7 = 42.7;
        let rug_fuzz_8 = 42.7;
        debug_assert_eq!(< i8 as FromPrimitive > ::from_f64(rug_fuzz_0), Some(42_i8));
        debug_assert_eq!(< i8 as FromPrimitive > ::from_f64(rug_fuzz_1), Some(127_i8));
        debug_assert_eq!(< i8 as FromPrimitive > ::from_f64(rug_fuzz_2), None);
        debug_assert_eq!(
            < i8 as FromPrimitive > ::from_f64(- rug_fuzz_3), Some(- 128_i8)
        );
        debug_assert_eq!(< i8 as FromPrimitive > ::from_f64(- rug_fuzz_4), None);
        debug_assert_eq!(< i8 as FromPrimitive > ::from_f64(rug_fuzz_5), Some(0_i8));
        debug_assert_eq!(< i8 as FromPrimitive > ::from_f64(- rug_fuzz_6), Some(0_i8));
        debug_assert_eq!(< i8 as FromPrimitive > ::from_f64(rug_fuzz_7), Some(42_i8));
        debug_assert_eq!(
            < i8 as FromPrimitive > ::from_f64(- rug_fuzz_8), Some(- 42_i8)
        );
        debug_assert_eq!(< i8 as FromPrimitive > ::from_f64(f64::NAN), None);
        debug_assert_eq!(< i8 as FromPrimitive > ::from_f64(f64::INFINITY), None);
        debug_assert_eq!(< i8 as FromPrimitive > ::from_f64(f64::NEG_INFINITY), None);
        let _rug_ed_tests_llm_16_1057_llm_16_1057_rrrruuuugggg_test_from_f64_to_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1058_llm_16_1058 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i128() {
        let _rug_st_tests_llm_16_1058_llm_16_1058_rrrruuuugggg_test_from_i128 = 0;
        let rug_fuzz_0 = 127i128;
        let rug_fuzz_1 = 128i128;
        let rug_fuzz_2 = 128i128;
        let rug_fuzz_3 = 129i128;
        debug_assert_eq!(< i8 as FromPrimitive > ::from_i128(rug_fuzz_0), Some(127i8));
        debug_assert_eq!(
            < i8 as FromPrimitive > ::from_i128(- rug_fuzz_1), Some(- 128i8)
        );
        debug_assert_eq!(< i8 as FromPrimitive > ::from_i128(rug_fuzz_2), None);
        debug_assert_eq!(< i8 as FromPrimitive > ::from_i128(- rug_fuzz_3), None);
        debug_assert_eq!(< i8 as FromPrimitive > ::from_i128(i128::MAX), None);
        debug_assert_eq!(< i8 as FromPrimitive > ::from_i128(i128::MIN), None);
        let _rug_ed_tests_llm_16_1058_llm_16_1058_rrrruuuugggg_test_from_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1059_llm_16_1059 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i16_for_i8() {
        let test_cases = [
            (0_i16, Some(0_i8)),
            (127_i16, Some(127_i8)),
            (128_i16, None),
            (-128_i16, Some(-128_i8)),
            (-129_i16, None),
            (i16::MAX, None),
            (i16::MIN, None),
        ];
        for (input, expected) in test_cases {
            assert_eq!(< i8 as FromPrimitive >::from_i16(input), expected);
        }
    }
}
#[cfg(test)]
mod tests_llm_16_1060_llm_16_1060 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i32() {
        let _rug_st_tests_llm_16_1060_llm_16_1060_rrrruuuugggg_test_from_i32 = 0;
        let rug_fuzz_0 = 0_i32;
        let rug_fuzz_1 = 127_i32;
        let rug_fuzz_2 = 128_i32;
        let rug_fuzz_3 = 128_i32;
        let rug_fuzz_4 = 129_i32;
        debug_assert_eq!(< i8 as FromPrimitive > ::from_i32(rug_fuzz_0), Some(0_i8));
        debug_assert_eq!(< i8 as FromPrimitive > ::from_i32(rug_fuzz_1), Some(127_i8));
        debug_assert_eq!(
            < i8 as FromPrimitive > ::from_i32(- rug_fuzz_2), Some(- 128_i8)
        );
        debug_assert_eq!(< i8 as FromPrimitive > ::from_i32(rug_fuzz_3), None);
        debug_assert_eq!(< i8 as FromPrimitive > ::from_i32(- rug_fuzz_4), None);
        let _rug_ed_tests_llm_16_1060_llm_16_1060_rrrruuuugggg_test_from_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1061_llm_16_1061 {
    use crate::FromPrimitive;
    #[test]
    fn from_i64_with_in_range_value() {
        let _rug_st_tests_llm_16_1061_llm_16_1061_rrrruuuugggg_from_i64_with_in_range_value = 0;
        let rug_fuzz_0 = 42;
        let n: i64 = rug_fuzz_0;
        let result = <i8 as FromPrimitive>::from_i64(n);
        debug_assert_eq!(result, Some(42i8));
        let _rug_ed_tests_llm_16_1061_llm_16_1061_rrrruuuugggg_from_i64_with_in_range_value = 0;
    }
    #[test]
    fn from_i64_with_value_too_large() {
        let _rug_st_tests_llm_16_1061_llm_16_1061_rrrruuuugggg_from_i64_with_value_too_large = 0;
        let rug_fuzz_0 = 128;
        let n: i64 = rug_fuzz_0;
        let result = <i8 as FromPrimitive>::from_i64(n);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1061_llm_16_1061_rrrruuuugggg_from_i64_with_value_too_large = 0;
    }
    #[test]
    fn from_i64_with_value_too_small() {
        let _rug_st_tests_llm_16_1061_llm_16_1061_rrrruuuugggg_from_i64_with_value_too_small = 0;
        let rug_fuzz_0 = 129;
        let n: i64 = -rug_fuzz_0;
        let result = <i8 as FromPrimitive>::from_i64(n);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1061_llm_16_1061_rrrruuuugggg_from_i64_with_value_too_small = 0;
    }
    #[test]
    fn from_i64_with_minimum_i8() {
        let _rug_st_tests_llm_16_1061_llm_16_1061_rrrruuuugggg_from_i64_with_minimum_i8 = 0;
        let n: i64 = i8::MIN as i64;
        let result = <i8 as FromPrimitive>::from_i64(n);
        debug_assert_eq!(result, Some(i8::MIN));
        let _rug_ed_tests_llm_16_1061_llm_16_1061_rrrruuuugggg_from_i64_with_minimum_i8 = 0;
    }
    #[test]
    fn from_i64_with_maximum_i8() {
        let _rug_st_tests_llm_16_1061_llm_16_1061_rrrruuuugggg_from_i64_with_maximum_i8 = 0;
        let n: i64 = i8::MAX as i64;
        let result = <i8 as FromPrimitive>::from_i64(n);
        debug_assert_eq!(result, Some(i8::MAX));
        let _rug_ed_tests_llm_16_1061_llm_16_1061_rrrruuuugggg_from_i64_with_maximum_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1062_llm_16_1062 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i8_with_positive_value() {
        let _rug_st_tests_llm_16_1062_llm_16_1062_rrrruuuugggg_test_from_i8_with_positive_value = 0;
        let rug_fuzz_0 = 42;
        let value: i8 = rug_fuzz_0;
        let result = <i8 as FromPrimitive>::from_i8(value);
        debug_assert_eq!(result, Some(42i8));
        let _rug_ed_tests_llm_16_1062_llm_16_1062_rrrruuuugggg_test_from_i8_with_positive_value = 0;
    }
    #[test]
    fn test_from_i8_with_negative_value() {
        let _rug_st_tests_llm_16_1062_llm_16_1062_rrrruuuugggg_test_from_i8_with_negative_value = 0;
        let rug_fuzz_0 = 42;
        let value: i8 = -rug_fuzz_0;
        let result = <i8 as FromPrimitive>::from_i8(value);
        debug_assert_eq!(result, Some(- 42i8));
        let _rug_ed_tests_llm_16_1062_llm_16_1062_rrrruuuugggg_test_from_i8_with_negative_value = 0;
    }
    #[test]
    fn test_from_i8_with_min_value() {
        let _rug_st_tests_llm_16_1062_llm_16_1062_rrrruuuugggg_test_from_i8_with_min_value = 0;
        let value: i8 = i8::MIN;
        let result = <i8 as FromPrimitive>::from_i8(value);
        debug_assert_eq!(result, Some(i8::MIN));
        let _rug_ed_tests_llm_16_1062_llm_16_1062_rrrruuuugggg_test_from_i8_with_min_value = 0;
    }
    #[test]
    fn test_from_i8_with_max_value() {
        let _rug_st_tests_llm_16_1062_llm_16_1062_rrrruuuugggg_test_from_i8_with_max_value = 0;
        let value: i8 = i8::MAX;
        let result = <i8 as FromPrimitive>::from_i8(value);
        debug_assert_eq!(result, Some(i8::MAX));
        let _rug_ed_tests_llm_16_1062_llm_16_1062_rrrruuuugggg_test_from_i8_with_max_value = 0;
    }
    #[test]
    fn test_from_i8_with_zero() {
        let _rug_st_tests_llm_16_1062_llm_16_1062_rrrruuuugggg_test_from_i8_with_zero = 0;
        let rug_fuzz_0 = 0;
        let value: i8 = rug_fuzz_0;
        let result = <i8 as FromPrimitive>::from_i8(value);
        debug_assert_eq!(result, Some(0i8));
        let _rug_ed_tests_llm_16_1062_llm_16_1062_rrrruuuugggg_test_from_i8_with_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1063_llm_16_1063 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_isize() {
        let _rug_st_tests_llm_16_1063_llm_16_1063_rrrruuuugggg_test_from_isize = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 127;
        let rug_fuzz_2 = 128;
        let rug_fuzz_3 = 128;
        let rug_fuzz_4 = 129;
        debug_assert_eq!(< i8 as FromPrimitive > ::from_isize(rug_fuzz_0), Some(0i8));
        debug_assert_eq!(< i8 as FromPrimitive > ::from_isize(rug_fuzz_1), Some(127i8));
        debug_assert_eq!(
            < i8 as FromPrimitive > ::from_isize(- rug_fuzz_2), Some(- 128i8)
        );
        debug_assert_eq!(< i8 as FromPrimitive > ::from_isize(rug_fuzz_3), None);
        debug_assert_eq!(< i8 as FromPrimitive > ::from_isize(- rug_fuzz_4), None);
        let _rug_ed_tests_llm_16_1063_llm_16_1063_rrrruuuugggg_test_from_isize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1064_llm_16_1064 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u128() {
        let _rug_st_tests_llm_16_1064_llm_16_1064_rrrruuuugggg_test_from_u128 = 0;
        let rug_fuzz_0 = 0_u128;
        let rug_fuzz_1 = 127_u128;
        let rug_fuzz_2 = 128_u128;
        debug_assert_eq!(< i8 as FromPrimitive > ::from_u128(rug_fuzz_0), Some(0));
        debug_assert_eq!(< i8 as FromPrimitive > ::from_u128(rug_fuzz_1), Some(127));
        debug_assert_eq!(< i8 as FromPrimitive > ::from_u128(rug_fuzz_2), None);
        debug_assert_eq!(< i8 as FromPrimitive > ::from_u128(u128::MAX), None);
        let _rug_ed_tests_llm_16_1064_llm_16_1064_rrrruuuugggg_test_from_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1065_llm_16_1065 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u16() {
        let _rug_st_tests_llm_16_1065_llm_16_1065_rrrruuuugggg_test_from_u16 = 0;
        let rug_fuzz_0 = 0_u16;
        let rug_fuzz_1 = 127_u16;
        let rug_fuzz_2 = 128_u16;
        let rug_fuzz_3 = 255_u16;
        debug_assert_eq!(< i8 as FromPrimitive > ::from_u16(rug_fuzz_0), Some(0_i8));
        debug_assert_eq!(< i8 as FromPrimitive > ::from_u16(rug_fuzz_1), Some(127_i8));
        debug_assert_eq!(< i8 as FromPrimitive > ::from_u16(rug_fuzz_2), None);
        debug_assert_eq!(< i8 as FromPrimitive > ::from_u16(rug_fuzz_3), None);
        debug_assert_eq!(< i8 as FromPrimitive > ::from_u16(u16::max_value()), None);
        let _rug_ed_tests_llm_16_1065_llm_16_1065_rrrruuuugggg_test_from_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1066_llm_16_1066 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u32() {
        let _rug_st_tests_llm_16_1066_llm_16_1066_rrrruuuugggg_test_from_u32 = 0;
        let rug_fuzz_0 = 0_u32;
        let rug_fuzz_1 = 127_u32;
        let rug_fuzz_2 = 128_u32;
        let rug_fuzz_3 = 255_u32;
        debug_assert_eq!(< i8 as FromPrimitive > ::from_u32(rug_fuzz_0), Some(0i8));
        debug_assert_eq!(< i8 as FromPrimitive > ::from_u32(rug_fuzz_1), Some(127i8));
        debug_assert_eq!(< i8 as FromPrimitive > ::from_u32(rug_fuzz_2), None);
        debug_assert_eq!(< i8 as FromPrimitive > ::from_u32(rug_fuzz_3), None);
        debug_assert_eq!(< i8 as FromPrimitive > ::from_u32(u32::MAX), None);
        let _rug_ed_tests_llm_16_1066_llm_16_1066_rrrruuuugggg_test_from_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1067_llm_16_1067 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u64_with_i8() {
        let _rug_st_tests_llm_16_1067_llm_16_1067_rrrruuuugggg_test_from_u64_with_i8 = 0;
        let rug_fuzz_0 = 0_u64;
        let rug_fuzz_1 = 127_u64;
        let rug_fuzz_2 = 128_u64;
        let rug_fuzz_3 = 255_u64;
        debug_assert_eq!(< i8 as FromPrimitive > ::from_u64(rug_fuzz_0), Some(0_i8));
        debug_assert_eq!(< i8 as FromPrimitive > ::from_u64(rug_fuzz_1), Some(127_i8));
        debug_assert_eq!(< i8 as FromPrimitive > ::from_u64(rug_fuzz_2), None);
        debug_assert_eq!(< i8 as FromPrimitive > ::from_u64(rug_fuzz_3), None);
        let _rug_ed_tests_llm_16_1067_llm_16_1067_rrrruuuugggg_test_from_u64_with_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1068_llm_16_1068 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u8() {
        let _rug_st_tests_llm_16_1068_llm_16_1068_rrrruuuugggg_test_from_u8 = 0;
        let rug_fuzz_0 = false;
        let rug_fuzz_1 = true;
        let rug_fuzz_2 = 100u8;
        let rug_fuzz_3 = 200u8;
        let val_u8_max = u8::MAX;
        match <i8 as FromPrimitive>::from_u8(val_u8_max) {
            Some(val) => debug_assert!(rug_fuzz_0, "Expected None for u8::MAX into i8"),
            None => debug_assert!(rug_fuzz_1),
        }
        let val_u8_in_range = rug_fuzz_2;
        debug_assert_eq!(
            < i8 as FromPrimitive > ::from_u8(val_u8_in_range), Some(val_u8_in_range as
            i8)
        );
        let val_u8_out_of_range = rug_fuzz_3;
        debug_assert_eq!(< i8 as FromPrimitive > ::from_u8(val_u8_out_of_range), None);
        let _rug_ed_tests_llm_16_1068_llm_16_1068_rrrruuuugggg_test_from_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1069_llm_16_1069 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_usize() {
        let _rug_st_tests_llm_16_1069_llm_16_1069_rrrruuuugggg_test_from_usize = 0;
        let rug_fuzz_0 = 0_usize;
        let rug_fuzz_1 = 127_usize;
        let rug_fuzz_2 = 128_usize;
        let rug_fuzz_3 = 255_usize;
        debug_assert_eq!(< i8 as FromPrimitive > ::from_usize(rug_fuzz_0), Some(0i8));
        debug_assert_eq!(< i8 as FromPrimitive > ::from_usize(rug_fuzz_1), Some(127i8));
        debug_assert_eq!(< i8 as FromPrimitive > ::from_usize(rug_fuzz_2), None);
        debug_assert_eq!(< i8 as FromPrimitive > ::from_usize(rug_fuzz_3), None);
        debug_assert_eq!(< i8 as FromPrimitive > ::from_usize(usize::MAX), None);
        let _rug_ed_tests_llm_16_1069_llm_16_1069_rrrruuuugggg_test_from_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1071_llm_16_1071 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_i8_to_f32() {
        let _rug_st_tests_llm_16_1071_llm_16_1071_rrrruuuugggg_test_i8_to_f32 = 0;
        let rug_fuzz_0 = 0_i8;
        let rug_fuzz_1 = 1_i8;
        let rug_fuzz_2 = 1_i8;
        debug_assert_eq!(ToPrimitive::to_f32(& rug_fuzz_0), Some(0.0f32));
        debug_assert_eq!(ToPrimitive::to_f32(& rug_fuzz_1), Some(1.0f32));
        debug_assert_eq!(ToPrimitive::to_f32(& - rug_fuzz_2), Some(- 1.0f32));
        debug_assert_eq!(ToPrimitive::to_f32(& i8::MAX), Some(127.0f32));
        debug_assert_eq!(ToPrimitive::to_f32(& i8::MIN), Some(- 128.0f32));
        let _rug_ed_tests_llm_16_1071_llm_16_1071_rrrruuuugggg_test_i8_to_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1072 {
    use super::*;
    use crate::*;
    #[test]
    fn test_i8_to_f64() {
        let _rug_st_tests_llm_16_1072_rrrruuuugggg_test_i8_to_f64 = 0;
        let rug_fuzz_0 = 0;
        let values: Vec<i8> = vec![rug_fuzz_0, 1, - 1, i8::MIN, i8::MAX];
        for &val in &values {
            let float_val: Option<f64> = val.to_f64();
            debug_assert_eq!(float_val, Some(val as f64));
        }
        let _rug_ed_tests_llm_16_1072_rrrruuuugggg_test_i8_to_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1073_llm_16_1073 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_i128() {
        let _rug_st_tests_llm_16_1073_llm_16_1073_rrrruuuugggg_test_to_i128 = 0;
        let rug_fuzz_0 = 0i8;
        let rug_fuzz_1 = 1i8;
        let rug_fuzz_2 = 1i8;
        debug_assert_eq!(rug_fuzz_0.to_i128(), Some(0i128));
        debug_assert_eq!(rug_fuzz_1.to_i128(), Some(1i128));
        debug_assert_eq!((- rug_fuzz_2).to_i128(), Some(- 1i128));
        debug_assert_eq!(i8::MAX.to_i128(), Some(i8::MAX as i128));
        debug_assert_eq!(i8::MIN.to_i128(), Some(i8::MIN as i128));
        let _rug_ed_tests_llm_16_1073_llm_16_1073_rrrruuuugggg_test_to_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1074_llm_16_1074 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_i16_with_i8() {
        let _rug_st_tests_llm_16_1074_llm_16_1074_rrrruuuugggg_test_to_i16_with_i8 = 0;
        let rug_fuzz_0 = 1i8;
        let rug_fuzz_1 = 1i16;
        let rug_fuzz_2 = 0i8;
        let rug_fuzz_3 = 0i16;
        let rug_fuzz_4 = 1i8;
        let rug_fuzz_5 = 1i16;
        let test_values = [
            (i8::MIN, Some(i16::MIN)),
            (-rug_fuzz_0, Some(-rug_fuzz_1)),
            (rug_fuzz_2, Some(rug_fuzz_3)),
            (rug_fuzz_4, Some(rug_fuzz_5)),
            (i8::MAX, Some(i16::MAX)),
        ];
        for &(val, expected) in test_values.iter() {
            debug_assert_eq!(val.to_i16(), expected);
        }
        let _rug_ed_tests_llm_16_1074_llm_16_1074_rrrruuuugggg_test_to_i16_with_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1075_llm_16_1075 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_i32_with_i8_within_bounds() {
        let _rug_st_tests_llm_16_1075_llm_16_1075_rrrruuuugggg_test_to_i32_with_i8_within_bounds = 0;
        let rug_fuzz_0 = 42;
        let value: i8 = rug_fuzz_0;
        let result = ToPrimitive::to_i32(&value);
        debug_assert_eq!(result, Some(42i32));
        let _rug_ed_tests_llm_16_1075_llm_16_1075_rrrruuuugggg_test_to_i32_with_i8_within_bounds = 0;
    }
    #[test]
    fn test_to_i32_with_i8_at_upper_bound() {
        let _rug_st_tests_llm_16_1075_llm_16_1075_rrrruuuugggg_test_to_i32_with_i8_at_upper_bound = 0;
        let value: i8 = i8::MAX;
        let result = ToPrimitive::to_i32(&value);
        debug_assert_eq!(result, Some(i32::from(i8::MAX)));
        let _rug_ed_tests_llm_16_1075_llm_16_1075_rrrruuuugggg_test_to_i32_with_i8_at_upper_bound = 0;
    }
    #[test]
    fn test_to_i32_with_i8_at_lower_bound() {
        let _rug_st_tests_llm_16_1075_llm_16_1075_rrrruuuugggg_test_to_i32_with_i8_at_lower_bound = 0;
        let value: i8 = i8::MIN;
        let result = ToPrimitive::to_i32(&value);
        debug_assert_eq!(result, Some(i32::from(i8::MIN)));
        let _rug_ed_tests_llm_16_1075_llm_16_1075_rrrruuuugggg_test_to_i32_with_i8_at_lower_bound = 0;
    }
    #[test]
    fn test_to_i32_with_i8_at_zero() {
        let _rug_st_tests_llm_16_1075_llm_16_1075_rrrruuuugggg_test_to_i32_with_i8_at_zero = 0;
        let rug_fuzz_0 = 0;
        let value: i8 = rug_fuzz_0;
        let result = ToPrimitive::to_i32(&value);
        debug_assert_eq!(result, Some(0i32));
        let _rug_ed_tests_llm_16_1075_llm_16_1075_rrrruuuugggg_test_to_i32_with_i8_at_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1076_llm_16_1076 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_i64_with_i8() {
        let _rug_st_tests_llm_16_1076_llm_16_1076_rrrruuuugggg_test_to_i64_with_i8 = 0;
        let rug_fuzz_0 = 0i8;
        let rug_fuzz_1 = 123i8;
        let rug_fuzz_2 = 123i8;
        let min_i8 = i8::MIN;
        let max_i8 = i8::MAX;
        let zero_i8 = rug_fuzz_0;
        let pos_i8 = rug_fuzz_1;
        let neg_i8 = -rug_fuzz_2;
        debug_assert_eq!(min_i8.to_i64(), Some(i64::from(min_i8)));
        debug_assert_eq!(max_i8.to_i64(), Some(i64::from(max_i8)));
        debug_assert_eq!(zero_i8.to_i64(), Some(i64::from(zero_i8)));
        debug_assert_eq!(pos_i8.to_i64(), Some(i64::from(pos_i8)));
        debug_assert_eq!(neg_i8.to_i64(), Some(i64::from(neg_i8)));
        let _rug_ed_tests_llm_16_1076_llm_16_1076_rrrruuuugggg_test_to_i64_with_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1077_llm_16_1077 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_i8_within_bounds() {
        let _rug_st_tests_llm_16_1077_llm_16_1077_rrrruuuugggg_test_to_i8_within_bounds = 0;
        let rug_fuzz_0 = 42i8;
        let rug_fuzz_1 = 0i8;
        let rug_fuzz_2 = 42i8;
        debug_assert_eq!((rug_fuzz_0).to_i8(), Some(42));
        debug_assert_eq!((rug_fuzz_1).to_i8(), Some(0));
        debug_assert_eq!((- rug_fuzz_2).to_i8(), Some(- 42));
        let _rug_ed_tests_llm_16_1077_llm_16_1077_rrrruuuugggg_test_to_i8_within_bounds = 0;
    }
    #[test]
    fn test_to_i8_out_of_bounds() {
        let _rug_st_tests_llm_16_1077_llm_16_1077_rrrruuuugggg_test_to_i8_out_of_bounds = 0;
        let rug_fuzz_0 = 300i16;
        let rug_fuzz_1 = 300i16;
        let rug_fuzz_2 = 300i32;
        let rug_fuzz_3 = 300i32;
        let rug_fuzz_4 = 300i64;
        let rug_fuzz_5 = 300i64;
        let rug_fuzz_6 = 300i128;
        let rug_fuzz_7 = 300i128;
        debug_assert_eq!((rug_fuzz_0).to_i8(), None);
        debug_assert_eq!((- rug_fuzz_1).to_i8(), None);
        debug_assert_eq!((rug_fuzz_2).to_i8(), None);
        debug_assert_eq!((- rug_fuzz_3).to_i8(), None);
        debug_assert_eq!((rug_fuzz_4).to_i8(), None);
        debug_assert_eq!((- rug_fuzz_5).to_i8(), None);
        debug_assert_eq!((rug_fuzz_6).to_i8(), None);
        debug_assert_eq!((- rug_fuzz_7).to_i8(), None);
        let _rug_ed_tests_llm_16_1077_llm_16_1077_rrrruuuugggg_test_to_i8_out_of_bounds = 0;
    }
    #[test]
    fn test_to_i8_edge_cases() {
        let _rug_st_tests_llm_16_1077_llm_16_1077_rrrruuuugggg_test_to_i8_edge_cases = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 1;
        debug_assert_eq!((i8::MAX as i16).to_i8(), Some(i8::MAX));
        debug_assert_eq!((i8::MIN as i16).to_i8(), Some(i8::MIN));
        debug_assert_eq!(((i8::MAX as i16) + rug_fuzz_0).to_i8(), None);
        debug_assert_eq!(((i8::MIN as i16) - rug_fuzz_1).to_i8(), None);
        let _rug_ed_tests_llm_16_1077_llm_16_1077_rrrruuuugggg_test_to_i8_edge_cases = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1078_llm_16_1078 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_isize() {
        assert_eq!(0i8.to_isize(), Some(0isize));
        assert_eq!(127i8.to_isize(), Some(127isize));
        assert_eq!((- 128i8).to_isize(), Some(- 128isize));
    }
}
#[cfg(test)]
mod tests_llm_16_1079_llm_16_1079 {
    use crate::ToPrimitive;
    #[test]
    fn test_to_u128_with_positive_i8() {
        let _rug_st_tests_llm_16_1079_llm_16_1079_rrrruuuugggg_test_to_u128_with_positive_i8 = 0;
        let rug_fuzz_0 = 100;
        let value: i8 = rug_fuzz_0;
        debug_assert_eq!(value.to_u128(), Some(100_u128));
        let _rug_ed_tests_llm_16_1079_llm_16_1079_rrrruuuugggg_test_to_u128_with_positive_i8 = 0;
    }
    #[test]
    fn test_to_u128_with_zero_i8() {
        let _rug_st_tests_llm_16_1079_llm_16_1079_rrrruuuugggg_test_to_u128_with_zero_i8 = 0;
        let rug_fuzz_0 = 0;
        let value: i8 = rug_fuzz_0;
        debug_assert_eq!(value.to_u128(), Some(0_u128));
        let _rug_ed_tests_llm_16_1079_llm_16_1079_rrrruuuugggg_test_to_u128_with_zero_i8 = 0;
    }
    #[test]
    fn test_to_u128_with_negative_i8() {
        let _rug_st_tests_llm_16_1079_llm_16_1079_rrrruuuugggg_test_to_u128_with_negative_i8 = 0;
        let rug_fuzz_0 = 1;
        let value: i8 = -rug_fuzz_0;
        debug_assert_eq!(value.to_u128(), None);
        let _rug_ed_tests_llm_16_1079_llm_16_1079_rrrruuuugggg_test_to_u128_with_negative_i8 = 0;
    }
    #[test]
    fn test_to_u128_with_max_i8() {
        let _rug_st_tests_llm_16_1079_llm_16_1079_rrrruuuugggg_test_to_u128_with_max_i8 = 0;
        let value: i8 = i8::MAX;
        debug_assert_eq!(value.to_u128(), Some(i8::MAX as u128));
        let _rug_ed_tests_llm_16_1079_llm_16_1079_rrrruuuugggg_test_to_u128_with_max_i8 = 0;
    }
    #[test]
    fn test_to_u128_with_min_i8() {
        let _rug_st_tests_llm_16_1079_llm_16_1079_rrrruuuugggg_test_to_u128_with_min_i8 = 0;
        let value: i8 = i8::MIN;
        debug_assert_eq!(value.to_u128(), None);
        let _rug_ed_tests_llm_16_1079_llm_16_1079_rrrruuuugggg_test_to_u128_with_min_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1080_llm_16_1080 {
    use crate::cast::ToPrimitive;
    #[test]
    fn to_u16_with_positive_i8() {
        let _rug_st_tests_llm_16_1080_llm_16_1080_rrrruuuugggg_to_u16_with_positive_i8 = 0;
        let rug_fuzz_0 = 127;
        let value: i8 = rug_fuzz_0;
        debug_assert_eq!(value.to_u16(), Some(127u16));
        let _rug_ed_tests_llm_16_1080_llm_16_1080_rrrruuuugggg_to_u16_with_positive_i8 = 0;
    }
    #[test]
    fn to_u16_with_negative_i8() {
        let _rug_st_tests_llm_16_1080_llm_16_1080_rrrruuuugggg_to_u16_with_negative_i8 = 0;
        let rug_fuzz_0 = 1;
        let value: i8 = -rug_fuzz_0;
        debug_assert_eq!(value.to_u16(), None);
        let _rug_ed_tests_llm_16_1080_llm_16_1080_rrrruuuugggg_to_u16_with_negative_i8 = 0;
    }
    #[test]
    fn to_u16_with_zero_i8() {
        let _rug_st_tests_llm_16_1080_llm_16_1080_rrrruuuugggg_to_u16_with_zero_i8 = 0;
        let rug_fuzz_0 = 0;
        let value: i8 = rug_fuzz_0;
        debug_assert_eq!(value.to_u16(), Some(0u16));
        let _rug_ed_tests_llm_16_1080_llm_16_1080_rrrruuuugggg_to_u16_with_zero_i8 = 0;
    }
    #[test]
    fn to_u16_with_i8_exceeding_u16() {
        let _rug_st_tests_llm_16_1080_llm_16_1080_rrrruuuugggg_to_u16_with_i8_exceeding_u16 = 0;
        let _rug_ed_tests_llm_16_1080_llm_16_1080_rrrruuuugggg_to_u16_with_i8_exceeding_u16 = 0;
    }
    #[test]
    fn to_u16_with_i8_min_value() {
        let _rug_st_tests_llm_16_1080_llm_16_1080_rrrruuuugggg_to_u16_with_i8_min_value = 0;
        let value: i8 = i8::MIN;
        debug_assert_eq!(value.to_u16(), None);
        let _rug_ed_tests_llm_16_1080_llm_16_1080_rrrruuuugggg_to_u16_with_i8_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1081_llm_16_1081 {
    use crate::ToPrimitive;
    #[test]
    fn to_u32_with_positive_i8() {
        let _rug_st_tests_llm_16_1081_llm_16_1081_rrrruuuugggg_to_u32_with_positive_i8 = 0;
        let rug_fuzz_0 = 100;
        let value: i8 = rug_fuzz_0;
        debug_assert_eq!(value.to_u32(), Some(100_u32));
        let _rug_ed_tests_llm_16_1081_llm_16_1081_rrrruuuugggg_to_u32_with_positive_i8 = 0;
    }
    #[test]
    fn to_u32_with_zero_i8() {
        let _rug_st_tests_llm_16_1081_llm_16_1081_rrrruuuugggg_to_u32_with_zero_i8 = 0;
        let rug_fuzz_0 = 0;
        let value: i8 = rug_fuzz_0;
        debug_assert_eq!(value.to_u32(), Some(0_u32));
        let _rug_ed_tests_llm_16_1081_llm_16_1081_rrrruuuugggg_to_u32_with_zero_i8 = 0;
    }
    #[test]
    fn to_u32_with_negative_i8() {
        let _rug_st_tests_llm_16_1081_llm_16_1081_rrrruuuugggg_to_u32_with_negative_i8 = 0;
        let rug_fuzz_0 = 1;
        let value: i8 = -rug_fuzz_0;
        debug_assert_eq!(value.to_u32(), None);
        let _rug_ed_tests_llm_16_1081_llm_16_1081_rrrruuuugggg_to_u32_with_negative_i8 = 0;
    }
    #[test]
    fn to_u32_with_i8_max() {
        let _rug_st_tests_llm_16_1081_llm_16_1081_rrrruuuugggg_to_u32_with_i8_max = 0;
        let value: i8 = i8::MAX;
        debug_assert_eq!(value.to_u32(), Some(i8::MAX as u32));
        let _rug_ed_tests_llm_16_1081_llm_16_1081_rrrruuuugggg_to_u32_with_i8_max = 0;
    }
    #[test]
    fn to_u32_with_i8_min() {
        let _rug_st_tests_llm_16_1081_llm_16_1081_rrrruuuugggg_to_u32_with_i8_min = 0;
        let value: i8 = i8::MIN;
        debug_assert_eq!(value.to_u32(), None);
        let _rug_ed_tests_llm_16_1081_llm_16_1081_rrrruuuugggg_to_u32_with_i8_min = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1082_llm_16_1082 {
    use crate::cast::ToPrimitive;
    #[test]
    fn to_u64_with_positive_i8() {
        let _rug_st_tests_llm_16_1082_llm_16_1082_rrrruuuugggg_to_u64_with_positive_i8 = 0;
        let rug_fuzz_0 = 42;
        let value: i8 = rug_fuzz_0;
        let result = value.to_u64();
        debug_assert_eq!(result, Some(42u64));
        let _rug_ed_tests_llm_16_1082_llm_16_1082_rrrruuuugggg_to_u64_with_positive_i8 = 0;
    }
    #[test]
    fn to_u64_with_negative_i8() {
        let _rug_st_tests_llm_16_1082_llm_16_1082_rrrruuuugggg_to_u64_with_negative_i8 = 0;
        let rug_fuzz_0 = 42;
        let value: i8 = -rug_fuzz_0;
        let result = value.to_u64();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1082_llm_16_1082_rrrruuuugggg_to_u64_with_negative_i8 = 0;
    }
    #[test]
    fn to_u64_with_i8_max() {
        let _rug_st_tests_llm_16_1082_llm_16_1082_rrrruuuugggg_to_u64_with_i8_max = 0;
        let value: i8 = i8::MAX;
        let result = value.to_u64();
        debug_assert_eq!(result, Some(i8::MAX as u64));
        let _rug_ed_tests_llm_16_1082_llm_16_1082_rrrruuuugggg_to_u64_with_i8_max = 0;
    }
    #[test]
    fn to_u64_with_i8_min() {
        let _rug_st_tests_llm_16_1082_llm_16_1082_rrrruuuugggg_to_u64_with_i8_min = 0;
        let value: i8 = i8::MIN;
        let result = value.to_u64();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1082_llm_16_1082_rrrruuuugggg_to_u64_with_i8_min = 0;
    }
    #[test]
    fn to_u64_with_zero_i8() {
        let _rug_st_tests_llm_16_1082_llm_16_1082_rrrruuuugggg_to_u64_with_zero_i8 = 0;
        let rug_fuzz_0 = 0;
        let value: i8 = rug_fuzz_0;
        let result = value.to_u64();
        debug_assert_eq!(result, Some(0u64));
        let _rug_ed_tests_llm_16_1082_llm_16_1082_rrrruuuugggg_to_u64_with_zero_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1083_llm_16_1083 {
    use crate::ToPrimitive;
    #[test]
    fn i8_to_u8_cast_within_bounds() {
        assert_eq!(0i8.to_u8(), Some(0u8));
        assert_eq!(1i8.to_u8(), Some(1u8));
        assert_eq!(127i8.to_u8(), Some(127u8));
    }
    #[test]
    fn i8_to_u8_cast_out_of_bounds() {
        assert_eq!((- 1i8).to_u8(), None);
        assert_eq!((- 128i8).to_u8(), None);
    }
}
#[cfg(test)]
mod tests_llm_16_1084_llm_16_1084 {
    use crate::ToPrimitive;
    #[test]
    fn test_to_usize_with_i8() {
        let _rug_st_tests_llm_16_1084_llm_16_1084_rrrruuuugggg_test_to_usize_with_i8 = 0;
        let rug_fuzz_0 = 0i8;
        let rug_fuzz_1 = 1i8;
        let rug_fuzz_2 = 1i8;
        debug_assert_eq!(rug_fuzz_0.to_usize(), Some(0usize));
        debug_assert_eq!(rug_fuzz_1.to_usize(), Some(1usize));
        debug_assert_eq!((- rug_fuzz_2).to_usize(), None);
        debug_assert_eq!(i8::MAX.to_usize(), Some(127usize));
        debug_assert_eq!(i8::MIN.to_usize(), None);
        let _rug_ed_tests_llm_16_1084_llm_16_1084_rrrruuuugggg_test_to_usize_with_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1152_llm_16_1152 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_isize_to_f32() {
        let _rug_st_tests_llm_16_1152_llm_16_1152_rrrruuuugggg_test_as_primitive_isize_to_f32 = 0;
        let rug_fuzz_0 = 42;
        let value: isize = rug_fuzz_0;
        let result: f32 = AsPrimitive::<f32>::as_(value);
        debug_assert_eq!(result, 42.0_f32);
        let _rug_ed_tests_llm_16_1152_llm_16_1152_rrrruuuugggg_test_as_primitive_isize_to_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1153_llm_16_1153 {
    use super::*;
    use crate::*;
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_isize_to_f64() {
        let _rug_st_tests_llm_16_1153_llm_16_1153_rrrruuuugggg_test_as_primitive_isize_to_f64 = 0;
        let rug_fuzz_0 = 42;
        let value: isize = rug_fuzz_0;
        let result: f64 = value.as_();
        debug_assert_eq!(result, 42f64);
        let _rug_ed_tests_llm_16_1153_llm_16_1153_rrrruuuugggg_test_as_primitive_isize_to_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1154_llm_16_1154 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_i128() {
        let _rug_st_tests_llm_16_1154_llm_16_1154_rrrruuuugggg_test_as_primitive_i128 = 0;
        let rug_fuzz_0 = 42;
        let x: isize = rug_fuzz_0;
        let y: i128 = AsPrimitive::<i128>::as_(x);
        debug_assert_eq!(y, 42i128);
        let max_isize: isize = isize::MAX;
        let max_i128: i128 = AsPrimitive::<i128>::as_(max_isize);
        debug_assert_eq!(max_i128, isize::MAX as i128);
        let min_isize: isize = isize::MIN;
        let min_i128: i128 = AsPrimitive::<i128>::as_(min_isize);
        debug_assert_eq!(min_i128, isize::MIN as i128);
        let _rug_ed_tests_llm_16_1154_llm_16_1154_rrrruuuugggg_test_as_primitive_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1155_llm_16_1155 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_isize_to_i16() {
        let _rug_st_tests_llm_16_1155_llm_16_1155_rrrruuuugggg_test_as_isize_to_i16 = 0;
        let rug_fuzz_0 = 42;
        let val_isize: isize = rug_fuzz_0;
        let val_i16: i16 = AsPrimitive::as_(val_isize);
        debug_assert_eq!(val_i16, 42i16);
        let _rug_ed_tests_llm_16_1155_llm_16_1155_rrrruuuugggg_test_as_isize_to_i16 = 0;
    }
    #[test]
    fn test_as_isize_to_i16_negative() {
        let _rug_st_tests_llm_16_1155_llm_16_1155_rrrruuuugggg_test_as_isize_to_i16_negative = 0;
        let rug_fuzz_0 = 42;
        let val_isize: isize = -rug_fuzz_0;
        let val_i16: i16 = AsPrimitive::as_(val_isize);
        debug_assert_eq!(val_i16, - 42i16);
        let _rug_ed_tests_llm_16_1155_llm_16_1155_rrrruuuugggg_test_as_isize_to_i16_negative = 0;
    }
    #[test]
    #[should_panic(expected = "attempt to cast with overflow")]
    fn test_as_isize_to_i16_overflow() {
        let _rug_st_tests_llm_16_1155_llm_16_1155_rrrruuuugggg_test_as_isize_to_i16_overflow = 0;
        let rug_fuzz_0 = 1;
        let val_isize: isize = i16::MAX as isize + rug_fuzz_0;
        let _val_i16: i16 = AsPrimitive::as_(val_isize);
        let _rug_ed_tests_llm_16_1155_llm_16_1155_rrrruuuugggg_test_as_isize_to_i16_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1156_llm_16_1156 {
    use crate::AsPrimitive;
    #[test]
    fn as_primitive_from_isize_to_i32() {
        let _rug_st_tests_llm_16_1156_llm_16_1156_rrrruuuugggg_as_primitive_from_isize_to_i32 = 0;
        let rug_fuzz_0 = 42;
        let value: isize = rug_fuzz_0;
        let result: i32 = AsPrimitive::<i32>::as_(value);
        debug_assert_eq!(result, 42i32);
        let _rug_ed_tests_llm_16_1156_llm_16_1156_rrrruuuugggg_as_primitive_from_isize_to_i32 = 0;
    }
    #[test]
    fn as_primitive_from_large_isize_to_i32() {
        let _rug_st_tests_llm_16_1156_llm_16_1156_rrrruuuugggg_as_primitive_from_large_isize_to_i32 = 0;
        let value: isize = isize::MAX;
        let result: i32 = AsPrimitive::<i32>::as_(value);
        debug_assert_eq!(result as isize, isize::MAX.min(i32::MAX as isize));
        let _rug_ed_tests_llm_16_1156_llm_16_1156_rrrruuuugggg_as_primitive_from_large_isize_to_i32 = 0;
    }
    #[test]
    fn as_primitive_from_small_isize_to_i32() {
        let _rug_st_tests_llm_16_1156_llm_16_1156_rrrruuuugggg_as_primitive_from_small_isize_to_i32 = 0;
        let value: isize = isize::MIN;
        let result: i32 = AsPrimitive::<i32>::as_(value);
        debug_assert_eq!(result as isize, isize::MIN.max(i32::MIN as isize));
        let _rug_ed_tests_llm_16_1156_llm_16_1156_rrrruuuugggg_as_primitive_from_small_isize_to_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1157_llm_16_1157 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_isize_to_i64() {
        let _rug_st_tests_llm_16_1157_llm_16_1157_rrrruuuugggg_test_as_primitive_isize_to_i64 = 0;
        let rug_fuzz_0 = 42;
        let value: isize = rug_fuzz_0;
        let result: i64 = AsPrimitive::<i64>::as_(value);
        debug_assert_eq!(result, 42i64);
        let _rug_ed_tests_llm_16_1157_llm_16_1157_rrrruuuugggg_test_as_primitive_isize_to_i64 = 0;
    }
    #[test]
    fn test_as_primitive_isize_to_i64_negative() {
        let _rug_st_tests_llm_16_1157_llm_16_1157_rrrruuuugggg_test_as_primitive_isize_to_i64_negative = 0;
        let rug_fuzz_0 = 42;
        let value: isize = -rug_fuzz_0;
        let result: i64 = AsPrimitive::<i64>::as_(value);
        debug_assert_eq!(result, - 42i64);
        let _rug_ed_tests_llm_16_1157_llm_16_1157_rrrruuuugggg_test_as_primitive_isize_to_i64_negative = 0;
    }
    #[test]
    fn test_as_primitive_isize_to_i64_min() {
        let _rug_st_tests_llm_16_1157_llm_16_1157_rrrruuuugggg_test_as_primitive_isize_to_i64_min = 0;
        let value: isize = isize::MIN;
        let result: i64 = AsPrimitive::<i64>::as_(value);
        debug_assert_eq!(result, isize::MIN as i64);
        let _rug_ed_tests_llm_16_1157_llm_16_1157_rrrruuuugggg_test_as_primitive_isize_to_i64_min = 0;
    }
    #[test]
    fn test_as_primitive_isize_to_i64_max() {
        let _rug_st_tests_llm_16_1157_llm_16_1157_rrrruuuugggg_test_as_primitive_isize_to_i64_max = 0;
        let value: isize = isize::MAX;
        let result: i64 = AsPrimitive::<i64>::as_(value);
        debug_assert_eq!(result, isize::MAX as i64);
        let _rug_ed_tests_llm_16_1157_llm_16_1157_rrrruuuugggg_test_as_primitive_isize_to_i64_max = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1158_llm_16_1158 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_isize_to_i8() {
        let _rug_st_tests_llm_16_1158_llm_16_1158_rrrruuuugggg_test_as_primitive_isize_to_i8 = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = 128;
        let rug_fuzz_3 = 129;
        let value: isize = rug_fuzz_0;
        let result: i8 = AsPrimitive::as_(value);
        debug_assert_eq!(result, 42i8);
        let value: isize = -rug_fuzz_1;
        let result: i8 = AsPrimitive::as_(value);
        debug_assert_eq!(result, - 42i8);
        let value: isize = rug_fuzz_2;
        let result: i8 = AsPrimitive::as_(value);
        debug_assert_eq!(result, value as i8);
        let value: isize = -rug_fuzz_3;
        let result: i8 = AsPrimitive::as_(value);
        debug_assert_eq!(result, value as i8);
        let _rug_ed_tests_llm_16_1158_llm_16_1158_rrrruuuugggg_test_as_primitive_isize_to_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1159_llm_16_1159 {
    use crate::AsPrimitive;
    #[test]
    fn test_as_primitive_isize_to_isize() {
        let _rug_st_tests_llm_16_1159_llm_16_1159_rrrruuuugggg_test_as_primitive_isize_to_isize = 0;
        let rug_fuzz_0 = 42;
        let x: isize = rug_fuzz_0;
        let y: isize = AsPrimitive::<isize>::as_(x);
        debug_assert_eq!(y, 42);
        let _rug_ed_tests_llm_16_1159_llm_16_1159_rrrruuuugggg_test_as_primitive_isize_to_isize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1161_llm_16_1161 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_isize_to_u16() {
        let _rug_st_tests_llm_16_1161_llm_16_1161_rrrruuuugggg_test_as_primitive_isize_to_u16 = 0;
        let rug_fuzz_0 = 42;
        let x: isize = rug_fuzz_0;
        let y: u16 = AsPrimitive::<u16>::as_(x);
        debug_assert_eq!(y, 42u16);
        let _rug_ed_tests_llm_16_1161_llm_16_1161_rrrruuuugggg_test_as_primitive_isize_to_u16 = 0;
    }
    #[test]
    #[should_panic(expected = "attempt to cast with overflow")]
    fn test_as_primitive_isize_to_u16_overflow() {
        let _rug_st_tests_llm_16_1161_llm_16_1161_rrrruuuugggg_test_as_primitive_isize_to_u16_overflow = 0;
        let x: isize = isize::MAX;
        let _: u16 = AsPrimitive::<u16>::as_(x);
        let _rug_ed_tests_llm_16_1161_llm_16_1161_rrrruuuugggg_test_as_primitive_isize_to_u16_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1162_llm_16_1162 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_isize_to_u32() {
        let _rug_st_tests_llm_16_1162_llm_16_1162_rrrruuuugggg_test_as_primitive_isize_to_u32 = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1;
        debug_assert_eq!(< isize as AsPrimitive < u32 > > ::as_(- rug_fuzz_0), u32::MAX);
        debug_assert_eq!(< isize as AsPrimitive < u32 > > ::as_(rug_fuzz_1), 0_u32);
        debug_assert_eq!(< isize as AsPrimitive < u32 > > ::as_(rug_fuzz_2), 1_u32);
        debug_assert_eq!(
            < isize as AsPrimitive < u32 > > ::as_(isize::MAX), isize::MAX as u32
        );
        let _rug_ed_tests_llm_16_1162_llm_16_1162_rrrruuuugggg_test_as_primitive_isize_to_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1163_llm_16_1163 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_isize_to_u64() {
        let _rug_st_tests_llm_16_1163_llm_16_1163_rrrruuuugggg_test_as_primitive_isize_to_u64 = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 1;
        let value: isize = rug_fuzz_0;
        let result: u64 = <isize as AsPrimitive<u64>>::as_(value);
        debug_assert_eq!(result, 42u64);
        let negative_value: isize = -rug_fuzz_1;
        let wrapping_result: u64 = <isize as AsPrimitive<u64>>::as_(negative_value);
        let expected_wrapping_value: u64 = negative_value as u64;
        debug_assert_eq!(wrapping_result, expected_wrapping_value);
        let _rug_ed_tests_llm_16_1163_llm_16_1163_rrrruuuugggg_test_as_primitive_isize_to_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1165_llm_16_1165 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_isize_to_usize() {
        let _rug_st_tests_llm_16_1165_llm_16_1165_rrrruuuugggg_test_as_primitive_isize_to_usize = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42;
        let val_isize: isize = rug_fuzz_0;
        let expected_usize: usize = rug_fuzz_1;
        let result: usize = val_isize.as_();
        debug_assert_eq!(
            result, expected_usize,
            "Casting from isize to usize did not produce the expected result."
        );
        let _rug_ed_tests_llm_16_1165_llm_16_1165_rrrruuuugggg_test_as_primitive_isize_to_usize = 0;
    }
    #[test]
    #[should_panic(
        expected = "attempt to cast to usize with value greater than usize::MAX"
    )]
    fn test_as_primitive_isize_to_usize_overflow() {
        let _rug_st_tests_llm_16_1165_llm_16_1165_rrrruuuugggg_test_as_primitive_isize_to_usize_overflow = 0;
        let val_isize: isize = isize::MAX;
        let _: usize = val_isize.as_();
        let _rug_ed_tests_llm_16_1165_llm_16_1165_rrrruuuugggg_test_as_primitive_isize_to_usize_overflow = 0;
    }
    #[test]
    fn test_as_primitive_isize_to_usize_negative() {
        let _rug_st_tests_llm_16_1165_llm_16_1165_rrrruuuugggg_test_as_primitive_isize_to_usize_negative = 0;
        let rug_fuzz_0 = 42;
        let val_isize: isize = -rug_fuzz_0;
        let result: usize = val_isize.as_();
        debug_assert_eq!(
            result, isize::MAX as usize - 41,
            "Casting from negative isize to usize did not yield the expected result."
        );
        let _rug_ed_tests_llm_16_1165_llm_16_1165_rrrruuuugggg_test_as_primitive_isize_to_usize_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1166_llm_16_1166 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_f32() {
        let _rug_st_tests_llm_16_1166_llm_16_1166_rrrruuuugggg_test_from_f32 = 0;
        let rug_fuzz_0 = 42.0;
        let rug_fuzz_1 = 42.0;
        let rug_fuzz_2 = 42.0;
        let rug_fuzz_3 = 42.5;
        let rug_fuzz_4 = 0.0;
        let f: f32 = rug_fuzz_0;
        debug_assert_eq!(< isize as FromPrimitive > ::from_f32(f), Some(42));
        let f_nan: f32 = f32::NAN;
        debug_assert_eq!(< isize as FromPrimitive > ::from_f32(f_nan), None);
        let f_infinity: f32 = f32::INFINITY;
        debug_assert_eq!(< isize as FromPrimitive > ::from_f32(f_infinity), None);
        let f_neg_infinity: f32 = f32::NEG_INFINITY;
        debug_assert_eq!(< isize as FromPrimitive > ::from_f32(f_neg_infinity), None);
        let f_large: f32 = std::isize::MAX as f32;
        debug_assert_eq!(
            < isize as FromPrimitive > ::from_f32(f_large), Some(std::isize::MAX)
        );
        let f_small: f32 = std::isize::MIN as f32;
        debug_assert_eq!(
            < isize as FromPrimitive > ::from_f32(f_small), Some(std::isize::MIN)
        );
        let f_too_large: f32 = f32::MAX;
        debug_assert_eq!(< isize as FromPrimitive > ::from_f32(f_too_large), None);
        let f_too_small: f32 = f32::MIN;
        debug_assert_eq!(< isize as FromPrimitive > ::from_f32(f_too_small), None);
        let f_negative: f32 = -rug_fuzz_1;
        debug_assert_eq!(< isize as FromPrimitive > ::from_f32(f_negative), Some(- 42));
        let f_positive: f32 = rug_fuzz_2;
        debug_assert_eq!(< isize as FromPrimitive > ::from_f32(f_positive), Some(42));
        let f_fraction: f32 = rug_fuzz_3;
        debug_assert_eq!(< isize as FromPrimitive > ::from_f32(f_fraction), None);
        let f_zero: f32 = rug_fuzz_4;
        debug_assert_eq!(< isize as FromPrimitive > ::from_f32(f_zero), Some(0));
        let _rug_ed_tests_llm_16_1166_llm_16_1166_rrrruuuugggg_test_from_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1167_llm_16_1167 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_f64() {
        let _rug_st_tests_llm_16_1167_llm_16_1167_rrrruuuugggg_test_from_f64 = 0;
        let rug_fuzz_0 = 42.0;
        let rug_fuzz_1 = 42.7;
        let rug_fuzz_2 = 42.7;
        debug_assert_eq!(< isize as FromPrimitive > ::from_f64(rug_fuzz_0), Some(42));
        debug_assert_eq!(< isize as FromPrimitive > ::from_f64(rug_fuzz_1), Some(42));
        debug_assert_eq!(
            < isize as FromPrimitive > ::from_f64(- rug_fuzz_2), Some(- 42)
        );
        debug_assert_eq!(< isize as FromPrimitive > ::from_f64(f64::MAX), None);
        debug_assert_eq!(< isize as FromPrimitive > ::from_f64(f64::MIN), None);
        debug_assert_eq!(< isize as FromPrimitive > ::from_f64(f64::NAN), None);
        let _rug_ed_tests_llm_16_1167_llm_16_1167_rrrruuuugggg_test_from_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1168_llm_16_1168 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i128_within_bounds() {
        let _rug_st_tests_llm_16_1168_llm_16_1168_rrrruuuugggg_test_from_i128_within_bounds = 0;
        let min_isize = isize::MIN as i128;
        let max_isize = isize::MAX as i128;
        debug_assert_eq!(
            < isize as FromPrimitive > ::from_i128(min_isize), Some(isize::MIN)
        );
        debug_assert_eq!(
            < isize as FromPrimitive > ::from_i128(max_isize), Some(isize::MAX)
        );
        let _rug_ed_tests_llm_16_1168_llm_16_1168_rrrruuuugggg_test_from_i128_within_bounds = 0;
    }
    #[test]
    fn test_from_i128_out_of_bounds() {
        let _rug_st_tests_llm_16_1168_llm_16_1168_rrrruuuugggg_test_from_i128_out_of_bounds = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 1;
        let below_min_isize = isize::MIN as i128 - rug_fuzz_0;
        let above_max_isize = isize::MAX as i128 + rug_fuzz_1;
        debug_assert_eq!(< isize as FromPrimitive > ::from_i128(below_min_isize), None);
        debug_assert_eq!(< isize as FromPrimitive > ::from_i128(above_max_isize), None);
        let _rug_ed_tests_llm_16_1168_llm_16_1168_rrrruuuugggg_test_from_i128_out_of_bounds = 0;
    }
    #[test]
    fn test_from_i128_within_i64_bounds() {
        let _rug_st_tests_llm_16_1168_llm_16_1168_rrrruuuugggg_test_from_i128_within_i64_bounds = 0;
        let test_values = vec![i128::MIN, 0, i128::MAX];
        for &val in &test_values {
            let expected = val as isize;
            debug_assert_eq!(
                < isize as FromPrimitive > ::from_i128(val), Some(expected)
            );
        }
        let _rug_ed_tests_llm_16_1168_llm_16_1168_rrrruuuugggg_test_from_i128_within_i64_bounds = 0;
    }
    #[test]
    fn test_from_i128_exactly_isize() {
        let _rug_st_tests_llm_16_1168_llm_16_1168_rrrruuuugggg_test_from_i128_exactly_isize = 0;
        let test_values = vec![isize::MIN as i128, isize::MAX as i128];
        for &val in &test_values {
            debug_assert_eq!(
                < isize as FromPrimitive > ::from_i128(val), Some(val as isize)
            );
        }
        let _rug_ed_tests_llm_16_1168_llm_16_1168_rrrruuuugggg_test_from_i128_exactly_isize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1169_llm_16_1169 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i16() {
        let _rug_st_tests_llm_16_1169_llm_16_1169_rrrruuuugggg_test_from_i16 = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        debug_assert_eq!(
            < isize as FromPrimitive > ::from_i16(rug_fuzz_0), Some(0isize)
        );
        debug_assert_eq!(
            < isize as FromPrimitive > ::from_i16(- rug_fuzz_1), Some(- 1isize)
        );
        debug_assert_eq!(
            < isize as FromPrimitive > ::from_i16(i16::MAX), Some(i16::MAX as isize)
        );
        debug_assert_eq!(
            < isize as FromPrimitive > ::from_i16(i16::MIN), Some(i16::MIN as isize)
        );
        let _rug_ed_tests_llm_16_1169_llm_16_1169_rrrruuuugggg_test_from_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1170_llm_16_1170 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i32() {
        let _rug_st_tests_llm_16_1170_llm_16_1170_rrrruuuugggg_test_from_i32 = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        debug_assert_eq!(
            < isize as FromPrimitive > ::from_i32(rug_fuzz_0), Some(0isize)
        );
        debug_assert_eq!(
            < isize as FromPrimitive > ::from_i32(- rug_fuzz_1), Some(- 1isize)
        );
        debug_assert_eq!(
            < isize as FromPrimitive > ::from_i32(i32::MAX), Some(i32::MAX as isize)
        );
        #[cfg(target_pointer_width = "64")]
        debug_assert_eq!(
            < isize as FromPrimitive > ::from_i32(i32::MIN), Some(i32::MIN as isize)
        );
        #[cfg(not(target_pointer_width = "64"))]
        debug_assert_eq!(< isize as FromPrimitive > ::from_i32(i32::MIN), None);
        let _rug_ed_tests_llm_16_1170_llm_16_1170_rrrruuuugggg_test_from_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1172_llm_16_1172 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i8() {
        assert_eq!(< isize as FromPrimitive >::from_i8(0), Some(0isize));
        assert_eq!(< isize as FromPrimitive >::from_i8(127), Some(127isize));
        assert_eq!(< isize as FromPrimitive >::from_i8(- 128), Some(- 128isize));
    }
}
#[cfg(test)]
mod tests_llm_16_1173 {
    use crate::FromPrimitive;
    #[test]
    fn test_from_isize() {
        let _rug_st_tests_llm_16_1173_rrrruuuugggg_test_from_isize = 0;
        let rug_fuzz_0 = 42;
        type TestType = i32;
        let value: isize = rug_fuzz_0;
        let result = <TestType as FromPrimitive>::from_isize(value);
        debug_assert_eq!(result, Some(value as TestType));
        let too_large: isize = isize::MAX;
        let result = <TestType as FromPrimitive>::from_isize(too_large);
        if too_large as TestType as isize != too_large {
            debug_assert_eq!(result, None);
        }
        let too_small: isize = isize::MIN;
        let result = <TestType as FromPrimitive>::from_isize(too_small);
        if too_small as TestType as isize != too_small {
            debug_assert_eq!(result, None);
        }
        let _rug_ed_tests_llm_16_1173_rrrruuuugggg_test_from_isize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1174_llm_16_1174 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u128() {
        let _rug_st_tests_llm_16_1174_llm_16_1174_rrrruuuugggg_test_from_u128 = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 2;
        let rug_fuzz_4 = 1;
        let max_value = isize::MAX as u128;
        debug_assert_eq!(
            < isize as FromPrimitive > ::from_u128(max_value), Some(isize::MAX)
        );
        debug_assert_eq!(
            < isize as FromPrimitive > ::from_u128(max_value + rug_fuzz_0), None
        );
        debug_assert_eq!(< isize as FromPrimitive > ::from_u128(rug_fuzz_1), Some(0));
        debug_assert_eq!(< isize as FromPrimitive > ::from_u128(rug_fuzz_2), Some(1));
        if max_value as u128 > rug_fuzz_3 {
            debug_assert_eq!(
                < isize as FromPrimitive > ::from_u128(max_value - rug_fuzz_4),
                Some((max_value - 1) as isize)
            );
        }
        let _rug_ed_tests_llm_16_1174_llm_16_1174_rrrruuuugggg_test_from_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1175_llm_16_1175 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u16() {
        let _rug_st_tests_llm_16_1175_llm_16_1175_rrrruuuugggg_test_from_u16 = 0;
        let rug_fuzz_0 = 0_u16;
        debug_assert_eq!(
            < isize as FromPrimitive > ::from_u16(rug_fuzz_0), Some(0_isize)
        );
        debug_assert_eq!(
            < isize as FromPrimitive > ::from_u16(u16::MAX), Some(u16::MAX as isize)
        );
        let _rug_ed_tests_llm_16_1175_llm_16_1175_rrrruuuugggg_test_from_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1177_llm_16_1177 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u64() {
        assert_eq!(< isize as FromPrimitive >::from_u64(0_u64), Some(0_isize));
        let max_value = if cfg!(target_pointer_width = "64") {
            None
        } else {
            Some((u64::MAX as isize).wrapping_abs())
        };
        assert_eq!(< isize as FromPrimitive >::from_u64(u64::MAX), max_value);
        assert_eq!(
            < isize as FromPrimitive >::from_u64(isize::MAX as u64), Some(isize::MAX)
        );
    }
}
#[cfg(test)]
mod tests_llm_16_1178_llm_16_1178 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u8() {
        let _rug_st_tests_llm_16_1178_llm_16_1178_rrrruuuugggg_test_from_u8 = 0;
        let rug_fuzz_0 = 42;
        let number_u8: u8 = rug_fuzz_0;
        let number_isize: Option<isize> = <isize as FromPrimitive>::from_u8(number_u8);
        debug_assert_eq!(number_isize, Some(42 as isize));
        let max_u8: u8 = u8::MAX;
        let max_isize: Option<isize> = <isize as FromPrimitive>::from_u8(max_u8);
        if max_u8 as u64 <= isize::MAX as u64 {
            debug_assert_eq!(max_isize, Some(max_u8 as isize));
        } else {
            debug_assert_eq!(max_isize, None);
        }
        let _rug_ed_tests_llm_16_1178_llm_16_1178_rrrruuuugggg_test_from_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1179_llm_16_1179 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_usize() {
        let _rug_st_tests_llm_16_1179_llm_16_1179_rrrruuuugggg_test_from_usize = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 64;
        let rug_fuzz_2 = 2;
        debug_assert_eq!(
            < isize as FromPrimitive > ::from_usize(rug_fuzz_0), Some(0isize)
        );
        debug_assert_eq!(< isize as FromPrimitive > ::from_usize(usize::MAX), None);
        if isize::BITS == rug_fuzz_1 {
            debug_assert_eq!(
                < isize as FromPrimitive > ::from_usize(usize::MAX / rug_fuzz_2),
                Some((usize::MAX / 2) as isize)
            );
        } else {
            debug_assert_eq!(< isize as FromPrimitive > ::from_usize(usize::MAX), None);
        }
        let _rug_ed_tests_llm_16_1179_llm_16_1179_rrrruuuugggg_test_from_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1181 {
    use super::*;
    use crate::*;
    #[test]
    fn test_isize_to_f32() {
        let _rug_st_tests_llm_16_1181_rrrruuuugggg_test_isize_to_f32 = 0;
        let rug_fuzz_0 = 0isize;
        let rug_fuzz_1 = 1isize;
        let rug_fuzz_2 = 1isize;
        debug_assert_eq!((rug_fuzz_0).to_f32(), Some(0.0f32));
        debug_assert_eq!((rug_fuzz_1).to_f32(), Some(1.0f32));
        debug_assert_eq!((- rug_fuzz_2).to_f32(), Some(- 1.0f32));
        debug_assert_eq!((isize::MAX).to_f32(), Some(isize::MAX as f32));
        debug_assert_eq!((isize::MIN).to_f32(), Some(isize::MIN as f32));
        let _rug_ed_tests_llm_16_1181_rrrruuuugggg_test_isize_to_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1182 {
    use super::*;
    use crate::*;
    #[test]
    fn test_isize_to_f64() {
        let _rug_st_tests_llm_16_1182_rrrruuuugggg_test_isize_to_f64 = 0;
        let rug_fuzz_0 = 0;
        debug_assert_eq!(
            < isize as cast::ToPrimitive > ::to_f64(& rug_fuzz_0), Some(0.0f64)
        );
        debug_assert_eq!(
            < isize as cast::ToPrimitive > ::to_f64(& isize::MAX), Some(isize::MAX as
            f64)
        );
        debug_assert_eq!(
            < isize as cast::ToPrimitive > ::to_f64(& isize::MIN), Some(isize::MIN as
            f64)
        );
        let _rug_ed_tests_llm_16_1182_rrrruuuugggg_test_isize_to_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1183_llm_16_1183 {
    use crate::ToPrimitive;
    #[test]
    fn to_i128_with_isize_within_bounds() {
        let _rug_st_tests_llm_16_1183_llm_16_1183_rrrruuuugggg_to_i128_with_isize_within_bounds = 0;
        let value: isize = isize::MAX;
        debug_assert_eq!(value.to_i128(), Some(isize::MAX as i128));
        let _rug_ed_tests_llm_16_1183_llm_16_1183_rrrruuuugggg_to_i128_with_isize_within_bounds = 0;
    }
    #[test]
    fn to_i128_with_isize_at_lower_bound() {
        let _rug_st_tests_llm_16_1183_llm_16_1183_rrrruuuugggg_to_i128_with_isize_at_lower_bound = 0;
        let value: isize = isize::MIN;
        debug_assert_eq!(value.to_i128(), Some(isize::MIN as i128));
        let _rug_ed_tests_llm_16_1183_llm_16_1183_rrrruuuugggg_to_i128_with_isize_at_lower_bound = 0;
    }
    #[test]
    fn to_i128_with_isize_at_upper_bound() {
        let _rug_st_tests_llm_16_1183_llm_16_1183_rrrruuuugggg_to_i128_with_isize_at_upper_bound = 0;
        let value: isize = isize::MAX;
        debug_assert_eq!(value.to_i128(), Some(isize::MAX as i128));
        let _rug_ed_tests_llm_16_1183_llm_16_1183_rrrruuuugggg_to_i128_with_isize_at_upper_bound = 0;
    }
    #[test]
    fn to_i128_with_isize_overflow() {
        let _rug_st_tests_llm_16_1183_llm_16_1183_rrrruuuugggg_to_i128_with_isize_overflow = 0;
        let value: isize = isize::MIN;
        let _rug_ed_tests_llm_16_1183_llm_16_1183_rrrruuuugggg_to_i128_with_isize_overflow = 0;
    }
    #[test]
    fn to_i128_with_isize_underflow() {
        let _rug_st_tests_llm_16_1183_llm_16_1183_rrrruuuugggg_to_i128_with_isize_underflow = 0;
        let value: isize = isize::MAX;
        let _rug_ed_tests_llm_16_1183_llm_16_1183_rrrruuuugggg_to_i128_with_isize_underflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1184_llm_16_1184 {
    use super::*;
    use crate::*;
    #[test]
    fn test_to_i16_with_isize() {
        let _rug_st_tests_llm_16_1184_llm_16_1184_rrrruuuugggg_test_to_i16_with_isize = 0;
        let rug_fuzz_0 = 0isize;
        let rug_fuzz_1 = 1isize;
        let rug_fuzz_2 = 1isize;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 1;
        debug_assert_eq!(rug_fuzz_0.to_i16(), Some(0i16));
        debug_assert_eq!(rug_fuzz_1.to_i16(), Some(1i16));
        debug_assert_eq!((- rug_fuzz_2).to_i16(), Some(- 1i16));
        debug_assert_eq!((i16::MAX as isize).to_i16(), Some(i16::MAX));
        debug_assert_eq!(((i16::MAX as isize) + rug_fuzz_3).to_i16(), None);
        debug_assert_eq!(((i16::MIN as isize) - rug_fuzz_4).to_i16(), None);
        let _rug_ed_tests_llm_16_1184_llm_16_1184_rrrruuuugggg_test_to_i16_with_isize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1185_llm_16_1185 {
    use crate::ToPrimitive;
    #[test]
    fn to_i32_with_isize() {
        let _rug_st_tests_llm_16_1185_llm_16_1185_rrrruuuugggg_to_i32_with_isize = 0;
        let rug_fuzz_0 = 0isize;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let min_i32_isize = isize::MIN.max(i32::MIN as isize);
        let max_i32_isize = isize::MAX.min(i32::MAX as isize);
        debug_assert_eq!(min_i32_isize.to_i32(), Some(i32::MIN));
        debug_assert_eq!(rug_fuzz_0.to_i32(), Some(0i32));
        debug_assert_eq!(max_i32_isize.to_i32(), Some(i32::MAX));
        if min_i32_isize > isize::MIN {
            debug_assert_eq!((min_i32_isize - rug_fuzz_1).to_i32(), None);
        }
        if max_i32_isize < isize::MAX {
            debug_assert_eq!((max_i32_isize + rug_fuzz_2).to_i32(), None);
        }
        let _rug_ed_tests_llm_16_1185_llm_16_1185_rrrruuuugggg_to_i32_with_isize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1186_llm_16_1186 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_i64() {
        let _rug_st_tests_llm_16_1186_llm_16_1186_rrrruuuugggg_test_to_i64 = 0;
        let rug_fuzz_0 = 0isize;
        let rug_fuzz_1 = 1isize;
        debug_assert_eq!((rug_fuzz_0).to_i64(), Some(0i64));
        debug_assert_eq!((- rug_fuzz_1).to_i64(), Some(- 1i64));
        debug_assert_eq!((isize::MAX).to_i64(), Some(i64::MAX));
        if std::mem::size_of::<isize>() < std::mem::size_of::<i64>() {
            debug_assert_eq!((isize::MIN).to_i64(), Some(i64::MIN));
        }
        let _rug_ed_tests_llm_16_1186_llm_16_1186_rrrruuuugggg_test_to_i64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1187_llm_16_1187 {
    use crate::cast::ToPrimitive;
    #[test]
    fn to_i8_in_range() {
        let _rug_st_tests_llm_16_1187_llm_16_1187_rrrruuuugggg_to_i8_in_range = 0;
        let rug_fuzz_0 = 0isize;
        let rug_fuzz_1 = 127isize;
        let rug_fuzz_2 = 128isize;
        debug_assert_eq!((rug_fuzz_0).to_i8(), Some(0i8));
        debug_assert_eq!((rug_fuzz_1).to_i8(), Some(127i8));
        debug_assert_eq!((- rug_fuzz_2).to_i8(), Some(- 128i8));
        let _rug_ed_tests_llm_16_1187_llm_16_1187_rrrruuuugggg_to_i8_in_range = 0;
    }
    #[test]
    fn to_i8_out_of_range() {
        let _rug_st_tests_llm_16_1187_llm_16_1187_rrrruuuugggg_to_i8_out_of_range = 0;
        let rug_fuzz_0 = 128isize;
        let rug_fuzz_1 = 129isize;
        debug_assert_eq!((rug_fuzz_0).to_i8(), None);
        debug_assert_eq!((- rug_fuzz_1).to_i8(), None);
        let _rug_ed_tests_llm_16_1187_llm_16_1187_rrrruuuugggg_to_i8_out_of_range = 0;
    }
    #[test]
    fn to_i8_edge_cases() {
        let _rug_st_tests_llm_16_1187_llm_16_1187_rrrruuuugggg_to_i8_edge_cases = 0;
        debug_assert_eq!(isize::MAX.to_i8(), None);
        debug_assert_eq!(isize::MIN.to_i8(), None);
        let _rug_ed_tests_llm_16_1187_llm_16_1187_rrrruuuugggg_to_i8_edge_cases = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1189 {
    use super::*;
    use crate::*;
    use std::mem::size_of;
    #[test]
    fn to_u128_positive_isize() {
        let _rug_st_tests_llm_16_1189_rrrruuuugggg_to_u128_positive_isize = 0;
        let rug_fuzz_0 = 42;
        let value: isize = rug_fuzz_0;
        debug_assert_eq!(< isize as ToPrimitive > ::to_u128(& value), Some(42_u128));
        let _rug_ed_tests_llm_16_1189_rrrruuuugggg_to_u128_positive_isize = 0;
    }
    #[test]
    fn to_u128_max_isize() {
        let _rug_st_tests_llm_16_1189_rrrruuuugggg_to_u128_max_isize = 0;
        let value: isize = isize::MAX;
        debug_assert_eq!(
            < isize as ToPrimitive > ::to_u128(& value), Some(isize::MAX as u128)
        );
        let _rug_ed_tests_llm_16_1189_rrrruuuugggg_to_u128_max_isize = 0;
    }
    #[test]
    fn to_u128_min_isize() {
        let _rug_st_tests_llm_16_1189_rrrruuuugggg_to_u128_min_isize = 0;
        let value: isize = isize::MIN;
        debug_assert_eq!(< isize as ToPrimitive > ::to_u128(& value), None);
        let _rug_ed_tests_llm_16_1189_rrrruuuugggg_to_u128_min_isize = 0;
    }
    #[test]
    fn to_u128_zero_isize() {
        let _rug_st_tests_llm_16_1189_rrrruuuugggg_to_u128_zero_isize = 0;
        let rug_fuzz_0 = 0;
        let value: isize = rug_fuzz_0;
        debug_assert_eq!(< isize as ToPrimitive > ::to_u128(& value), Some(0_u128));
        let _rug_ed_tests_llm_16_1189_rrrruuuugggg_to_u128_zero_isize = 0;
    }
    #[test]
    fn to_u128_boundary_isize() {
        let _rug_st_tests_llm_16_1189_rrrruuuugggg_to_u128_boundary_isize = 0;
        let max_u128_as_isize = u128::MAX as isize;
        if size_of::<isize>() < size_of::<u128>() {
            debug_assert_eq!(
                < isize as ToPrimitive > ::to_u128(& max_u128_as_isize), None
            );
        } else {
            debug_assert_eq!(
                < isize as ToPrimitive > ::to_u128(& max_u128_as_isize), Some(u128::MAX)
            );
        }
        let _rug_ed_tests_llm_16_1189_rrrruuuugggg_to_u128_boundary_isize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1190_llm_16_1190 {
    use crate::ToPrimitive;
    #[test]
    fn test_isize_to_u16() {
        let _rug_st_tests_llm_16_1190_llm_16_1190_rrrruuuugggg_test_isize_to_u16 = 0;
        let rug_fuzz_0 = 0isize;
        let rug_fuzz_1 = 1isize;
        let rug_fuzz_2 = 1isize;
        let rug_fuzz_3 = 1;
        debug_assert_eq!((rug_fuzz_0).to_u16(), Some(0u16));
        debug_assert_eq!((rug_fuzz_1).to_u16(), Some(1u16));
        debug_assert_eq!((- rug_fuzz_2).to_u16(), None);
        let max_value: isize = u16::MAX as isize;
        debug_assert_eq!((max_value).to_u16(), Some(u16::MAX));
        debug_assert_eq!((max_value + rug_fuzz_3).to_u16(), None);
        debug_assert_eq!((isize::MIN).to_u16(), None);
        let _rug_ed_tests_llm_16_1190_llm_16_1190_rrrruuuugggg_test_isize_to_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1191_llm_16_1191 {
    use crate::cast::ToPrimitive;
    use std::mem::size_of;
    #[test]
    fn to_u32_with_positive_isize() {
        let _rug_st_tests_llm_16_1191_llm_16_1191_rrrruuuugggg_to_u32_with_positive_isize = 0;
        let rug_fuzz_0 = 123_isize;
        let val = rug_fuzz_0;
        debug_assert_eq!(val.to_u32(), Some(123_u32));
        let _rug_ed_tests_llm_16_1191_llm_16_1191_rrrruuuugggg_to_u32_with_positive_isize = 0;
    }
    #[test]
    fn to_u32_with_negative_isize() {
        let _rug_st_tests_llm_16_1191_llm_16_1191_rrrruuuugggg_to_u32_with_negative_isize = 0;
        let rug_fuzz_0 = 123_isize;
        let val = -rug_fuzz_0;
        debug_assert_eq!(val.to_u32(), None);
        let _rug_ed_tests_llm_16_1191_llm_16_1191_rrrruuuugggg_to_u32_with_negative_isize = 0;
    }
    #[test]
    fn to_u32_with_isize_max() {
        let _rug_st_tests_llm_16_1191_llm_16_1191_rrrruuuugggg_to_u32_with_isize_max = 0;
        let val = isize::MAX;
        debug_assert_eq!(val.to_u32(), Some(isize::MAX as u32));
        let _rug_ed_tests_llm_16_1191_llm_16_1191_rrrruuuugggg_to_u32_with_isize_max = 0;
    }
    #[test]
    fn to_u32_with_isize_min() {
        let _rug_st_tests_llm_16_1191_llm_16_1191_rrrruuuugggg_to_u32_with_isize_min = 0;
        let val = isize::MIN;
        debug_assert_eq!(val.to_u32(), None);
        let _rug_ed_tests_llm_16_1191_llm_16_1191_rrrruuuugggg_to_u32_with_isize_min = 0;
    }
    #[test]
    fn to_u32_with_u32_max_isize() {
        let _rug_st_tests_llm_16_1191_llm_16_1191_rrrruuuugggg_to_u32_with_u32_max_isize = 0;
        let val = u32::MAX as isize;
        if size_of::<isize>() >= size_of::<u32>() {
            debug_assert_eq!(val.to_u32(), Some(u32::MAX));
        } else {
            debug_assert_eq!(val.to_u32(), None);
        }
        let _rug_ed_tests_llm_16_1191_llm_16_1191_rrrruuuugggg_to_u32_with_u32_max_isize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1192_llm_16_1192 {
    use crate::ToPrimitive;
    #[test]
    fn to_u64_on_isize() {
        let _rug_st_tests_llm_16_1192_llm_16_1192_rrrruuuugggg_to_u64_on_isize = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 1;
        let a: isize = rug_fuzz_0;
        debug_assert_eq!(a.to_u64(), Some(42u64));
        let b: isize = -rug_fuzz_1;
        debug_assert_eq!(b.to_u64(), None);
        let max: isize = isize::MAX;
        debug_assert_eq!(max.to_u64(), Some(isize::MAX as u64));
        let min: isize = isize::MIN;
        debug_assert_eq!(min.to_u64(), None);
        if std::mem::size_of::<isize>() < std::mem::size_of::<u64>() {
            let c: isize = isize::MAX;
            debug_assert_eq!(c.to_u64(), Some(isize::MAX as u64));
        }
        let _rug_ed_tests_llm_16_1192_llm_16_1192_rrrruuuugggg_to_u64_on_isize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1193 {
    use super::*;
    use crate::*;
    #[test]
    fn test_to_u8_with_positive_isize() {
        let _rug_st_tests_llm_16_1193_rrrruuuugggg_test_to_u8_with_positive_isize = 0;
        let rug_fuzz_0 = 42;
        let value: isize = rug_fuzz_0;
        let result = value.to_u8();
        debug_assert_eq!(result, Some(42u8));
        let _rug_ed_tests_llm_16_1193_rrrruuuugggg_test_to_u8_with_positive_isize = 0;
    }
    #[test]
    fn test_to_u8_with_negative_isize() {
        let _rug_st_tests_llm_16_1193_rrrruuuugggg_test_to_u8_with_negative_isize = 0;
        let rug_fuzz_0 = 1;
        let value: isize = -rug_fuzz_0;
        let result = value.to_u8();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1193_rrrruuuugggg_test_to_u8_with_negative_isize = 0;
    }
    #[test]
    fn test_to_u8_with_large_isize() {
        let _rug_st_tests_llm_16_1193_rrrruuuugggg_test_to_u8_with_large_isize = 0;
        let value: isize = isize::MAX;
        let result = value.to_u8();
        if size_of::<isize>() > size_of::<u8>() {
            debug_assert_eq!(result, None);
        } else {
            debug_assert_eq!(result, Some(isize::MAX as u8));
        }
        let _rug_ed_tests_llm_16_1193_rrrruuuugggg_test_to_u8_with_large_isize = 0;
    }
    #[test]
    fn test_to_u8_with_large_negative_isize() {
        let _rug_st_tests_llm_16_1193_rrrruuuugggg_test_to_u8_with_large_negative_isize = 0;
        let value: isize = isize::MIN;
        let result = value.to_u8();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1193_rrrruuuugggg_test_to_u8_with_large_negative_isize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1194_llm_16_1194 {
    use crate::cast::ToPrimitive;
    #[test]
    fn to_usize_positive_isize() {
        let _rug_st_tests_llm_16_1194_llm_16_1194_rrrruuuugggg_to_usize_positive_isize = 0;
        let rug_fuzz_0 = 42;
        let num: isize = rug_fuzz_0;
        debug_assert_eq!(num.to_usize(), Some(42_usize));
        let _rug_ed_tests_llm_16_1194_llm_16_1194_rrrruuuugggg_to_usize_positive_isize = 0;
    }
    #[test]
    fn to_usize_negative_isize() {
        let _rug_st_tests_llm_16_1194_llm_16_1194_rrrruuuugggg_to_usize_negative_isize = 0;
        let rug_fuzz_0 = 42;
        let num: isize = -rug_fuzz_0;
        debug_assert_eq!(num.to_usize(), None);
        let _rug_ed_tests_llm_16_1194_llm_16_1194_rrrruuuugggg_to_usize_negative_isize = 0;
    }
    #[test]
    fn to_usize_isize_max() {
        let _rug_st_tests_llm_16_1194_llm_16_1194_rrrruuuugggg_to_usize_isize_max = 0;
        let num = isize::MAX;
        debug_assert_eq!(num.to_usize(), Some(isize::MAX as usize));
        let _rug_ed_tests_llm_16_1194_llm_16_1194_rrrruuuugggg_to_usize_isize_max = 0;
    }
    #[test]
    fn to_usize_isize_min() {
        let _rug_st_tests_llm_16_1194_llm_16_1194_rrrruuuugggg_to_usize_isize_min = 0;
        let num = isize::MIN;
        debug_assert_eq!(num.to_usize(), None);
        let _rug_ed_tests_llm_16_1194_llm_16_1194_rrrruuuugggg_to_usize_isize_min = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1262_llm_16_1262 {
    use super::*;
    use crate::*;
    use std::num::Wrapping;
    #[test]
    fn test_from_f32_to_wrapping() {
        let _rug_st_tests_llm_16_1262_llm_16_1262_rrrruuuugggg_test_from_f32_to_wrapping = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 1.0;
        let rug_fuzz_2 = 1.5;
        let rug_fuzz_3 = 1.0;
        let rug_fuzz_4 = 0.0;
        let rug_fuzz_5 = 1.0;
        debug_assert_eq!(
            < Wrapping < u32 > as FromPrimitive > ::from_f32(rug_fuzz_0),
            Some(Wrapping(0u32))
        );
        debug_assert_eq!(
            < Wrapping < u32 > as FromPrimitive > ::from_f32(rug_fuzz_1),
            Some(Wrapping(1u32))
        );
        debug_assert_eq!(
            < Wrapping < u32 > as FromPrimitive > ::from_f32(rug_fuzz_2),
            Some(Wrapping(1u32))
        );
        debug_assert_eq!(
            < Wrapping < u32 > as FromPrimitive > ::from_f32(- rug_fuzz_3), None
        );
        debug_assert_eq!(
            < Wrapping < i32 > as FromPrimitive > ::from_f32(rug_fuzz_4),
            Some(Wrapping(0i32))
        );
        debug_assert_eq!(
            < Wrapping < i32 > as FromPrimitive > ::from_f32(- rug_fuzz_5),
            Some(Wrapping(- 1i32))
        );
        debug_assert_eq!(
            < Wrapping < i32 > as FromPrimitive > ::from_f32(std::f32::MAX),
            Some(Wrapping(i32::MAX))
        );
        debug_assert_eq!(
            < Wrapping < i32 > as FromPrimitive > ::from_f32(std::f32::MIN),
            Some(Wrapping(i32::MIN))
        );
        debug_assert_eq!(
            < Wrapping < i32 > as FromPrimitive > ::from_f32(std::f32::INFINITY), None
        );
        debug_assert_eq!(
            < Wrapping < i32 > as FromPrimitive > ::from_f32(std::f32::NEG_INFINITY),
            None
        );
        debug_assert_eq!(
            < Wrapping < i32 > as FromPrimitive > ::from_f32(std::f32::NAN), None
        );
        let _rug_ed_tests_llm_16_1262_llm_16_1262_rrrruuuugggg_test_from_f32_to_wrapping = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1263_llm_16_1263 {
    use crate::cast::FromPrimitive;
    use std::num::Wrapping;
    #[test]
    fn test_from_f64() {
        let _rug_st_tests_llm_16_1263_llm_16_1263_rrrruuuugggg_test_from_f64 = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 1.0;
        let rug_fuzz_2 = 0.0;
        let test_values: Vec<f64> = vec![
            rug_fuzz_0, 1.234, - 1.234, 2.0_f64.powi(30), - 2.0_f64.powi(30)
        ];
        for &val in &test_values {
            let wrapped_val = <Wrapping<i32> as FromPrimitive>::from_f64(val);
            if let Some(wrapped_int) = wrapped_val {
                let expected = Wrapping(val as i32);
                debug_assert_eq!(
                    wrapped_int, expected,
                    "from_f64({}) did not return the expected value: {:?}", val, expected
                );
            } else {
                debug_assert!(
                    val < (i32::MIN as f64) || val >= (i32::MAX as f64) + rug_fuzz_1 ||
                    val.fract() != rug_fuzz_2,
                    "from_f64({}) should not convert but got Some value", val
                );
            }
        }
        let _rug_ed_tests_llm_16_1263_llm_16_1263_rrrruuuugggg_test_from_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1264_llm_16_1264 {
    use crate::cast::FromPrimitive;
    use std::num::Wrapping;
    #[test]
    fn test_from_i128() {
        let _rug_st_tests_llm_16_1264_llm_16_1264_rrrruuuugggg_test_from_i128 = 0;
        let rug_fuzz_0 = 123_i128;
        let rug_fuzz_1 = 123_i128;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 1;
        let rug_fuzz_4 = 123_i128;
        let rug_fuzz_5 = 123_i128;
        let wrapped_i32_positive = <Wrapping<
            i32,
        > as FromPrimitive>::from_i128(rug_fuzz_0);
        debug_assert_eq!(wrapped_i32_positive, Some(Wrapping(123_i32)));
        let wrapped_i32_negative = <Wrapping<
            i32,
        > as FromPrimitive>::from_i128(-rug_fuzz_1);
        debug_assert_eq!(wrapped_i32_negative, Some(Wrapping(- 123_i32)));
        let wrapped_i32_overflow = <Wrapping<
            i32,
        > as FromPrimitive>::from_i128(i128::from(i32::MAX) + rug_fuzz_2);
        debug_assert_eq!(wrapped_i32_overflow, None);
        let wrapped_i32_underflow = <Wrapping<
            i32,
        > as FromPrimitive>::from_i128(i128::from(i32::MIN) - rug_fuzz_3);
        debug_assert_eq!(wrapped_i32_underflow, None);
        let wrapped_u32_positive = <Wrapping<
            u32,
        > as FromPrimitive>::from_i128(rug_fuzz_4);
        debug_assert_eq!(wrapped_u32_positive, Some(Wrapping(123_u32)));
        let wrapped_u32_negative = <Wrapping<
            u32,
        > as FromPrimitive>::from_i128(-rug_fuzz_5);
        debug_assert_eq!(wrapped_u32_negative, None);
        let _rug_ed_tests_llm_16_1264_llm_16_1264_rrrruuuugggg_test_from_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1265_llm_16_1265 {
    use crate::cast::FromPrimitive;
    use std::num::Wrapping;
    #[test]
    fn test_from_i16_for_wrapping() {
        let _rug_st_tests_llm_16_1265_llm_16_1265_rrrruuuugggg_test_from_i16_for_wrapping = 0;
        let rug_fuzz_0 = 123;
        let rug_fuzz_1 = 123;
        let num_i16: i16 = rug_fuzz_0;
        let wrapped_num: Option<Wrapping<i16>> = FromPrimitive::from_i16(num_i16);
        debug_assert_eq!(wrapped_num, Some(Wrapping(123)));
        let num_i16: i16 = -rug_fuzz_1;
        let wrapped_num: Option<Wrapping<i16>> = FromPrimitive::from_i16(num_i16);
        debug_assert_eq!(wrapped_num, Some(Wrapping(- 123)));
        let num_i16: i16 = i16::MAX;
        let wrapped_num: Option<Wrapping<i16>> = FromPrimitive::from_i16(num_i16);
        debug_assert_eq!(wrapped_num, Some(Wrapping(i16::MAX)));
        let num_i16: i16 = i16::MIN;
        let wrapped_num: Option<Wrapping<i16>> = FromPrimitive::from_i16(num_i16);
        debug_assert_eq!(wrapped_num, Some(Wrapping(i16::MIN)));
        let _rug_ed_tests_llm_16_1265_llm_16_1265_rrrruuuugggg_test_from_i16_for_wrapping = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1266_llm_16_1266 {
    use crate::cast::FromPrimitive;
    use std::num::Wrapping;
    #[test]
    fn test_from_i32() {
        let _rug_st_tests_llm_16_1266_llm_16_1266_rrrruuuugggg_test_from_i32 = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 1;
        let num: i32 = rug_fuzz_0;
        let wrapped_num: Option<Wrapping<i32>> = FromPrimitive::from_i32(num);
        debug_assert_eq!(wrapped_num, Some(Wrapping(42)));
        let big_num: i32 = i32::MAX;
        let wrapped_big_num: Option<Wrapping<i32>> = FromPrimitive::from_i32(big_num);
        debug_assert_eq!(wrapped_big_num, Some(Wrapping(i32::MAX)));
        let negative_num: i32 = -rug_fuzz_1;
        let wrapped_negative_num: Option<Wrapping<i32>> = FromPrimitive::from_i32(
            negative_num,
        );
        debug_assert_eq!(wrapped_negative_num, Some(Wrapping(- 1)));
        let _rug_ed_tests_llm_16_1266_llm_16_1266_rrrruuuugggg_test_from_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1267_llm_16_1267 {
    use crate::cast::FromPrimitive;
    use std::num::Wrapping;
    #[test]
    fn test_from_i64() {
        let _rug_st_tests_llm_16_1267_llm_16_1267_rrrruuuugggg_test_from_i64 = 0;
        let rug_fuzz_0 = 123456789;
        let num: i64 = rug_fuzz_0;
        let wrapped_num: Option<Wrapping<i64>> = FromPrimitive::from_i64(num);
        debug_assert_eq!(wrapped_num, Some(Wrapping(num)));
        let max_num = i64::MAX;
        let wrapped_max: Option<Wrapping<i64>> = FromPrimitive::from_i64(max_num);
        debug_assert_eq!(wrapped_max, Some(Wrapping(max_num)));
        let min_num = i64::MIN;
        let wrapped_min: Option<Wrapping<i64>> = FromPrimitive::from_i64(min_num);
        debug_assert_eq!(wrapped_min, Some(Wrapping(min_num)));
        let wrapped_num_small: Option<Wrapping<i8>> = FromPrimitive::from_i64(num);
        debug_assert_eq!(wrapped_num_small, None);
        let _rug_ed_tests_llm_16_1267_llm_16_1267_rrrruuuugggg_test_from_i64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1269_llm_16_1269 {
    use crate::cast::FromPrimitive;
    use std::num::Wrapping;
    #[test]
    fn test_from_isize() {
        let _rug_st_tests_llm_16_1269_llm_16_1269_rrrruuuugggg_test_from_isize = 0;
        let rug_fuzz_0 = 42_isize;
        let rug_fuzz_1 = 1_isize;
        let rug_fuzz_2 = 1_isize;
        debug_assert_eq!(
            < Wrapping < u32 > as FromPrimitive > ::from_isize(rug_fuzz_0),
            Some(Wrapping(42_u32))
        );
        debug_assert_eq!(
            < Wrapping < u32 > as FromPrimitive > ::from_isize(- rug_fuzz_1), None
        );
        debug_assert_eq!(
            < Wrapping < i32 > as FromPrimitive > ::from_isize(- rug_fuzz_2),
            Some(Wrapping(- 1_i32))
        );
        let _rug_ed_tests_llm_16_1269_llm_16_1269_rrrruuuugggg_test_from_isize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1270_llm_16_1270 {
    use crate::cast::FromPrimitive;
    use std::num::Wrapping;
    #[test]
    fn test_from_u128() {
        let _rug_st_tests_llm_16_1270_llm_16_1270_rrrruuuugggg_test_from_u128 = 0;
        let rug_fuzz_0 = 255_u128;
        let rug_fuzz_1 = 256_u128;
        let rug_fuzz_2 = 65_535_u128;
        let rug_fuzz_3 = 65_536_u128;
        let rug_fuzz_4 = 4_294_967_295_u128;
        let rug_fuzz_5 = 4_294_967_296_u128;
        let rug_fuzz_6 = 18_446_744_073_709_551_615_u128;
        let rug_fuzz_7 = 18_446_744_073_709_551_616_u128;
        let rug_fuzz_8 = 127_u128;
        let rug_fuzz_9 = 128_u128;
        let rug_fuzz_10 = 32_767_u128;
        let rug_fuzz_11 = 32_768_u128;
        let rug_fuzz_12 = 2_147_483_647_u128;
        let rug_fuzz_13 = 2_147_483_648_u128;
        let rug_fuzz_14 = 9_223_372_036_854_775_807_u128;
        let rug_fuzz_15 = 9_223_372_036_854_775_808_u128;
        debug_assert_eq!(
            < Wrapping < u8 > as FromPrimitive > ::from_u128(rug_fuzz_0),
            Some(Wrapping(255_u8))
        );
        debug_assert_eq!(
            < Wrapping < u8 > as FromPrimitive > ::from_u128(rug_fuzz_1), None
        );
        debug_assert_eq!(
            < Wrapping < u16 > as FromPrimitive > ::from_u128(rug_fuzz_2),
            Some(Wrapping(65_535_u16))
        );
        debug_assert_eq!(
            < Wrapping < u16 > as FromPrimitive > ::from_u128(rug_fuzz_3), None
        );
        debug_assert_eq!(
            < Wrapping < u32 > as FromPrimitive > ::from_u128(rug_fuzz_4),
            Some(Wrapping(4_294_967_295_u32))
        );
        debug_assert_eq!(
            < Wrapping < u32 > as FromPrimitive > ::from_u128(rug_fuzz_5), None
        );
        debug_assert_eq!(
            < Wrapping < u64 > as FromPrimitive > ::from_u128(rug_fuzz_6),
            Some(Wrapping(18_446_744_073_709_551_615_u64))
        );
        debug_assert_eq!(
            < Wrapping < u64 > as FromPrimitive > ::from_u128(rug_fuzz_7), None
        );
        debug_assert_eq!(
            < Wrapping < u128 > as FromPrimitive > ::from_u128(u128::MAX),
            Some(Wrapping(u128::MAX))
        );
        debug_assert_eq!(
            < Wrapping < i8 > as FromPrimitive > ::from_u128(rug_fuzz_8),
            Some(Wrapping(127_i8))
        );
        debug_assert_eq!(
            < Wrapping < i8 > as FromPrimitive > ::from_u128(rug_fuzz_9), None
        );
        debug_assert_eq!(
            < Wrapping < i16 > as FromPrimitive > ::from_u128(rug_fuzz_10),
            Some(Wrapping(32_767_i16))
        );
        debug_assert_eq!(
            < Wrapping < i16 > as FromPrimitive > ::from_u128(rug_fuzz_11), None
        );
        debug_assert_eq!(
            < Wrapping < i32 > as FromPrimitive > ::from_u128(rug_fuzz_12),
            Some(Wrapping(2_147_483_647_i32))
        );
        debug_assert_eq!(
            < Wrapping < i32 > as FromPrimitive > ::from_u128(rug_fuzz_13), None
        );
        debug_assert_eq!(
            < Wrapping < i64 > as FromPrimitive > ::from_u128(rug_fuzz_14),
            Some(Wrapping(9_223_372_036_854_775_807_i64))
        );
        debug_assert_eq!(
            < Wrapping < i64 > as FromPrimitive > ::from_u128(rug_fuzz_15), None
        );
        debug_assert_eq!(
            < Wrapping < i128 > as FromPrimitive > ::from_u128(u128::MAX),
            Some(Wrapping(u128::MAX as i128))
        );
        let _rug_ed_tests_llm_16_1270_llm_16_1270_rrrruuuugggg_test_from_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1271_llm_16_1271 {
    use std::num::Wrapping;
    use crate::cast::FromPrimitive;
    #[test]
    fn from_u16_wrapping() {
        let _rug_st_tests_llm_16_1271_llm_16_1271_rrrruuuugggg_from_u16_wrapping = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = 1;
        let num: u16 = rug_fuzz_0;
        let wrapped: Option<Wrapping<u16>> = FromPrimitive::from_u16(num);
        debug_assert_eq!(wrapped, Some(Wrapping(num)));
        let num_small: u16 = rug_fuzz_1;
        let wrapped_small: Option<Wrapping<u8>> = FromPrimitive::from_u16(num_small);
        debug_assert_eq!(wrapped_small, Some(Wrapping(num_small as u8)));
        let num_overflow: u16 = u8::MAX as u16 + rug_fuzz_2;
        let wrapped_overflow: Option<Wrapping<u8>> = FromPrimitive::from_u16(
            num_overflow,
        );
        debug_assert_eq!(wrapped_overflow, Some(Wrapping(num_overflow as u8)));
        let _rug_ed_tests_llm_16_1271_llm_16_1271_rrrruuuugggg_from_u16_wrapping = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1272_llm_16_1272 {
    use crate::cast::FromPrimitive;
    use std::num::Wrapping;
    #[test]
    fn test_from_u32() {
        let _rug_st_tests_llm_16_1272_llm_16_1272_rrrruuuugggg_test_from_u32 = 0;
        let rug_fuzz_0 = 123_u32;
        let rug_fuzz_1 = 123_u32;
        let rug_fuzz_2 = 123_u32;
        let rug_fuzz_3 = 123_u32;
        debug_assert_eq!(
            < Wrapping < i32 > as FromPrimitive > ::from_u32(rug_fuzz_0),
            Some(Wrapping(123))
        );
        debug_assert_eq!(
            < Wrapping < i32 > as FromPrimitive > ::from_u32(u32::MAX),
            Some(Wrapping(i32::MAX as u32 as i32))
        );
        debug_assert_eq!(
            < Wrapping < u32 > as FromPrimitive > ::from_u32(rug_fuzz_1),
            Some(Wrapping(123))
        );
        debug_assert_eq!(
            < Wrapping < u32 > as FromPrimitive > ::from_u32(u32::MAX),
            Some(Wrapping(u32::MAX))
        );
        debug_assert_eq!(
            < Wrapping < i8 > as FromPrimitive > ::from_u32(rug_fuzz_2),
            Some(Wrapping(123 as i8))
        );
        debug_assert_eq!(
            < Wrapping < i8 > as FromPrimitive > ::from_u32(u32::MAX), None
        );
        debug_assert_eq!(
            < Wrapping < i16 > as FromPrimitive > ::from_u32(rug_fuzz_3),
            Some(Wrapping(123 as i16))
        );
        debug_assert_eq!(
            < Wrapping < i16 > as FromPrimitive > ::from_u32(u32::MAX), None
        );
        let _rug_ed_tests_llm_16_1272_llm_16_1272_rrrruuuugggg_test_from_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1273_llm_16_1273 {
    use crate::cast::FromPrimitive;
    use std::num::Wrapping;
    #[test]
    fn test_from_u64_for_wrapping() {
        let _rug_st_tests_llm_16_1273_llm_16_1273_rrrruuuugggg_test_from_u64_for_wrapping = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 1;
        let num_u64: u64 = rug_fuzz_0;
        let wrapped_u32: Option<Wrapping<u32>> = Wrapping::<u32>::from_u64(num_u64);
        debug_assert_eq!(wrapped_u32, Some(Wrapping(42u32)));
        let wrapped_u8: Option<Wrapping<u8>> = Wrapping::<u8>::from_u64(num_u64);
        debug_assert_eq!(wrapped_u8, Some(Wrapping(42u8)));
        let num_large: u64 = u32::MAX as u64 + rug_fuzz_1;
        let wrapped_u32_large: Option<Wrapping<u32>> = Wrapping::<
            u32,
        >::from_u64(num_large);
        debug_assert_eq!(wrapped_u32_large, Some(Wrapping(0)));
        let _rug_ed_tests_llm_16_1273_llm_16_1273_rrrruuuugggg_test_from_u64_for_wrapping = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1274_llm_16_1274 {
    use super::*;
    use crate::*;
    use std::num::Wrapping;
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u8() {
        let _rug_st_tests_llm_16_1274_llm_16_1274_rrrruuuugggg_test_from_u8 = 0;
        let rug_fuzz_0 = 0u8;
        let rug_fuzz_1 = 255u8;
        debug_assert_eq!(Wrapping::from_u8(rug_fuzz_0), Some(Wrapping(0)));
        debug_assert_eq!(Wrapping::from_u8(rug_fuzz_1), Some(Wrapping(255)));
        let _rug_ed_tests_llm_16_1274_llm_16_1274_rrrruuuugggg_test_from_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1275_llm_16_1275 {
    use crate::cast::FromPrimitive;
    use std::num::Wrapping;
    #[test]
    fn test_from_usize() {
        let _rug_st_tests_llm_16_1275_llm_16_1275_rrrruuuugggg_test_from_usize = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42;
        debug_assert_eq!(
            < Wrapping < i32 > as FromPrimitive > ::from_usize(rug_fuzz_0),
            Some(Wrapping(42i32))
        );
        debug_assert_eq!(
            < Wrapping < i32 > as FromPrimitive > ::from_usize(usize::MAX), None
        );
        debug_assert_eq!(
            < Wrapping < u32 > as FromPrimitive > ::from_usize(rug_fuzz_1),
            Some(Wrapping(42u32))
        );
        let _rug_ed_tests_llm_16_1275_llm_16_1275_rrrruuuugggg_test_from_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1276_llm_16_1276 {
    use crate::cast::NumCast;
    use crate::cast::ToPrimitive;
    use std::num::Wrapping;
    #[test]
    fn test_from() {
        let _rug_st_tests_llm_16_1276_llm_16_1276_rrrruuuugggg_test_from = 0;
        let rug_fuzz_0 = 5i32;
        let rug_fuzz_1 = 5i8;
        let rug_fuzz_2 = 5u32;
        let rug_fuzz_3 = 5u8;
        let rug_fuzz_4 = 5.0f32;
        let rug_fuzz_5 = 5.0f64;
        let rug_fuzz_6 = 5u32;
        let rug_fuzz_7 = 5i8;
        let rug_fuzz_8 = 5u8;
        let rug_fuzz_9 = 5.0f32;
        let rug_fuzz_10 = 5.0f64;
        let rug_fuzz_11 = 5i64;
        let rug_fuzz_12 = 0xFFFFFFFFu32;
        let rug_fuzz_13 = 300.0f32;
        let rug_fuzz_14 = 300i32;
        let rug_fuzz_15 = 5.5f32;
        let rug_fuzz_16 = 5.5f64;
        debug_assert_eq!(
            < Wrapping < i32 > as NumCast > ::from(rug_fuzz_0), Some(Wrapping(5i32))
        );
        debug_assert_eq!(
            < Wrapping < i32 > as NumCast > ::from(rug_fuzz_1), Some(Wrapping(5i32))
        );
        debug_assert_eq!(
            < Wrapping < i32 > as NumCast > ::from(rug_fuzz_2), Some(Wrapping(5i32))
        );
        debug_assert_eq!(
            < Wrapping < i32 > as NumCast > ::from(rug_fuzz_3), Some(Wrapping(5i32))
        );
        debug_assert_eq!(
            < Wrapping < i32 > as NumCast > ::from(rug_fuzz_4), Some(Wrapping(5i32))
        );
        debug_assert_eq!(
            < Wrapping < i32 > as NumCast > ::from(rug_fuzz_5), Some(Wrapping(5i32))
        );
        debug_assert_eq!(
            < Wrapping < u32 > as NumCast > ::from(rug_fuzz_6), Some(Wrapping(5u32))
        );
        debug_assert_eq!(
            < Wrapping < u32 > as NumCast > ::from(rug_fuzz_7), Some(Wrapping(5u32))
        );
        debug_assert_eq!(
            < Wrapping < u32 > as NumCast > ::from(rug_fuzz_8), Some(Wrapping(5u32))
        );
        debug_assert_eq!(
            < Wrapping < u32 > as NumCast > ::from(rug_fuzz_9), Some(Wrapping(5u32))
        );
        debug_assert_eq!(
            < Wrapping < u32 > as NumCast > ::from(rug_fuzz_10), Some(Wrapping(5u32))
        );
        debug_assert_eq!(
            < Wrapping < i32 > as NumCast > ::from(rug_fuzz_11), Some(Wrapping(5i32))
        );
        debug_assert_eq!(
            < Wrapping < i32 > as NumCast > ::from(rug_fuzz_12 as i64), None
        );
        debug_assert_eq!(< Wrapping < i32 > as NumCast > ::from(rug_fuzz_13), None);
        debug_assert_eq!(< Wrapping < u8 > as NumCast > ::from(rug_fuzz_14), None);
        debug_assert_eq!(
            < Wrapping < u8 > as NumCast > ::from(rug_fuzz_15), Some(Wrapping(5u8))
        );
        debug_assert_eq!(
            < Wrapping < i32 > as NumCast > ::from(rug_fuzz_16), Some(Wrapping(5i32))
        );
        let _rug_ed_tests_llm_16_1276_llm_16_1276_rrrruuuugggg_test_from = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1277_llm_16_1277 {
    use std::num::Wrapping;
    use crate::cast::ToPrimitive;
    #[test]
    fn test_wrapping_to_f32() {
        let _rug_st_tests_llm_16_1277_llm_16_1277_rrrruuuugggg_test_wrapping_to_f32 = 0;
        let rug_fuzz_0 = 0i32;
        let rug_fuzz_1 = 1i32;
        let rug_fuzz_2 = 1i32;
        let rug_fuzz_3 = 1u32;
        let rug_fuzz_4 = 1i64;
        let rug_fuzz_5 = 1u64;
        debug_assert_eq!(Wrapping(rug_fuzz_0).to_f32(), Some(0.0f32));
        debug_assert_eq!(Wrapping(rug_fuzz_1).to_f32(), Some(1.0f32));
        debug_assert_eq!(Wrapping(- rug_fuzz_2).to_f32(), Some(- 1.0f32));
        debug_assert_eq!(Wrapping(i32::MAX).to_f32(), Some(i32::MAX as f32));
        debug_assert_eq!(Wrapping(i32::MIN).to_f32(), Some(i32::MIN as f32));
        debug_assert_eq!(Wrapping(rug_fuzz_3).to_f32(), Some(1.0f32));
        debug_assert_eq!(Wrapping(rug_fuzz_4).to_f32(), Some(1.0f32));
        debug_assert_eq!(Wrapping(rug_fuzz_5).to_f32(), Some(1.0f32));
        let _rug_ed_tests_llm_16_1277_llm_16_1277_rrrruuuugggg_test_wrapping_to_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1278_llm_16_1278 {
    use crate::cast::ToPrimitive;
    use std::num::Wrapping;
    #[test]
    fn test_to_f64_for_wrapping_i32() {
        let _rug_st_tests_llm_16_1278_llm_16_1278_rrrruuuugggg_test_to_f64_for_wrapping_i32 = 0;
        let rug_fuzz_0 = 123i32;
        let num = Wrapping(rug_fuzz_0);
        debug_assert_eq!(num.to_f64(), Some(123f64));
        let _rug_ed_tests_llm_16_1278_llm_16_1278_rrrruuuugggg_test_to_f64_for_wrapping_i32 = 0;
    }
    #[test]
    fn test_to_f64_for_wrapping_i64() {
        let _rug_st_tests_llm_16_1278_llm_16_1278_rrrruuuugggg_test_to_f64_for_wrapping_i64 = 0;
        let rug_fuzz_0 = 123i64;
        let num = Wrapping(rug_fuzz_0);
        debug_assert_eq!(num.to_f64(), Some(123f64));
        let _rug_ed_tests_llm_16_1278_llm_16_1278_rrrruuuugggg_test_to_f64_for_wrapping_i64 = 0;
    }
    #[test]
    fn test_to_f64_for_wrapping_u32() {
        let _rug_st_tests_llm_16_1278_llm_16_1278_rrrruuuugggg_test_to_f64_for_wrapping_u32 = 0;
        let rug_fuzz_0 = 123u32;
        let num = Wrapping(rug_fuzz_0);
        debug_assert_eq!(num.to_f64(), Some(123f64));
        let _rug_ed_tests_llm_16_1278_llm_16_1278_rrrruuuugggg_test_to_f64_for_wrapping_u32 = 0;
    }
    #[test]
    fn test_to_f64_for_wrapping_u64() {
        let _rug_st_tests_llm_16_1278_llm_16_1278_rrrruuuugggg_test_to_f64_for_wrapping_u64 = 0;
        let rug_fuzz_0 = 123u64;
        let num = Wrapping(rug_fuzz_0);
        debug_assert_eq!(num.to_f64(), Some(123f64));
        let _rug_ed_tests_llm_16_1278_llm_16_1278_rrrruuuugggg_test_to_f64_for_wrapping_u64 = 0;
    }
    #[test]
    fn test_to_f64_for_wrapping_f32() {
        let _rug_st_tests_llm_16_1278_llm_16_1278_rrrruuuugggg_test_to_f64_for_wrapping_f32 = 0;
        let rug_fuzz_0 = 123f32;
        let num = Wrapping(rug_fuzz_0);
        debug_assert_eq!(num.to_f64(), Some(123f32 as f64));
        let _rug_ed_tests_llm_16_1278_llm_16_1278_rrrruuuugggg_test_to_f64_for_wrapping_f32 = 0;
    }
    #[test]
    fn test_to_f64_for_wrapping_none() {
        let _rug_st_tests_llm_16_1278_llm_16_1278_rrrruuuugggg_test_to_f64_for_wrapping_none = 0;
        let num: Wrapping<u128> = Wrapping(u128::max_value());
        debug_assert_eq!(num.to_f64(), None);
        let _rug_ed_tests_llm_16_1278_llm_16_1278_rrrruuuugggg_test_to_f64_for_wrapping_none = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1279_llm_16_1279 {
    use std::num::Wrapping;
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_i128() {
        assert_eq!(Wrapping(0_i8).to_i128(), Some(0_i128));
        assert_eq!(Wrapping(127_i8).to_i128(), Some(127_i128));
        assert_eq!(Wrapping(- 128_i8).to_i128(), Some(- 128_i128));
        assert_eq!(Wrapping(0_i16).to_i128(), Some(0_i128));
        assert_eq!(Wrapping(32767_i16).to_i128(), Some(32767_i128));
        assert_eq!(Wrapping(- 32768_i16).to_i128(), Some(- 32768_i128));
        assert_eq!(Wrapping(0_i32).to_i128(), Some(0_i128));
        assert_eq!(Wrapping(2147483647_i32).to_i128(), Some(2147483647_i128));
        assert_eq!(Wrapping(- 2147483648_i32).to_i128(), Some(- 2147483648_i128));
        assert_eq!(Wrapping(0_i64).to_i128(), Some(0_i128));
        assert_eq!(
            Wrapping(9223372036854775807_i64).to_i128(), Some(9223372036854775807_i128)
        );
        assert_eq!(
            Wrapping(- 9223372036854775808_i64).to_i128(), Some(-
            9223372036854775808_i128)
        );
        assert_eq!(Wrapping(0_i128).to_i128(), Some(0_i128));
        assert_eq!(Wrapping(i128::MAX).to_i128(), Some(i128::MAX));
        assert_eq!(Wrapping(i128::MIN).to_i128(), Some(i128::MIN));
        assert_eq!(Wrapping(0_u8).to_i128(), Some(0_i128));
        assert_eq!(Wrapping(255_u8).to_i128(), Some(255_i128));
        assert_eq!(Wrapping(0_u16).to_i128(), Some(0_i128));
        assert_eq!(Wrapping(65535_u16).to_i128(), Some(65535_i128));
        assert_eq!(Wrapping(0_u32).to_i128(), Some(0_i128));
        assert_eq!(Wrapping(4294967295_u32).to_i128(), Some(4294967295_i128));
        assert_eq!(Wrapping(0_u64).to_i128(), Some(0_i128));
        assert_eq!(
            Wrapping(18446744073709551615_u64).to_i128(), Some(18446744073709551615_i128)
        );
        let max_u128_to_i128 = u128::MAX / 2;
        assert_eq!(Wrapping(0_u128).to_i128(), Some(0_i128));
        assert_eq!(Wrapping(max_u128_to_i128).to_i128(), Some(max_u128_to_i128 as i128));
    }
}
#[cfg(test)]
mod tests_llm_16_1280_llm_16_1280 {
    use crate::cast::ToPrimitive;
    use std::num::Wrapping;
    #[test]
    fn test_to_i16() {
        assert_eq!(Wrapping(5_i16).to_i16(), Some(5_i16));
        assert_eq!(Wrapping(300_i16).to_i16(), Some(300_i16));
        assert_eq!(Wrapping(- 300_i16).to_i16(), Some(- 300_i16));
        assert_eq!(Wrapping(32767_i16).to_i16(), Some(32767_i16));
        assert_eq!(Wrapping(- 32768_i16).to_i16(), Some(- 32768_i16));
        assert_eq!(Wrapping(i16::MAX).to_i16(), Some(i16::MAX));
        assert_eq!(Wrapping(i16::MIN).to_i16(), Some(i16::MIN));
        assert_eq!(Wrapping(5_u8).to_i16(), Some(5_i16));
        assert_eq!(Wrapping(5_i8).to_i16(), Some(5_i16));
        assert_eq!(Wrapping(5_u16).to_i16(), Some(5_i16));
        assert_eq!(Wrapping(5_i32).to_i16(), Some(5_i16));
        assert_eq!(Wrapping(5_u32).to_i16(), Some(5_i16));
        assert_eq!(Wrapping(5_i64).to_i16(), Some(5_i16));
        assert_eq!(Wrapping(5_u64).to_i16(), Some(5_i16));
        assert_eq!(Wrapping(5_i128).to_i16(), Some(5_i16));
        assert_eq!(Wrapping(5_u128).to_i16(), Some(5_i16));
        assert_eq!(Wrapping(70000_i32).to_i16(), None);
        assert_eq!(Wrapping(70000_u32).to_i16(), None);
        assert_eq!(Wrapping(i32::MAX).to_i16(), None);
        assert_eq!(Wrapping(u32::MAX).to_i16(), None);
        assert_eq!(Wrapping(i64::MAX).to_i16(), None);
        assert_eq!(Wrapping(u64::MAX).to_i16(), None);
        assert_eq!(Wrapping(i128::MAX).to_i16(), None);
        assert_eq!(Wrapping(u128::MAX).to_i16(), None);
    }
}
#[cfg(test)]
mod tests_llm_16_1281_llm_16_1281 {
    use crate::cast::ToPrimitive;
    use std::num::Wrapping;
    #[test]
    fn test_to_i32_with_i32() {
        let _rug_st_tests_llm_16_1281_llm_16_1281_rrrruuuugggg_test_to_i32_with_i32 = 0;
        let rug_fuzz_0 = 42i32;
        let original = Wrapping(rug_fuzz_0);
        debug_assert_eq!(ToPrimitive::to_i32(& original), Some(42i32));
        let _rug_ed_tests_llm_16_1281_llm_16_1281_rrrruuuugggg_test_to_i32_with_i32 = 0;
    }
    #[test]
    fn test_to_i32_with_i64() {
        let _rug_st_tests_llm_16_1281_llm_16_1281_rrrruuuugggg_test_to_i32_with_i64 = 0;
        let rug_fuzz_0 = 42i64;
        let original = Wrapping(rug_fuzz_0);
        debug_assert_eq!(ToPrimitive::to_i32(& original), Some(42i32));
        let _rug_ed_tests_llm_16_1281_llm_16_1281_rrrruuuugggg_test_to_i32_with_i64 = 0;
    }
    #[test]
    fn test_to_i32_with_u32() {
        let _rug_st_tests_llm_16_1281_llm_16_1281_rrrruuuugggg_test_to_i32_with_u32 = 0;
        let rug_fuzz_0 = 42u32;
        let original = Wrapping(rug_fuzz_0);
        debug_assert_eq!(ToPrimitive::to_i32(& original), Some(42i32));
        let _rug_ed_tests_llm_16_1281_llm_16_1281_rrrruuuugggg_test_to_i32_with_u32 = 0;
    }
    #[test]
    fn test_to_i32_with_u64() {
        let _rug_st_tests_llm_16_1281_llm_16_1281_rrrruuuugggg_test_to_i32_with_u64 = 0;
        let rug_fuzz_0 = 42u64;
        let original = Wrapping(rug_fuzz_0);
        debug_assert_eq!(ToPrimitive::to_i32(& original), Some(42i32));
        let _rug_ed_tests_llm_16_1281_llm_16_1281_rrrruuuugggg_test_to_i32_with_u64 = 0;
    }
    #[test]
    fn test_to_i32_with_large_i64() {
        let _rug_st_tests_llm_16_1281_llm_16_1281_rrrruuuugggg_test_to_i32_with_large_i64 = 0;
        let original = Wrapping(i64::MAX);
        debug_assert_eq!(ToPrimitive::to_i32(& original), None);
        let _rug_ed_tests_llm_16_1281_llm_16_1281_rrrruuuugggg_test_to_i32_with_large_i64 = 0;
    }
    #[test]
    fn test_to_i32_with_large_u64() {
        let _rug_st_tests_llm_16_1281_llm_16_1281_rrrruuuugggg_test_to_i32_with_large_u64 = 0;
        let original = Wrapping(u64::MAX);
        debug_assert_eq!(ToPrimitive::to_i32(& original), None);
        let _rug_ed_tests_llm_16_1281_llm_16_1281_rrrruuuugggg_test_to_i32_with_large_u64 = 0;
    }
    #[test]
    fn test_to_i32_with_u32_out_of_i32_range() {
        let _rug_st_tests_llm_16_1281_llm_16_1281_rrrruuuugggg_test_to_i32_with_u32_out_of_i32_range = 0;
        let original = Wrapping(u32::MAX);
        debug_assert_eq!(ToPrimitive::to_i32(& original), None);
        let _rug_ed_tests_llm_16_1281_llm_16_1281_rrrruuuugggg_test_to_i32_with_u32_out_of_i32_range = 0;
    }
    #[test]
    fn test_to_i32_with_large_i32() {
        let _rug_st_tests_llm_16_1281_llm_16_1281_rrrruuuugggg_test_to_i32_with_large_i32 = 0;
        let original = Wrapping(i32::MAX);
        debug_assert_eq!(ToPrimitive::to_i32(& original), Some(i32::MAX));
        let _rug_ed_tests_llm_16_1281_llm_16_1281_rrrruuuugggg_test_to_i32_with_large_i32 = 0;
    }
    #[test]
    fn test_to_i32_with_large_i32_negative() {
        let _rug_st_tests_llm_16_1281_llm_16_1281_rrrruuuugggg_test_to_i32_with_large_i32_negative = 0;
        let original = Wrapping(i32::MIN);
        debug_assert_eq!(ToPrimitive::to_i32(& original), Some(i32::MIN));
        let _rug_ed_tests_llm_16_1281_llm_16_1281_rrrruuuugggg_test_to_i32_with_large_i32_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1282_llm_16_1282 {
    use crate::cast::ToPrimitive;
    use std::num::Wrapping;
    #[test]
    fn test_to_i64() {
        let _rug_st_tests_llm_16_1282_llm_16_1282_rrrruuuugggg_test_to_i64 = 0;
        let rug_fuzz_0 = 10i64;
        let rug_fuzz_1 = 10i64;
        let rug_fuzz_2 = 0i64;
        let small_value = Wrapping(rug_fuzz_0);
        debug_assert_eq!(small_value.to_i64(), Some(10i64));
        let large_value = Wrapping(i64::max_value());
        debug_assert_eq!(large_value.to_i64(), Some(i64::max_value()));
        let negative_value = Wrapping(-rug_fuzz_1);
        debug_assert_eq!(negative_value.to_i64(), Some(- 10i64));
        let zero_value = Wrapping(rug_fuzz_2);
        debug_assert_eq!(zero_value.to_i64(), Some(0i64));
        let _rug_ed_tests_llm_16_1282_llm_16_1282_rrrruuuugggg_test_to_i64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1283_llm_16_1283 {
    use crate::cast::ToPrimitive;
    use std::num::Wrapping;
    #[test]
    fn to_i8_for_wrapping() {
        let _rug_st_tests_llm_16_1283_llm_16_1283_rrrruuuugggg_to_i8_for_wrapping = 0;
        let rug_fuzz_0 = 100i8;
        let rug_fuzz_1 = 200u8;
        let rug_fuzz_2 = 0i32;
        let rug_fuzz_3 = 127i32;
        let rug_fuzz_4 = 128i32;
        let rug_fuzz_5 = 129i32;
        let rug_fuzz_6 = 255u8;
        let rug_fuzz_7 = 1i32;
        debug_assert_eq!(Wrapping(rug_fuzz_0).to_i8(), Some(100i8));
        debug_assert_eq!(Wrapping(rug_fuzz_1).to_i8(), Some(- 56i8));
        debug_assert_eq!(Wrapping(rug_fuzz_2).to_i8(), Some(0i8));
        debug_assert_eq!(Wrapping(rug_fuzz_3).to_i8(), Some(127i8));
        debug_assert_eq!(Wrapping(rug_fuzz_4).to_i8(), Some(- 128i8));
        debug_assert_eq!(Wrapping(- rug_fuzz_5).to_i8(), Some(127i8));
        debug_assert_eq!(Wrapping(rug_fuzz_6).to_i8(), Some(- 1i8));
        debug_assert_eq!(Wrapping(- rug_fuzz_7).to_i8(), Some(- 1i8));
        debug_assert_eq!(Wrapping(i16::MAX).to_i8(), Some(- 1i8));
        debug_assert_eq!(Wrapping(i16::MIN).to_i8(), Some(0i8));
        debug_assert_eq!(Wrapping(i32::MAX).to_i8(), Some(- 1i8));
        debug_assert_eq!(Wrapping(i32::MIN).to_i8(), Some(0i8));
        debug_assert_eq!(Wrapping(i64::MAX).to_i8(), Some(- 1i8));
        debug_assert_eq!(Wrapping(i64::MIN).to_i8(), Some(0i8));
        debug_assert_eq!(Wrapping(i128::MAX).to_i8(), Some(- 1i8));
        debug_assert_eq!(Wrapping(i128::MIN).to_i8(), Some(0i8));
        debug_assert_eq!(Wrapping(u128::MAX).to_i8(), Some(- 1i8));
        let _rug_ed_tests_llm_16_1283_llm_16_1283_rrrruuuugggg_to_i8_for_wrapping = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1284_llm_16_1284 {
    use crate::cast::ToPrimitive;
    use std::num::Wrapping;
    #[test]
    fn test_to_isize() {
        let _rug_st_tests_llm_16_1284_llm_16_1284_rrrruuuugggg_test_to_isize = 0;
        let rug_fuzz_0 = 5_i8;
        let rug_fuzz_1 = 5_i8;
        let rug_fuzz_2 = 5_i16;
        let rug_fuzz_3 = 5_i16;
        let rug_fuzz_4 = 5_i32;
        let rug_fuzz_5 = 5_i32;
        let rug_fuzz_6 = 5_i64;
        let rug_fuzz_7 = 5_i64;
        let rug_fuzz_8 = 5_i128;
        let rug_fuzz_9 = 5_i128;
        debug_assert_eq!(Wrapping(rug_fuzz_0).to_isize(), Some(5_isize));
        debug_assert_eq!(Wrapping(- rug_fuzz_1).to_isize(), Some(- 5_isize));
        debug_assert_eq!(Wrapping(rug_fuzz_2).to_isize(), Some(5_isize));
        debug_assert_eq!(Wrapping(- rug_fuzz_3).to_isize(), Some(- 5_isize));
        debug_assert_eq!(Wrapping(rug_fuzz_4).to_isize(), Some(5_isize));
        debug_assert_eq!(Wrapping(- rug_fuzz_5).to_isize(), Some(- 5_isize));
        debug_assert_eq!(Wrapping(rug_fuzz_6).to_isize(), Some(5_isize));
        debug_assert_eq!(Wrapping(- rug_fuzz_7).to_isize(), Some(- 5_isize));
        debug_assert_eq!(Wrapping(rug_fuzz_8).to_isize(), Some(5_isize));
        debug_assert_eq!(Wrapping(- rug_fuzz_9).to_isize(), Some(- 5_isize));
        #[cfg(target_pointer_width = "32")]
        {
            debug_assert_eq!(Wrapping(i32::MAX).to_isize(), Some(i32::MAX as isize));
            debug_assert_eq!(Wrapping(i32::MIN).to_isize(), Some(i32::MIN as isize));
            debug_assert_eq!(Wrapping(i64::MAX).to_isize(), None);
        }
        #[cfg(target_pointer_width = "64")]
        {
            debug_assert_eq!(Wrapping(i64::MAX).to_isize(), Some(i64::MAX as isize));
            debug_assert_eq!(Wrapping(i64::MIN).to_isize(), Some(i64::MIN as isize));
            debug_assert_eq!(Wrapping(i128::MAX).to_isize(), None);
        }
        let _rug_ed_tests_llm_16_1284_llm_16_1284_rrrruuuugggg_test_to_isize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1285_llm_16_1285 {
    use std::num::Wrapping;
    use crate::cast::ToPrimitive;
    #[test]
    fn to_u128_wrapping_i32() {
        let _rug_st_tests_llm_16_1285_llm_16_1285_rrrruuuugggg_to_u128_wrapping_i32 = 0;
        let rug_fuzz_0 = 123;
        let value: Wrapping<i32> = Wrapping(rug_fuzz_0);
        debug_assert_eq!(value.to_u128(), Some(123_u128));
        let _rug_ed_tests_llm_16_1285_llm_16_1285_rrrruuuugggg_to_u128_wrapping_i32 = 0;
    }
    #[test]
    fn to_u128_wrapping_i64() {
        let _rug_st_tests_llm_16_1285_llm_16_1285_rrrruuuugggg_to_u128_wrapping_i64 = 0;
        let rug_fuzz_0 = 123456789012;
        let value: Wrapping<i64> = Wrapping(rug_fuzz_0);
        debug_assert_eq!(value.to_u128(), Some(123456789012_u128));
        let _rug_ed_tests_llm_16_1285_llm_16_1285_rrrruuuugggg_to_u128_wrapping_i64 = 0;
    }
    #[test]
    fn to_u128_wrapping_u8() {
        let _rug_st_tests_llm_16_1285_llm_16_1285_rrrruuuugggg_to_u128_wrapping_u8 = 0;
        let rug_fuzz_0 = 255;
        let value: Wrapping<u8> = Wrapping(rug_fuzz_0);
        debug_assert_eq!(value.to_u128(), Some(255_u128));
        let _rug_ed_tests_llm_16_1285_llm_16_1285_rrrruuuugggg_to_u128_wrapping_u8 = 0;
    }
    #[test]
    fn to_u128_wrapping_u64() {
        let _rug_st_tests_llm_16_1285_llm_16_1285_rrrruuuugggg_to_u128_wrapping_u64 = 0;
        let value: Wrapping<u64> = Wrapping(u64::MAX);
        debug_assert_eq!(value.to_u128(), Some(u64::MAX as u128));
        let _rug_ed_tests_llm_16_1285_llm_16_1285_rrrruuuugggg_to_u128_wrapping_u64 = 0;
    }
    #[test]
    fn to_u128_wrapping_u128() {
        let _rug_st_tests_llm_16_1285_llm_16_1285_rrrruuuugggg_to_u128_wrapping_u128 = 0;
        let value: Wrapping<u128> = Wrapping(u128::MAX);
        debug_assert_eq!(value.to_u128(), Some(u128::MAX));
        let _rug_ed_tests_llm_16_1285_llm_16_1285_rrrruuuugggg_to_u128_wrapping_u128 = 0;
    }
    #[test]
    fn to_u128_wrapping_i128() {
        let _rug_st_tests_llm_16_1285_llm_16_1285_rrrruuuugggg_to_u128_wrapping_i128 = 0;
        let value: Wrapping<i128> = Wrapping(i128::MAX);
        debug_assert_eq!(value.to_u128(), Some(i128::MAX as u128));
        let _rug_ed_tests_llm_16_1285_llm_16_1285_rrrruuuugggg_to_u128_wrapping_i128 = 0;
    }
    #[test]
    fn to_u128_wrapping_i128_negative() {
        let _rug_st_tests_llm_16_1285_llm_16_1285_rrrruuuugggg_to_u128_wrapping_i128_negative = 0;
        let rug_fuzz_0 = 123456789012_i128;
        let value: Wrapping<i128> = Wrapping(-rug_fuzz_0);
        debug_assert_eq!(value.to_u128(), None);
        let _rug_ed_tests_llm_16_1285_llm_16_1285_rrrruuuugggg_to_u128_wrapping_i128_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1286_llm_16_1286 {
    use crate::cast::ToPrimitive;
    use std::num::Wrapping;
    #[test]
    fn test_to_u16() {
        let _rug_st_tests_llm_16_1286_llm_16_1286_rrrruuuugggg_test_to_u16 = 0;
        let rug_fuzz_0 = 5u16;
        let rug_fuzz_1 = 5i16;
        let rug_fuzz_2 = 65535u16;
        let rug_fuzz_3 = 65535i32;
        let rug_fuzz_4 = 0i16;
        let rug_fuzz_5 = 0i32;
        let rug_fuzz_6 = 100u16;
        let rug_fuzz_7 = 100i16;
        let rug_fuzz_8 = 0u16;
        let rug_fuzz_9 = 0i32;
        let rug_fuzz_10 = 100000i32;
        let rug_fuzz_11 = 100000i32;
        let rug_fuzz_12 = 70000u32;
        let rug_fuzz_13 = 70000i32;
        debug_assert_eq!(Wrapping(rug_fuzz_0).to_u16(), Some(5u16));
        debug_assert_eq!(Wrapping(rug_fuzz_1).to_u16(), Some(5u16));
        debug_assert_eq!(Wrapping(rug_fuzz_2).to_u16(), Some(65535u16));
        debug_assert_eq!(Wrapping(rug_fuzz_3).to_u16(), Some(65535u16));
        debug_assert_eq!(Wrapping(rug_fuzz_4).to_u16(), Some(0u16));
        debug_assert_eq!(Wrapping(rug_fuzz_5).to_u16(), Some(0u16));
        debug_assert_eq!(Wrapping(rug_fuzz_6).to_u16(), Some(100u16));
        debug_assert_eq!(Wrapping(rug_fuzz_7).to_u16(), Some(100u16));
        debug_assert_eq!(Wrapping(rug_fuzz_8).to_u16(), Some(0u16));
        debug_assert_eq!(Wrapping(rug_fuzz_9).to_u16(), Some(0u16));
        debug_assert_eq!(Wrapping(rug_fuzz_10).to_u16(), Some(34464u16));
        debug_assert_eq!(Wrapping(- rug_fuzz_11).to_u16(), Some(31072u16));
        debug_assert_eq!(Wrapping(rug_fuzz_12).to_u16(), Some(4464u16));
        debug_assert_eq!(Wrapping(- rug_fuzz_13).to_u16(), Some(31072u16));
        let _rug_ed_tests_llm_16_1286_llm_16_1286_rrrruuuugggg_test_to_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1287_llm_16_1287 {
    use crate::cast::ToPrimitive;
    use std::num::Wrapping;
    #[test]
    fn to_u32_for_wrapping_u32() {
        let _rug_st_tests_llm_16_1287_llm_16_1287_rrrruuuugggg_to_u32_for_wrapping_u32 = 0;
        let rug_fuzz_0 = 123u32;
        let value = Wrapping(rug_fuzz_0);
        debug_assert_eq!(value.to_u32(), Some(123u32));
        let _rug_ed_tests_llm_16_1287_llm_16_1287_rrrruuuugggg_to_u32_for_wrapping_u32 = 0;
    }
    #[test]
    fn to_u32_for_wrapping_i32() {
        let _rug_st_tests_llm_16_1287_llm_16_1287_rrrruuuugggg_to_u32_for_wrapping_i32 = 0;
        let rug_fuzz_0 = 123i32;
        let value = Wrapping(rug_fuzz_0);
        debug_assert_eq!(value.to_u32(), Some(123u32));
        let _rug_ed_tests_llm_16_1287_llm_16_1287_rrrruuuugggg_to_u32_for_wrapping_i32 = 0;
    }
    #[test]
    fn to_u32_for_wrapping_u64() {
        let _rug_st_tests_llm_16_1287_llm_16_1287_rrrruuuugggg_to_u32_for_wrapping_u64 = 0;
        let rug_fuzz_0 = 123u64;
        let value = Wrapping(rug_fuzz_0);
        debug_assert_eq!(value.to_u32(), Some(123u32));
        let _rug_ed_tests_llm_16_1287_llm_16_1287_rrrruuuugggg_to_u32_for_wrapping_u64 = 0;
    }
    #[test]
    fn to_u32_for_wrapping_i64() {
        let _rug_st_tests_llm_16_1287_llm_16_1287_rrrruuuugggg_to_u32_for_wrapping_i64 = 0;
        let rug_fuzz_0 = 123i64;
        let value = Wrapping(rug_fuzz_0);
        debug_assert_eq!(value.to_u32(), Some(123u32));
        let _rug_ed_tests_llm_16_1287_llm_16_1287_rrrruuuugggg_to_u32_for_wrapping_i64 = 0;
    }
    #[test]
    fn to_u32_for_wrapping_u64_large() {
        let _rug_st_tests_llm_16_1287_llm_16_1287_rrrruuuugggg_to_u32_for_wrapping_u64_large = 0;
        let value = Wrapping(u64::MAX);
        debug_assert_eq!(value.to_u32(), None);
        let _rug_ed_tests_llm_16_1287_llm_16_1287_rrrruuuugggg_to_u32_for_wrapping_u64_large = 0;
    }
    #[test]
    fn to_u32_for_wrapping_i64_large() {
        let _rug_st_tests_llm_16_1287_llm_16_1287_rrrruuuugggg_to_u32_for_wrapping_i64_large = 0;
        let value = Wrapping(i64::MAX);
        debug_assert_eq!(value.to_u32(), None);
        let _rug_ed_tests_llm_16_1287_llm_16_1287_rrrruuuugggg_to_u32_for_wrapping_i64_large = 0;
    }
    #[test]
    fn to_u32_for_wrapping_i32_negative() {
        let _rug_st_tests_llm_16_1287_llm_16_1287_rrrruuuugggg_to_u32_for_wrapping_i32_negative = 0;
        let rug_fuzz_0 = 123i32;
        let value = Wrapping(-rug_fuzz_0);
        debug_assert_eq!(value.to_u32(), None);
        let _rug_ed_tests_llm_16_1287_llm_16_1287_rrrruuuugggg_to_u32_for_wrapping_i32_negative = 0;
    }
    #[test]
    fn to_u32_for_wrapping_i64_negative() {
        let _rug_st_tests_llm_16_1287_llm_16_1287_rrrruuuugggg_to_u32_for_wrapping_i64_negative = 0;
        let rug_fuzz_0 = 123i64;
        let value = Wrapping(-rug_fuzz_0);
        debug_assert_eq!(value.to_u32(), None);
        let _rug_ed_tests_llm_16_1287_llm_16_1287_rrrruuuugggg_to_u32_for_wrapping_i64_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1288_llm_16_1288 {
    use crate::cast::ToPrimitive;
    use std::num::Wrapping;
    #[test]
    fn test_to_u64() {
        let _rug_st_tests_llm_16_1288_llm_16_1288_rrrruuuugggg_test_to_u64 = 0;
        let rug_fuzz_0 = 42u32;
        let small_value = Wrapping(rug_fuzz_0);
        debug_assert_eq!(small_value.to_u64(), Some(42u64));
        let max_u64 = Wrapping(u64::MAX);
        debug_assert_eq!(max_u64.to_u64(), Some(u64::MAX));
        let large_value = Wrapping(usize::MAX);
        debug_assert_eq!(large_value.to_u64(), Some(usize::MAX as u64));
        let _rug_ed_tests_llm_16_1288_llm_16_1288_rrrruuuugggg_test_to_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1289_llm_16_1289 {
    use super::*;
    use crate::*;
    use crate::*;
    use std::num::Wrapping;
    #[test]
    fn test_to_u8() {
        let _rug_st_tests_llm_16_1289_llm_16_1289_rrrruuuugggg_test_to_u8 = 0;
        let rug_fuzz_0 = 0u8;
        let rug_fuzz_1 = 255u8;
        let rug_fuzz_2 = 0u16;
        let rug_fuzz_3 = 255u16;
        let rug_fuzz_4 = 256u16;
        let rug_fuzz_5 = 0i16;
        let rug_fuzz_6 = 255i16;
        let rug_fuzz_7 = 256i16;
        let rug_fuzz_8 = 1i16;
        debug_assert_eq!(Wrapping(rug_fuzz_0).to_u8(), Some(0));
        debug_assert_eq!(Wrapping(rug_fuzz_1).to_u8(), Some(255));
        debug_assert_eq!(Wrapping(rug_fuzz_2).to_u8(), Some(0));
        debug_assert_eq!(Wrapping(rug_fuzz_3).to_u8(), Some(255));
        debug_assert_eq!(Wrapping(rug_fuzz_4).to_u8(), None);
        debug_assert_eq!(Wrapping(rug_fuzz_5).to_u8(), Some(0));
        debug_assert_eq!(Wrapping(rug_fuzz_6).to_u8(), Some(255));
        debug_assert_eq!(Wrapping(rug_fuzz_7).to_u8(), None);
        debug_assert_eq!(Wrapping(- rug_fuzz_8).to_u8(), None);
        let _rug_ed_tests_llm_16_1289_llm_16_1289_rrrruuuugggg_test_to_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1290_llm_16_1290 {
    use crate::cast::ToPrimitive;
    use std::num::Wrapping;
    #[test]
    fn to_usize_for_wrapping_u32() {
        let _rug_st_tests_llm_16_1290_llm_16_1290_rrrruuuugggg_to_usize_for_wrapping_u32 = 0;
        let rug_fuzz_0 = 123u32;
        let value = Wrapping(rug_fuzz_0);
        debug_assert_eq!(value.to_usize(), Some(123usize));
        let _rug_ed_tests_llm_16_1290_llm_16_1290_rrrruuuugggg_to_usize_for_wrapping_u32 = 0;
    }
    #[test]
    fn to_usize_for_wrapping_usize_max() {
        let _rug_st_tests_llm_16_1290_llm_16_1290_rrrruuuugggg_to_usize_for_wrapping_usize_max = 0;
        let value = Wrapping(usize::MAX);
        debug_assert_eq!(value.to_usize(), Some(usize::MAX));
        let _rug_ed_tests_llm_16_1290_llm_16_1290_rrrruuuugggg_to_usize_for_wrapping_usize_max = 0;
    }
    #[test]
    fn to_usize_for_wrapping_i32_within_usize_range() {
        let _rug_st_tests_llm_16_1290_llm_16_1290_rrrruuuugggg_to_usize_for_wrapping_i32_within_usize_range = 0;
        let rug_fuzz_0 = 123i32;
        let value = Wrapping(rug_fuzz_0);
        debug_assert_eq!(value.to_usize(), Some(123usize));
        let _rug_ed_tests_llm_16_1290_llm_16_1290_rrrruuuugggg_to_usize_for_wrapping_i32_within_usize_range = 0;
    }
    #[test]
    fn to_usize_for_wrapping_i32_out_of_usize_range() {
        let _rug_st_tests_llm_16_1290_llm_16_1290_rrrruuuugggg_to_usize_for_wrapping_i32_out_of_usize_range = 0;
        let rug_fuzz_0 = 123i32;
        let value = Wrapping(-rug_fuzz_0);
        debug_assert_eq!(value.to_usize(), None);
        let _rug_ed_tests_llm_16_1290_llm_16_1290_rrrruuuugggg_to_usize_for_wrapping_i32_out_of_usize_range = 0;
    }
    #[test]
    fn to_usize_for_wrapping_i64_within_usize_range() {
        let _rug_st_tests_llm_16_1290_llm_16_1290_rrrruuuugggg_to_usize_for_wrapping_i64_within_usize_range = 0;
        let rug_fuzz_0 = 123i64;
        let value = Wrapping(rug_fuzz_0);
        debug_assert!(value.to_usize().is_some());
        let _rug_ed_tests_llm_16_1290_llm_16_1290_rrrruuuugggg_to_usize_for_wrapping_i64_within_usize_range = 0;
    }
    #[test]
    #[cfg(target_pointer_width = "64")]
    fn to_usize_for_wrapping_i64_out_of_usize_range_on_64bit() {
        let _rug_st_tests_llm_16_1290_llm_16_1290_rrrruuuugggg_to_usize_for_wrapping_i64_out_of_usize_range_on_64bit = 0;
        let value = Wrapping(i64::MAX);
        debug_assert_eq!(value.to_usize(), None);
        let _rug_ed_tests_llm_16_1290_llm_16_1290_rrrruuuugggg_to_usize_for_wrapping_i64_out_of_usize_range_on_64bit = 0;
    }
    #[test]
    #[cfg(target_pointer_width = "32")]
    fn to_usize_for_wrapping_i64_out_of_usize_range_on_32bit() {
        let _rug_st_tests_llm_16_1290_llm_16_1290_rrrruuuugggg_to_usize_for_wrapping_i64_out_of_usize_range_on_32bit = 0;
        let value = Wrapping(i64::MAX);
        debug_assert_eq!(value.to_usize(), Some(i64::MAX as usize));
        let _rug_ed_tests_llm_16_1290_llm_16_1290_rrrruuuugggg_to_usize_for_wrapping_i64_out_of_usize_range_on_32bit = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1358_llm_16_1358 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_u128_as_f32() {
        let _rug_st_tests_llm_16_1358_llm_16_1358_rrrruuuugggg_test_u128_as_f32 = 0;
        let rug_fuzz_0 = 123456789012345678901234567890_u128;
        let value = rug_fuzz_0;
        let result: f32 = AsPrimitive::<f32>::as_(value);
        let expected = value as f32;
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_1358_llm_16_1358_rrrruuuugggg_test_u128_as_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1359_llm_16_1359 {
    use crate::cast::AsPrimitive;
    #[test]
    fn u128_as_f64() {
        let _rug_st_tests_llm_16_1359_llm_16_1359_rrrruuuugggg_u128_as_f64 = 0;
        let rug_fuzz_0 = 12345678901234567890;
        let value: u128 = rug_fuzz_0;
        let result = AsPrimitive::<f64>::as_(value);
        let expected = value as f64;
        debug_assert!((result - expected).abs() < f64::EPSILON);
        let _rug_ed_tests_llm_16_1359_llm_16_1359_rrrruuuugggg_u128_as_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1361_llm_16_1361 {
    use crate::AsPrimitive;
    #[test]
    fn test_as_primitive_u128_to_i16() {
        let _rug_st_tests_llm_16_1361_llm_16_1361_rrrruuuugggg_test_as_primitive_u128_to_i16 = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 1;
        let value_u128: u128 = rug_fuzz_0;
        let value_i16: i16 = AsPrimitive::<i16>::as_(value_u128);
        debug_assert_eq!(value_i16, 42i16);
        let max_i16_as_u128: u128 = i16::MAX as u128;
        let max_i16: i16 = AsPrimitive::<i16>::as_(max_i16_as_u128);
        debug_assert_eq!(max_i16, i16::MAX);
        let beyond_max_i16_as_u128: u128 = (i16::MAX as u128) + rug_fuzz_1;
        let beyond_max_i16: i16 = AsPrimitive::<i16>::as_(beyond_max_i16_as_u128);
        debug_assert_eq!(beyond_max_i16, i16::MIN);
        let value_u128_negative: u128 = u128::MAX;
        let value_i16_negative: i16 = AsPrimitive::<i16>::as_(value_u128_negative);
        debug_assert_eq!(value_i16_negative, - 1i16);
        let _rug_ed_tests_llm_16_1361_llm_16_1361_rrrruuuugggg_test_as_primitive_u128_to_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1362_llm_16_1362 {
    use crate::AsPrimitive;
    #[test]
    fn u128_as_i32() {
        let _rug_st_tests_llm_16_1362_llm_16_1362_rrrruuuugggg_u128_as_i32 = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 31;
        let value: u128 = rug_fuzz_0;
        let cast_value: i32 = value.as_();
        debug_assert_eq!(cast_value, 42i32);
        let large_value: u128 = rug_fuzz_1 << rug_fuzz_2;
        let cast_large_value: i32 = large_value.as_();
        debug_assert_eq!(cast_large_value, i32::MIN);
        let max_safe_value: u128 = i32::MAX as u128;
        let cast_max_safe_value: i32 = max_safe_value.as_();
        debug_assert_eq!(cast_max_safe_value, i32::MAX);
        let _rug_ed_tests_llm_16_1362_llm_16_1362_rrrruuuugggg_u128_as_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1363_llm_16_1363 {
    use crate::cast::AsPrimitive;
    #[test]
    fn u128_to_i64_casting() {
        let _rug_st_tests_llm_16_1363_llm_16_1363_rrrruuuugggg_u128_to_i64_casting = 0;
        let rug_fuzz_0 = 123456789;
        let val: u128 = rug_fuzz_0;
        let casted_val: i64 = AsPrimitive::<i64>::as_(val);
        debug_assert_eq!(casted_val, 123456789_i64);
        let _rug_ed_tests_llm_16_1363_llm_16_1363_rrrruuuugggg_u128_to_i64_casting = 0;
    }
    #[test]
    fn u128_to_i64_casting_edge_case() {
        let _rug_st_tests_llm_16_1363_llm_16_1363_rrrruuuugggg_u128_to_i64_casting_edge_case = 0;
        let val: u128 = i64::MAX as u128;
        let casted_val: i64 = AsPrimitive::<i64>::as_(val);
        debug_assert_eq!(casted_val, i64::MAX);
        let _rug_ed_tests_llm_16_1363_llm_16_1363_rrrruuuugggg_u128_to_i64_casting_edge_case = 0;
    }
    #[test]
    #[should_panic]
    fn u128_to_i64_casting_overflow() {
        let _rug_st_tests_llm_16_1363_llm_16_1363_rrrruuuugggg_u128_to_i64_casting_overflow = 0;
        let rug_fuzz_0 = 1;
        let val: u128 = (i64::MAX as u128) + rug_fuzz_0;
        let _casted_val: i64 = AsPrimitive::<i64>::as_(val);
        let _rug_ed_tests_llm_16_1363_llm_16_1363_rrrruuuugggg_u128_to_i64_casting_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1364_llm_16_1364 {
    use crate::cast::AsPrimitive;
    #[test]
    fn u128_as_i8() {
        let _rug_st_tests_llm_16_1364_llm_16_1364_rrrruuuugggg_u128_as_i8 = 0;
        let rug_fuzz_0 = 255;
        let val: u128 = rug_fuzz_0;
        let casted_val: i8 = AsPrimitive::<i8>::as_(val);
        debug_assert_eq!(casted_val, - 1i8);
        let _rug_ed_tests_llm_16_1364_llm_16_1364_rrrruuuugggg_u128_as_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1365_llm_16_1365 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_u128_as_isize() {
        let _rug_st_tests_llm_16_1365_llm_16_1365_rrrruuuugggg_test_u128_as_isize = 0;
        let rug_fuzz_0 = 0_u128;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = "Overflow occurred";
        let max_isize = isize::MAX as u128;
        debug_assert_eq!(AsPrimitive:: < isize > ::as_(rug_fuzz_0), 0_isize);
        debug_assert_eq!(AsPrimitive:: < isize > ::as_(max_isize), isize::MAX);
        #[cfg(target_pointer_width = "64")]
        {
            let over_isize = max_isize.checked_add(rug_fuzz_1).expect(rug_fuzz_2);
            debug_assert!(over_isize > max_isize);
        }
        let _rug_ed_tests_llm_16_1365_llm_16_1365_rrrruuuugggg_test_u128_as_isize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1366_llm_16_1366 {
    use crate::AsPrimitive;
    #[test]
    fn test_as_primitive_u128_to_u128() {
        let _rug_st_tests_llm_16_1366_llm_16_1366_rrrruuuugggg_test_as_primitive_u128_to_u128 = 0;
        let rug_fuzz_0 = 123_u128;
        let value: u128 = rug_fuzz_0;
        let result: u128 = AsPrimitive::<u128>::as_(value);
        debug_assert_eq!(value, result);
        let _rug_ed_tests_llm_16_1366_llm_16_1366_rrrruuuugggg_test_as_primitive_u128_to_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1367_llm_16_1367 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_u128_to_u16() {
        let _rug_st_tests_llm_16_1367_llm_16_1367_rrrruuuugggg_test_as_primitive_u128_to_u16 = 0;
        let rug_fuzz_0 = 65535;
        let value_u128: u128 = rug_fuzz_0;
        let value_u16: u16 = value_u128.as_();
        debug_assert_eq!(value_u16, 65535u16);
        let value_u128_big: u128 = u128::MAX;
        let value_u16_big: u16 = value_u128_big.as_();
        debug_assert_eq!(value_u16_big, u16::MAX);
        let _rug_ed_tests_llm_16_1367_llm_16_1367_rrrruuuugggg_test_as_primitive_u128_to_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1368_llm_16_1368 {
    use crate::cast::AsPrimitive;
    #[test]
    fn u128_as_u32() {
        let _rug_st_tests_llm_16_1368_llm_16_1368_rrrruuuugggg_u128_as_u32 = 0;
        let value: u128 = u128::max_value();
        let result: u32 = AsPrimitive::<u32>::as_(value);
        debug_assert_eq!(result, u32::max_value());
        let _rug_ed_tests_llm_16_1368_llm_16_1368_rrrruuuugggg_u128_as_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1369_llm_16_1369 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_u128_as_u64() {
        let _rug_st_tests_llm_16_1369_llm_16_1369_rrrruuuugggg_test_u128_as_u64 = 0;
        let val_u128: u128 = u128::max_value();
        let val_u64: u64 = val_u128.as_();
        debug_assert_eq!(val_u64, u64::max_value());
        let _rug_ed_tests_llm_16_1369_llm_16_1369_rrrruuuugggg_test_u128_as_u64 = 0;
    }
    #[test]
    fn test_as_primitive_zero() {
        let _rug_st_tests_llm_16_1369_llm_16_1369_rrrruuuugggg_test_as_primitive_zero = 0;
        let rug_fuzz_0 = 0;
        let val_u128: u128 = rug_fuzz_0;
        let val_u64: u64 = val_u128.as_();
        debug_assert_eq!(val_u64, 0u64);
        let _rug_ed_tests_llm_16_1369_llm_16_1369_rrrruuuugggg_test_as_primitive_zero = 0;
    }
    #[test]
    fn test_as_primitive_one() {
        let _rug_st_tests_llm_16_1369_llm_16_1369_rrrruuuugggg_test_as_primitive_one = 0;
        let rug_fuzz_0 = 1;
        let val_u128: u128 = rug_fuzz_0;
        let val_u64: u64 = val_u128.as_();
        debug_assert_eq!(val_u64, 1u64);
        let _rug_ed_tests_llm_16_1369_llm_16_1369_rrrruuuugggg_test_as_primitive_one = 0;
    }
    #[test]
    fn test_as_primitive_edge_case() {
        let _rug_st_tests_llm_16_1369_llm_16_1369_rrrruuuugggg_test_as_primitive_edge_case = 0;
        let val_u128: u128 = u64::max_value() as u128;
        let val_u64: u64 = val_u128.as_();
        debug_assert_eq!(val_u64, u64::max_value());
        let _rug_ed_tests_llm_16_1369_llm_16_1369_rrrruuuugggg_test_as_primitive_edge_case = 0;
    }
    #[test]
    #[should_panic]
    fn test_as_primitive_overflow() {
        let _rug_st_tests_llm_16_1369_llm_16_1369_rrrruuuugggg_test_as_primitive_overflow = 0;
        let val_u128: u128 = u128::max_value();
        let _val_u64: u64 = val_u128.as_();
        let _rug_ed_tests_llm_16_1369_llm_16_1369_rrrruuuugggg_test_as_primitive_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1370_llm_16_1370 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_u128_as_u8() {
        let _rug_st_tests_llm_16_1370_llm_16_1370_rrrruuuugggg_test_u128_as_u8 = 0;
        let rug_fuzz_0 = 256;
        let rug_fuzz_1 = 255;
        let rug_fuzz_2 = 0;
        let val: u128 = rug_fuzz_0;
        let result = <u128 as AsPrimitive<u8>>::as_(val);
        debug_assert_eq!(
            result, 0, "Casting u128::256 to u8 should overflow and wrap to 0"
        );
        let val: u128 = rug_fuzz_1;
        let result = <u128 as AsPrimitive<u8>>::as_(val);
        debug_assert_eq!(result, 255, "Casting u128::255 to u8 should yield 255");
        let val: u128 = rug_fuzz_2;
        let result = <u128 as AsPrimitive<u8>>::as_(val);
        debug_assert_eq!(result, 0, "Casting u128::0 to u8 should yield 0");
        let _rug_ed_tests_llm_16_1370_llm_16_1370_rrrruuuugggg_test_u128_as_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1371_llm_16_1371 {
    use crate::cast::AsPrimitive;
    #[test]
    fn u128_as_usize() {
        let _rug_st_tests_llm_16_1371_llm_16_1371_rrrruuuugggg_u128_as_usize = 0;
        let rug_fuzz_0 = 123;
        let value: u128 = rug_fuzz_0;
        let cast_value: usize = AsPrimitive::<usize>::as_(value);
        debug_assert_eq!(cast_value, 123 as usize);
        let _rug_ed_tests_llm_16_1371_llm_16_1371_rrrruuuugggg_u128_as_usize = 0;
    }
    #[test]
    fn u128_as_usize_max() {
        let _rug_st_tests_llm_16_1371_llm_16_1371_rrrruuuugggg_u128_as_usize_max = 0;
        let value: u128 = usize::MAX as u128;
        let cast_value: usize = AsPrimitive::<usize>::as_(value);
        debug_assert_eq!(cast_value, usize::MAX);
        let _rug_ed_tests_llm_16_1371_llm_16_1371_rrrruuuugggg_u128_as_usize_max = 0;
    }
    #[test]
    #[should_panic(expected = "attempt to cast to usize with overflow")]
    #[cfg(target_pointer_width = "64")]
    fn u128_as_usize_overflow() {
        let _rug_st_tests_llm_16_1371_llm_16_1371_rrrruuuugggg_u128_as_usize_overflow = 0;
        let rug_fuzz_0 = 1;
        let value: u128 = (usize::MAX as u128) + rug_fuzz_0;
        let _cast_value: usize = AsPrimitive::<usize>::as_(value);
        let _rug_ed_tests_llm_16_1371_llm_16_1371_rrrruuuugggg_u128_as_usize_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1372_llm_16_1372 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_f32_to_u128() {
        let _rug_st_tests_llm_16_1372_llm_16_1372_rrrruuuugggg_test_from_f32_to_u128 = 0;
        let rug_fuzz_0 = 0f32;
        let rug_fuzz_1 = 0u128;
        let inputs_and_expected = vec![
            (rug_fuzz_0, Some(rug_fuzz_1)), (1f32, Some(1u128)), (1.999f32, Some(1u128)),
            (- 1f32, None), (u128::MAX as f32, None), (f32::INFINITY, None),
            (f32::NEG_INFINITY, None), (f32::NAN, None)
        ];
        for (input, expected) in inputs_and_expected {
            let result = u128::from_f32(input);
            debug_assert_eq!(result, expected, "Testing with input: {:?}", input);
        }
        let _rug_ed_tests_llm_16_1372_llm_16_1372_rrrruuuugggg_test_from_f32_to_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1373 {
    use crate::FromPrimitive;
    #[test]
    fn test_u128_from_f64() {
        let _rug_st_tests_llm_16_1373_rrrruuuugggg_test_u128_from_f64 = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 1.0;
        let rug_fuzz_2 = 1.0;
        let rug_fuzz_3 = 2.0_f64;
        let rug_fuzz_4 = 64;
        let rug_fuzz_5 = 0.1;
        let rug_fuzz_6 = 0.1;
        debug_assert_eq!(u128::from_f64(rug_fuzz_0), Some(0));
        debug_assert_eq!(u128::from_f64(rug_fuzz_1), Some(1));
        debug_assert_eq!(u128::from_f64(- rug_fuzz_2), None);
        debug_assert_eq!(u128::from_f64(f64::MAX), None);
        debug_assert_eq!(u128::from_f64(u128::MAX as f64), None);
        debug_assert_eq!(u128::from_f64(f64::MIN), None);
        debug_assert_eq!(u128::from_f64(f64::INFINITY), None);
        debug_assert_eq!(u128::from_f64(f64::NEG_INFINITY), None);
        debug_assert_eq!(u128::from_f64(f64::NAN), None);
        debug_assert_eq!(
            u128::from_f64(rug_fuzz_3.powi(rug_fuzz_4)), Some(18446744073709551616)
        );
        debug_assert_eq!(u128::from_f64(- rug_fuzz_5), None);
        debug_assert_eq!(u128::from_f64(rug_fuzz_6), None);
        debug_assert_eq!(u128::from_f64(f64::EPSILON), None);
        let _rug_ed_tests_llm_16_1373_rrrruuuugggg_test_u128_from_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1374_llm_16_1374 {
    use crate::FromPrimitive;
    #[test]
    fn from_i128_with_positive_in_range() {
        let _rug_st_tests_llm_16_1374_llm_16_1374_rrrruuuugggg_from_i128_with_positive_in_range = 0;
        let rug_fuzz_0 = 123;
        let value: i128 = rug_fuzz_0;
        let result: Option<u128> = u128::from_i128(value);
        debug_assert_eq!(result, Some(123_u128));
        let _rug_ed_tests_llm_16_1374_llm_16_1374_rrrruuuugggg_from_i128_with_positive_in_range = 0;
    }
    #[test]
    fn from_i128_with_zero() {
        let _rug_st_tests_llm_16_1374_llm_16_1374_rrrruuuugggg_from_i128_with_zero = 0;
        let rug_fuzz_0 = 0;
        let value: i128 = rug_fuzz_0;
        let result = u128::from_i128(value);
        debug_assert_eq!(result, Some(0_u128));
        let _rug_ed_tests_llm_16_1374_llm_16_1374_rrrruuuugggg_from_i128_with_zero = 0;
    }
    #[test]
    fn from_i128_with_negative() {
        let _rug_st_tests_llm_16_1374_llm_16_1374_rrrruuuugggg_from_i128_with_negative = 0;
        let rug_fuzz_0 = 123;
        let value: i128 = -rug_fuzz_0;
        let result = u128::from_i128(value);
        debug_assert!(result.is_none());
        let _rug_ed_tests_llm_16_1374_llm_16_1374_rrrruuuugggg_from_i128_with_negative = 0;
    }
    #[test]
    fn from_i128_with_positive_out_of_range() {
        let _rug_st_tests_llm_16_1374_llm_16_1374_rrrruuuugggg_from_i128_with_positive_out_of_range = 0;
        let value: i128 = i128::max_value();
        let result = u128::from_i128(value);
        debug_assert!(result.is_none());
        let _rug_ed_tests_llm_16_1374_llm_16_1374_rrrruuuugggg_from_i128_with_positive_out_of_range = 0;
    }
    #[test]
    fn from_i128_with_negative_out_of_range() {
        let _rug_st_tests_llm_16_1374_llm_16_1374_rrrruuuugggg_from_i128_with_negative_out_of_range = 0;
        let value: i128 = i128::min_value();
        let result = u128::from_i128(value);
        debug_assert!(result.is_none());
        let _rug_ed_tests_llm_16_1374_llm_16_1374_rrrruuuugggg_from_i128_with_negative_out_of_range = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1375_llm_16_1375 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i16() {
        let _rug_st_tests_llm_16_1375_llm_16_1375_rrrruuuugggg_test_from_i16 = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        debug_assert_eq!(< u128 as FromPrimitive > ::from_i16(rug_fuzz_0), Some(0u128));
        debug_assert_eq!(< u128 as FromPrimitive > ::from_i16(rug_fuzz_1), Some(1u128));
        debug_assert_eq!(< u128 as FromPrimitive > ::from_i16(- rug_fuzz_2), None);
        debug_assert_eq!(
            < u128 as FromPrimitive > ::from_i16(i16::MAX), Some(i16::MAX as u128)
        );
        debug_assert_eq!(< u128 as FromPrimitive > ::from_i16(i16::MIN), None);
        let _rug_ed_tests_llm_16_1375_llm_16_1375_rrrruuuugggg_test_from_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1376_llm_16_1376 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i32() {
        let _rug_st_tests_llm_16_1376_llm_16_1376_rrrruuuugggg_test_from_i32 = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        debug_assert_eq!(< u128 as FromPrimitive > ::from_i32(rug_fuzz_0), Some(0u128));
        debug_assert_eq!(< u128 as FromPrimitive > ::from_i32(rug_fuzz_1), Some(1u128));
        debug_assert_eq!(< u128 as FromPrimitive > ::from_i32(- rug_fuzz_2), None);
        debug_assert_eq!(
            < u128 as FromPrimitive > ::from_i32(i32::MAX), Some(i32::MAX as u128)
        );
        debug_assert_eq!(< u128 as FromPrimitive > ::from_i32(i32::MIN), None);
        let _rug_ed_tests_llm_16_1376_llm_16_1376_rrrruuuugggg_test_from_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1377_llm_16_1377 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i64() {
        let _rug_st_tests_llm_16_1377_llm_16_1377_rrrruuuugggg_test_from_i64 = 0;
        let rug_fuzz_0 = 0_i64;
        let rug_fuzz_1 = 1_i64;
        debug_assert_eq!(< u128 as FromPrimitive > ::from_i64(rug_fuzz_0), Some(0_u128));
        debug_assert_eq!(< u128 as FromPrimitive > ::from_i64(- rug_fuzz_1), None);
        debug_assert_eq!(
            < u128 as FromPrimitive > ::from_i64(i64::MAX), Some(i64::MAX as u128)
        );
        debug_assert_eq!(< u128 as FromPrimitive > ::from_i64(i64::MIN), None);
        let _rug_ed_tests_llm_16_1377_llm_16_1377_rrrruuuugggg_test_from_i64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1378_llm_16_1378 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i8() {
        let _rug_st_tests_llm_16_1378_llm_16_1378_rrrruuuugggg_test_from_i8 = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1;
        debug_assert_eq!(< u128 as FromPrimitive > ::from_i8(- rug_fuzz_0), None);
        debug_assert_eq!(< u128 as FromPrimitive > ::from_i8(rug_fuzz_1), Some(0u128));
        debug_assert_eq!(< u128 as FromPrimitive > ::from_i8(rug_fuzz_2), Some(1u128));
        debug_assert_eq!(< u128 as FromPrimitive > ::from_i8(i8::MAX), Some(127u128));
        let _rug_ed_tests_llm_16_1378_llm_16_1378_rrrruuuugggg_test_from_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1379_llm_16_1379 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_isize() {
        let _rug_st_tests_llm_16_1379_llm_16_1379_rrrruuuugggg_test_from_isize = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 1;
        debug_assert_eq!(< u128 as FromPrimitive > ::from_isize(- rug_fuzz_0), None);
        debug_assert_eq!(
            < u128 as FromPrimitive > ::from_isize(rug_fuzz_1), Some(0u128)
        );
        debug_assert_eq!(
            < u128 as FromPrimitive > ::from_isize(rug_fuzz_2), Some(1u128)
        );
        debug_assert_eq!(
            < u128 as FromPrimitive > ::from_isize(isize::MAX), Some(isize::MAX as u128)
        );
        let _rug_ed_tests_llm_16_1379_llm_16_1379_rrrruuuugggg_test_from_isize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1380_llm_16_1380 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u128_in_bounds() {
        let _rug_st_tests_llm_16_1380_llm_16_1380_rrrruuuugggg_test_from_u128_in_bounds = 0;
        let rug_fuzz_0 = 255_u128;
        let rug_fuzz_1 = 65_535_u128;
        let rug_fuzz_2 = 4_294_967_295_u128;
        let rug_fuzz_3 = 18_446_744_073_709_551_615_u128;
        debug_assert_eq!(< u8 as FromPrimitive > ::from_u128(rug_fuzz_0), Some(255_u8));
        debug_assert_eq!(
            < u16 as FromPrimitive > ::from_u128(rug_fuzz_1), Some(65_535_u16)
        );
        debug_assert_eq!(
            < u32 as FromPrimitive > ::from_u128(rug_fuzz_2), Some(4_294_967_295_u32)
        );
        debug_assert_eq!(
            < u64 as FromPrimitive > ::from_u128(rug_fuzz_3),
            Some(18_446_744_073_709_551_615_u64)
        );
        debug_assert_eq!(
            < u128 as FromPrimitive > ::from_u128(u128::MAX), Some(u128::MAX)
        );
        debug_assert_eq!(
            < usize as FromPrimitive > ::from_u128(usize::MAX as u128), Some(usize::MAX)
        );
        let _rug_ed_tests_llm_16_1380_llm_16_1380_rrrruuuugggg_test_from_u128_in_bounds = 0;
    }
    #[test]
    fn test_from_u128_out_of_bounds() {
        let _rug_st_tests_llm_16_1380_llm_16_1380_rrrruuuugggg_test_from_u128_out_of_bounds = 0;
        let rug_fuzz_0 = 256_u128;
        let rug_fuzz_1 = 65_536_u128;
        let rug_fuzz_2 = 4_294_967_296_u128;
        let rug_fuzz_3 = 18_446_744_073_709_551_616_u128;
        let rug_fuzz_4 = 1;
        debug_assert_eq!(< u8 as FromPrimitive > ::from_u128(rug_fuzz_0), None);
        debug_assert_eq!(< u16 as FromPrimitive > ::from_u128(rug_fuzz_1), None);
        debug_assert_eq!(< u32 as FromPrimitive > ::from_u128(rug_fuzz_2), None);
        debug_assert_eq!(< u64 as FromPrimitive > ::from_u128(rug_fuzz_3), None);
        debug_assert_eq!(
            < usize as FromPrimitive > ::from_u128((usize::MAX as u128) + rug_fuzz_4),
            None
        );
        let _rug_ed_tests_llm_16_1380_llm_16_1380_rrrruuuugggg_test_from_u128_out_of_bounds = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1381_llm_16_1381 {
    use crate::cast::FromPrimitive;
    use crate::cast;
    #[test]
    fn test_from_u16() {
        let _rug_st_tests_llm_16_1381_llm_16_1381_rrrruuuugggg_test_from_u16 = 0;
        let rug_fuzz_0 = 0u16;
        let rug_fuzz_1 = 1u16;
        debug_assert_eq!(< u128 as FromPrimitive > ::from_u16(rug_fuzz_0), Some(0u128));
        debug_assert_eq!(< u128 as FromPrimitive > ::from_u16(rug_fuzz_1), Some(1u128));
        debug_assert_eq!(
            < u128 as FromPrimitive > ::from_u16(u16::MAX), Some(u16::MAX as u128)
        );
        let _rug_ed_tests_llm_16_1381_llm_16_1381_rrrruuuugggg_test_from_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1382_llm_16_1382 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u32() {
        let _rug_st_tests_llm_16_1382_llm_16_1382_rrrruuuugggg_test_from_u32 = 0;
        let rug_fuzz_0 = 0u32;
        debug_assert_eq!(< u128 as FromPrimitive > ::from_u32(rug_fuzz_0), Some(0u128));
        debug_assert_eq!(
            < u128 as FromPrimitive > ::from_u32(u32::MAX), Some(u32::MAX as u128)
        );
        debug_assert_eq!(
            < u128 as FromPrimitive > ::from_u32(u32::MIN), Some(u32::MIN as u128)
        );
        let _rug_ed_tests_llm_16_1382_llm_16_1382_rrrruuuugggg_test_from_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1383_llm_16_1383 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_u128_from_u64() {
        let _rug_st_tests_llm_16_1383_llm_16_1383_rrrruuuugggg_test_u128_from_u64 = 0;
        let rug_fuzz_0 = 0_u64;
        let rug_fuzz_1 = 123_u64;
        debug_assert_eq!(< u128 as FromPrimitive > ::from_u64(rug_fuzz_0), Some(0_u128));
        debug_assert_eq!(
            < u128 as FromPrimitive > ::from_u64(rug_fuzz_1), Some(123_u128)
        );
        debug_assert_eq!(
            < u128 as FromPrimitive > ::from_u64(u64::MAX), Some(u128::from(u64::MAX))
        );
        let _rug_ed_tests_llm_16_1383_llm_16_1383_rrrruuuugggg_test_u128_from_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1384_llm_16_1384 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u8() {
        let _rug_st_tests_llm_16_1384_llm_16_1384_rrrruuuugggg_test_from_u8 = 0;
        let rug_fuzz_0 = 0u8;
        let rug_fuzz_1 = 255u8;
        let rug_fuzz_2 = 128u8;
        debug_assert_eq!(< u128 as FromPrimitive > ::from_u8(rug_fuzz_0), Some(0u128));
        debug_assert_eq!(< u128 as FromPrimitive > ::from_u8(rug_fuzz_1), Some(255u128));
        debug_assert_eq!(< u128 as FromPrimitive > ::from_u8(rug_fuzz_2), Some(128u128));
        let _rug_ed_tests_llm_16_1384_llm_16_1384_rrrruuuugggg_test_from_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1385_llm_16_1385 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_usize() {
        let _rug_st_tests_llm_16_1385_llm_16_1385_rrrruuuugggg_test_from_usize = 0;
        let rug_fuzz_0 = 123456789;
        let rug_fuzz_1 = 0;
        let max_value = u128::MAX as usize;
        let value_within_range: usize = rug_fuzz_0;
        let result_within_range = <u128 as FromPrimitive>::from_usize(
            value_within_range,
        );
        debug_assert_eq!(result_within_range, Some(123456789u128));
        let zero_value: usize = rug_fuzz_1;
        let result_zero_value = <u128 as FromPrimitive>::from_usize(zero_value);
        debug_assert_eq!(result_zero_value, Some(0u128));
        let result_max_value = <u128 as FromPrimitive>::from_usize(max_value);
        if max_value as u128 == u128::MAX {
            debug_assert_eq!(result_max_value, Some(u128::MAX));
        } else {
            debug_assert!(max_value < u128::MAX as usize);
            debug_assert_eq!(result_max_value, Some(max_value as u128));
        }
        let _rug_ed_tests_llm_16_1385_llm_16_1385_rrrruuuugggg_test_from_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1386_llm_16_1386 {
    use crate::cast::{NumCast, ToPrimitive};
    use std::num::Wrapping;
    #[test]
    fn test_from_u128_wrapping() {
        let _rug_st_tests_llm_16_1386_llm_16_1386_rrrruuuugggg_test_from_u128_wrapping = 0;
        let rug_fuzz_0 = 123_u128;
        let wrapped_source = Wrapping(rug_fuzz_0);
        let result: Option<Wrapping<u128>> = wrapped_source.to_u128().map(Wrapping);
        debug_assert_eq!(result, Some(Wrapping(123_u128)));
        let _rug_ed_tests_llm_16_1386_llm_16_1386_rrrruuuugggg_test_from_u128_wrapping = 0;
    }
    #[test]
    fn test_from_i64_wrapping() {
        let _rug_st_tests_llm_16_1386_llm_16_1386_rrrruuuugggg_test_from_i64_wrapping = 0;
        let rug_fuzz_0 = 123_i64;
        let wrapped_source = Wrapping(-rug_fuzz_0);
        let result: Option<Wrapping<u128>> = wrapped_source.to_u128().map(Wrapping);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1386_llm_16_1386_rrrruuuugggg_test_from_i64_wrapping = 0;
    }
    #[test]
    fn test_from_u64_wrapping() {
        let _rug_st_tests_llm_16_1386_llm_16_1386_rrrruuuugggg_test_from_u64_wrapping = 0;
        let wrapped_source = Wrapping(u64::MAX);
        let result: Option<Wrapping<u128>> = wrapped_source.to_u128().map(Wrapping);
        debug_assert_eq!(result, Some(Wrapping(u64::MAX as u128)));
        let _rug_ed_tests_llm_16_1386_llm_16_1386_rrrruuuugggg_test_from_u64_wrapping = 0;
    }
    #[test]
    fn test_from_f64_wrapping() {
        let _rug_st_tests_llm_16_1386_llm_16_1386_rrrruuuugggg_test_from_f64_wrapping = 0;
        let rug_fuzz_0 = 12.34_f64;
        let wrapped_source = Wrapping(-rug_fuzz_0);
        let result: Option<Wrapping<u128>> = wrapped_source.to_u128().map(Wrapping);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1386_llm_16_1386_rrrruuuugggg_test_from_f64_wrapping = 0;
    }
    #[test]
    fn test_from_f64_wrapping_positive() {
        let _rug_st_tests_llm_16_1386_llm_16_1386_rrrruuuugggg_test_from_f64_wrapping_positive = 0;
        let rug_fuzz_0 = 12.34_f64;
        let wrapped_source = Wrapping(rug_fuzz_0);
        let result: Option<Wrapping<u128>> = wrapped_source.to_u128().map(Wrapping);
        debug_assert!(matches!(result, Some(Wrapping(12_u128))));
        let _rug_ed_tests_llm_16_1386_llm_16_1386_rrrruuuugggg_test_from_f64_wrapping_positive = 0;
    }
    #[test]
    fn test_from_f64_wrapping_edge_case() {
        let _rug_st_tests_llm_16_1386_llm_16_1386_rrrruuuugggg_test_from_f64_wrapping_edge_case = 0;
        let wrapped_source = Wrapping(f64::MAX);
        let result: Option<Wrapping<u128>> = wrapped_source.to_u128().map(Wrapping);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1386_llm_16_1386_rrrruuuugggg_test_from_f64_wrapping_edge_case = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1387 {
    use crate::ToPrimitive;
    #[test]
    fn u128_to_f32() {
        let _rug_st_tests_llm_16_1387_rrrruuuugggg_u128_to_f32 = 0;
        let rug_fuzz_0 = 123456789012345678901234567890u128;
        let val: u128 = rug_fuzz_0;
        let float_opt: Option<f32> = val.to_f32();
        debug_assert_eq!(float_opt, Some(val as f32));
        let _rug_ed_tests_llm_16_1387_rrrruuuugggg_u128_to_f32 = 0;
    }
    #[test]
    fn u128_to_f32_max_value() {
        let _rug_st_tests_llm_16_1387_rrrruuuugggg_u128_to_f32_max_value = 0;
        let val: u128 = u128::MAX;
        let float_opt: Option<f32> = val.to_f32();
        debug_assert!(float_opt.is_some());
        debug_assert!(float_opt.unwrap().is_infinite());
        let _rug_ed_tests_llm_16_1387_rrrruuuugggg_u128_to_f32_max_value = 0;
    }
    #[test]
    fn u128_to_f32_zero() {
        let _rug_st_tests_llm_16_1387_rrrruuuugggg_u128_to_f32_zero = 0;
        let rug_fuzz_0 = 0;
        let val: u128 = rug_fuzz_0;
        let float_opt: Option<f32> = val.to_f32();
        debug_assert_eq!(float_opt, Some(0.0));
        let _rug_ed_tests_llm_16_1387_rrrruuuugggg_u128_to_f32_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1388 {
    use super::*;
    use crate::*;
    #[test]
    fn u128_to_f64_test() {
        let _rug_st_tests_llm_16_1388_rrrruuuugggg_u128_to_f64_test = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 2;
        debug_assert_eq!(
            < u128 as cast::ToPrimitive > ::to_f64(& rug_fuzz_0), Some(0.0f64)
        );
        debug_assert_eq!(
            < u128 as cast::ToPrimitive > ::to_f64(& rug_fuzz_1), Some(1.0f64)
        );
        debug_assert_eq!(
            < u128 as cast::ToPrimitive > ::to_f64(& u128::MAX), Some(u128::MAX as f64)
        );
        let large_value = u128::MAX / rug_fuzz_2;
        let large_value_f64 = large_value as f64;
        let converted_value = <u128 as cast::ToPrimitive>::to_f64(&large_value);
        debug_assert!(matches!(converted_value, Some(v) if v == large_value_f64));
        let _rug_ed_tests_llm_16_1388_rrrruuuugggg_u128_to_f64_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1389 {
    use super::*;
    use crate::*;
    #[test]
    fn u128_to_i128_within_range() {
        let _rug_st_tests_llm_16_1389_rrrruuuugggg_u128_to_i128_within_range = 0;
        let x: u128 = i128::MAX as u128;
        debug_assert_eq!(x.to_i128(), Some(i128::MAX));
        let _rug_ed_tests_llm_16_1389_rrrruuuugggg_u128_to_i128_within_range = 0;
    }
    #[test]
    fn u128_to_i128_out_of_range() {
        let _rug_st_tests_llm_16_1389_rrrruuuugggg_u128_to_i128_out_of_range = 0;
        let rug_fuzz_0 = 1;
        let x: u128 = (i128::MAX as u128).wrapping_add(rug_fuzz_0);
        debug_assert_eq!(x.to_i128(), None);
        let _rug_ed_tests_llm_16_1389_rrrruuuugggg_u128_to_i128_out_of_range = 0;
    }
    #[test]
    fn u128_to_i128_zero() {
        let _rug_st_tests_llm_16_1389_rrrruuuugggg_u128_to_i128_zero = 0;
        let rug_fuzz_0 = 0;
        let x: u128 = rug_fuzz_0;
        debug_assert_eq!(x.to_i128(), Some(0));
        let _rug_ed_tests_llm_16_1389_rrrruuuugggg_u128_to_i128_zero = 0;
    }
    #[test]
    fn u128_to_i128_edge_case() {
        let _rug_st_tests_llm_16_1389_rrrruuuugggg_u128_to_i128_edge_case = 0;
        let x: u128 = i128::MAX as u128;
        debug_assert_eq!(x.to_i128(), Some(i128::MAX));
        let _rug_ed_tests_llm_16_1389_rrrruuuugggg_u128_to_i128_edge_case = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1390_llm_16_1390 {
    use crate::ToPrimitive;
    #[test]
    fn u128_to_i16_max_value() {
        let _rug_st_tests_llm_16_1390_llm_16_1390_rrrruuuugggg_u128_to_i16_max_value = 0;
        let max_u128_within_i16 = u128::from(i16::MAX as u128);
        debug_assert_eq!(max_u128_within_i16.to_i16(), Some(i16::MAX));
        let _rug_ed_tests_llm_16_1390_llm_16_1390_rrrruuuugggg_u128_to_i16_max_value = 0;
    }
    #[test]
    fn u128_to_i16_within_bounds() {
        let _rug_st_tests_llm_16_1390_llm_16_1390_rrrruuuugggg_u128_to_i16_within_bounds = 0;
        let rug_fuzz_0 = 32767_u128;
        let value = rug_fuzz_0;
        debug_assert_eq!(value.to_i16(), Some(32767_i16));
        let _rug_ed_tests_llm_16_1390_llm_16_1390_rrrruuuugggg_u128_to_i16_within_bounds = 0;
    }
    #[test]
    fn u128_to_i16_below_zero() {
        let _rug_st_tests_llm_16_1390_llm_16_1390_rrrruuuugggg_u128_to_i16_below_zero = 0;
        let rug_fuzz_0 = 0_u128;
        let value = rug_fuzz_0;
        debug_assert_eq!(value.to_i16(), Some(0_i16));
        let _rug_ed_tests_llm_16_1390_llm_16_1390_rrrruuuugggg_u128_to_i16_below_zero = 0;
    }
    #[test]
    fn u128_to_i16_above_max() {
        let _rug_st_tests_llm_16_1390_llm_16_1390_rrrruuuugggg_u128_to_i16_above_max = 0;
        let rug_fuzz_0 = 1;
        let value = u128::from(i16::MAX as u128) + rug_fuzz_0;
        debug_assert_eq!(value.to_i16(), None);
        let _rug_ed_tests_llm_16_1390_llm_16_1390_rrrruuuugggg_u128_to_i16_above_max = 0;
    }
    #[test]
    fn u128_to_i16_much_above_max() {
        let _rug_st_tests_llm_16_1390_llm_16_1390_rrrruuuugggg_u128_to_i16_much_above_max = 0;
        let value = u128::MAX;
        debug_assert_eq!(value.to_i16(), None);
        let _rug_ed_tests_llm_16_1390_llm_16_1390_rrrruuuugggg_u128_to_i16_much_above_max = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1391_llm_16_1391 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_u128_to_i32() {
        let _rug_st_tests_llm_16_1391_llm_16_1391_rrrruuuugggg_test_u128_to_i32 = 0;
        let rug_fuzz_0 = 0u128;
        let rug_fuzz_1 = 1u128;
        let rug_fuzz_2 = 1;
        debug_assert_eq!((rug_fuzz_0).to_i32(), Some(0i32));
        debug_assert_eq!((rug_fuzz_1).to_i32(), Some(1i32));
        debug_assert_eq!((i32::MAX as u128).to_i32(), Some(i32::MAX));
        debug_assert_eq!(((i32::MAX as u128) + rug_fuzz_2).to_i32(), None);
        debug_assert_eq!((u128::MAX).to_i32(), None);
        let _rug_ed_tests_llm_16_1391_llm_16_1391_rrrruuuugggg_test_u128_to_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1392 {
    use super::*;
    use crate::*;
    #[test]
    fn u128_to_i64_max_value() {
        let _rug_st_tests_llm_16_1392_rrrruuuugggg_u128_to_i64_max_value = 0;
        let value: u128 = i64::MAX as u128;
        let result = value.to_i64();
        debug_assert_eq!(result, Some(i64::MAX));
        let _rug_ed_tests_llm_16_1392_rrrruuuugggg_u128_to_i64_max_value = 0;
    }
    #[test]
    fn u128_to_i64_min_value() {
        let _rug_st_tests_llm_16_1392_rrrruuuugggg_u128_to_i64_min_value = 0;
        let rug_fuzz_0 = 0;
        let value: u128 = rug_fuzz_0;
        let result = value.to_i64();
        debug_assert_eq!(result, Some(0));
        let _rug_ed_tests_llm_16_1392_rrrruuuugggg_u128_to_i64_min_value = 0;
    }
    #[test]
    fn u128_to_i64_above_max() {
        let _rug_st_tests_llm_16_1392_rrrruuuugggg_u128_to_i64_above_max = 0;
        let rug_fuzz_0 = 1;
        let value: u128 = (i64::MAX as u128) + rug_fuzz_0;
        let result = value.to_i64();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1392_rrrruuuugggg_u128_to_i64_above_max = 0;
    }
    #[test]
    fn u128_to_i64_extreme_value() {
        let _rug_st_tests_llm_16_1392_rrrruuuugggg_u128_to_i64_extreme_value = 0;
        let value: u128 = u128::MAX;
        let result = value.to_i64();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1392_rrrruuuugggg_u128_to_i64_extreme_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1393_llm_16_1393 {
    use crate::cast::ToPrimitive;
    #[test]
    fn u128_to_i8_within_bounds() {
        let _rug_st_tests_llm_16_1393_llm_16_1393_rrrruuuugggg_u128_to_i8_within_bounds = 0;
        let rug_fuzz_0 = 127;
        let value: u128 = rug_fuzz_0;
        let result = value.to_i8();
        debug_assert_eq!(result, Some(127i8));
        let _rug_ed_tests_llm_16_1393_llm_16_1393_rrrruuuugggg_u128_to_i8_within_bounds = 0;
    }
    #[test]
    fn u128_to_i8_exceeds_bounds() {
        let _rug_st_tests_llm_16_1393_llm_16_1393_rrrruuuugggg_u128_to_i8_exceeds_bounds = 0;
        let rug_fuzz_0 = 128;
        let value: u128 = rug_fuzz_0;
        let result = value.to_i8();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1393_llm_16_1393_rrrruuuugggg_u128_to_i8_exceeds_bounds = 0;
    }
    #[test]
    fn u128_to_i8_negative() {
        let _rug_st_tests_llm_16_1393_llm_16_1393_rrrruuuugggg_u128_to_i8_negative = 0;
        let value: u128 = u128::MAX;
        let result = value.to_i8();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1393_llm_16_1393_rrrruuuugggg_u128_to_i8_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1394_llm_16_1394 {
    use crate::cast::ToPrimitive;
    #[test]
    fn u128_to_isize_in_range() {
        let _rug_st_tests_llm_16_1394_llm_16_1394_rrrruuuugggg_u128_to_isize_in_range = 0;
        let value: u128 = isize::MAX as u128;
        let result = value.to_isize();
        debug_assert_eq!(result, Some(isize::MAX));
        let _rug_ed_tests_llm_16_1394_llm_16_1394_rrrruuuugggg_u128_to_isize_in_range = 0;
    }
    #[test]
    fn u128_to_isize_out_of_range() {
        let _rug_st_tests_llm_16_1394_llm_16_1394_rrrruuuugggg_u128_to_isize_out_of_range = 0;
        let rug_fuzz_0 = 1;
        let value: u128 = (isize::MAX as u128).wrapping_add(rug_fuzz_0);
        let result = value.to_isize();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1394_llm_16_1394_rrrruuuugggg_u128_to_isize_out_of_range = 0;
    }
    #[test]
    fn u128_to_isize_zero() {
        let _rug_st_tests_llm_16_1394_llm_16_1394_rrrruuuugggg_u128_to_isize_zero = 0;
        let rug_fuzz_0 = 0;
        let value: u128 = rug_fuzz_0;
        let result = value.to_isize();
        debug_assert_eq!(result, Some(0));
        let _rug_ed_tests_llm_16_1394_llm_16_1394_rrrruuuugggg_u128_to_isize_zero = 0;
    }
    #[test]
    fn u128_to_isize_small_value() {
        let _rug_st_tests_llm_16_1394_llm_16_1394_rrrruuuugggg_u128_to_isize_small_value = 0;
        let rug_fuzz_0 = 42;
        let value: u128 = rug_fuzz_0;
        let result = value.to_isize();
        debug_assert_eq!(result, Some(42));
        let _rug_ed_tests_llm_16_1394_llm_16_1394_rrrruuuugggg_u128_to_isize_small_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1395_llm_16_1395 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_u128_within_bounds() {
        let _rug_st_tests_llm_16_1395_llm_16_1395_rrrruuuugggg_test_to_u128_within_bounds = 0;
        let rug_fuzz_0 = 12345678901234567890;
        let val: u128 = rug_fuzz_0;
        debug_assert_eq!(val.to_u128(), Some(val));
        let _rug_ed_tests_llm_16_1395_llm_16_1395_rrrruuuugggg_test_to_u128_within_bounds = 0;
    }
    #[test]
    fn test_to_u128_at_bounds() {
        let _rug_st_tests_llm_16_1395_llm_16_1395_rrrruuuugggg_test_to_u128_at_bounds = 0;
        let val: u128 = u128::MAX;
        debug_assert_eq!(val.to_u128(), Some(val));
        let _rug_ed_tests_llm_16_1395_llm_16_1395_rrrruuuugggg_test_to_u128_at_bounds = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1396_llm_16_1396 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_u128_to_u16_within_bounds() {
        let _rug_st_tests_llm_16_1396_llm_16_1396_rrrruuuugggg_test_u128_to_u16_within_bounds = 0;
        let value: u128 = u16::MAX as u128;
        let result = value.to_u16();
        debug_assert_eq!(result, Some(u16::MAX));
        let _rug_ed_tests_llm_16_1396_llm_16_1396_rrrruuuugggg_test_u128_to_u16_within_bounds = 0;
    }
    #[test]
    fn test_u128_to_u16_below_bounds() {
        let _rug_st_tests_llm_16_1396_llm_16_1396_rrrruuuugggg_test_u128_to_u16_below_bounds = 0;
        let rug_fuzz_0 = 0;
        let value: u128 = rug_fuzz_0;
        let result = value.to_u16();
        debug_assert_eq!(result, Some(0));
        let _rug_ed_tests_llm_16_1396_llm_16_1396_rrrruuuugggg_test_u128_to_u16_below_bounds = 0;
    }
    #[test]
    fn test_u128_to_u16_above_bounds() {
        let _rug_st_tests_llm_16_1396_llm_16_1396_rrrruuuugggg_test_u128_to_u16_above_bounds = 0;
        let rug_fuzz_0 = 1;
        let value: u128 = (u16::MAX as u128) + rug_fuzz_0;
        let result = value.to_u16();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1396_llm_16_1396_rrrruuuugggg_test_u128_to_u16_above_bounds = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1397_llm_16_1397 {
    use crate::cast::ToPrimitive;
    #[test]
    fn u128_to_u32_within_range() {
        let _rug_st_tests_llm_16_1397_llm_16_1397_rrrruuuugggg_u128_to_u32_within_range = 0;
        let val: u128 = u32::MAX as u128;
        debug_assert_eq!(val.to_u32(), Some(u32::MAX));
        let _rug_ed_tests_llm_16_1397_llm_16_1397_rrrruuuugggg_u128_to_u32_within_range = 0;
    }
    #[test]
    fn u128_to_u32_exceeding_range() {
        let _rug_st_tests_llm_16_1397_llm_16_1397_rrrruuuugggg_u128_to_u32_exceeding_range = 0;
        let rug_fuzz_0 = 1;
        let val: u128 = (u32::MAX as u128) + rug_fuzz_0;
        debug_assert_eq!(val.to_u32(), None);
        let _rug_ed_tests_llm_16_1397_llm_16_1397_rrrruuuugggg_u128_to_u32_exceeding_range = 0;
    }
    #[test]
    fn u128_to_u32_zero() {
        let _rug_st_tests_llm_16_1397_llm_16_1397_rrrruuuugggg_u128_to_u32_zero = 0;
        let rug_fuzz_0 = 0;
        let val: u128 = rug_fuzz_0;
        debug_assert_eq!(val.to_u32(), Some(0));
        let _rug_ed_tests_llm_16_1397_llm_16_1397_rrrruuuugggg_u128_to_u32_zero = 0;
    }
    #[test]
    fn u128_to_u32_small_value() {
        let _rug_st_tests_llm_16_1397_llm_16_1397_rrrruuuugggg_u128_to_u32_small_value = 0;
        let rug_fuzz_0 = 123;
        let val: u128 = rug_fuzz_0;
        debug_assert_eq!(val.to_u32(), Some(123));
        let _rug_ed_tests_llm_16_1397_llm_16_1397_rrrruuuugggg_u128_to_u32_small_value = 0;
    }
    #[test]
    fn u128_to_u32_exact_u32_max() {
        let _rug_st_tests_llm_16_1397_llm_16_1397_rrrruuuugggg_u128_to_u32_exact_u32_max = 0;
        let val: u128 = u32::MAX as u128;
        debug_assert_eq!(val.to_u32(), Some(u32::MAX));
        let _rug_ed_tests_llm_16_1397_llm_16_1397_rrrruuuugggg_u128_to_u32_exact_u32_max = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1398 {
    use crate::ToPrimitive;
    #[test]
    fn u128_to_u64_max_value() {
        let _rug_st_tests_llm_16_1398_rrrruuuugggg_u128_to_u64_max_value = 0;
        let value: u128 = u64::MAX as u128;
        debug_assert_eq!(value.to_u64(), Some(u64::MAX));
        let _rug_ed_tests_llm_16_1398_rrrruuuugggg_u128_to_u64_max_value = 0;
    }
    #[test]
    fn u128_to_u64_within_bounds() {
        let _rug_st_tests_llm_16_1398_rrrruuuugggg_u128_to_u64_within_bounds = 0;
        let rug_fuzz_0 = 1234567890;
        let value: u128 = rug_fuzz_0;
        debug_assert_eq!(value.to_u64(), Some(1234567890));
        let _rug_ed_tests_llm_16_1398_rrrruuuugggg_u128_to_u64_within_bounds = 0;
    }
    #[test]
    fn u128_to_u64_out_of_bounds() {
        let _rug_st_tests_llm_16_1398_rrrruuuugggg_u128_to_u64_out_of_bounds = 0;
        let value: u128 = u128::MAX;
        debug_assert_eq!(value.to_u64(), None);
        let _rug_ed_tests_llm_16_1398_rrrruuuugggg_u128_to_u64_out_of_bounds = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1399_llm_16_1399 {
    use crate::cast::ToPrimitive;
    #[test]
    fn to_u8_max_value() {
        let _rug_st_tests_llm_16_1399_llm_16_1399_rrrruuuugggg_to_u8_max_value = 0;
        let max_u128: u128 = u8::MAX.into();
        debug_assert_eq!(max_u128.to_u8(), Some(u8::MAX));
        let _rug_ed_tests_llm_16_1399_llm_16_1399_rrrruuuugggg_to_u8_max_value = 0;
    }
    #[test]
    fn to_u8_within_bounds() {
        let _rug_st_tests_llm_16_1399_llm_16_1399_rrrruuuugggg_to_u8_within_bounds = 0;
        let rug_fuzz_0 = 100;
        let value: u128 = rug_fuzz_0;
        debug_assert_eq!(value.to_u8(), Some(100_u8));
        let _rug_ed_tests_llm_16_1399_llm_16_1399_rrrruuuugggg_to_u8_within_bounds = 0;
    }
    #[test]
    fn to_u8_above_bounds() {
        let _rug_st_tests_llm_16_1399_llm_16_1399_rrrruuuugggg_to_u8_above_bounds = 0;
        let rug_fuzz_0 = 1;
        let value: u128 = u8::MAX as u128 + rug_fuzz_0;
        debug_assert_eq!(value.to_u8(), None);
        let _rug_ed_tests_llm_16_1399_llm_16_1399_rrrruuuugggg_to_u8_above_bounds = 0;
    }
    #[test]
    fn to_u8_zero() {
        let _rug_st_tests_llm_16_1399_llm_16_1399_rrrruuuugggg_to_u8_zero = 0;
        let rug_fuzz_0 = 0;
        let value: u128 = rug_fuzz_0;
        debug_assert_eq!(value.to_u8(), Some(0_u8));
        let _rug_ed_tests_llm_16_1399_llm_16_1399_rrrruuuugggg_to_u8_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1400_llm_16_1400 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_usize_within_bounds() {
        let _rug_st_tests_llm_16_1400_llm_16_1400_rrrruuuugggg_test_to_usize_within_bounds = 0;
        let rug_fuzz_0 = 42;
        let value: u128 = rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_usize(& value), Some(42usize));
        let _rug_ed_tests_llm_16_1400_llm_16_1400_rrrruuuugggg_test_to_usize_within_bounds = 0;
    }
    #[test]
    fn test_to_usize_at_usize_max() {
        let _rug_st_tests_llm_16_1400_llm_16_1400_rrrruuuugggg_test_to_usize_at_usize_max = 0;
        let value: u128 = usize::MAX as u128;
        debug_assert_eq!(ToPrimitive::to_usize(& value), Some(usize::MAX));
        let _rug_ed_tests_llm_16_1400_llm_16_1400_rrrruuuugggg_test_to_usize_at_usize_max = 0;
    }
    #[test]
    fn test_to_usize_exceeds_usize_max() {
        let _rug_st_tests_llm_16_1400_llm_16_1400_rrrruuuugggg_test_to_usize_exceeds_usize_max = 0;
        let rug_fuzz_0 = 1;
        let value: u128 = (usize::MAX as u128) + rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_usize(& value), None);
        let _rug_ed_tests_llm_16_1400_llm_16_1400_rrrruuuugggg_test_to_usize_exceeds_usize_max = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1463 {
    use crate::AsPrimitive;
    #[test]
    fn u16_as_f32() {
        let _rug_st_tests_llm_16_1463_rrrruuuugggg_u16_as_f32 = 0;
        let rug_fuzz_0 = 42;
        let x: u16 = rug_fuzz_0;
        let y: f32 = AsPrimitive::<f32>::as_(x);
        debug_assert_eq!(y, 42.0_f32);
        let _rug_ed_tests_llm_16_1463_rrrruuuugggg_u16_as_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1464_llm_16_1464 {
    use crate::cast::AsPrimitive;
    #[test]
    fn u16_as_f64() {
        let _rug_st_tests_llm_16_1464_llm_16_1464_rrrruuuugggg_u16_as_f64 = 0;
        let rug_fuzz_0 = 42;
        let value: u16 = rug_fuzz_0;
        let result: f64 = AsPrimitive::<f64>::as_(value);
        debug_assert_eq!(result, 42.0f64);
        let _rug_ed_tests_llm_16_1464_llm_16_1464_rrrruuuugggg_u16_as_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1465_llm_16_1465 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_u16_as_i128() {
        let _rug_st_tests_llm_16_1465_llm_16_1465_rrrruuuugggg_test_u16_as_i128 = 0;
        let rug_fuzz_0 = 42;
        let value: u16 = rug_fuzz_0;
        let casted_value: i128 = AsPrimitive::<i128>::as_(value);
        debug_assert_eq!(casted_value, 42i128);
        let _rug_ed_tests_llm_16_1465_llm_16_1465_rrrruuuugggg_test_u16_as_i128 = 0;
    }
    #[test]
    fn test_u16_as_i128_max() {
        let _rug_st_tests_llm_16_1465_llm_16_1465_rrrruuuugggg_test_u16_as_i128_max = 0;
        let value: u16 = u16::MAX;
        let casted_value: i128 = AsPrimitive::<i128>::as_(value);
        debug_assert_eq!(casted_value, u16::MAX as i128);
        let _rug_ed_tests_llm_16_1465_llm_16_1465_rrrruuuugggg_test_u16_as_i128_max = 0;
    }
    #[test]
    fn test_u16_as_i128_zero() {
        let _rug_st_tests_llm_16_1465_llm_16_1465_rrrruuuugggg_test_u16_as_i128_zero = 0;
        let rug_fuzz_0 = 0;
        let value: u16 = rug_fuzz_0;
        let casted_value: i128 = AsPrimitive::<i128>::as_(value);
        debug_assert_eq!(casted_value, 0i128);
        let _rug_ed_tests_llm_16_1465_llm_16_1465_rrrruuuugggg_test_u16_as_i128_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1466_llm_16_1466 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_u16_as_i16() {
        let _rug_st_tests_llm_16_1466_llm_16_1466_rrrruuuugggg_test_u16_as_i16 = 0;
        let rug_fuzz_0 = 42;
        let u16_val: u16 = rug_fuzz_0;
        let i16_val: i16 = u16_val.as_();
        debug_assert_eq!(i16_val, 42i16);
        let u16_val: u16 = u16::MAX;
        let i16_val: i16 = u16_val.as_();
        debug_assert_eq!(i16_val as u16, u16_val);
        let _rug_ed_tests_llm_16_1466_llm_16_1466_rrrruuuugggg_test_u16_as_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1467_llm_16_1467 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_u16_to_i32() {
        let _rug_st_tests_llm_16_1467_llm_16_1467_rrrruuuugggg_test_as_primitive_u16_to_i32 = 0;
        let rug_fuzz_0 = 42;
        let value_u16: u16 = rug_fuzz_0;
        let value_i32: i32 = AsPrimitive::<i32>::as_(value_u16);
        debug_assert_eq!(value_i32, 42i32);
        let _rug_ed_tests_llm_16_1467_llm_16_1467_rrrruuuugggg_test_as_primitive_u16_to_i32 = 0;
    }
    #[test]
    fn test_as_primitive_u16_to_i32_max() {
        let _rug_st_tests_llm_16_1467_llm_16_1467_rrrruuuugggg_test_as_primitive_u16_to_i32_max = 0;
        let value_u16: u16 = u16::MAX;
        let value_i32: i32 = AsPrimitive::<i32>::as_(value_u16);
        debug_assert_eq!(value_i32, u16::MAX as i32);
        let _rug_ed_tests_llm_16_1467_llm_16_1467_rrrruuuugggg_test_as_primitive_u16_to_i32_max = 0;
    }
    #[test]
    fn test_as_primitive_u16_to_i32_zero() {
        let _rug_st_tests_llm_16_1467_llm_16_1467_rrrruuuugggg_test_as_primitive_u16_to_i32_zero = 0;
        let rug_fuzz_0 = 0;
        let value_u16: u16 = rug_fuzz_0;
        let value_i32: i32 = AsPrimitive::<i32>::as_(value_u16);
        debug_assert_eq!(value_i32, 0i32);
        let _rug_ed_tests_llm_16_1467_llm_16_1467_rrrruuuugggg_test_as_primitive_u16_to_i32_zero = 0;
    }
    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_as_primitive_u16_to_i32_overflow() {
        let _rug_st_tests_llm_16_1467_llm_16_1467_rrrruuuugggg_test_as_primitive_u16_to_i32_overflow = 0;
        let rug_fuzz_0 = 0xffff;
        let rug_fuzz_1 = 0;
        let value_u16: u16 = rug_fuzz_0;
        let value_i32: i32 = AsPrimitive::<i32>::as_(value_u16);
        debug_assert!(value_i32 < rug_fuzz_1);
        let _rug_ed_tests_llm_16_1467_llm_16_1467_rrrruuuugggg_test_as_primitive_u16_to_i32_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1468_llm_16_1468 {
    use crate::AsPrimitive;
    #[test]
    fn test_as_primitive_u16_to_i64() {
        let _rug_st_tests_llm_16_1468_llm_16_1468_rrrruuuugggg_test_as_primitive_u16_to_i64 = 0;
        let rug_fuzz_0 = 12345;
        let val_u16: u16 = rug_fuzz_0;
        let val_i64: i64 = val_u16.as_();
        debug_assert_eq!(val_i64, 12345i64);
        let _rug_ed_tests_llm_16_1468_llm_16_1468_rrrruuuugggg_test_as_primitive_u16_to_i64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1469_llm_16_1469 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_u16_as_i8() {
        let _rug_st_tests_llm_16_1469_llm_16_1469_rrrruuuugggg_test_u16_as_i8 = 0;
        let rug_fuzz_0 = 255;
        let rug_fuzz_1 = 127;
        let rug_fuzz_2 = 128;
        let val_u16: u16 = rug_fuzz_0;
        let val_i8: i8 = val_u16.as_();
        debug_assert_eq!(val_i8, - 1i8);
        let val_u16: u16 = rug_fuzz_1;
        let val_i8: i8 = val_u16.as_();
        debug_assert_eq!(val_i8, 127i8);
        let val_u16: u16 = rug_fuzz_2;
        let val_i8: i8 = val_u16.as_();
        let _rug_ed_tests_llm_16_1469_llm_16_1469_rrrruuuugggg_test_u16_as_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1470_llm_16_1470 {
    use crate::AsPrimitive;
    #[test]
    fn u16_to_isize() {
        let _rug_st_tests_llm_16_1470_llm_16_1470_rrrruuuugggg_u16_to_isize = 0;
        let rug_fuzz_0 = 42;
        let val_u16: u16 = rug_fuzz_0;
        let val_isize: isize = val_u16.as_();
        debug_assert_eq!(val_isize, 42isize);
        let _rug_ed_tests_llm_16_1470_llm_16_1470_rrrruuuugggg_u16_to_isize = 0;
    }
    #[test]
    fn min_u16_to_isize() {
        let _rug_st_tests_llm_16_1470_llm_16_1470_rrrruuuugggg_min_u16_to_isize = 0;
        let val_u16: u16 = u16::MIN;
        let val_isize: isize = val_u16.as_();
        debug_assert_eq!(val_isize, u16::MIN as isize);
        let _rug_ed_tests_llm_16_1470_llm_16_1470_rrrruuuugggg_min_u16_to_isize = 0;
    }
    #[test]
    fn max_u16_to_isize() {
        let _rug_st_tests_llm_16_1470_llm_16_1470_rrrruuuugggg_max_u16_to_isize = 0;
        let val_u16: u16 = u16::MAX;
        let val_isize: isize = val_u16.as_();
        debug_assert_eq!(val_isize, u16::MAX as isize);
        let _rug_ed_tests_llm_16_1470_llm_16_1470_rrrruuuugggg_max_u16_to_isize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1471_llm_16_1471 {
    use crate::AsPrimitive;
    #[test]
    fn test_as_primitive_u16_to_u128() {
        let _rug_st_tests_llm_16_1471_llm_16_1471_rrrruuuugggg_test_as_primitive_u16_to_u128 = 0;
        let rug_fuzz_0 = 42;
        let value_u16: u16 = rug_fuzz_0;
        let value_u128: u128 = AsPrimitive::<u128>::as_(value_u16);
        debug_assert_eq!(value_u128, 42u128);
        let _rug_ed_tests_llm_16_1471_llm_16_1471_rrrruuuugggg_test_as_primitive_u16_to_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1472_llm_16_1472 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_u16_to_u16() {
        let _rug_st_tests_llm_16_1472_llm_16_1472_rrrruuuugggg_test_as_primitive_u16_to_u16 = 0;
        let rug_fuzz_0 = 12345;
        let value: u16 = rug_fuzz_0;
        let result: u16 = AsPrimitive::<u16>::as_(value);
        debug_assert_eq!(result, value);
        let _rug_ed_tests_llm_16_1472_llm_16_1472_rrrruuuugggg_test_as_primitive_u16_to_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1473_llm_16_1473 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_u16_as_u32() {
        let _rug_st_tests_llm_16_1473_llm_16_1473_rrrruuuugggg_test_u16_as_u32 = 0;
        let rug_fuzz_0 = 12345;
        let value: u16 = rug_fuzz_0;
        let result: u32 = value.as_();
        debug_assert_eq!(result as u16, value);
        let _rug_ed_tests_llm_16_1473_llm_16_1473_rrrruuuugggg_test_u16_as_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1474_llm_16_1474 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_u16_to_u64() {
        let _rug_st_tests_llm_16_1474_llm_16_1474_rrrruuuugggg_test_as_u16_to_u64 = 0;
        let rug_fuzz_0 = 42;
        let value_u16: u16 = rug_fuzz_0;
        let value_u64: u64 = AsPrimitive::<u64>::as_(value_u16);
        debug_assert_eq!(value_u64, 42u64);
        let _rug_ed_tests_llm_16_1474_llm_16_1474_rrrruuuugggg_test_as_u16_to_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1475_llm_16_1475 {
    use crate::cast::AsPrimitive;
    #[test]
    fn u16_to_u8_casting() {
        let _rug_st_tests_llm_16_1475_llm_16_1475_rrrruuuugggg_u16_to_u8_casting = 0;
        let rug_fuzz_0 = 256;
        let rug_fuzz_1 = 255;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 0;
        let value: u16 = rug_fuzz_0;
        let casted_value: u8 = AsPrimitive::<u8>::as_(value);
        debug_assert_eq!(casted_value, 0);
        let value: u16 = rug_fuzz_1;
        let casted_value: u8 = AsPrimitive::<u8>::as_(value);
        debug_assert_eq!(casted_value, 255);
        let value: u16 = rug_fuzz_2;
        let casted_value: u8 = AsPrimitive::<u8>::as_(value);
        debug_assert_eq!(casted_value, 1);
        let value: u16 = rug_fuzz_3;
        let casted_value: u8 = AsPrimitive::<u8>::as_(value);
        debug_assert_eq!(casted_value, 0);
        let _rug_ed_tests_llm_16_1475_llm_16_1475_rrrruuuugggg_u16_to_u8_casting = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1476_llm_16_1476 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_u16_to_usize() {
        let _rug_st_tests_llm_16_1476_llm_16_1476_rrrruuuugggg_test_as_primitive_u16_to_usize = 0;
        let rug_fuzz_0 = 42;
        let value: u16 = rug_fuzz_0;
        let result: usize = AsPrimitive::<usize>::as_(value);
        debug_assert_eq!(result, 42usize);
        let _rug_ed_tests_llm_16_1476_llm_16_1476_rrrruuuugggg_test_as_primitive_u16_to_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1477_llm_16_1477 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_f32_to_u16() {
        let _rug_st_tests_llm_16_1477_llm_16_1477_rrrruuuugggg_test_from_f32_to_u16 = 0;
        let rug_fuzz_0 = 0.0_f32;
        let rug_fuzz_1 = 0_u16;
        let rug_fuzz_2 = 1.0_f32;
        let rug_fuzz_3 = 1_u16;
        let rug_fuzz_4 = 1.5_f32;
        let rug_fuzz_5 = 1_u16;
        let rug_fuzz_6 = 1.0;
        let rug_fuzz_7 = 1.0_f32;
        let values = [
            (rug_fuzz_0, Some(rug_fuzz_1)),
            (rug_fuzz_2, Some(rug_fuzz_3)),
            (rug_fuzz_4, Some(rug_fuzz_5)),
            (u16::MAX as f32, Some(u16::MAX)),
            ((u16::MAX as f32) + rug_fuzz_6, None),
            (-rug_fuzz_7, None),
            (f32::NAN, None),
            (f32::INFINITY, None),
            (f32::NEG_INFINITY, None),
        ];
        for &(input, expected) in &values {
            let result = u16::from_f32(input);
            debug_assert_eq!(
                result, expected, "u16::from_f32({}) did not return {:?}", input,
                expected
            );
        }
        let _rug_ed_tests_llm_16_1477_llm_16_1477_rrrruuuugggg_test_from_f32_to_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1478_llm_16_1478 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_f64_to_u16_with_in_range_value() {
        let _rug_st_tests_llm_16_1478_llm_16_1478_rrrruuuugggg_test_from_f64_to_u16_with_in_range_value = 0;
        let rug_fuzz_0 = 42.0_f64;
        let value = rug_fuzz_0;
        let result = <u16 as FromPrimitive>::from_f64(value);
        debug_assert_eq!(result, Some(42_u16));
        let _rug_ed_tests_llm_16_1478_llm_16_1478_rrrruuuugggg_test_from_f64_to_u16_with_in_range_value = 0;
    }
    #[test]
    fn test_from_f64_to_u16_with_value_too_large() {
        let _rug_st_tests_llm_16_1478_llm_16_1478_rrrruuuugggg_test_from_f64_to_u16_with_value_too_large = 0;
        let rug_fuzz_0 = 1.0;
        let value = <f64 as From<u16>>::from(u16::MAX) + rug_fuzz_0;
        let result = <u16 as FromPrimitive>::from_f64(value);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1478_llm_16_1478_rrrruuuugggg_test_from_f64_to_u16_with_value_too_large = 0;
    }
    #[test]
    fn test_from_f64_to_u16_with_negative_value() {
        let _rug_st_tests_llm_16_1478_llm_16_1478_rrrruuuugggg_test_from_f64_to_u16_with_negative_value = 0;
        let rug_fuzz_0 = 42.0_f64;
        let value = -rug_fuzz_0;
        let result = <u16 as FromPrimitive>::from_f64(value);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1478_llm_16_1478_rrrruuuugggg_test_from_f64_to_u16_with_negative_value = 0;
    }
    #[test]
    fn test_from_f64_to_u16_with_nan() {
        let _rug_st_tests_llm_16_1478_llm_16_1478_rrrruuuugggg_test_from_f64_to_u16_with_nan = 0;
        let value = f64::NAN;
        let result = <u16 as FromPrimitive>::from_f64(value);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1478_llm_16_1478_rrrruuuugggg_test_from_f64_to_u16_with_nan = 0;
    }
    #[test]
    fn test_from_f64_to_u16_with_infinity() {
        let _rug_st_tests_llm_16_1478_llm_16_1478_rrrruuuugggg_test_from_f64_to_u16_with_infinity = 0;
        let value = f64::INFINITY;
        let result = <u16 as FromPrimitive>::from_f64(value);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1478_llm_16_1478_rrrruuuugggg_test_from_f64_to_u16_with_infinity = 0;
    }
    #[test]
    fn test_from_f64_to_u16_with_negative_infinity() {
        let _rug_st_tests_llm_16_1478_llm_16_1478_rrrruuuugggg_test_from_f64_to_u16_with_negative_infinity = 0;
        let value = f64::NEG_INFINITY;
        let result = <u16 as FromPrimitive>::from_f64(value);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1478_llm_16_1478_rrrruuuugggg_test_from_f64_to_u16_with_negative_infinity = 0;
    }
    #[test]
    fn test_from_f64_to_u16_with_subnormal_value() {
        let _rug_st_tests_llm_16_1478_llm_16_1478_rrrruuuugggg_test_from_f64_to_u16_with_subnormal_value = 0;
        let rug_fuzz_0 = 1e-40_f64;
        let rug_fuzz_1 = 0_u16;
        let value = rug_fuzz_0;
        let expected = Some(rug_fuzz_1);
        let result = <u16 as FromPrimitive>::from_f64(value);
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_1478_llm_16_1478_rrrruuuugggg_test_from_f64_to_u16_with_subnormal_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1479_llm_16_1479 {
    use crate::cast::FromPrimitive;
    #[test]
    fn from_i128_within_range() {
        let _rug_st_tests_llm_16_1479_llm_16_1479_rrrruuuugggg_from_i128_within_range = 0;
        let rug_fuzz_0 = 42;
        let value: i128 = rug_fuzz_0;
        let result = <u16 as FromPrimitive>::from_i128(value);
        debug_assert_eq!(result, Some(42u16));
        let _rug_ed_tests_llm_16_1479_llm_16_1479_rrrruuuugggg_from_i128_within_range = 0;
    }
    #[test]
    fn from_i128_negative() {
        let _rug_st_tests_llm_16_1479_llm_16_1479_rrrruuuugggg_from_i128_negative = 0;
        let rug_fuzz_0 = 42;
        let value: i128 = -rug_fuzz_0;
        let result = <u16 as FromPrimitive>::from_i128(value);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1479_llm_16_1479_rrrruuuugggg_from_i128_negative = 0;
    }
    #[test]
    fn from_i128_above_u16() {
        let _rug_st_tests_llm_16_1479_llm_16_1479_rrrruuuugggg_from_i128_above_u16 = 0;
        let rug_fuzz_0 = 70000;
        let value: i128 = rug_fuzz_0;
        let result = <u16 as FromPrimitive>::from_i128(value);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1479_llm_16_1479_rrrruuuugggg_from_i128_above_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1480_llm_16_1480 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i16_with_positive() {
        let _rug_st_tests_llm_16_1480_llm_16_1480_rrrruuuugggg_test_from_i16_with_positive = 0;
        let rug_fuzz_0 = 123;
        let value: i16 = rug_fuzz_0;
        let result = <u16 as FromPrimitive>::from_i16(value);
        debug_assert_eq!(result, Some(123u16));
        let _rug_ed_tests_llm_16_1480_llm_16_1480_rrrruuuugggg_test_from_i16_with_positive = 0;
    }
    #[test]
    fn test_from_i16_with_zero() {
        let _rug_st_tests_llm_16_1480_llm_16_1480_rrrruuuugggg_test_from_i16_with_zero = 0;
        let rug_fuzz_0 = 0;
        let value: i16 = rug_fuzz_0;
        let result = <u16 as FromPrimitive>::from_i16(value);
        debug_assert_eq!(result, Some(0u16));
        let _rug_ed_tests_llm_16_1480_llm_16_1480_rrrruuuugggg_test_from_i16_with_zero = 0;
    }
    #[test]
    fn test_from_i16_with_negative() {
        let _rug_st_tests_llm_16_1480_llm_16_1480_rrrruuuugggg_test_from_i16_with_negative = 0;
        let rug_fuzz_0 = 123;
        let value: i16 = -rug_fuzz_0;
        let result = <u16 as FromPrimitive>::from_i16(value);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1480_llm_16_1480_rrrruuuugggg_test_from_i16_with_negative = 0;
    }
    #[test]
    fn test_from_i16_with_max() {
        let _rug_st_tests_llm_16_1480_llm_16_1480_rrrruuuugggg_test_from_i16_with_max = 0;
        let value: i16 = i16::MAX;
        let result = <u16 as FromPrimitive>::from_i16(value);
        debug_assert_eq!(result, Some(i16::MAX as u16));
        let _rug_ed_tests_llm_16_1480_llm_16_1480_rrrruuuugggg_test_from_i16_with_max = 0;
    }
    #[test]
    fn test_from_i16_with_min() {
        let _rug_st_tests_llm_16_1480_llm_16_1480_rrrruuuugggg_test_from_i16_with_min = 0;
        let value: i16 = i16::MIN;
        let result = <u16 as FromPrimitive>::from_i16(value);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1480_llm_16_1480_rrrruuuugggg_test_from_i16_with_min = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1481_llm_16_1481 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i32() {
        let _rug_st_tests_llm_16_1481_llm_16_1481_rrrruuuugggg_test_from_i32 = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 65535;
        let rug_fuzz_3 = 65536;
        debug_assert_eq!(< u16 as FromPrimitive > ::from_i32(- rug_fuzz_0), None);
        debug_assert_eq!(< u16 as FromPrimitive > ::from_i32(rug_fuzz_1), Some(0));
        debug_assert_eq!(< u16 as FromPrimitive > ::from_i32(rug_fuzz_2), Some(65535));
        debug_assert_eq!(< u16 as FromPrimitive > ::from_i32(rug_fuzz_3), None);
        let _rug_ed_tests_llm_16_1481_llm_16_1481_rrrruuuugggg_test_from_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1482_llm_16_1482 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i64() {
        let _rug_st_tests_llm_16_1482_llm_16_1482_rrrruuuugggg_test_from_i64 = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 65_535;
        let rug_fuzz_2 = 65_536;
        let rug_fuzz_3 = 1;
        debug_assert_eq!(< u16 as FromPrimitive > ::from_i64(rug_fuzz_0), Some(0u16));
        debug_assert_eq!(
            < u16 as FromPrimitive > ::from_i64(rug_fuzz_1), Some(65_535u16)
        );
        debug_assert_eq!(< u16 as FromPrimitive > ::from_i64(rug_fuzz_2), None);
        debug_assert_eq!(< u16 as FromPrimitive > ::from_i64(- rug_fuzz_3), None);
        let _rug_ed_tests_llm_16_1482_llm_16_1482_rrrruuuugggg_test_from_i64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1483_llm_16_1483 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i8_to_u16() {
        assert_eq!(< u16 as FromPrimitive >::from_i8(0), Some(0_u16));
        assert_eq!(< u16 as FromPrimitive >::from_i8(127), Some(127_u16));
        assert_eq!(< u16 as FromPrimitive >::from_i8(- 1), None);
        assert_eq!(< u16 as FromPrimitive >::from_i8(- 128), None);
    }
}
#[cfg(test)]
mod tests_llm_16_1484_llm_16_1484 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_isize() {
        let _rug_st_tests_llm_16_1484_llm_16_1484_rrrruuuugggg_test_from_isize = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let test_values_isize: Vec<isize> = vec![rug_fuzz_0, 1, 10, 32767, - 1, - 32768];
        let test_values_u16: Vec<Option<u16>> = vec![
            Some(rug_fuzz_1), Some(1), Some(10), Some(32767), None, None
        ];
        for (&isize_val, &expected_u16) in test_values_isize
            .iter()
            .zip(test_values_u16.iter())
        {
            let result = <u16 as FromPrimitive>::from_isize(isize_val);
            debug_assert_eq!(
                result, expected_u16, "isize to u16 conversion failed for value: {}",
                isize_val
            );
        }
        let _rug_ed_tests_llm_16_1484_llm_16_1484_rrrruuuugggg_test_from_isize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1485_llm_16_1485 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u128_with_in_range_value() {
        let _rug_st_tests_llm_16_1485_llm_16_1485_rrrruuuugggg_test_from_u128_with_in_range_value = 0;
        let rug_fuzz_0 = 42u128;
        debug_assert_eq!(< u16 as FromPrimitive > ::from_u128(rug_fuzz_0), Some(42u16));
        let _rug_ed_tests_llm_16_1485_llm_16_1485_rrrruuuugggg_test_from_u128_with_in_range_value = 0;
    }
    #[test]
    fn test_from_u128_with_out_of_range_value() {
        let _rug_st_tests_llm_16_1485_llm_16_1485_rrrruuuugggg_test_from_u128_with_out_of_range_value = 0;
        debug_assert_eq!(< u16 as FromPrimitive > ::from_u128(u128::MAX), None);
        let _rug_ed_tests_llm_16_1485_llm_16_1485_rrrruuuugggg_test_from_u128_with_out_of_range_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1486_llm_16_1486 {
    use crate::cast::FromPrimitive;
    #[test]
    fn from_u16_test_u8() {
        let _rug_st_tests_llm_16_1486_llm_16_1486_rrrruuuugggg_from_u16_test_u8 = 0;
        let rug_fuzz_0 = 0_u16;
        let rug_fuzz_1 = 1_u16;
        debug_assert_eq!(< u8 as FromPrimitive > ::from_u16(rug_fuzz_0), Some(0_u8));
        debug_assert_eq!(< u8 as FromPrimitive > ::from_u16(rug_fuzz_1), Some(1_u8));
        debug_assert_eq!(< u8 as FromPrimitive > ::from_u16(u16::MAX), None);
        let _rug_ed_tests_llm_16_1486_llm_16_1486_rrrruuuugggg_from_u16_test_u8 = 0;
    }
    #[test]
    fn from_u16_test_u32() {
        let _rug_st_tests_llm_16_1486_llm_16_1486_rrrruuuugggg_from_u16_test_u32 = 0;
        let rug_fuzz_0 = 0_u16;
        let rug_fuzz_1 = 1_u16;
        debug_assert_eq!(< u32 as FromPrimitive > ::from_u16(rug_fuzz_0), Some(0_u32));
        debug_assert_eq!(< u32 as FromPrimitive > ::from_u16(rug_fuzz_1), Some(1_u32));
        debug_assert_eq!(
            < u32 as FromPrimitive > ::from_u16(u16::MAX), Some(u16::MAX as u32)
        );
        let _rug_ed_tests_llm_16_1486_llm_16_1486_rrrruuuugggg_from_u16_test_u32 = 0;
    }
    #[test]
    fn from_u16_test_u64() {
        let _rug_st_tests_llm_16_1486_llm_16_1486_rrrruuuugggg_from_u16_test_u64 = 0;
        let rug_fuzz_0 = 0_u16;
        let rug_fuzz_1 = 1_u16;
        debug_assert_eq!(< u64 as FromPrimitive > ::from_u16(rug_fuzz_0), Some(0_u64));
        debug_assert_eq!(< u64 as FromPrimitive > ::from_u16(rug_fuzz_1), Some(1_u64));
        debug_assert_eq!(
            < u64 as FromPrimitive > ::from_u16(u16::MAX), Some(u16::MAX as u64)
        );
        let _rug_ed_tests_llm_16_1486_llm_16_1486_rrrruuuugggg_from_u16_test_u64 = 0;
    }
    #[test]
    fn from_u16_test_i32() {
        let _rug_st_tests_llm_16_1486_llm_16_1486_rrrruuuugggg_from_u16_test_i32 = 0;
        let rug_fuzz_0 = 0_u16;
        let rug_fuzz_1 = 1_u16;
        debug_assert_eq!(< i32 as FromPrimitive > ::from_u16(rug_fuzz_0), Some(0_i32));
        debug_assert_eq!(< i32 as FromPrimitive > ::from_u16(rug_fuzz_1), Some(1_i32));
        debug_assert_eq!(
            < i32 as FromPrimitive > ::from_u16(u16::MAX), Some(u16::MAX as i32)
        );
        let _rug_ed_tests_llm_16_1486_llm_16_1486_rrrruuuugggg_from_u16_test_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1487_llm_16_1487 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u32_to_u16() {
        let _rug_st_tests_llm_16_1487_llm_16_1487_rrrruuuugggg_test_from_u32_to_u16 = 0;
        let rug_fuzz_0 = 0u32;
        let rug_fuzz_1 = 65535u32;
        let rug_fuzz_2 = 65536u32;
        debug_assert_eq!(< u16 as FromPrimitive > ::from_u32(rug_fuzz_0), Some(0u16));
        debug_assert_eq!(
            < u16 as FromPrimitive > ::from_u32(rug_fuzz_1), Some(65535u16)
        );
        debug_assert_eq!(< u16 as FromPrimitive > ::from_u32(rug_fuzz_2), None);
        let _rug_ed_tests_llm_16_1487_llm_16_1487_rrrruuuugggg_test_from_u32_to_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1488_llm_16_1488 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u64_with_u16() {
        let _rug_st_tests_llm_16_1488_llm_16_1488_rrrruuuugggg_test_from_u64_with_u16 = 0;
        let rug_fuzz_0 = 0_u64;
        let rug_fuzz_1 = 65535_u64;
        let rug_fuzz_2 = 65536_u64;
        debug_assert_eq!(< u16 as FromPrimitive > ::from_u64(rug_fuzz_0), Some(0_u16));
        debug_assert_eq!(
            < u16 as FromPrimitive > ::from_u64(rug_fuzz_1), Some(65535_u16)
        );
        debug_assert_eq!(< u16 as FromPrimitive > ::from_u64(rug_fuzz_2), None);
        debug_assert_eq!(< u16 as FromPrimitive > ::from_u64(u64::MAX), None);
        let _rug_ed_tests_llm_16_1488_llm_16_1488_rrrruuuugggg_test_from_u64_with_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1489_llm_16_1489 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u8() {
        let _rug_st_tests_llm_16_1489_llm_16_1489_rrrruuuugggg_test_from_u8 = 0;
        let rug_fuzz_0 = 0_u8;
        let rug_fuzz_1 = 127_u8;
        let rug_fuzz_2 = 255_u8;
        debug_assert_eq!(< u16 as FromPrimitive > ::from_u8(rug_fuzz_0), Some(0_u16));
        debug_assert_eq!(< u16 as FromPrimitive > ::from_u8(rug_fuzz_1), Some(127_u16));
        debug_assert_eq!(< u16 as FromPrimitive > ::from_u8(rug_fuzz_2), Some(255_u16));
        let _rug_ed_tests_llm_16_1489_llm_16_1489_rrrruuuugggg_test_from_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1490_llm_16_1490 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_usize() {
        let _rug_st_tests_llm_16_1490_llm_16_1490_rrrruuuugggg_test_from_usize = 0;
        let rug_fuzz_0 = 42_usize;
        let rug_fuzz_1 = 100_000_usize;
        debug_assert_eq!(
            < u16 as FromPrimitive > ::from_usize(rug_fuzz_0), Some(42_u16)
        );
        debug_assert_eq!(< u16 as FromPrimitive > ::from_usize(rug_fuzz_1), None);
        debug_assert_eq!(
            < u16 as FromPrimitive > ::from_usize(usize::max_value()), None
        );
        debug_assert_eq!(
            < u16 as FromPrimitive > ::from_usize(u16::MAX as usize), Some(u16::MAX)
        );
        let _rug_ed_tests_llm_16_1490_llm_16_1490_rrrruuuugggg_test_from_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1491_llm_16_1491 {
    use crate::cast::NumCast;
    use std::num::Wrapping;
    #[test]
    fn test_from_i32_to_u16() {
        let _rug_st_tests_llm_16_1491_llm_16_1491_rrrruuuugggg_test_from_i32_to_u16 = 0;
        let rug_fuzz_0 = 42;
        let val: i32 = rug_fuzz_0;
        let wrapped_val = Wrapping(val);
        let casted_val: Option<u16> = NumCast::from(wrapped_val);
        debug_assert_eq!(casted_val, Some(42u16));
        let _rug_ed_tests_llm_16_1491_llm_16_1491_rrrruuuugggg_test_from_i32_to_u16 = 0;
    }
    #[test]
    fn test_from_u32_to_u16() {
        let _rug_st_tests_llm_16_1491_llm_16_1491_rrrruuuugggg_test_from_u32_to_u16 = 0;
        let rug_fuzz_0 = 65535;
        let val: u32 = rug_fuzz_0;
        let wrapped_val = Wrapping(val);
        let casted_val: Option<u16> = NumCast::from(wrapped_val);
        debug_assert_eq!(casted_val, Some(65535u16));
        let _rug_ed_tests_llm_16_1491_llm_16_1491_rrrruuuugggg_test_from_u32_to_u16 = 0;
    }
    #[test]
    fn test_from_i32_overflow_to_u16() {
        let _rug_st_tests_llm_16_1491_llm_16_1491_rrrruuuugggg_test_from_i32_overflow_to_u16 = 0;
        let rug_fuzz_0 = 1;
        let val: i32 = -rug_fuzz_0;
        let wrapped_val = Wrapping(val);
        let casted_val: Option<u16> = NumCast::from(wrapped_val);
        debug_assert_eq!(casted_val, None);
        let _rug_ed_tests_llm_16_1491_llm_16_1491_rrrruuuugggg_test_from_i32_overflow_to_u16 = 0;
    }
    #[test]
    fn test_from_u32_overflow_to_u16() {
        let _rug_st_tests_llm_16_1491_llm_16_1491_rrrruuuugggg_test_from_u32_overflow_to_u16 = 0;
        let rug_fuzz_0 = 65536;
        let val: u32 = rug_fuzz_0;
        let wrapped_val = Wrapping(val);
        let casted_val: Option<u16> = NumCast::from(wrapped_val);
        debug_assert_eq!(casted_val, None);
        let _rug_ed_tests_llm_16_1491_llm_16_1491_rrrruuuugggg_test_from_u32_overflow_to_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1492_llm_16_1492 {
    #[test]
    fn test_u16_to_f32() {
        let _rug_st_tests_llm_16_1492_llm_16_1492_rrrruuuugggg_test_u16_to_f32 = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 12345;
        debug_assert_eq!(
            < u16 as crate ::cast::ToPrimitive > ::to_f32(& rug_fuzz_0), Some(0.0_f32)
        );
        debug_assert_eq!(
            < u16 as crate ::cast::ToPrimitive > ::to_f32(& rug_fuzz_1), Some(1.0_f32)
        );
        debug_assert_eq!(
            < u16 as crate ::cast::ToPrimitive > ::to_f32(& rug_fuzz_2),
            Some(12345.0_f32)
        );
        debug_assert_eq!(
            < u16 as crate ::cast::ToPrimitive > ::to_f32(& u16::MAX), Some(u16::MAX as
            f32)
        );
        let _rug_ed_tests_llm_16_1492_llm_16_1492_rrrruuuugggg_test_u16_to_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1493_llm_16_1493 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_u16_to_f64() {
        let _rug_st_tests_llm_16_1493_llm_16_1493_rrrruuuugggg_test_u16_to_f64 = 0;
        let rug_fuzz_0 = 42;
        let val: u16 = rug_fuzz_0;
        let float_val: Option<f64> = ToPrimitive::to_f64(&val);
        debug_assert_eq!(float_val, Some(42.0f64));
        let _rug_ed_tests_llm_16_1493_llm_16_1493_rrrruuuugggg_test_u16_to_f64 = 0;
    }
    #[test]
    fn test_u16_to_f64_max_value() {
        let _rug_st_tests_llm_16_1493_llm_16_1493_rrrruuuugggg_test_u16_to_f64_max_value = 0;
        let val: u16 = u16::MAX;
        let float_val: Option<f64> = ToPrimitive::to_f64(&val);
        debug_assert_eq!(float_val, Some(u16::MAX as f64));
        let _rug_ed_tests_llm_16_1493_llm_16_1493_rrrruuuugggg_test_u16_to_f64_max_value = 0;
    }
    #[test]
    fn test_u16_to_f64_zero() {
        let _rug_st_tests_llm_16_1493_llm_16_1493_rrrruuuugggg_test_u16_to_f64_zero = 0;
        let rug_fuzz_0 = 0;
        let val: u16 = rug_fuzz_0;
        let float_val: Option<f64> = ToPrimitive::to_f64(&val);
        debug_assert_eq!(float_val, Some(0.0f64));
        let _rug_ed_tests_llm_16_1493_llm_16_1493_rrrruuuugggg_test_u16_to_f64_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1494_llm_16_1494 {
    use crate::cast::ToPrimitive;
    #[test]
    fn u16_to_i128_conversion_within_bounds() {
        let _rug_st_tests_llm_16_1494_llm_16_1494_rrrruuuugggg_u16_to_i128_conversion_within_bounds = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42;
        let val: u16 = rug_fuzz_0;
        let expected: Option<i128> = Some(rug_fuzz_1);
        debug_assert_eq!(val.to_i128(), expected);
        let _rug_ed_tests_llm_16_1494_llm_16_1494_rrrruuuugggg_u16_to_i128_conversion_within_bounds = 0;
    }
    #[test]
    fn u16_to_i128_conversion_at_upper_bound() {
        let _rug_st_tests_llm_16_1494_llm_16_1494_rrrruuuugggg_u16_to_i128_conversion_at_upper_bound = 0;
        let val: u16 = u16::MAX;
        let expected: Option<i128> = Some(u16::MAX as i128);
        debug_assert_eq!(val.to_i128(), expected);
        let _rug_ed_tests_llm_16_1494_llm_16_1494_rrrruuuugggg_u16_to_i128_conversion_at_upper_bound = 0;
    }
    #[test]
    fn u16_to_i128_conversion_at_zero() {
        let _rug_st_tests_llm_16_1494_llm_16_1494_rrrruuuugggg_u16_to_i128_conversion_at_zero = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        let val: u16 = rug_fuzz_0;
        let expected: Option<i128> = Some(rug_fuzz_1);
        debug_assert_eq!(val.to_i128(), expected);
        let _rug_ed_tests_llm_16_1494_llm_16_1494_rrrruuuugggg_u16_to_i128_conversion_at_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1495_llm_16_1495 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_i16() {
        let _rug_st_tests_llm_16_1495_llm_16_1495_rrrruuuugggg_test_to_i16 = 0;
        let rug_fuzz_0 = 0u16;
        let rug_fuzz_1 = 32767u16;
        let rug_fuzz_2 = 65535u16;
        debug_assert_eq!(rug_fuzz_0.to_i16(), Some(0i16));
        debug_assert_eq!(rug_fuzz_1.to_i16(), Some(32767i16));
        debug_assert_eq!(rug_fuzz_2.to_i16(), None);
        let _rug_ed_tests_llm_16_1495_llm_16_1495_rrrruuuugggg_test_to_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1496_llm_16_1496 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_i32_with_u16() {
        let _rug_st_tests_llm_16_1496_llm_16_1496_rrrruuuugggg_test_to_i32_with_u16 = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 65535;
        let small_value: u16 = rug_fuzz_0;
        let small_value_i32 = small_value.to_i32();
        debug_assert_eq!(small_value_i32, Some(42_i32));
        let large_value: u16 = rug_fuzz_1;
        let large_value_i32 = large_value.to_i32();
        debug_assert_eq!(large_value_i32, Some(65535_i32));
        let _rug_ed_tests_llm_16_1496_llm_16_1496_rrrruuuugggg_test_to_i32_with_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1497_llm_16_1497 {
    use crate::ToPrimitive;
    #[test]
    fn u16_to_i64() {
        let _rug_st_tests_llm_16_1497_llm_16_1497_rrrruuuugggg_u16_to_i64 = 0;
        let rug_fuzz_0 = 0u16;
        let rug_fuzz_1 = 1u16;
        let rug_fuzz_2 = 0;
        debug_assert_eq!(rug_fuzz_0.to_i64(), Some(0i64));
        debug_assert_eq!(rug_fuzz_1.to_i64(), Some(1i64));
        debug_assert_eq!(u16::MAX.to_i64(), Some(i64::from(u16::MAX)));
        let big_value: u16 = u16::MAX;
        debug_assert_eq!(big_value.to_i64(), Some(i64::from(u16::MAX)));
        let small_value: u16 = rug_fuzz_2;
        debug_assert_eq!(small_value.to_i64(), Some(0i64));
        let _rug_ed_tests_llm_16_1497_llm_16_1497_rrrruuuugggg_u16_to_i64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1498_llm_16_1498 {
    use crate::ToPrimitive;
    #[test]
    fn test_to_i8_with_in_range_value() {
        let _rug_st_tests_llm_16_1498_llm_16_1498_rrrruuuugggg_test_to_i8_with_in_range_value = 0;
        let rug_fuzz_0 = 100;
        let value: u16 = rug_fuzz_0;
        let result = value.to_i8();
        debug_assert_eq!(result, Some(100i8));
        let _rug_ed_tests_llm_16_1498_llm_16_1498_rrrruuuugggg_test_to_i8_with_in_range_value = 0;
    }
    #[test]
    fn test_to_i8_with_out_of_range_value() {
        let _rug_st_tests_llm_16_1498_llm_16_1498_rrrruuuugggg_test_to_i8_with_out_of_range_value = 0;
        let rug_fuzz_0 = 1000;
        let value: u16 = rug_fuzz_0;
        let result = value.to_i8();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1498_llm_16_1498_rrrruuuugggg_test_to_i8_with_out_of_range_value = 0;
    }
    #[test]
    fn test_to_i8_with_max_i8() {
        let _rug_st_tests_llm_16_1498_llm_16_1498_rrrruuuugggg_test_to_i8_with_max_i8 = 0;
        let value: u16 = i8::MAX as u16;
        let result = value.to_i8();
        debug_assert_eq!(result, Some(i8::MAX));
        let _rug_ed_tests_llm_16_1498_llm_16_1498_rrrruuuugggg_test_to_i8_with_max_i8 = 0;
    }
    #[test]
    fn test_to_i8_with_min_i8() {
        let _rug_st_tests_llm_16_1498_llm_16_1498_rrrruuuugggg_test_to_i8_with_min_i8 = 0;
        let rug_fuzz_0 = 0;
        let value: u16 = rug_fuzz_0;
        let result = value.to_i8();
        debug_assert_eq!(result, Some(0i8));
        let _rug_ed_tests_llm_16_1498_llm_16_1498_rrrruuuugggg_test_to_i8_with_min_i8 = 0;
    }
    #[test]
    fn test_to_i8_with_i8_max_plus_one() {
        let _rug_st_tests_llm_16_1498_llm_16_1498_rrrruuuugggg_test_to_i8_with_i8_max_plus_one = 0;
        let rug_fuzz_0 = 1;
        let value: u16 = (i8::MAX as u16) + rug_fuzz_0;
        let result = value.to_i8();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1498_llm_16_1498_rrrruuuugggg_test_to_i8_with_i8_max_plus_one = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1499_llm_16_1499 {
    use crate::cast::ToPrimitive;
    use std::mem::size_of;
    #[test]
    fn test_to_isize_within_range() {
        let _rug_st_tests_llm_16_1499_llm_16_1499_rrrruuuugggg_test_to_isize_within_range = 0;
        let value: u16 = isize::MAX as u16;
        let result = value.to_isize();
        debug_assert_eq!(result, Some(value as isize));
        let _rug_ed_tests_llm_16_1499_llm_16_1499_rrrruuuugggg_test_to_isize_within_range = 0;
    }
    #[test]
    fn test_to_isize_out_of_range() {
        let _rug_st_tests_llm_16_1499_llm_16_1499_rrrruuuugggg_test_to_isize_out_of_range = 0;
        let rug_fuzz_0 = 1;
        let value: u16 = (isize::MAX as u16).wrapping_add(rug_fuzz_0);
        let result = value.to_isize();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1499_llm_16_1499_rrrruuuugggg_test_to_isize_out_of_range = 0;
    }
    #[test]
    fn test_to_isize_zero() {
        let _rug_st_tests_llm_16_1499_llm_16_1499_rrrruuuugggg_test_to_isize_zero = 0;
        let rug_fuzz_0 = 0;
        let value: u16 = rug_fuzz_0;
        let result = value.to_isize();
        debug_assert_eq!(result, Some(0));
        let _rug_ed_tests_llm_16_1499_llm_16_1499_rrrruuuugggg_test_to_isize_zero = 0;
    }
    #[test]
    fn test_to_isize_max() {
        let _rug_st_tests_llm_16_1499_llm_16_1499_rrrruuuugggg_test_to_isize_max = 0;
        let value: u16 = u16::MAX;
        if size_of::<isize>() > size_of::<u16>() {
            let result = value.to_isize();
            debug_assert_eq!(result, Some(u16::MAX as isize));
        } else {
            let result = value.to_isize();
            debug_assert_eq!(result, None);
        }
        let _rug_ed_tests_llm_16_1499_llm_16_1499_rrrruuuugggg_test_to_isize_max = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1500_llm_16_1500 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_u16_to_u128() {
        let _rug_st_tests_llm_16_1500_llm_16_1500_rrrruuuugggg_test_u16_to_u128 = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 0;
        let small_value: u16 = rug_fuzz_0;
        debug_assert_eq!(small_value.to_u128(), Some(42_u128));
        let max_u16: u16 = u16::MAX;
        debug_assert_eq!(max_u16.to_u128(), Some(u16::MAX as u128));
        let zero: u16 = rug_fuzz_1;
        debug_assert_eq!(zero.to_u128(), Some(0_u128));
        let _rug_ed_tests_llm_16_1500_llm_16_1500_rrrruuuugggg_test_u16_to_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1501_llm_16_1501 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_u16_with_u16() {
        let _rug_st_tests_llm_16_1501_llm_16_1501_rrrruuuugggg_test_to_u16_with_u16 = 0;
        let rug_fuzz_0 = 42;
        let value: u16 = rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_u16(& value), Some(value));
        let _rug_ed_tests_llm_16_1501_llm_16_1501_rrrruuuugggg_test_to_u16_with_u16 = 0;
    }
    #[test]
    fn test_to_u16_with_u8() {
        let _rug_st_tests_llm_16_1501_llm_16_1501_rrrruuuugggg_test_to_u16_with_u8 = 0;
        let rug_fuzz_0 = 100;
        let value: u8 = rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_u16(& value), Some(100_u16));
        let _rug_ed_tests_llm_16_1501_llm_16_1501_rrrruuuugggg_test_to_u16_with_u8 = 0;
    }
    #[test]
    fn test_to_u16_with_u32() {
        let _rug_st_tests_llm_16_1501_llm_16_1501_rrrruuuugggg_test_to_u16_with_u32 = 0;
        let rug_fuzz_0 = 65_535;
        let rug_fuzz_1 = 65_536;
        let value: u32 = rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_u16(& value), Some(65_535_u16));
        let value: u32 = rug_fuzz_1;
        debug_assert_eq!(ToPrimitive::to_u16(& value), None);
        let _rug_ed_tests_llm_16_1501_llm_16_1501_rrrruuuugggg_test_to_u16_with_u32 = 0;
    }
    #[test]
    fn test_to_u16_with_i16() {
        let _rug_st_tests_llm_16_1501_llm_16_1501_rrrruuuugggg_test_to_u16_with_i16 = 0;
        let rug_fuzz_0 = 32_767;
        let rug_fuzz_1 = 1;
        let value: i16 = rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_u16(& value), Some(32_767_u16));
        let value: i16 = -rug_fuzz_1;
        debug_assert_eq!(ToPrimitive::to_u16(& value), None);
        let _rug_ed_tests_llm_16_1501_llm_16_1501_rrrruuuugggg_test_to_u16_with_i16 = 0;
    }
    #[test]
    fn test_to_u16_with_i32() {
        let _rug_st_tests_llm_16_1501_llm_16_1501_rrrruuuugggg_test_to_u16_with_i32 = 0;
        let rug_fuzz_0 = 65_535;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 65_536;
        let value: i32 = rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_u16(& value), Some(65_535_u16));
        let value: i32 = -rug_fuzz_1;
        debug_assert_eq!(ToPrimitive::to_u16(& value), None);
        let value: i32 = rug_fuzz_2;
        debug_assert_eq!(ToPrimitive::to_u16(& value), None);
        let _rug_ed_tests_llm_16_1501_llm_16_1501_rrrruuuugggg_test_to_u16_with_i32 = 0;
    }
    #[test]
    fn test_to_u16_with_i64() {
        let _rug_st_tests_llm_16_1501_llm_16_1501_rrrruuuugggg_test_to_u16_with_i64 = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let value: i64 = rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_u16(& value), Some(1_u16));
        let value: i64 = -rug_fuzz_1;
        debug_assert_eq!(ToPrimitive::to_u16(& value), None);
        let value: i64 = i64::from(u16::MAX);
        debug_assert_eq!(ToPrimitive::to_u16(& value), Some(u16::MAX));
        let value: i64 = (i64::from(u16::MAX) + rug_fuzz_2) as i64;
        debug_assert_eq!(ToPrimitive::to_u16(& value), None);
        let _rug_ed_tests_llm_16_1501_llm_16_1501_rrrruuuugggg_test_to_u16_with_i64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1503_llm_16_1503 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_u64_with_u16_max() {
        let _rug_st_tests_llm_16_1503_llm_16_1503_rrrruuuugggg_test_to_u64_with_u16_max = 0;
        let value: u16 = u16::MAX;
        let result = value.to_u64();
        debug_assert_eq!(result, Some(65535u64));
        let _rug_ed_tests_llm_16_1503_llm_16_1503_rrrruuuugggg_test_to_u64_with_u16_max = 0;
    }
    #[test]
    fn test_to_u64_with_zero() {
        let _rug_st_tests_llm_16_1503_llm_16_1503_rrrruuuugggg_test_to_u64_with_zero = 0;
        let rug_fuzz_0 = 0;
        let value: u16 = rug_fuzz_0;
        let result = value.to_u64();
        debug_assert_eq!(result, Some(0u64));
        let _rug_ed_tests_llm_16_1503_llm_16_1503_rrrruuuugggg_test_to_u64_with_zero = 0;
    }
    #[test]
    fn test_to_u64_with_regular_value() {
        let _rug_st_tests_llm_16_1503_llm_16_1503_rrrruuuugggg_test_to_u64_with_regular_value = 0;
        let rug_fuzz_0 = 42;
        let value: u16 = rug_fuzz_0;
        let result = value.to_u64();
        debug_assert_eq!(result, Some(42u64));
        let _rug_ed_tests_llm_16_1503_llm_16_1503_rrrruuuugggg_test_to_u64_with_regular_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1504_llm_16_1504 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_u8_in_range() {
        let _rug_st_tests_llm_16_1504_llm_16_1504_rrrruuuugggg_test_to_u8_in_range = 0;
        let rug_fuzz_0 = 100;
        let val: u16 = rug_fuzz_0;
        debug_assert_eq!(val.to_u8(), Some(100u8));
        let _rug_ed_tests_llm_16_1504_llm_16_1504_rrrruuuugggg_test_to_u8_in_range = 0;
    }
    #[test]
    fn test_to_u8_out_of_range() {
        let _rug_st_tests_llm_16_1504_llm_16_1504_rrrruuuugggg_test_to_u8_out_of_range = 0;
        let rug_fuzz_0 = 1000;
        let val: u16 = rug_fuzz_0;
        debug_assert!(val.to_u8().is_some());
        let _rug_ed_tests_llm_16_1504_llm_16_1504_rrrruuuugggg_test_to_u8_out_of_range = 0;
    }
    #[test]
    fn test_to_u8_at_max() {
        let _rug_st_tests_llm_16_1504_llm_16_1504_rrrruuuugggg_test_to_u8_at_max = 0;
        let val: u16 = u8::MAX.into();
        debug_assert_eq!(val.to_u8(), Some(u8::MAX));
        let _rug_ed_tests_llm_16_1504_llm_16_1504_rrrruuuugggg_test_to_u8_at_max = 0;
    }
    #[test]
    fn test_to_u8_above_max() {
        let _rug_st_tests_llm_16_1504_llm_16_1504_rrrruuuugggg_test_to_u8_above_max = 0;
        let rug_fuzz_0 = 1;
        let val: u16 = u8::MAX as u16 + rug_fuzz_0;
        debug_assert_eq!(val.to_u8(), None);
        let _rug_ed_tests_llm_16_1504_llm_16_1504_rrrruuuugggg_test_to_u8_above_max = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1505_llm_16_1505 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_usize_within_bounds() {
        let _rug_st_tests_llm_16_1505_llm_16_1505_rrrruuuugggg_test_to_usize_within_bounds = 0;
        let value: u16 = u16::MAX;
        debug_assert_eq!(ToPrimitive::to_usize(& value), Some(u16::MAX as usize));
        let _rug_ed_tests_llm_16_1505_llm_16_1505_rrrruuuugggg_test_to_usize_within_bounds = 0;
    }
    #[test]
    fn test_to_usize_out_of_bounds() {
        let _rug_st_tests_llm_16_1505_llm_16_1505_rrrruuuugggg_test_to_usize_out_of_bounds = 0;
        let value: u16 = u16::MAX;
        let max_usize = usize::MAX;
        if max_usize < (u16::MAX as usize) {
            debug_assert_eq!(ToPrimitive::to_usize(& value), None);
        } else {
            debug_assert_eq!(ToPrimitive::to_usize(& value), Some(u16::MAX as usize));
        }
        let _rug_ed_tests_llm_16_1505_llm_16_1505_rrrruuuugggg_test_to_usize_out_of_bounds = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1568_llm_16_1568 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_u32_as_f32() {
        let _rug_st_tests_llm_16_1568_llm_16_1568_rrrruuuugggg_test_u32_as_f32 = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42.0f32;
        let value: u32 = rug_fuzz_0;
        let result: f32 = AsPrimitive::<f32>::as_(value);
        let expected: f32 = rug_fuzz_1;
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_1568_llm_16_1568_rrrruuuugggg_test_u32_as_f32 = 0;
    }
    #[test]
    fn test_u32_as_f32_large_number() {
        let _rug_st_tests_llm_16_1568_llm_16_1568_rrrruuuugggg_test_u32_as_f32_large_number = 0;
        let value: u32 = u32::MAX;
        let result: f32 = AsPrimitive::<f32>::as_(value);
        let expected: f32 = u32::MAX as f32;
        debug_assert!(result >= expected);
        let _rug_ed_tests_llm_16_1568_llm_16_1568_rrrruuuugggg_test_u32_as_f32_large_number = 0;
    }
    #[test]
    fn test_u32_as_f32_zero() {
        let _rug_st_tests_llm_16_1568_llm_16_1568_rrrruuuugggg_test_u32_as_f32_zero = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0.0f32;
        let value: u32 = rug_fuzz_0;
        let result: f32 = AsPrimitive::<f32>::as_(value);
        let expected: f32 = rug_fuzz_1;
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_1568_llm_16_1568_rrrruuuugggg_test_u32_as_f32_zero = 0;
    }
    #[test]
    fn test_u32_as_f32_rounding() {
        let _rug_st_tests_llm_16_1568_llm_16_1568_rrrruuuugggg_test_u32_as_f32_rounding = 0;
        let rug_fuzz_0 = 1_000_000_000;
        let rug_fuzz_1 = 1_000_000_000f32;
        let value: u32 = rug_fuzz_0;
        let result: f32 = AsPrimitive::<f32>::as_(value);
        let expected: f32 = rug_fuzz_1;
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_1568_llm_16_1568_rrrruuuugggg_test_u32_as_f32_rounding = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1569_llm_16_1569 {
    use crate::AsPrimitive;
    #[test]
    fn test_u32_as_f64() {
        let _rug_st_tests_llm_16_1569_llm_16_1569_rrrruuuugggg_test_u32_as_f64 = 0;
        let rug_fuzz_0 = 42;
        let val: u32 = rug_fuzz_0;
        let result: f64 = val.as_();
        debug_assert_eq!(result, 42.0_f64);
        let _rug_ed_tests_llm_16_1569_llm_16_1569_rrrruuuugggg_test_u32_as_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1570_llm_16_1570 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_u32_to_i128() {
        let _rug_st_tests_llm_16_1570_llm_16_1570_rrrruuuugggg_test_as_primitive_u32_to_i128 = 0;
        let rug_fuzz_0 = 12345;
        let value: u32 = rug_fuzz_0;
        let cast_value: i128 = AsPrimitive::<i128>::as_(value);
        debug_assert_eq!(cast_value, 12345i128);
        let _rug_ed_tests_llm_16_1570_llm_16_1570_rrrruuuugggg_test_as_primitive_u32_to_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1571_llm_16_1571 {
    use crate::cast::AsPrimitive;
    #[test]
    fn u32_as_i16() {
        let _rug_st_tests_llm_16_1571_llm_16_1571_rrrruuuugggg_u32_as_i16 = 0;
        let rug_fuzz_0 = 42;
        let value: u32 = rug_fuzz_0;
        let result: i16 = AsPrimitive::<i16>::as_(value);
        debug_assert_eq!(result, 42i16);
        let _rug_ed_tests_llm_16_1571_llm_16_1571_rrrruuuugggg_u32_as_i16 = 0;
    }
    #[test]
    #[should_panic(expected = "attempt to cast with overflow")]
    fn u32_as_i16_overflow() {
        let _rug_st_tests_llm_16_1571_llm_16_1571_rrrruuuugggg_u32_as_i16_overflow = 0;
        let rug_fuzz_0 = 70000;
        let value: u32 = rug_fuzz_0;
        let _: i16 = AsPrimitive::<i16>::as_(value);
        let _rug_ed_tests_llm_16_1571_llm_16_1571_rrrruuuugggg_u32_as_i16_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1572_llm_16_1572 {
    use crate::AsPrimitive;
    #[test]
    fn test_as_primitive_u32_to_i32() {
        let _rug_st_tests_llm_16_1572_llm_16_1572_rrrruuuugggg_test_as_primitive_u32_to_i32 = 0;
        let rug_fuzz_0 = 123;
        let value_u32: u32 = rug_fuzz_0;
        let value_i32: i32 = value_u32.as_();
        debug_assert_eq!(value_i32, 123i32);
        let _rug_ed_tests_llm_16_1572_llm_16_1572_rrrruuuugggg_test_as_primitive_u32_to_i32 = 0;
    }
    #[test]
    fn test_as_primitive_u32_to_i32_max() {
        let _rug_st_tests_llm_16_1572_llm_16_1572_rrrruuuugggg_test_as_primitive_u32_to_i32_max = 0;
        let value_u32: u32 = u32::MAX;
        let value_i32: i32 = value_u32.as_();
        debug_assert_eq!(value_i32, u32::MAX as i32);
        let _rug_ed_tests_llm_16_1572_llm_16_1572_rrrruuuugggg_test_as_primitive_u32_to_i32_max = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1573_llm_16_1573 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_u32_to_i64() {
        let _rug_st_tests_llm_16_1573_llm_16_1573_rrrruuuugggg_test_as_primitive_u32_to_i64 = 0;
        let rug_fuzz_0 = 42;
        let value: u32 = rug_fuzz_0;
        let casted_value: i64 = AsPrimitive::<i64>::as_(value);
        debug_assert_eq!(casted_value, 42i64);
        let _rug_ed_tests_llm_16_1573_llm_16_1573_rrrruuuugggg_test_as_primitive_u32_to_i64 = 0;
    }
    #[test]
    fn test_as_primitive_u32_max_to_i64() {
        let _rug_st_tests_llm_16_1573_llm_16_1573_rrrruuuugggg_test_as_primitive_u32_max_to_i64 = 0;
        let value: u32 = u32::MAX;
        let casted_value: i64 = AsPrimitive::<i64>::as_(value);
        debug_assert_eq!(casted_value, u32::MAX as i64);
        let _rug_ed_tests_llm_16_1573_llm_16_1573_rrrruuuugggg_test_as_primitive_u32_max_to_i64 = 0;
    }
    #[test]
    fn test_as_primitive_u32_to_i64_negative() {
        let _rug_st_tests_llm_16_1573_llm_16_1573_rrrruuuugggg_test_as_primitive_u32_to_i64_negative = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 0;
        let value: u32 = rug_fuzz_0;
        let casted_value: i64 = AsPrimitive::<i64>::as_(value);
        debug_assert!(casted_value >= rug_fuzz_1);
        let _rug_ed_tests_llm_16_1573_llm_16_1573_rrrruuuugggg_test_as_primitive_u32_to_i64_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1574_llm_16_1574 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_u32_to_i8() {
        let _rug_st_tests_llm_16_1574_llm_16_1574_rrrruuuugggg_test_as_primitive_u32_to_i8 = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 300;
        let val_u32: u32 = rug_fuzz_0;
        let val_i8: i8 = AsPrimitive::<i8>::as_(val_u32);
        debug_assert_eq!(val_i8, 42i8);
        let val_u32: u32 = rug_fuzz_1;
        let val_i8: i8 = AsPrimitive::<i8>::as_(val_u32);
        debug_assert_eq!(val_i8 as u32, 300u32 as i8 as u32);
        let _rug_ed_tests_llm_16_1574_llm_16_1574_rrrruuuugggg_test_as_primitive_u32_to_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1575_llm_16_1575 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_u32_as_isize() {
        let _rug_st_tests_llm_16_1575_llm_16_1575_rrrruuuugggg_test_u32_as_isize = 0;
        let rug_fuzz_0 = 2;
        let value_u32: u32 = u32::max_value() / rug_fuzz_0;
        let casted_value = <u32 as AsPrimitive<isize>>::as_(value_u32);
        debug_assert_eq!(casted_value, (u32::max_value() / 2) as isize);
        let _rug_ed_tests_llm_16_1575_llm_16_1575_rrrruuuugggg_test_u32_as_isize = 0;
    }
    #[test]
    #[should_panic(expected = "attempt to cast with overflow")]
    fn test_u32_as_isize_overflow() {
        let _rug_st_tests_llm_16_1575_llm_16_1575_rrrruuuugggg_test_u32_as_isize_overflow = 0;
        let value_u32: u32 = u32::max_value();
        let _casted_value = <u32 as AsPrimitive<isize>>::as_(value_u32);
        let _rug_ed_tests_llm_16_1575_llm_16_1575_rrrruuuugggg_test_u32_as_isize_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1576_llm_16_1576 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_u32_to_u128() {
        let _rug_st_tests_llm_16_1576_llm_16_1576_rrrruuuugggg_test_as_u32_to_u128 = 0;
        let rug_fuzz_0 = 42;
        let value_u32: u32 = rug_fuzz_0;
        let value_u128: u128 = AsPrimitive::<u128>::as_(value_u32);
        debug_assert_eq!(value_u128, 42u128);
        let _rug_ed_tests_llm_16_1576_llm_16_1576_rrrruuuugggg_test_as_u32_to_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1578_llm_16_1578 {
    use crate::cast::AsPrimitive;
    #[test]
    fn u32_as_u32_identity() {
        let _rug_st_tests_llm_16_1578_llm_16_1578_rrrruuuugggg_u32_as_u32_identity = 0;
        let rug_fuzz_0 = 42;
        let value: u32 = rug_fuzz_0;
        let result: u32 = AsPrimitive::<u32>::as_(value);
        debug_assert_eq!(value, result);
        let _rug_ed_tests_llm_16_1578_llm_16_1578_rrrruuuugggg_u32_as_u32_identity = 0;
    }
    #[test]
    fn u32_as_u32_zero() {
        let _rug_st_tests_llm_16_1578_llm_16_1578_rrrruuuugggg_u32_as_u32_zero = 0;
        let rug_fuzz_0 = 0;
        let value: u32 = rug_fuzz_0;
        let result: u32 = AsPrimitive::<u32>::as_(value);
        debug_assert_eq!(value, result);
        let _rug_ed_tests_llm_16_1578_llm_16_1578_rrrruuuugggg_u32_as_u32_zero = 0;
    }
    #[test]
    fn u32_as_u32_max() {
        let _rug_st_tests_llm_16_1578_llm_16_1578_rrrruuuugggg_u32_as_u32_max = 0;
        let value: u32 = u32::MAX;
        let result: u32 = AsPrimitive::<u32>::as_(value);
        debug_assert_eq!(value, result);
        let _rug_ed_tests_llm_16_1578_llm_16_1578_rrrruuuugggg_u32_as_u32_max = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1579_llm_16_1579 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_u32_to_u64() {
        let _rug_st_tests_llm_16_1579_llm_16_1579_rrrruuuugggg_test_as_primitive_u32_to_u64 = 0;
        let rug_fuzz_0 = 42;
        let value_u32: u32 = rug_fuzz_0;
        let value_u64: u64 = AsPrimitive::<u64>::as_(value_u32);
        debug_assert_eq!(value_u64, 42u64);
        let _rug_ed_tests_llm_16_1579_llm_16_1579_rrrruuuugggg_test_as_primitive_u32_to_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1580 {
    use super::*;
    use crate::*;
    #[test]
    fn test_u32_as_u8_casting() {
        let _rug_st_tests_llm_16_1580_rrrruuuugggg_test_u32_as_u8_casting = 0;
        let rug_fuzz_0 = 0u32;
        let rug_fuzz_1 = 255u32;
        let rug_fuzz_2 = 256u32;
        debug_assert_eq!(< u32 as cast::AsPrimitive < u8 > > ::as_(rug_fuzz_0), 0u8);
        debug_assert_eq!(< u32 as cast::AsPrimitive < u8 > > ::as_(rug_fuzz_1), 255u8);
        debug_assert_eq!(< u32 as cast::AsPrimitive < u8 > > ::as_(rug_fuzz_2), 0u8);
        debug_assert_eq!(< u32 as cast::AsPrimitive < u8 > > ::as_(u32::MAX), 255u8);
        let _rug_ed_tests_llm_16_1580_rrrruuuugggg_test_u32_as_u8_casting = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1581_llm_16_1581 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_u32_to_usize() {
        let _rug_st_tests_llm_16_1581_llm_16_1581_rrrruuuugggg_test_as_primitive_u32_to_usize = 0;
        let rug_fuzz_0 = 42;
        let value: u32 = rug_fuzz_0;
        let as_usize: usize = AsPrimitive::<usize>::as_(value);
        debug_assert_eq!(as_usize, 42usize);
        let _rug_ed_tests_llm_16_1581_llm_16_1581_rrrruuuugggg_test_as_primitive_u32_to_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1582_llm_16_1582 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_f32() {
        let _rug_st_tests_llm_16_1582_llm_16_1582_rrrruuuugggg_test_from_f32 = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 1.0;
        let rug_fuzz_2 = 1.5;
        let rug_fuzz_3 = 1.0;
        let rug_fuzz_4 = 4294967295.0;
        let rug_fuzz_5 = 4294967295.5;
        let rug_fuzz_6 = 4294967296.0;
        let a: f32 = rug_fuzz_0;
        let b: f32 = rug_fuzz_1;
        let c: f32 = rug_fuzz_2;
        let d: f32 = -rug_fuzz_3;
        let e: f32 = f32::MAX;
        let f: f32 = f32::MIN;
        debug_assert_eq!(< u32 as FromPrimitive > ::from_f32(a), Some(0));
        debug_assert_eq!(< u32 as FromPrimitive > ::from_f32(b), Some(1));
        debug_assert_eq!(< u32 as FromPrimitive > ::from_f32(c), Some(1));
        debug_assert_eq!(< u32 as FromPrimitive > ::from_f32(d), None);
        debug_assert_eq!(< u32 as FromPrimitive > ::from_f32(e), None);
        debug_assert_eq!(< u32 as FromPrimitive > ::from_f32(f), None);
        debug_assert_eq!(
            < u32 as FromPrimitive > ::from_f32(rug_fuzz_4), Some(4294967295)
        );
        debug_assert_eq!(
            < u32 as FromPrimitive > ::from_f32(rug_fuzz_5), Some(4294967295)
        );
        debug_assert_eq!(< u32 as FromPrimitive > ::from_f32(rug_fuzz_6), None);
        let _rug_ed_tests_llm_16_1582_llm_16_1582_rrrruuuugggg_test_from_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1583_llm_16_1583 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_f64() {
        let _rug_st_tests_llm_16_1583_llm_16_1583_rrrruuuugggg_test_from_f64 = 0;
        let rug_fuzz_0 = 0.0_f64;
        let rug_fuzz_1 = 0_u32;
        let values = vec![
            (rug_fuzz_0, Some(rug_fuzz_1)), (1.0_f64, Some(1_u32)), (1.999_f64,
            Some(1_u32)), (f64::NAN, None), (f64::INFINITY, None), (f64::NEG_INFINITY,
            None), (- 1.0_f64, None), (4294967295.0_f64, Some(4294967295_u32)),
            (4294967296.0_f64, None), (- 0.9999999999999999_f64, None)
        ];
        for (input, expected) in values {
            debug_assert_eq!(< u32 as FromPrimitive > ::from_f64(input), expected);
        }
        let _rug_ed_tests_llm_16_1583_llm_16_1583_rrrruuuugggg_test_from_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1584_llm_16_1584 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i128_within_bounds() {
        let _rug_st_tests_llm_16_1584_llm_16_1584_rrrruuuugggg_test_from_i128_within_bounds = 0;
        let rug_fuzz_0 = 42;
        let value_within_bounds: i128 = rug_fuzz_0;
        debug_assert_eq!(u32::from_i128(value_within_bounds), Some(42u32));
        let _rug_ed_tests_llm_16_1584_llm_16_1584_rrrruuuugggg_test_from_i128_within_bounds = 0;
    }
    #[test]
    fn test_from_i128_above_bounds() {
        let _rug_st_tests_llm_16_1584_llm_16_1584_rrrruuuugggg_test_from_i128_above_bounds = 0;
        let rug_fuzz_0 = 1;
        let value_above_bounds: i128 = i128::from(u32::MAX) + rug_fuzz_0;
        debug_assert_eq!(u32::from_i128(value_above_bounds), None);
        let _rug_ed_tests_llm_16_1584_llm_16_1584_rrrruuuugggg_test_from_i128_above_bounds = 0;
    }
    #[test]
    fn test_from_i128_below_bounds() {
        let _rug_st_tests_llm_16_1584_llm_16_1584_rrrruuuugggg_test_from_i128_below_bounds = 0;
        let rug_fuzz_0 = 1;
        let value_below_bounds: i128 = -rug_fuzz_0;
        debug_assert_eq!(u32::from_i128(value_below_bounds), None);
        let _rug_ed_tests_llm_16_1584_llm_16_1584_rrrruuuugggg_test_from_i128_below_bounds = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1585_llm_16_1585 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i16() {
        let _rug_st_tests_llm_16_1585_llm_16_1585_rrrruuuugggg_test_from_i16 = 0;
        let rug_fuzz_0 = 0_i16;
        let rug_fuzz_1 = 1_i16;
        let rug_fuzz_2 = 1_i16;
        debug_assert_eq!(< u32 as FromPrimitive > ::from_i16(rug_fuzz_0), Some(0_u32));
        debug_assert_eq!(< u32 as FromPrimitive > ::from_i16(rug_fuzz_1), Some(1_u32));
        debug_assert_eq!(< u32 as FromPrimitive > ::from_i16(- rug_fuzz_2), None);
        debug_assert_eq!(
            < u32 as FromPrimitive > ::from_i16(i16::MAX), Some(i16::MAX as u32)
        );
        let _rug_ed_tests_llm_16_1585_llm_16_1585_rrrruuuugggg_test_from_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1586_llm_16_1586 {
    use crate::FromPrimitive;
    #[test]
    fn test_from_i32_with_positive() {
        let _rug_st_tests_llm_16_1586_llm_16_1586_rrrruuuugggg_test_from_i32_with_positive = 0;
        let rug_fuzz_0 = 123;
        let value: i32 = rug_fuzz_0;
        let result = <u32 as FromPrimitive>::from_i32(value);
        debug_assert_eq!(result, Some(123u32));
        let _rug_ed_tests_llm_16_1586_llm_16_1586_rrrruuuugggg_test_from_i32_with_positive = 0;
    }
    #[test]
    fn test_from_i32_with_zero() {
        let _rug_st_tests_llm_16_1586_llm_16_1586_rrrruuuugggg_test_from_i32_with_zero = 0;
        let rug_fuzz_0 = 0;
        let value: i32 = rug_fuzz_0;
        let result = <u32 as FromPrimitive>::from_i32(value);
        debug_assert_eq!(result, Some(0u32));
        let _rug_ed_tests_llm_16_1586_llm_16_1586_rrrruuuugggg_test_from_i32_with_zero = 0;
    }
    #[test]
    fn test_from_i32_with_negative() {
        let _rug_st_tests_llm_16_1586_llm_16_1586_rrrruuuugggg_test_from_i32_with_negative = 0;
        let rug_fuzz_0 = 123;
        let value: i32 = -rug_fuzz_0;
        let result = <u32 as FromPrimitive>::from_i32(value);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1586_llm_16_1586_rrrruuuugggg_test_from_i32_with_negative = 0;
    }
    #[test]
    fn test_from_i32_with_max_i32() {
        let _rug_st_tests_llm_16_1586_llm_16_1586_rrrruuuugggg_test_from_i32_with_max_i32 = 0;
        let value: i32 = i32::MAX;
        let result = <u32 as FromPrimitive>::from_i32(value);
        debug_assert_eq!(result, Some(i32::MAX as u32));
        let _rug_ed_tests_llm_16_1586_llm_16_1586_rrrruuuugggg_test_from_i32_with_max_i32 = 0;
    }
    #[test]
    fn test_from_i32_with_min_i32() {
        let _rug_st_tests_llm_16_1586_llm_16_1586_rrrruuuugggg_test_from_i32_with_min_i32 = 0;
        let value: i32 = i32::MIN;
        let result = <u32 as FromPrimitive>::from_i32(value);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1586_llm_16_1586_rrrruuuugggg_test_from_i32_with_min_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1587_llm_16_1587 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i64_with_positive_value_within_bounds() {
        let _rug_st_tests_llm_16_1587_llm_16_1587_rrrruuuugggg_test_from_i64_with_positive_value_within_bounds = 0;
        let rug_fuzz_0 = 123;
        let value: i64 = rug_fuzz_0;
        let result = <u32 as FromPrimitive>::from_i64(value);
        debug_assert_eq!(result, Some(123u32));
        let _rug_ed_tests_llm_16_1587_llm_16_1587_rrrruuuugggg_test_from_i64_with_positive_value_within_bounds = 0;
    }
    #[test]
    fn test_from_i64_with_negative_value() {
        let _rug_st_tests_llm_16_1587_llm_16_1587_rrrruuuugggg_test_from_i64_with_negative_value = 0;
        let rug_fuzz_0 = 1;
        let value: i64 = -rug_fuzz_0;
        let result = <u32 as FromPrimitive>::from_i64(value);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1587_llm_16_1587_rrrruuuugggg_test_from_i64_with_negative_value = 0;
    }
    #[test]
    fn test_from_i64_with_positive_value_out_of_bounds() {
        let _rug_st_tests_llm_16_1587_llm_16_1587_rrrruuuugggg_test_from_i64_with_positive_value_out_of_bounds = 0;
        let rug_fuzz_0 = 1;
        let value: i64 = u32::MAX as i64 + rug_fuzz_0;
        let result = <u32 as FromPrimitive>::from_i64(value);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1587_llm_16_1587_rrrruuuugggg_test_from_i64_with_positive_value_out_of_bounds = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1588_llm_16_1588 {
    use crate::cast::FromPrimitive;
    #[test]
    fn from_i8_test() {
        let _rug_st_tests_llm_16_1588_llm_16_1588_rrrruuuugggg_from_i8_test = 0;
        let rug_fuzz_0 = 0_i8;
        let rug_fuzz_1 = 1_i8;
        let rug_fuzz_2 = 127_i8;
        debug_assert_eq!(< u32 as FromPrimitive > ::from_i8(rug_fuzz_0), Some(0_u32));
        debug_assert_eq!(< u32 as FromPrimitive > ::from_i8(- rug_fuzz_1), None);
        debug_assert_eq!(< u32 as FromPrimitive > ::from_i8(rug_fuzz_2), Some(127_u32));
        let _rug_ed_tests_llm_16_1588_llm_16_1588_rrrruuuugggg_from_i8_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1589_llm_16_1589 {
    use super::*;
    use crate::*;
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_isize() {
        let _rug_st_tests_llm_16_1589_llm_16_1589_rrrruuuugggg_test_from_isize = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 123;
        let rug_fuzz_2 = 1;
        debug_assert_eq!(
            < u32 as FromPrimitive > ::from_isize(usize::MAX as isize), None
        );
        debug_assert_eq!(< u32 as FromPrimitive > ::from_isize(rug_fuzz_0), Some(0u32));
        debug_assert_eq!(< u32 as FromPrimitive > ::from_isize(isize::MAX), None);
        debug_assert_eq!(< u32 as FromPrimitive > ::from_isize(isize::MIN), None);
        debug_assert_eq!(
            < u32 as FromPrimitive > ::from_isize(rug_fuzz_1), Some(123u32)
        );
        debug_assert_eq!(< u32 as FromPrimitive > ::from_isize(- rug_fuzz_2), None);
        let _rug_ed_tests_llm_16_1589_llm_16_1589_rrrruuuugggg_test_from_isize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1590 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u128() {
        let _rug_st_tests_llm_16_1590_rrrruuuugggg_test_from_u128 = 0;
        let rug_fuzz_0 = 0u128;
        let rug_fuzz_1 = 1;
        debug_assert_eq!(< u32 as FromPrimitive > ::from_u128(rug_fuzz_0), Some(0u32));
        debug_assert_eq!(
            < u32 as FromPrimitive > ::from_u128(u32::MAX as u128), Some(u32::MAX)
        );
        debug_assert_eq!(
            < u32 as FromPrimitive > ::from_u128((u32::MAX as u128) + rug_fuzz_1), None
        );
        debug_assert_eq!(< u32 as FromPrimitive > ::from_u128(u128::MAX), None);
        let _rug_ed_tests_llm_16_1590_rrrruuuugggg_test_from_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1591_llm_16_1591 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u16() {
        let _rug_st_tests_llm_16_1591_llm_16_1591_rrrruuuugggg_test_from_u16 = 0;
        let rug_fuzz_0 = 0_u16;
        let rug_fuzz_1 = 12345_u16;
        debug_assert_eq!(< u32 as FromPrimitive > ::from_u16(rug_fuzz_0), Some(0_u32));
        debug_assert_eq!(
            < u32 as FromPrimitive > ::from_u16(rug_fuzz_1), Some(12345_u32)
        );
        debug_assert_eq!(
            < u32 as FromPrimitive > ::from_u16(u16::MAX), Some(u32::from(u16::MAX))
        );
        let _rug_ed_tests_llm_16_1591_llm_16_1591_rrrruuuugggg_test_from_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1592_llm_16_1592 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u32() {
        let _rug_st_tests_llm_16_1592_llm_16_1592_rrrruuuugggg_test_from_u32 = 0;
        let rug_fuzz_0 = 42;
        let value: u32 = rug_fuzz_0;
        let result = <u32 as FromPrimitive>::from_u32(value);
        debug_assert_eq!(result, Some(value));
        let _rug_ed_tests_llm_16_1592_llm_16_1592_rrrruuuugggg_test_from_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1593_llm_16_1593 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u64() {
        let _rug_st_tests_llm_16_1593_llm_16_1593_rrrruuuugggg_test_from_u64 = 0;
        let rug_fuzz_0 = 0_u64;
        let rug_fuzz_1 = 1;
        debug_assert_eq!(< u32 as FromPrimitive > ::from_u64(rug_fuzz_0), Some(0_u32));
        debug_assert_eq!(
            < u32 as FromPrimitive > ::from_u64(u32::MAX as u64), Some(u32::MAX)
        );
        debug_assert_eq!(
            < u32 as FromPrimitive > ::from_u64((u32::MAX as u64) + rug_fuzz_1), None
        );
        debug_assert_eq!(< u32 as FromPrimitive > ::from_u64(u64::MAX), None);
        let _rug_ed_tests_llm_16_1593_llm_16_1593_rrrruuuugggg_test_from_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1594_llm_16_1594 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u8() {
        let _rug_st_tests_llm_16_1594_llm_16_1594_rrrruuuugggg_test_from_u8 = 0;
        let rug_fuzz_0 = 0_u8;
        let rug_fuzz_1 = 127_u8;
        let rug_fuzz_2 = 255_u8;
        debug_assert_eq!(< u32 as FromPrimitive > ::from_u8(rug_fuzz_0), Some(0_u32));
        debug_assert_eq!(< u32 as FromPrimitive > ::from_u8(rug_fuzz_1), Some(127_u32));
        debug_assert_eq!(< u32 as FromPrimitive > ::from_u8(rug_fuzz_2), Some(255_u32));
        let _rug_ed_tests_llm_16_1594_llm_16_1594_rrrruuuugggg_test_from_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1595_llm_16_1595 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_usize() {
        let _rug_st_tests_llm_16_1595_llm_16_1595_rrrruuuugggg_test_from_usize = 0;
        let rug_fuzz_0 = 1;
        let small_usize: usize = u32::MAX as usize;
        debug_assert_eq!(
            < u32 as FromPrimitive > ::from_usize(small_usize), Some(u32::MAX)
        );
        let big_usize: usize = (u32::MAX as usize).wrapping_add(rug_fuzz_0);
        debug_assert_eq!(< u32 as FromPrimitive > ::from_usize(big_usize), None);
        let _rug_ed_tests_llm_16_1595_llm_16_1595_rrrruuuugggg_test_from_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1596_llm_16_1596 {
    use crate::cast::{NumCast, ToPrimitive};
    use crate::FromPrimitive;
    use std::num::Wrapping;
    #[test]
    fn test_u32_from_wrapping_u32() {
        let _rug_st_tests_llm_16_1596_llm_16_1596_rrrruuuugggg_test_u32_from_wrapping_u32 = 0;
        let rug_fuzz_0 = 42u32;
        let wrapped_u32 = Wrapping(rug_fuzz_0);
        let result = <u32 as NumCast>::from(wrapped_u32);
        debug_assert_eq!(result, Some(42u32));
        let _rug_ed_tests_llm_16_1596_llm_16_1596_rrrruuuugggg_test_u32_from_wrapping_u32 = 0;
    }
    #[test]
    fn test_u32_from_wrapping_i32() {
        let _rug_st_tests_llm_16_1596_llm_16_1596_rrrruuuugggg_test_u32_from_wrapping_i32 = 0;
        let rug_fuzz_0 = 42i32;
        let wrapped_i32 = Wrapping(rug_fuzz_0);
        let result = <u32 as NumCast>::from(wrapped_i32);
        debug_assert_eq!(result, Some(42u32));
        let _rug_ed_tests_llm_16_1596_llm_16_1596_rrrruuuugggg_test_u32_from_wrapping_i32 = 0;
    }
    #[test]
    fn test_u32_from_wrapping_usize() {
        let _rug_st_tests_llm_16_1596_llm_16_1596_rrrruuuugggg_test_u32_from_wrapping_usize = 0;
        let rug_fuzz_0 = 42usize;
        let wrapped_usize = Wrapping(rug_fuzz_0);
        let result = <u32 as NumCast>::from(wrapped_usize);
        debug_assert_eq!(result, Some(42u32));
        let _rug_ed_tests_llm_16_1596_llm_16_1596_rrrruuuugggg_test_u32_from_wrapping_usize = 0;
    }
    #[test]
    fn test_u32_from_wrapping_negative_i32() {
        let _rug_st_tests_llm_16_1596_llm_16_1596_rrrruuuugggg_test_u32_from_wrapping_negative_i32 = 0;
        let rug_fuzz_0 = 42i32;
        let wrapped_i32 = Wrapping(-rug_fuzz_0);
        let result = <u32 as NumCast>::from(wrapped_i32);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1596_llm_16_1596_rrrruuuugggg_test_u32_from_wrapping_negative_i32 = 0;
    }
    #[test]
    fn test_u32_from_wrapping_u64() {
        let _rug_st_tests_llm_16_1596_llm_16_1596_rrrruuuugggg_test_u32_from_wrapping_u64 = 0;
        let rug_fuzz_0 = 42u64;
        let wrapped_u64 = Wrapping(rug_fuzz_0);
        let result = <u32 as NumCast>::from(wrapped_u64);
        debug_assert_eq!(result, Some(42u32));
        let _rug_ed_tests_llm_16_1596_llm_16_1596_rrrruuuugggg_test_u32_from_wrapping_u64 = 0;
    }
    #[test]
    fn test_u32_from_wrapping_large_u64() {
        let _rug_st_tests_llm_16_1596_llm_16_1596_rrrruuuugggg_test_u32_from_wrapping_large_u64 = 0;
        let wrapped_u64 = Wrapping(u64::max_value());
        let result = <u32 as NumCast>::from(wrapped_u64);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1596_llm_16_1596_rrrruuuugggg_test_u32_from_wrapping_large_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1597_llm_16_1597 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_u32_to_f32() {
        let _rug_st_tests_llm_16_1597_llm_16_1597_rrrruuuugggg_test_u32_to_f32 = 0;
        let rug_fuzz_0 = 42u32;
        let num = rug_fuzz_0;
        let result = ToPrimitive::to_f32(&num);
        debug_assert_eq!(result, Some(42f32));
        let _rug_ed_tests_llm_16_1597_llm_16_1597_rrrruuuugggg_test_u32_to_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1598_llm_16_1598 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_u32_to_f64() {
        let _rug_st_tests_llm_16_1598_llm_16_1598_rrrruuuugggg_test_u32_to_f64 = 0;
        let rug_fuzz_0 = 42;
        let val: u32 = rug_fuzz_0;
        let float_val: Option<f64> = val.to_f64();
        debug_assert_eq!(float_val, Some(42.0f64));
        let _rug_ed_tests_llm_16_1598_llm_16_1598_rrrruuuugggg_test_u32_to_f64 = 0;
    }
    #[test]
    fn test_u32_to_f64_max_value() {
        let _rug_st_tests_llm_16_1598_llm_16_1598_rrrruuuugggg_test_u32_to_f64_max_value = 0;
        let val: u32 = u32::MAX;
        let float_val: Option<f64> = val.to_f64();
        debug_assert_eq!(float_val, Some(u32::MAX as f64));
        let _rug_ed_tests_llm_16_1598_llm_16_1598_rrrruuuugggg_test_u32_to_f64_max_value = 0;
    }
    #[test]
    fn test_u32_to_f64_zero() {
        let _rug_st_tests_llm_16_1598_llm_16_1598_rrrruuuugggg_test_u32_to_f64_zero = 0;
        let rug_fuzz_0 = 0;
        let val: u32 = rug_fuzz_0;
        let float_val: Option<f64> = val.to_f64();
        debug_assert_eq!(float_val, Some(0.0f64));
        let _rug_ed_tests_llm_16_1598_llm_16_1598_rrrruuuugggg_test_u32_to_f64_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1599_llm_16_1599 {
    use crate::cast::ToPrimitive;
    #[test]
    fn u32_to_i128_success() {
        let _rug_st_tests_llm_16_1599_llm_16_1599_rrrruuuugggg_u32_to_i128_success = 0;
        let value: u32 = u32::MAX;
        let result = value.to_i128();
        debug_assert_eq!(result, Some(i128::from(u32::MAX)));
        let _rug_ed_tests_llm_16_1599_llm_16_1599_rrrruuuugggg_u32_to_i128_success = 0;
    }
    #[test]
    fn u32_to_i128_small_value_success() {
        let _rug_st_tests_llm_16_1599_llm_16_1599_rrrruuuugggg_u32_to_i128_small_value_success = 0;
        let rug_fuzz_0 = 42;
        let value: u32 = rug_fuzz_0;
        let result = value.to_i128();
        debug_assert_eq!(result, Some(42i128));
        let _rug_ed_tests_llm_16_1599_llm_16_1599_rrrruuuugggg_u32_to_i128_small_value_success = 0;
    }
    #[test]
    fn u32_to_i128_zero_success() {
        let _rug_st_tests_llm_16_1599_llm_16_1599_rrrruuuugggg_u32_to_i128_zero_success = 0;
        let rug_fuzz_0 = 0;
        let value: u32 = rug_fuzz_0;
        let result = value.to_i128();
        debug_assert_eq!(result, Some(0i128));
        let _rug_ed_tests_llm_16_1599_llm_16_1599_rrrruuuugggg_u32_to_i128_zero_success = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1600_llm_16_1600 {
    use crate::cast::ToPrimitive;
    #[test]
    fn u32_to_i16_max_value() {
        let _rug_st_tests_llm_16_1600_llm_16_1600_rrrruuuugggg_u32_to_i16_max_value = 0;
        let max_u32_for_i16 = i16::MAX as u32;
        debug_assert_eq!(max_u32_for_i16.to_i16(), Some(i16::MAX));
        let _rug_ed_tests_llm_16_1600_llm_16_1600_rrrruuuugggg_u32_to_i16_max_value = 0;
    }
    #[test]
    fn u32_to_i16_within_bounds() {
        let _rug_st_tests_llm_16_1600_llm_16_1600_rrrruuuugggg_u32_to_i16_within_bounds = 0;
        let rug_fuzz_0 = 32767u32;
        let value = rug_fuzz_0;
        debug_assert_eq!(value.to_i16(), Some(32767i16));
        let _rug_ed_tests_llm_16_1600_llm_16_1600_rrrruuuugggg_u32_to_i16_within_bounds = 0;
    }
    #[test]
    fn u32_to_i16_exceed_bounds() {
        let _rug_st_tests_llm_16_1600_llm_16_1600_rrrruuuugggg_u32_to_i16_exceed_bounds = 0;
        let rug_fuzz_0 = 65535u32;
        let value = rug_fuzz_0;
        debug_assert_eq!(value.to_i16(), None);
        let _rug_ed_tests_llm_16_1600_llm_16_1600_rrrruuuugggg_u32_to_i16_exceed_bounds = 0;
    }
    #[test]
    fn u32_to_i16_zero() {
        let _rug_st_tests_llm_16_1600_llm_16_1600_rrrruuuugggg_u32_to_i16_zero = 0;
        let rug_fuzz_0 = 0u32;
        let value = rug_fuzz_0;
        debug_assert_eq!(value.to_i16(), Some(0i16));
        let _rug_ed_tests_llm_16_1600_llm_16_1600_rrrruuuugggg_u32_to_i16_zero = 0;
    }
    #[test]
    fn u32_to_i16_exactly_i16_max() {
        let _rug_st_tests_llm_16_1600_llm_16_1600_rrrruuuugggg_u32_to_i16_exactly_i16_max = 0;
        let value = i16::MAX as u32;
        debug_assert_eq!(value.to_i16(), Some(i16::MAX));
        let _rug_ed_tests_llm_16_1600_llm_16_1600_rrrruuuugggg_u32_to_i16_exactly_i16_max = 0;
    }
    #[test]
    fn u32_to_i16_one_more_than_i16_max() {
        let _rug_st_tests_llm_16_1600_llm_16_1600_rrrruuuugggg_u32_to_i16_one_more_than_i16_max = 0;
        let rug_fuzz_0 = 1;
        let value = i16::MAX as u32 + rug_fuzz_0;
        debug_assert_eq!(value.to_i16(), None);
        let _rug_ed_tests_llm_16_1600_llm_16_1600_rrrruuuugggg_u32_to_i16_one_more_than_i16_max = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1601_llm_16_1601 {
    use crate::cast::ToPrimitive;
    #[test]
    fn u32_to_i32_cast_within_bounds() {
        let _rug_st_tests_llm_16_1601_llm_16_1601_rrrruuuugggg_u32_to_i32_cast_within_bounds = 0;
        let rug_fuzz_0 = 0u32;
        let rug_fuzz_1 = 2147483647u32;
        debug_assert_eq!((rug_fuzz_0).to_i32(), Some(0i32));
        debug_assert_eq!((rug_fuzz_1).to_i32(), Some(2147483647i32));
        let _rug_ed_tests_llm_16_1601_llm_16_1601_rrrruuuugggg_u32_to_i32_cast_within_bounds = 0;
    }
    #[test]
    fn u32_to_i32_cast_out_of_bounds() {
        let _rug_st_tests_llm_16_1601_llm_16_1601_rrrruuuugggg_u32_to_i32_cast_out_of_bounds = 0;
        let rug_fuzz_0 = 2147483648u32;
        debug_assert_eq!((rug_fuzz_0).to_i32(), None);
        debug_assert_eq!(u32::MAX.to_i32(), None);
        let _rug_ed_tests_llm_16_1601_llm_16_1601_rrrruuuugggg_u32_to_i32_cast_out_of_bounds = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1602 {
    use super::*;
    use crate::*;
    #[test]
    fn test_u32_to_i64_within_bounds() {
        let _rug_st_tests_llm_16_1602_rrrruuuugggg_test_u32_to_i64_within_bounds = 0;
        let rug_fuzz_0 = 42;
        let value: u32 = rug_fuzz_0;
        let result = value.to_i64();
        debug_assert_eq!(result, Some(42_i64));
        let _rug_ed_tests_llm_16_1602_rrrruuuugggg_test_u32_to_i64_within_bounds = 0;
    }
    #[test]
    fn test_u32_to_i64_at_max() {
        let _rug_st_tests_llm_16_1602_rrrruuuugggg_test_u32_to_i64_at_max = 0;
        let value: u32 = i64::MAX as u32;
        let result = value.to_i64();
        debug_assert_eq!(result, Some(i64::MAX));
        let _rug_ed_tests_llm_16_1602_rrrruuuugggg_test_u32_to_i64_at_max = 0;
    }
    #[test]
    fn test_u32_to_i64_above_max() {
        let _rug_st_tests_llm_16_1602_rrrruuuugggg_test_u32_to_i64_above_max = 0;
        let rug_fuzz_0 = 1;
        let value: u32 = (i64::MAX as u32).wrapping_add(rug_fuzz_0);
        let result = value.to_i64();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1602_rrrruuuugggg_test_u32_to_i64_above_max = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1603 {
    use crate::ToPrimitive;
    #[test]
    fn u32_to_i8() {
        let _rug_st_tests_llm_16_1603_rrrruuuugggg_u32_to_i8 = 0;
        let rug_fuzz_0 = 0_u32;
        let rug_fuzz_1 = 127_u32;
        let rug_fuzz_2 = 128_u32;
        let rug_fuzz_3 = 255_u32;
        let rug_fuzz_4 = 256_u32;
        debug_assert_eq!(rug_fuzz_0.to_i8(), Some(0_i8));
        debug_assert_eq!(rug_fuzz_1.to_i8(), Some(127_i8));
        debug_assert_eq!(rug_fuzz_2.to_i8(), Some(- 128_i8));
        debug_assert_eq!(rug_fuzz_3.to_i8(), None);
        debug_assert_eq!(rug_fuzz_4.to_i8(), None);
        debug_assert_eq!(u32::max_value().to_i8(), None);
        let _rug_ed_tests_llm_16_1603_rrrruuuugggg_u32_to_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1604_llm_16_1604 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_u32_to_isize_within_range() {
        let _rug_st_tests_llm_16_1604_llm_16_1604_rrrruuuugggg_test_u32_to_isize_within_range = 0;
        let value: u32 = isize::MAX as u32;
        debug_assert_eq!(value.to_isize(), Some(isize::MAX));
        let _rug_ed_tests_llm_16_1604_llm_16_1604_rrrruuuugggg_test_u32_to_isize_within_range = 0;
    }
    #[test]
    fn test_u32_to_isize_out_of_range() {
        let _rug_st_tests_llm_16_1604_llm_16_1604_rrrruuuugggg_test_u32_to_isize_out_of_range = 0;
        let rug_fuzz_0 = 1;
        let value: u32 = isize::MAX as u32;
        let out_of_range_value = value.wrapping_add(rug_fuzz_0);
        debug_assert_eq!(out_of_range_value.to_isize(), None);
        let _rug_ed_tests_llm_16_1604_llm_16_1604_rrrruuuugggg_test_u32_to_isize_out_of_range = 0;
    }
    #[test]
    fn test_u32_to_isize_zero() {
        let _rug_st_tests_llm_16_1604_llm_16_1604_rrrruuuugggg_test_u32_to_isize_zero = 0;
        let rug_fuzz_0 = 0;
        let value: u32 = rug_fuzz_0;
        debug_assert_eq!(value.to_isize(), Some(0));
        let _rug_ed_tests_llm_16_1604_llm_16_1604_rrrruuuugggg_test_u32_to_isize_zero = 0;
    }
    #[test]
    fn test_u32_to_isize_negative() {
        let _rug_st_tests_llm_16_1604_llm_16_1604_rrrruuuugggg_test_u32_to_isize_negative = 0;
        let value: u32 = isize::MIN as u32;
        debug_assert_eq!(value.to_isize(), Some(isize::MIN));
        let _rug_ed_tests_llm_16_1604_llm_16_1604_rrrruuuugggg_test_u32_to_isize_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1605_llm_16_1605 {
    use crate::cast::ToPrimitive;
    #[test]
    fn u32_to_u128() {
        let _rug_st_tests_llm_16_1605_llm_16_1605_rrrruuuugggg_u32_to_u128 = 0;
        let rug_fuzz_0 = 0u32;
        let rug_fuzz_1 = 1u32;
        debug_assert_eq!(rug_fuzz_0.to_u128(), Some(0u128));
        debug_assert_eq!(rug_fuzz_1.to_u128(), Some(1u128));
        debug_assert_eq!(u32::MAX.to_u128(), Some(u32::MAX as u128));
        let _rug_ed_tests_llm_16_1605_llm_16_1605_rrrruuuugggg_u32_to_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1606_llm_16_1606 {
    use crate::ToPrimitive;
    #[test]
    fn test_to_u16_with_u32_within_range() {
        let _rug_st_tests_llm_16_1606_llm_16_1606_rrrruuuugggg_test_to_u16_with_u32_within_range = 0;
        let value: u32 = u16::MAX as u32;
        debug_assert_eq!(value.to_u16(), Some(u16::MAX));
        let _rug_ed_tests_llm_16_1606_llm_16_1606_rrrruuuugggg_test_to_u16_with_u32_within_range = 0;
    }
    #[test]
    fn test_to_u16_with_u32_out_of_range() {
        let _rug_st_tests_llm_16_1606_llm_16_1606_rrrruuuugggg_test_to_u16_with_u32_out_of_range = 0;
        let rug_fuzz_0 = 1;
        let value: u32 = u16::MAX as u32 + rug_fuzz_0;
        debug_assert_eq!(value.to_u16(), None);
        let _rug_ed_tests_llm_16_1606_llm_16_1606_rrrruuuugggg_test_to_u16_with_u32_out_of_range = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1607_llm_16_1607 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_u32_with_u32() {
        let _rug_st_tests_llm_16_1607_llm_16_1607_rrrruuuugggg_test_to_u32_with_u32 = 0;
        let rug_fuzz_0 = 123;
        let x: u32 = rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_u32(& x), Some(123));
        let _rug_ed_tests_llm_16_1607_llm_16_1607_rrrruuuugggg_test_to_u32_with_u32 = 0;
    }
    #[test]
    fn test_to_u32_with_i32() {
        let _rug_st_tests_llm_16_1607_llm_16_1607_rrrruuuugggg_test_to_u32_with_i32 = 0;
        let rug_fuzz_0 = 123;
        let x: i32 = rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_u32(& x), Some(123));
        let _rug_ed_tests_llm_16_1607_llm_16_1607_rrrruuuugggg_test_to_u32_with_i32 = 0;
    }
    #[test]
    fn test_to_u32_with_i32_negative() {
        let _rug_st_tests_llm_16_1607_llm_16_1607_rrrruuuugggg_test_to_u32_with_i32_negative = 0;
        let rug_fuzz_0 = 123;
        let x: i32 = -rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_u32(& x), None);
        let _rug_ed_tests_llm_16_1607_llm_16_1607_rrrruuuugggg_test_to_u32_with_i32_negative = 0;
    }
    #[test]
    fn test_to_u32_with_i64_large() {
        let _rug_st_tests_llm_16_1607_llm_16_1607_rrrruuuugggg_test_to_u32_with_i64_large = 0;
        let rug_fuzz_0 = 5_000_000_000;
        let x: i64 = rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_u32(& x), None);
        let _rug_ed_tests_llm_16_1607_llm_16_1607_rrrruuuugggg_test_to_u32_with_i64_large = 0;
    }
    #[test]
    fn test_to_u32_with_i64_small() {
        let _rug_st_tests_llm_16_1607_llm_16_1607_rrrruuuugggg_test_to_u32_with_i64_small = 0;
        let rug_fuzz_0 = 123;
        let x: i64 = rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_u32(& x), Some(123));
        let _rug_ed_tests_llm_16_1607_llm_16_1607_rrrruuuugggg_test_to_u32_with_i64_small = 0;
    }
    #[test]
    fn test_to_u32_with_u64_large() {
        let _rug_st_tests_llm_16_1607_llm_16_1607_rrrruuuugggg_test_to_u32_with_u64_large = 0;
        let x: u64 = u64::MAX;
        debug_assert_eq!(ToPrimitive::to_u32(& x), None);
        let _rug_ed_tests_llm_16_1607_llm_16_1607_rrrruuuugggg_test_to_u32_with_u64_large = 0;
    }
    #[test]
    fn test_to_u32_with_u64_small() {
        let _rug_st_tests_llm_16_1607_llm_16_1607_rrrruuuugggg_test_to_u32_with_u64_small = 0;
        let rug_fuzz_0 = 123;
        let x: u64 = rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_u32(& x), Some(123));
        let _rug_ed_tests_llm_16_1607_llm_16_1607_rrrruuuugggg_test_to_u32_with_u64_small = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1608_llm_16_1608 {
    use crate::ToPrimitive;
    #[test]
    fn test_u32_to_u64() {
        let _rug_st_tests_llm_16_1608_llm_16_1608_rrrruuuugggg_test_u32_to_u64 = 0;
        let rug_fuzz_0 = 42_u32;
        let val = rug_fuzz_0;
        debug_assert_eq!(val.to_u64(), Some(42_u64));
        let val_max = u32::MAX;
        debug_assert_eq!(val_max.to_u64(), Some(u32::MAX as u64));
        let val_min = u32::MIN;
        debug_assert_eq!(val_min.to_u64(), Some(0_u64));
        let _rug_ed_tests_llm_16_1608_llm_16_1608_rrrruuuugggg_test_u32_to_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1609 {
    use super::*;
    use crate::*;
    #[test]
    fn test_to_u8_with_in_range_value() {
        let _rug_st_tests_llm_16_1609_rrrruuuugggg_test_to_u8_with_in_range_value = 0;
        let rug_fuzz_0 = 100;
        let value: u32 = rug_fuzz_0;
        let result = <u32 as cast::ToPrimitive>::to_u8(&value);
        debug_assert_eq!(result, Some(100u8));
        let _rug_ed_tests_llm_16_1609_rrrruuuugggg_test_to_u8_with_in_range_value = 0;
    }
    #[test]
    fn test_to_u8_with_out_of_range_value() {
        let _rug_st_tests_llm_16_1609_rrrruuuugggg_test_to_u8_with_out_of_range_value = 0;
        let rug_fuzz_0 = 1000;
        let value: u32 = rug_fuzz_0;
        let result = <u32 as cast::ToPrimitive>::to_u8(&value);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1609_rrrruuuugggg_test_to_u8_with_out_of_range_value = 0;
    }
    #[test]
    fn test_to_u8_with_max_u8_value() {
        let _rug_st_tests_llm_16_1609_rrrruuuugggg_test_to_u8_with_max_u8_value = 0;
        let value: u32 = u8::MAX.into();
        let result = <u32 as cast::ToPrimitive>::to_u8(&value);
        debug_assert_eq!(result, Some(u8::MAX));
        let _rug_ed_tests_llm_16_1609_rrrruuuugggg_test_to_u8_with_max_u8_value = 0;
    }
    #[test]
    fn test_to_u8_with_zero() {
        let _rug_st_tests_llm_16_1609_rrrruuuugggg_test_to_u8_with_zero = 0;
        let rug_fuzz_0 = 0;
        let value: u32 = rug_fuzz_0;
        let result = <u32 as cast::ToPrimitive>::to_u8(&value);
        debug_assert_eq!(result, Some(0u8));
        let _rug_ed_tests_llm_16_1609_rrrruuuugggg_test_to_u8_with_zero = 0;
    }
    #[test]
    fn test_to_u8_with_max_u32_value() {
        let _rug_st_tests_llm_16_1609_rrrruuuugggg_test_to_u8_with_max_u32_value = 0;
        let value: u32 = u32::MAX;
        let result = <u32 as cast::ToPrimitive>::to_u8(&value);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1609_rrrruuuugggg_test_to_u8_with_max_u32_value = 0;
    }
    #[test]
    fn test_to_u8_with_boundary_value() {
        let _rug_st_tests_llm_16_1609_rrrruuuugggg_test_to_u8_with_boundary_value = 0;
        let rug_fuzz_0 = 255;
        let rug_fuzz_1 = 256;
        let value: u32 = rug_fuzz_0;
        let result = <u32 as cast::ToPrimitive>::to_u8(&value);
        debug_assert_eq!(result, Some(u8::MAX));
        let value: u32 = rug_fuzz_1;
        let result = <u32 as cast::ToPrimitive>::to_u8(&value);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1609_rrrruuuugggg_test_to_u8_with_boundary_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1610_llm_16_1610 {
    use crate::ToPrimitive;
    #[test]
    fn to_usize_within_bounds() {
        let _rug_st_tests_llm_16_1610_llm_16_1610_rrrruuuugggg_to_usize_within_bounds = 0;
        let rug_fuzz_0 = 2;
        let value: u32 = u32::MAX / rug_fuzz_0;
        let result = value.to_usize();
        debug_assert_eq!(result, Some(value as usize));
        let _rug_ed_tests_llm_16_1610_llm_16_1610_rrrruuuugggg_to_usize_within_bounds = 0;
    }
    #[test]
    fn to_usize_max_value() {
        let _rug_st_tests_llm_16_1610_llm_16_1610_rrrruuuugggg_to_usize_max_value = 0;
        let value: u32 = u32::MAX;
        if value as u64 <= usize::MAX as u64 {
            let result = value.to_usize();
            debug_assert_eq!(result, Some(value as usize));
        } else {
            let result = value.to_usize();
            debug_assert_eq!(result, None);
        }
        let _rug_ed_tests_llm_16_1610_llm_16_1610_rrrruuuugggg_to_usize_max_value = 0;
    }
    #[test]
    fn to_usize_zero() {
        let _rug_st_tests_llm_16_1610_llm_16_1610_rrrruuuugggg_to_usize_zero = 0;
        let rug_fuzz_0 = 0;
        let value: u32 = rug_fuzz_0;
        let result = value.to_usize();
        debug_assert_eq!(result, Some(0));
        let _rug_ed_tests_llm_16_1610_llm_16_1610_rrrruuuugggg_to_usize_zero = 0;
    }
    #[test]
    fn to_usize_overflow() {
        let _rug_st_tests_llm_16_1610_llm_16_1610_rrrruuuugggg_to_usize_overflow = 0;
        let value: u32 = u32::MAX;
        if usize::MAX < u32::MAX as usize {
            let result = value.to_usize();
            debug_assert_eq!(result, None);
        }
        let _rug_ed_tests_llm_16_1610_llm_16_1610_rrrruuuugggg_to_usize_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1673_llm_16_1673 {
    use crate::AsPrimitive;
    #[test]
    fn test_u64_as_f32() {
        let _rug_st_tests_llm_16_1673_llm_16_1673_rrrruuuugggg_test_u64_as_f32 = 0;
        let rug_fuzz_0 = 42;
        let value_u64: u64 = rug_fuzz_0;
        let value_f32: f32 = value_u64.as_();
        debug_assert_eq!(value_f32, 42.0f32);
        let _rug_ed_tests_llm_16_1673_llm_16_1673_rrrruuuugggg_test_u64_as_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1674_llm_16_1674 {
    use crate::cast::AsPrimitive;
    #[test]
    fn u64_as_f64() {
        let _rug_st_tests_llm_16_1674_llm_16_1674_rrrruuuugggg_u64_as_f64 = 0;
        let rug_fuzz_0 = 42;
        let x: u64 = rug_fuzz_0;
        let y: f64 = AsPrimitive::<f64>::as_(x);
        debug_assert_eq!(y, 42f64);
        let _rug_ed_tests_llm_16_1674_llm_16_1674_rrrruuuugggg_u64_as_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1675_llm_16_1675 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_u64_to_i128() {
        let _rug_st_tests_llm_16_1675_llm_16_1675_rrrruuuugggg_test_as_primitive_u64_to_i128 = 0;
        let rug_fuzz_0 = 42;
        let value: u64 = rug_fuzz_0;
        let result: i128 = <u64 as AsPrimitive<i128>>::as_(value);
        debug_assert_eq!(result, 42i128);
        let _rug_ed_tests_llm_16_1675_llm_16_1675_rrrruuuugggg_test_as_primitive_u64_to_i128 = 0;
    }
    #[test]
    #[should_panic(expected = "attempt to cast with overflow")]
    fn test_as_primitive_u64_to_i128_overflow() {
        let _rug_st_tests_llm_16_1675_llm_16_1675_rrrruuuugggg_test_as_primitive_u64_to_i128_overflow = 0;
        let value: u64 = u64::MAX;
        let _result: i128 = <u64 as AsPrimitive<i128>>::as_(value);
        let _rug_ed_tests_llm_16_1675_llm_16_1675_rrrruuuugggg_test_as_primitive_u64_to_i128_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1676_llm_16_1676 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_u64_to_i16() {
        let _rug_st_tests_llm_16_1676_llm_16_1676_rrrruuuugggg_test_as_primitive_u64_to_i16 = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42;
        let val_u64: u64 = rug_fuzz_0;
        let expected_i16: i16 = rug_fuzz_1;
        let result_i16: i16 = AsPrimitive::<i16>::as_(val_u64);
        debug_assert_eq!(result_i16, expected_i16);
        let _rug_ed_tests_llm_16_1676_llm_16_1676_rrrruuuugggg_test_as_primitive_u64_to_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1677_llm_16_1677 {
    use crate::AsPrimitive;
    #[test]
    fn test_as_primitive_u64_to_i32() {
        let _rug_st_tests_llm_16_1677_llm_16_1677_rrrruuuugggg_test_as_primitive_u64_to_i32 = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 1;
        let x: u64 = rug_fuzz_0;
        let y: i32 = AsPrimitive::<i32>::as_(x);
        debug_assert_eq!(y, 42i32);
        let x: u64 = i32::MAX as u64;
        let y: i32 = AsPrimitive::<i32>::as_(x);
        debug_assert_eq!(y, i32::MAX);
        let x: u64 = (i32::MAX as u64) + rug_fuzz_1;
        let y: i32 = AsPrimitive::<i32>::as_(x);
        debug_assert_eq!(y, i32::MIN);
        let x: u64 = u64::MAX;
        let y: i32 = AsPrimitive::<i32>::as_(x);
        debug_assert_eq!(y, - 1);
        let _rug_ed_tests_llm_16_1677_llm_16_1677_rrrruuuugggg_test_as_primitive_u64_to_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1678_llm_16_1678 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_u64_to_i64() {
        let _rug_st_tests_llm_16_1678_llm_16_1678_rrrruuuugggg_test_as_primitive_u64_to_i64 = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 0;
        let val_u64: u64 = rug_fuzz_0;
        let val_i64: i64 = AsPrimitive::<i64>::as_(val_u64);
        debug_assert_eq!(val_i64, 42i64);
        let max_u64_as_i64: i64 = AsPrimitive::<i64>::as_(u64::MAX);
        debug_assert!(
            max_u64_as_i64 < rug_fuzz_1,
            "Casting u64::MAX to i64 should overflow and give a negative value"
        );
        let _rug_ed_tests_llm_16_1678_llm_16_1678_rrrruuuugggg_test_as_primitive_u64_to_i64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1679_llm_16_1679 {
    use crate::AsPrimitive;
    #[test]
    fn test_as_primitive_u64_to_i8() {
        let _rug_st_tests_llm_16_1679_llm_16_1679_rrrruuuugggg_test_as_primitive_u64_to_i8 = 0;
        let rug_fuzz_0 = 100;
        let val_u64: u64 = rug_fuzz_0;
        let val_i8: i8 = AsPrimitive::<i8>::as_(val_u64);
        debug_assert_eq!(val_i8, 100i8);
        let _rug_ed_tests_llm_16_1679_llm_16_1679_rrrruuuugggg_test_as_primitive_u64_to_i8 = 0;
    }
    #[test]
    #[should_panic]
    fn test_as_primitive_u64_to_i8_overflow() {
        let _rug_st_tests_llm_16_1679_llm_16_1679_rrrruuuugggg_test_as_primitive_u64_to_i8_overflow = 0;
        let rug_fuzz_0 = 1000;
        let val_u64: u64 = rug_fuzz_0;
        let _val_i8: i8 = AsPrimitive::<i8>::as_(val_u64);
        let _rug_ed_tests_llm_16_1679_llm_16_1679_rrrruuuugggg_test_as_primitive_u64_to_i8_overflow = 0;
    }
    #[test]
    fn test_as_primitive_u64_to_i8_negative() {
        let _rug_st_tests_llm_16_1679_llm_16_1679_rrrruuuugggg_test_as_primitive_u64_to_i8_negative = 0;
        let val_u64: u64 = u64::MAX;
        let val_i8: i8 = AsPrimitive::<i8>::as_(val_u64);
        debug_assert_eq!(val_i8, - 1i8);
        let _rug_ed_tests_llm_16_1679_llm_16_1679_rrrruuuugggg_test_as_primitive_u64_to_i8_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1680_llm_16_1680 {
    use crate::cast::AsPrimitive;
    #[test]
    fn u64_as_isize() {
        let _rug_st_tests_llm_16_1680_llm_16_1680_rrrruuuugggg_u64_as_isize = 0;
        let rug_fuzz_0 = 42;
        let val: u64 = rug_fuzz_0;
        let casted_val: isize = AsPrimitive::<isize>::as_(val);
        debug_assert_eq!(casted_val, 42isize);
        let _rug_ed_tests_llm_16_1680_llm_16_1680_rrrruuuugggg_u64_as_isize = 0;
    }
    #[cfg(target_pointer_width = "64")]
    #[test]
    #[should_panic(expected = "attempt to cast with overflow")]
    fn u64_as_isize_overflow() {
        let _rug_st_tests_llm_16_1680_llm_16_1680_rrrruuuugggg_u64_as_isize_overflow = 0;
        let val: u64 = u64::MAX;
        let _casted_val: isize = AsPrimitive::<isize>::as_(val);
        let _rug_ed_tests_llm_16_1680_llm_16_1680_rrrruuuugggg_u64_as_isize_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1681_llm_16_1681 {
    use crate::cast::AsPrimitive;
    #[test]
    fn u64_as_u128() {
        let _rug_st_tests_llm_16_1681_llm_16_1681_rrrruuuugggg_u64_as_u128 = 0;
        let rug_fuzz_0 = 12345;
        let value: u64 = rug_fuzz_0;
        let result: u128 = AsPrimitive::<u128>::as_(value);
        debug_assert_eq!(result, 12345u128);
        let _rug_ed_tests_llm_16_1681_llm_16_1681_rrrruuuugggg_u64_as_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1682_llm_16_1682 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_u64_to_u16() {
        let _rug_st_tests_llm_16_1682_llm_16_1682_rrrruuuugggg_test_as_primitive_u64_to_u16 = 0;
        let rug_fuzz_0 = 42;
        let value: u64 = rug_fuzz_0;
        let result: u16 = <u64 as AsPrimitive<u16>>::as_(value);
        debug_assert_eq!(result, 42u16);
        let big_value: u64 = u64::MAX;
        let result: u16 = <u64 as AsPrimitive<u16>>::as_(big_value);
        let expected: u16 = big_value as u16;
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_1682_llm_16_1682_rrrruuuugggg_test_as_primitive_u64_to_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1683_llm_16_1683 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_u64_to_u32() {
        let _rug_st_tests_llm_16_1683_llm_16_1683_rrrruuuugggg_test_as_primitive_u64_to_u32 = 0;
        let rug_fuzz_0 = 1234;
        let value_u64: u64 = rug_fuzz_0;
        let value_u32: u32 = AsPrimitive::<u32>::as_(value_u64);
        debug_assert_eq!(value_u32, 1234u32);
        let _rug_ed_tests_llm_16_1683_llm_16_1683_rrrruuuugggg_test_as_primitive_u64_to_u32 = 0;
    }
    #[test]
    fn test_as_primitive_u64_to_u32_truncated() {
        let _rug_st_tests_llm_16_1683_llm_16_1683_rrrruuuugggg_test_as_primitive_u64_to_u32_truncated = 0;
        let value_u64: u64 = u64::MAX;
        let value_u32: u32 = AsPrimitive::<u32>::as_(value_u64);
        debug_assert_eq!(value_u32, u32::MAX);
        let _rug_ed_tests_llm_16_1683_llm_16_1683_rrrruuuugggg_test_as_primitive_u64_to_u32_truncated = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1684_llm_16_1684 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_u64_to_u64() {
        let _rug_st_tests_llm_16_1684_llm_16_1684_rrrruuuugggg_test_as_primitive_u64_to_u64 = 0;
        let rug_fuzz_0 = 12345;
        let value: u64 = rug_fuzz_0;
        let result: u64 = AsPrimitive::<u64>::as_(value);
        debug_assert_eq!(result, 12345u64);
        let _rug_ed_tests_llm_16_1684_llm_16_1684_rrrruuuugggg_test_as_primitive_u64_to_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1685 {
    use crate::AsPrimitive;
    #[test]
    fn u64_as_u8() {
        let _rug_st_tests_llm_16_1685_rrrruuuugggg_u64_as_u8 = 0;
        let rug_fuzz_0 = 0u64;
        let rug_fuzz_1 = 255u64;
        let rug_fuzz_2 = 256u64;
        debug_assert_eq!(< u64 as AsPrimitive < u8 > > ::as_(rug_fuzz_0), 0u8);
        debug_assert_eq!(< u64 as AsPrimitive < u8 > > ::as_(rug_fuzz_1), 255u8);
        debug_assert_eq!(< u64 as AsPrimitive < u8 > > ::as_(rug_fuzz_2), 0u8);
        debug_assert_eq!(< u64 as AsPrimitive < u8 > > ::as_(u64::MAX), 255u8);
        let _rug_ed_tests_llm_16_1685_rrrruuuugggg_u64_as_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1686_llm_16_1686 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_u64_to_usize() {
        let _rug_st_tests_llm_16_1686_llm_16_1686_rrrruuuugggg_test_as_primitive_u64_to_usize = 0;
        let rug_fuzz_0 = 123;
        let value_u64: u64 = rug_fuzz_0;
        let value_usize: usize = value_u64.as_();
        debug_assert_eq!(value_usize, 123usize);
        let _rug_ed_tests_llm_16_1686_llm_16_1686_rrrruuuugggg_test_as_primitive_u64_to_usize = 0;
    }
    #[test]
    #[should_panic]
    fn test_as_primitive_u64_to_usize_overflow() {
        let _rug_st_tests_llm_16_1686_llm_16_1686_rrrruuuugggg_test_as_primitive_u64_to_usize_overflow = 0;
        let value_u64: u64 = u64::max_value();
        if u64::max_value() > usize::max_value() as u64 {
            let _value_usize: usize = value_u64.as_();
        } else {
            panic!("attempt to cast to usize with overflow");
        }
        let _rug_ed_tests_llm_16_1686_llm_16_1686_rrrruuuugggg_test_as_primitive_u64_to_usize_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1687_llm_16_1687 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_f32() {
        let _rug_st_tests_llm_16_1687_llm_16_1687_rrrruuuugggg_test_from_f32 = 0;
        let rug_fuzz_0 = 0.0_f32;
        let rug_fuzz_1 = 1.0_f32;
        let rug_fuzz_2 = 1.5_f32;
        let rug_fuzz_3 = 1.0_f32;
        let rug_fuzz_4 = 16777216.0_f32;
        let rug_fuzz_5 = 16777217.0_f32;
        debug_assert_eq!(< u64 as FromPrimitive > ::from_f32(rug_fuzz_0), Some(0_u64));
        debug_assert_eq!(< u64 as FromPrimitive > ::from_f32(rug_fuzz_1), Some(1_u64));
        debug_assert_eq!(< u64 as FromPrimitive > ::from_f32(rug_fuzz_2), Some(1_u64));
        debug_assert_eq!(< u64 as FromPrimitive > ::from_f32(- rug_fuzz_3), None);
        debug_assert_eq!(
            < u64 as FromPrimitive > ::from_f32(rug_fuzz_4), Some(16777216_u64)
        );
        debug_assert_ne!(
            < u64 as FromPrimitive > ::from_f32(rug_fuzz_5), Some(16777217_u64)
        );
        debug_assert_eq!(< u64 as FromPrimitive > ::from_f32(f32::MAX), None);
        debug_assert_eq!(< u64 as FromPrimitive > ::from_f32(f32::MIN), None);
        debug_assert_eq!(< u64 as FromPrimitive > ::from_f32(f32::INFINITY), None);
        debug_assert_eq!(< u64 as FromPrimitive > ::from_f32(f32::NEG_INFINITY), None);
        debug_assert_eq!(< u64 as FromPrimitive > ::from_f32(f32::NAN), None);
        let _rug_ed_tests_llm_16_1687_llm_16_1687_rrrruuuugggg_test_from_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1688_llm_16_1688 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_f64() {
        let _rug_st_tests_llm_16_1688_llm_16_1688_rrrruuuugggg_test_from_f64 = 0;
        let rug_fuzz_0 = 42.0;
        let rug_fuzz_1 = 42.0;
        debug_assert_eq!(< u64 as FromPrimitive > ::from_f64(rug_fuzz_0), Some(42u64));
        debug_assert_eq!(< u64 as FromPrimitive > ::from_f64(- rug_fuzz_1), None);
        debug_assert_eq!(< u64 as FromPrimitive > ::from_f64(f64::MAX), None);
        debug_assert_eq!(< u64 as FromPrimitive > ::from_f64(f64::MIN), None);
        debug_assert_eq!(< u64 as FromPrimitive > ::from_f64(f64::INFINITY), None);
        debug_assert_eq!(< u64 as FromPrimitive > ::from_f64(f64::NEG_INFINITY), None);
        debug_assert_eq!(< u64 as FromPrimitive > ::from_f64(f64::NAN), None);
        let _rug_ed_tests_llm_16_1688_llm_16_1688_rrrruuuugggg_test_from_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1689_llm_16_1689 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i128_with_positive_value() {
        let _rug_st_tests_llm_16_1689_llm_16_1689_rrrruuuugggg_test_from_i128_with_positive_value = 0;
        let rug_fuzz_0 = 42;
        let value: i128 = rug_fuzz_0;
        let result = <u64 as FromPrimitive>::from_i128(value);
        debug_assert_eq!(result, Some(42u64));
        let _rug_ed_tests_llm_16_1689_llm_16_1689_rrrruuuugggg_test_from_i128_with_positive_value = 0;
    }
    #[test]
    fn test_from_i128_with_zero() {
        let _rug_st_tests_llm_16_1689_llm_16_1689_rrrruuuugggg_test_from_i128_with_zero = 0;
        let rug_fuzz_0 = 0;
        let value: i128 = rug_fuzz_0;
        let result = <u64 as FromPrimitive>::from_i128(value);
        debug_assert_eq!(result, Some(0u64));
        let _rug_ed_tests_llm_16_1689_llm_16_1689_rrrruuuugggg_test_from_i128_with_zero = 0;
    }
    #[test]
    fn test_from_i128_with_negative_value() {
        let _rug_st_tests_llm_16_1689_llm_16_1689_rrrruuuugggg_test_from_i128_with_negative_value = 0;
        let rug_fuzz_0 = 42;
        let value: i128 = -rug_fuzz_0;
        let result = <u64 as FromPrimitive>::from_i128(value);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1689_llm_16_1689_rrrruuuugggg_test_from_i128_with_negative_value = 0;
    }
    #[test]
    fn test_from_i128_with_value_exceeding_u64() {
        let _rug_st_tests_llm_16_1689_llm_16_1689_rrrruuuugggg_test_from_i128_with_value_exceeding_u64 = 0;
        let rug_fuzz_0 = 1;
        let value: i128 = u64::MAX as i128 + rug_fuzz_0;
        let result = <u64 as FromPrimitive>::from_i128(value);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1689_llm_16_1689_rrrruuuugggg_test_from_i128_with_value_exceeding_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1690_llm_16_1690 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i16_with_positive() {
        let _rug_st_tests_llm_16_1690_llm_16_1690_rrrruuuugggg_test_from_i16_with_positive = 0;
        let rug_fuzz_0 = 123;
        let value: i16 = rug_fuzz_0;
        let result = <u64 as FromPrimitive>::from_i16(value);
        debug_assert_eq!(result, Some(123u64));
        let _rug_ed_tests_llm_16_1690_llm_16_1690_rrrruuuugggg_test_from_i16_with_positive = 0;
    }
    #[test]
    fn test_from_i16_with_negative() {
        let _rug_st_tests_llm_16_1690_llm_16_1690_rrrruuuugggg_test_from_i16_with_negative = 0;
        let rug_fuzz_0 = 123;
        let value: i16 = -rug_fuzz_0;
        let result = <u64 as FromPrimitive>::from_i16(value);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1690_llm_16_1690_rrrruuuugggg_test_from_i16_with_negative = 0;
    }
    #[test]
    fn test_from_i16_with_zero() {
        let _rug_st_tests_llm_16_1690_llm_16_1690_rrrruuuugggg_test_from_i16_with_zero = 0;
        let rug_fuzz_0 = 0;
        let value: i16 = rug_fuzz_0;
        let result = <u64 as FromPrimitive>::from_i16(value);
        debug_assert_eq!(result, Some(0u64));
        let _rug_ed_tests_llm_16_1690_llm_16_1690_rrrruuuugggg_test_from_i16_with_zero = 0;
    }
    #[test]
    fn test_from_i16_with_max() {
        let _rug_st_tests_llm_16_1690_llm_16_1690_rrrruuuugggg_test_from_i16_with_max = 0;
        let value: i16 = i16::MAX;
        let result = <u64 as FromPrimitive>::from_i16(value);
        debug_assert_eq!(result, Some(i16::MAX as u64));
        let _rug_ed_tests_llm_16_1690_llm_16_1690_rrrruuuugggg_test_from_i16_with_max = 0;
    }
    #[test]
    fn test_from_i16_with_min() {
        let _rug_st_tests_llm_16_1690_llm_16_1690_rrrruuuugggg_test_from_i16_with_min = 0;
        let value: i16 = i16::MIN;
        let result = <u64 as FromPrimitive>::from_i16(value);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1690_llm_16_1690_rrrruuuugggg_test_from_i16_with_min = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1691_llm_16_1691 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i32() {
        let _rug_st_tests_llm_16_1691_llm_16_1691_rrrruuuugggg_test_from_i32 = 0;
        let rug_fuzz_0 = 0_i32;
        let rug_fuzz_1 = 10_i32;
        let rug_fuzz_2 = 1_i32;
        debug_assert_eq!(< u64 as FromPrimitive > ::from_i32(rug_fuzz_0), Some(0_u64));
        debug_assert_eq!(< u64 as FromPrimitive > ::from_i32(rug_fuzz_1), Some(10_u64));
        debug_assert_eq!(< u64 as FromPrimitive > ::from_i32(- rug_fuzz_2), None);
        debug_assert_eq!(
            < u64 as FromPrimitive > ::from_i32(i32::MAX), Some(i32::MAX as u64)
        );
        let _rug_ed_tests_llm_16_1691_llm_16_1691_rrrruuuugggg_test_from_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1692_llm_16_1692 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i64() {
        let _rug_st_tests_llm_16_1692_llm_16_1692_rrrruuuugggg_test_from_i64 = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1234;
        let rug_fuzz_2 = 1;
        debug_assert_eq!(< u64 as FromPrimitive > ::from_i64(rug_fuzz_0), Some(0u64));
        debug_assert_eq!(< u64 as FromPrimitive > ::from_i64(rug_fuzz_1), Some(1234u64));
        debug_assert_eq!(< u64 as FromPrimitive > ::from_i64(- rug_fuzz_2), None);
        debug_assert_eq!(
            < u64 as FromPrimitive > ::from_i64(i64::MAX), Some(i64::MAX as u64)
        );
        debug_assert_eq!(< u64 as FromPrimitive > ::from_i64(i64::MIN), None);
        let _rug_ed_tests_llm_16_1692_llm_16_1692_rrrruuuugggg_test_from_i64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1693_llm_16_1693 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i8() {
        let _rug_st_tests_llm_16_1693_llm_16_1693_rrrruuuugggg_test_from_i8 = 0;
        let rug_fuzz_0 = 0i8;
        let rug_fuzz_1 = 1i8;
        let rug_fuzz_2 = 127i8;
        debug_assert_eq!(< u64 as FromPrimitive > ::from_i8(rug_fuzz_0), Some(0u64));
        debug_assert_eq!(< u64 as FromPrimitive > ::from_i8(- rug_fuzz_1), None);
        debug_assert_eq!(< u64 as FromPrimitive > ::from_i8(rug_fuzz_2), Some(127u64));
        let _rug_ed_tests_llm_16_1693_llm_16_1693_rrrruuuugggg_test_from_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1694_llm_16_1694 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_isize() {
        let _rug_st_tests_llm_16_1694_llm_16_1694_rrrruuuugggg_test_from_isize = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 1;
        debug_assert_eq!(< u64 as FromPrimitive > ::from_isize(rug_fuzz_0), Some(0u64));
        debug_assert_eq!(< u64 as FromPrimitive > ::from_isize(- rug_fuzz_1), None);
        debug_assert_eq!(< u64 as FromPrimitive > ::from_isize(rug_fuzz_2), Some(1u64));
        debug_assert_eq!(
            < u64 as FromPrimitive > ::from_isize(isize::MAX), Some(isize::MAX as u64)
        );
        if usize::BITS > u64::BITS {
            debug_assert_eq!(< u64 as FromPrimitive > ::from_isize(- rug_fuzz_3), None);
        }
        let _rug_ed_tests_llm_16_1694_llm_16_1694_rrrruuuugggg_test_from_isize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1695_llm_16_1695 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u128() {
        let _rug_st_tests_llm_16_1695_llm_16_1695_rrrruuuugggg_test_from_u128 = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = 123456789;
        let value_within_range: u128 = u64::MAX as u128;
        debug_assert_eq!(
            < u64 as FromPrimitive > ::from_u128(value_within_range), Some(u64::MAX)
        );
        let value_outside_range: u128 = (u64::MAX as u128) + rug_fuzz_0;
        debug_assert_eq!(
            < u64 as FromPrimitive > ::from_u128(value_outside_range), None
        );
        debug_assert_eq!(< u64 as FromPrimitive > ::from_u128(rug_fuzz_1), Some(0));
        let random_value_within_range: u128 = rug_fuzz_2;
        debug_assert_eq!(
            < u64 as FromPrimitive > ::from_u128(random_value_within_range),
            Some(123456789)
        );
        let _rug_ed_tests_llm_16_1695_llm_16_1695_rrrruuuugggg_test_from_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1696_llm_16_1696 {
    use super::*;
    use crate::*;
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u16() {
        let _rug_st_tests_llm_16_1696_llm_16_1696_rrrruuuugggg_test_from_u16 = 0;
        let rug_fuzz_0 = 0u16;
        let rug_fuzz_1 = 42u16;
        debug_assert_eq!(< u64 as FromPrimitive > ::from_u16(rug_fuzz_0), Some(0u64));
        debug_assert_eq!(< u64 as FromPrimitive > ::from_u16(rug_fuzz_1), Some(42u64));
        debug_assert_eq!(
            < u64 as FromPrimitive > ::from_u16(u16::MAX), Some(u16::MAX as u64)
        );
        let _rug_ed_tests_llm_16_1696_llm_16_1696_rrrruuuugggg_test_from_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1697_llm_16_1697 {
    use crate::cast::FromPrimitive;
    #[test]
    fn from_u32_max_value() {
        let _rug_st_tests_llm_16_1697_llm_16_1697_rrrruuuugggg_from_u32_max_value = 0;
        debug_assert_eq!(
            < u64 as FromPrimitive > ::from_u32(u32::MAX), Some(u64::from(u32::MAX))
        );
        let _rug_ed_tests_llm_16_1697_llm_16_1697_rrrruuuugggg_from_u32_max_value = 0;
    }
    #[test]
    fn from_u32_zero() {
        let _rug_st_tests_llm_16_1697_llm_16_1697_rrrruuuugggg_from_u32_zero = 0;
        let rug_fuzz_0 = 0;
        debug_assert_eq!(< u64 as FromPrimitive > ::from_u32(rug_fuzz_0), Some(0u64));
        let _rug_ed_tests_llm_16_1697_llm_16_1697_rrrruuuugggg_from_u32_zero = 0;
    }
    #[test]
    fn from_u32_arbitrary() {
        let _rug_st_tests_llm_16_1697_llm_16_1697_rrrruuuugggg_from_u32_arbitrary = 0;
        let rug_fuzz_0 = 42;
        debug_assert_eq!(< u64 as FromPrimitive > ::from_u32(rug_fuzz_0), Some(42u64));
        let _rug_ed_tests_llm_16_1697_llm_16_1697_rrrruuuugggg_from_u32_arbitrary = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1699_llm_16_1699 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u8() {
        let _rug_st_tests_llm_16_1699_llm_16_1699_rrrruuuugggg_test_from_u8 = 0;
        let rug_fuzz_0 = 0u8;
        let rug_fuzz_1 = 255u8;
        let rug_fuzz_2 = 100u8;
        debug_assert_eq!(< u64 as FromPrimitive > ::from_u8(rug_fuzz_0), Some(0u64));
        debug_assert_eq!(< u64 as FromPrimitive > ::from_u8(rug_fuzz_1), Some(255u64));
        debug_assert_eq!(< u64 as FromPrimitive > ::from_u8(rug_fuzz_2), Some(100u64));
        let _rug_ed_tests_llm_16_1699_llm_16_1699_rrrruuuugggg_test_from_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1700_llm_16_1700 {
    use super::*;
    use crate::*;
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_usize_within_bounds() {
        let _rug_st_tests_llm_16_1700_llm_16_1700_rrrruuuugggg_test_from_usize_within_bounds = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42_usize;
        let value: u64 = rug_fuzz_0;
        let result = <u64 as FromPrimitive>::from_usize(rug_fuzz_1);
        debug_assert_eq!(result, Some(value));
        let _rug_ed_tests_llm_16_1700_llm_16_1700_rrrruuuugggg_test_from_usize_within_bounds = 0;
    }
    #[test]
    fn test_from_usize_zero() {
        let _rug_st_tests_llm_16_1700_llm_16_1700_rrrruuuugggg_test_from_usize_zero = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0_usize;
        let value: u64 = rug_fuzz_0;
        let result = <u64 as FromPrimitive>::from_usize(rug_fuzz_1);
        debug_assert_eq!(result, Some(value));
        let _rug_ed_tests_llm_16_1700_llm_16_1700_rrrruuuugggg_test_from_usize_zero = 0;
    }
    #[test]
    fn test_from_usize_max() {
        let _rug_st_tests_llm_16_1700_llm_16_1700_rrrruuuugggg_test_from_usize_max = 0;
        let value: u64 = u64::MAX;
        let result = <u64 as FromPrimitive>::from_usize(usize::MAX);
        if usize::MAX as u64 <= u64::MAX {
            debug_assert_eq!(result, Some(value));
        } else {
            debug_assert_eq!(result, None);
        }
        let _rug_ed_tests_llm_16_1700_llm_16_1700_rrrruuuugggg_test_from_usize_max = 0;
    }
    #[test]
    fn test_from_usize_overflow() {
        let _rug_st_tests_llm_16_1700_llm_16_1700_rrrruuuugggg_test_from_usize_overflow = 0;
        let result = <u64 as FromPrimitive>::from_usize(usize::MAX);
        if std::mem::size_of::<usize>() > std::mem::size_of::<u64>() {
            debug_assert_eq!(result, None);
        } else {
            debug_assert_eq!(result, Some(usize::MAX as u64));
        }
        let _rug_ed_tests_llm_16_1700_llm_16_1700_rrrruuuugggg_test_from_usize_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1701_llm_16_1701 {
    use super::*;
    use crate::*;
    use crate::cast::{NumCast, ToPrimitive};
    use std::num::Wrapping;
    #[test]
    fn test_u64_from_wrapping() {
        let _rug_st_tests_llm_16_1701_llm_16_1701_rrrruuuugggg_test_u64_from_wrapping = 0;
        let rug_fuzz_0 = 42u8;
        let rug_fuzz_1 = 42u16;
        let rug_fuzz_2 = 42u32;
        let rug_fuzz_3 = 42u64;
        let rug_fuzz_4 = 42usize;
        let rug_fuzz_5 = 42i8;
        let rug_fuzz_6 = 42i16;
        let rug_fuzz_7 = 42i32;
        let rug_fuzz_8 = 42i64;
        let rug_fuzz_9 = 42isize;
        let rug_fuzz_10 = 3.0f32;
        let rug_fuzz_11 = 3.0f64;
        let rug_fuzz_12 = 42i8;
        let rug_fuzz_13 = 42i16;
        let rug_fuzz_14 = 42i32;
        let rug_fuzz_15 = 42i64;
        let rug_fuzz_16 = 42isize;
        debug_assert_eq!(< u64 as NumCast > ::from(Wrapping(rug_fuzz_0)), Some(42u64));
        debug_assert_eq!(< u64 as NumCast > ::from(Wrapping(rug_fuzz_1)), Some(42u64));
        debug_assert_eq!(< u64 as NumCast > ::from(Wrapping(rug_fuzz_2)), Some(42u64));
        debug_assert_eq!(< u64 as NumCast > ::from(Wrapping(rug_fuzz_3)), Some(42u64));
        debug_assert_eq!(< u64 as NumCast > ::from(Wrapping(rug_fuzz_4)), Some(42u64));
        debug_assert_eq!(< u64 as NumCast > ::from(Wrapping(rug_fuzz_5)), Some(42u64));
        debug_assert_eq!(< u64 as NumCast > ::from(Wrapping(rug_fuzz_6)), Some(42u64));
        debug_assert_eq!(< u64 as NumCast > ::from(Wrapping(rug_fuzz_7)), Some(42u64));
        debug_assert_eq!(< u64 as NumCast > ::from(Wrapping(rug_fuzz_8)), Some(42u64));
        debug_assert_eq!(< u64 as NumCast > ::from(Wrapping(rug_fuzz_9)), Some(42u64));
        debug_assert_eq!(< u64 as NumCast > ::from(Wrapping(rug_fuzz_10)), Some(3u64));
        debug_assert_eq!(< u64 as NumCast > ::from(Wrapping(rug_fuzz_11)), Some(3u64));
        debug_assert_eq!(< u64 as NumCast > ::from(Wrapping(- rug_fuzz_12)), None);
        debug_assert_eq!(< u64 as NumCast > ::from(Wrapping(- rug_fuzz_13)), None);
        debug_assert_eq!(< u64 as NumCast > ::from(Wrapping(- rug_fuzz_14)), None);
        debug_assert_eq!(< u64 as NumCast > ::from(Wrapping(- rug_fuzz_15)), None);
        debug_assert_eq!(< u64 as NumCast > ::from(Wrapping(- rug_fuzz_16)), None);
        let _rug_ed_tests_llm_16_1701_llm_16_1701_rrrruuuugggg_test_u64_from_wrapping = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1702 {
    use super::*;
    use crate::*;
    #[test]
    fn test_u64_to_f32_cast() {
        let _rug_st_tests_llm_16_1702_rrrruuuugggg_test_u64_to_f32_cast = 0;
        let rug_fuzz_0 = 0u64;
        let rug_fuzz_1 = 1u64;
        let rug_fuzz_2 = 1;
        let rug_fuzz_3 = 24;
        let rug_fuzz_4 = 1;
        let rug_fuzz_5 = 1;
        debug_assert_eq!(
            < u64 as cast::ToPrimitive > ::to_f32(& rug_fuzz_0), Some(0f32)
        );
        debug_assert_eq!(
            < u64 as cast::ToPrimitive > ::to_f32(& rug_fuzz_1), Some(1f32)
        );
        debug_assert_eq!(
            < u64 as cast::ToPrimitive > ::to_f32(& u64::MAX), Some(u64::MAX as f32)
        );
        let max_precise_val: u64 = rug_fuzz_2 << rug_fuzz_3;
        debug_assert_eq!(
            < u64 as cast::ToPrimitive > ::to_f32(& max_precise_val),
            Some(max_precise_val as f32)
        );
        debug_assert_eq!(
            < u64 as cast::ToPrimitive > ::to_f32(& (max_precise_val - rug_fuzz_4)),
            Some((max_precise_val - 1) as f32)
        );
        debug_assert_eq!(
            < u64 as cast::ToPrimitive > ::to_f32(& (max_precise_val + rug_fuzz_5)),
            Some((max_precise_val + 1) as f32)
        );
        let _rug_ed_tests_llm_16_1702_rrrruuuugggg_test_u64_to_f32_cast = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1703 {
    use crate::ToPrimitive;
    #[test]
    fn test_u64_to_f64() {
        let _rug_st_tests_llm_16_1703_rrrruuuugggg_test_u64_to_f64 = 0;
        let rug_fuzz_0 = 0u64;
        let rug_fuzz_1 = 1u64;
        debug_assert_eq!(rug_fuzz_0.to_f64(), Some(0.0_f64));
        debug_assert_eq!(rug_fuzz_1.to_f64(), Some(1.0_f64));
        debug_assert_eq!(u64::MAX.to_f64(), Some(u64::MAX as f64));
        let _rug_ed_tests_llm_16_1703_rrrruuuugggg_test_u64_to_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1704_llm_16_1704 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_u64_to_i128() {
        let _rug_st_tests_llm_16_1704_llm_16_1704_rrrruuuugggg_test_u64_to_i128 = 0;
        let rug_fuzz_0 = 0u64;
        let rug_fuzz_1 = 1u64;
        debug_assert_eq!(rug_fuzz_0.to_i128(), Some(0i128));
        debug_assert_eq!(rug_fuzz_1.to_i128(), Some(1i128));
        debug_assert_eq!(u64::MAX.to_i128(), Some(u64::MAX as i128));
        let _rug_ed_tests_llm_16_1704_llm_16_1704_rrrruuuugggg_test_u64_to_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1705_llm_16_1705 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_i16() {
        let _rug_st_tests_llm_16_1705_llm_16_1705_rrrruuuugggg_test_to_i16 = 0;
        let rug_fuzz_0 = 0u64;
        let rug_fuzz_1 = 1u64;
        let rug_fuzz_2 = 1;
        debug_assert_eq!(rug_fuzz_0.to_i16(), Some(0i16));
        debug_assert_eq!(rug_fuzz_1.to_i16(), Some(1i16));
        debug_assert_eq!((i16::MAX as u64).to_i16(), Some(i16::MAX));
        debug_assert_eq!((i16::MAX as u64 + rug_fuzz_2).to_i16(), None);
        debug_assert_eq!(u64::MAX.to_i16(), None);
        let _rug_ed_tests_llm_16_1705_llm_16_1705_rrrruuuugggg_test_to_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1706_llm_16_1706 {
    use crate::ToPrimitive;
    #[test]
    fn test_u64_to_i32() {
        let _rug_st_tests_llm_16_1706_llm_16_1706_rrrruuuugggg_test_u64_to_i32 = 0;
        let rug_fuzz_0 = 42u64;
        let rug_fuzz_1 = 1;
        debug_assert_eq!((rug_fuzz_0).to_i32(), Some(42i32));
        debug_assert_eq!((i32::MAX as u64).to_i32(), Some(i32::MAX));
        debug_assert_eq!(((i32::MAX as u64) + rug_fuzz_1).to_i32(), None);
        debug_assert_eq!((u64::MAX).to_i32(), None);
        let _rug_ed_tests_llm_16_1706_llm_16_1706_rrrruuuugggg_test_u64_to_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1707_llm_16_1707 {
    use super::*;
    use crate::*;
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_i64_for_u64_within_bounds() {
        let _rug_st_tests_llm_16_1707_llm_16_1707_rrrruuuugggg_test_to_i64_for_u64_within_bounds = 0;
        let value_within_bounds: u64 = i64::MAX as u64;
        debug_assert_eq!(value_within_bounds.to_i64(), Some(i64::MAX));
        let _rug_ed_tests_llm_16_1707_llm_16_1707_rrrruuuugggg_test_to_i64_for_u64_within_bounds = 0;
    }
    #[test]
    fn test_to_i64_for_u64_out_of_bounds() {
        let _rug_st_tests_llm_16_1707_llm_16_1707_rrrruuuugggg_test_to_i64_for_u64_out_of_bounds = 0;
        let rug_fuzz_0 = 1;
        let value_out_of_bounds: u64 = i64::MAX as u64 + rug_fuzz_0;
        debug_assert_eq!(value_out_of_bounds.to_i64(), None);
        let _rug_ed_tests_llm_16_1707_llm_16_1707_rrrruuuugggg_test_to_i64_for_u64_out_of_bounds = 0;
    }
    #[test]
    fn test_to_i64_for_zero() {
        let _rug_st_tests_llm_16_1707_llm_16_1707_rrrruuuugggg_test_to_i64_for_zero = 0;
        let rug_fuzz_0 = 0;
        let zero: u64 = rug_fuzz_0;
        debug_assert_eq!(zero.to_i64(), Some(0));
        let _rug_ed_tests_llm_16_1707_llm_16_1707_rrrruuuugggg_test_to_i64_for_zero = 0;
    }
    #[test]
    fn test_to_i64_for_u64_min_value() {
        let _rug_st_tests_llm_16_1707_llm_16_1707_rrrruuuugggg_test_to_i64_for_u64_min_value = 0;
        let min_value: u64 = u64::MIN;
        debug_assert_eq!(min_value.to_i64(), Some(0));
        let _rug_ed_tests_llm_16_1707_llm_16_1707_rrrruuuugggg_test_to_i64_for_u64_min_value = 0;
    }
    #[test]
    fn test_to_i64_for_u64_max_value() {
        let _rug_st_tests_llm_16_1707_llm_16_1707_rrrruuuugggg_test_to_i64_for_u64_max_value = 0;
        let max_value: u64 = u64::MAX;
        debug_assert_eq!(max_value.to_i64(), None);
        let _rug_ed_tests_llm_16_1707_llm_16_1707_rrrruuuugggg_test_to_i64_for_u64_max_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1708_llm_16_1708 {
    use crate::cast::ToPrimitive;
    #[test]
    fn u64_to_i8() {
        let _rug_st_tests_llm_16_1708_llm_16_1708_rrrruuuugggg_u64_to_i8 = 0;
        let rug_fuzz_0 = 0u64;
        let rug_fuzz_1 = 127u64;
        let rug_fuzz_2 = 128u64;
        debug_assert_eq!(rug_fuzz_0.to_i8(), Some(0i8));
        debug_assert_eq!(rug_fuzz_1.to_i8(), Some(127i8));
        debug_assert_eq!(rug_fuzz_2.to_i8(), None);
        debug_assert_eq!(u64::MAX.to_i8(), None);
        let _rug_ed_tests_llm_16_1708_llm_16_1708_rrrruuuugggg_u64_to_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1709_llm_16_1709 {
    use crate::cast::ToPrimitive;
    #[test]
    fn to_isize_max_value() {
        let _rug_st_tests_llm_16_1709_llm_16_1709_rrrruuuugggg_to_isize_max_value = 0;
        let value: u64 = isize::MAX as u64;
        debug_assert_eq!(value.to_isize(), Some(isize::MAX));
        let _rug_ed_tests_llm_16_1709_llm_16_1709_rrrruuuugggg_to_isize_max_value = 0;
    }
    #[test]
    fn to_isize_within_bounds() {
        let _rug_st_tests_llm_16_1709_llm_16_1709_rrrruuuugggg_to_isize_within_bounds = 0;
        let rug_fuzz_0 = 42;
        let value: u64 = rug_fuzz_0;
        debug_assert_eq!(value.to_isize(), Some(42));
        let _rug_ed_tests_llm_16_1709_llm_16_1709_rrrruuuugggg_to_isize_within_bounds = 0;
    }
    #[test]
    fn to_isize_out_of_bounds() {
        let _rug_st_tests_llm_16_1709_llm_16_1709_rrrruuuugggg_to_isize_out_of_bounds = 0;
        let rug_fuzz_0 = 1;
        let value: u64 = (isize::MAX as u64).wrapping_add(rug_fuzz_0);
        debug_assert_eq!(value.to_isize(), None);
        let _rug_ed_tests_llm_16_1709_llm_16_1709_rrrruuuugggg_to_isize_out_of_bounds = 0;
    }
    #[test]
    fn to_isize_zero() {
        let _rug_st_tests_llm_16_1709_llm_16_1709_rrrruuuugggg_to_isize_zero = 0;
        let rug_fuzz_0 = 0;
        let value: u64 = rug_fuzz_0;
        debug_assert_eq!(value.to_isize(), Some(0));
        let _rug_ed_tests_llm_16_1709_llm_16_1709_rrrruuuugggg_to_isize_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1710_llm_16_1710 {
    use crate::cast::ToPrimitive;
    #[test]
    fn u64_to_u128_within_bounds() {
        let _rug_st_tests_llm_16_1710_llm_16_1710_rrrruuuugggg_u64_to_u128_within_bounds = 0;
        let value: u64 = u64::MAX;
        debug_assert_eq!(ToPrimitive::to_u128(& value), Some(u128::from(u64::MAX)));
        let _rug_ed_tests_llm_16_1710_llm_16_1710_rrrruuuugggg_u64_to_u128_within_bounds = 0;
    }
    #[test]
    fn u64_to_u128_lower_bounds() {
        let _rug_st_tests_llm_16_1710_llm_16_1710_rrrruuuugggg_u64_to_u128_lower_bounds = 0;
        let rug_fuzz_0 = 0;
        let value: u64 = rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_u128(& value), Some(u128::from(0u64)));
        let _rug_ed_tests_llm_16_1710_llm_16_1710_rrrruuuugggg_u64_to_u128_lower_bounds = 0;
    }
    #[test]
    fn u64_to_u128_typical() {
        let _rug_st_tests_llm_16_1710_llm_16_1710_rrrruuuugggg_u64_to_u128_typical = 0;
        let rug_fuzz_0 = 123456789;
        let value: u64 = rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_u128(& value), Some(123456789u128));
        let _rug_ed_tests_llm_16_1710_llm_16_1710_rrrruuuugggg_u64_to_u128_typical = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1711_llm_16_1711 {
    use crate::cast::ToPrimitive;
    #[test]
    fn u64_to_u16_cast_within_bounds() {
        let _rug_st_tests_llm_16_1711_llm_16_1711_rrrruuuugggg_u64_to_u16_cast_within_bounds = 0;
        let value: u64 = u16::MAX as u64;
        debug_assert_eq!(value.to_u16(), Some(u16::MAX));
        let _rug_ed_tests_llm_16_1711_llm_16_1711_rrrruuuugggg_u64_to_u16_cast_within_bounds = 0;
    }
    #[test]
    fn u64_to_u16_cast_exceeding_bounds() {
        let _rug_st_tests_llm_16_1711_llm_16_1711_rrrruuuugggg_u64_to_u16_cast_exceeding_bounds = 0;
        let rug_fuzz_0 = 1;
        let value: u64 = u64::from(u16::MAX) + rug_fuzz_0;
        debug_assert_eq!(value.to_u16(), None);
        let _rug_ed_tests_llm_16_1711_llm_16_1711_rrrruuuugggg_u64_to_u16_cast_exceeding_bounds = 0;
    }
    #[test]
    fn u64_to_u16_cast_zero() {
        let _rug_st_tests_llm_16_1711_llm_16_1711_rrrruuuugggg_u64_to_u16_cast_zero = 0;
        let rug_fuzz_0 = 0;
        let value: u64 = rug_fuzz_0;
        debug_assert_eq!(value.to_u16(), Some(0));
        let _rug_ed_tests_llm_16_1711_llm_16_1711_rrrruuuugggg_u64_to_u16_cast_zero = 0;
    }
    #[test]
    fn u64_to_u16_cast_positive() {
        let _rug_st_tests_llm_16_1711_llm_16_1711_rrrruuuugggg_u64_to_u16_cast_positive = 0;
        let rug_fuzz_0 = 42;
        let value: u64 = rug_fuzz_0;
        debug_assert_eq!(value.to_u16(), Some(42));
        let _rug_ed_tests_llm_16_1711_llm_16_1711_rrrruuuugggg_u64_to_u16_cast_positive = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1712_llm_16_1712 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_u32_within_bounds() {
        let _rug_st_tests_llm_16_1712_llm_16_1712_rrrruuuugggg_test_to_u32_within_bounds = 0;
        let value: u64 = u32::MAX as u64;
        debug_assert_eq!(ToPrimitive::to_u32(& value), Some(u32::MAX));
        let _rug_ed_tests_llm_16_1712_llm_16_1712_rrrruuuugggg_test_to_u32_within_bounds = 0;
    }
    #[test]
    fn test_to_u32_out_of_bounds() {
        let _rug_st_tests_llm_16_1712_llm_16_1712_rrrruuuugggg_test_to_u32_out_of_bounds = 0;
        let rug_fuzz_0 = 1;
        let value: u64 = (u32::MAX as u64) + rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_u32(& value), None);
        let _rug_ed_tests_llm_16_1712_llm_16_1712_rrrruuuugggg_test_to_u32_out_of_bounds = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1713_llm_16_1713 {
    use crate::ToPrimitive;
    #[test]
    fn test_to_u64_with_u64() {
        let _rug_st_tests_llm_16_1713_llm_16_1713_rrrruuuugggg_test_to_u64_with_u64 = 0;
        let val: u64 = u64::MAX;
        debug_assert_eq!(ToPrimitive::to_u64(& val), Some(u64::MAX));
        let _rug_ed_tests_llm_16_1713_llm_16_1713_rrrruuuugggg_test_to_u64_with_u64 = 0;
    }
    #[test]
    fn test_to_u64_with_u32() {
        let _rug_st_tests_llm_16_1713_llm_16_1713_rrrruuuugggg_test_to_u64_with_u32 = 0;
        let val: u32 = u32::MAX;
        debug_assert_eq!(ToPrimitive::to_u64(& val), Some(u32::MAX as u64));
        let _rug_ed_tests_llm_16_1713_llm_16_1713_rrrruuuugggg_test_to_u64_with_u32 = 0;
    }
    #[test]
    fn test_to_u64_with_i32() {
        let _rug_st_tests_llm_16_1713_llm_16_1713_rrrruuuugggg_test_to_u64_with_i32 = 0;
        let val_positive: i32 = i32::MAX;
        let val_negative: i32 = i32::MIN;
        debug_assert_eq!(ToPrimitive::to_u64(& val_positive), Some(i32::MAX as u64));
        debug_assert_eq!(ToPrimitive::to_u64(& val_negative), None);
        let _rug_ed_tests_llm_16_1713_llm_16_1713_rrrruuuugggg_test_to_u64_with_i32 = 0;
    }
    #[test]
    fn test_to_u64_with_i64() {
        let _rug_st_tests_llm_16_1713_llm_16_1713_rrrruuuugggg_test_to_u64_with_i64 = 0;
        let val_positive: i64 = i64::MAX;
        let val_negative: i64 = i64::MIN;
        debug_assert_eq!(ToPrimitive::to_u64(& val_positive), Some(i64::MAX as u64));
        debug_assert_eq!(ToPrimitive::to_u64(& val_negative), None);
        let _rug_ed_tests_llm_16_1713_llm_16_1713_rrrruuuugggg_test_to_u64_with_i64 = 0;
    }
    #[test]
    fn test_to_u64_with_f32() {
        let _rug_st_tests_llm_16_1713_llm_16_1713_rrrruuuugggg_test_to_u64_with_f32 = 0;
        let rug_fuzz_0 = 42.0;
        let val_positive: f32 = f32::MAX;
        let val_negative: f32 = f32::MIN;
        let val_in_range: f32 = rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_u64(& val_positive), None);
        debug_assert_eq!(ToPrimitive::to_u64(& val_negative), None);
        debug_assert_eq!(ToPrimitive::to_u64(& val_in_range), Some(42));
        let _rug_ed_tests_llm_16_1713_llm_16_1713_rrrruuuugggg_test_to_u64_with_f32 = 0;
    }
    #[test]
    fn test_to_u64_with_f64() {
        let _rug_st_tests_llm_16_1713_llm_16_1713_rrrruuuugggg_test_to_u64_with_f64 = 0;
        let rug_fuzz_0 = 42.0;
        let val_positive: f64 = f64::MAX;
        let val_negative: f64 = f64::MIN;
        let val_in_range: f64 = rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_u64(& val_positive), None);
        debug_assert_eq!(ToPrimitive::to_u64(& val_negative), None);
        debug_assert_eq!(ToPrimitive::to_u64(& val_in_range), Some(42));
        let _rug_ed_tests_llm_16_1713_llm_16_1713_rrrruuuugggg_test_to_u64_with_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1714_llm_16_1714 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_u64_to_u8() {
        let _rug_st_tests_llm_16_1714_llm_16_1714_rrrruuuugggg_test_u64_to_u8 = 0;
        let rug_fuzz_0 = 0u64;
        let rug_fuzz_1 = 255u64;
        let rug_fuzz_2 = 256u64;
        debug_assert_eq!(rug_fuzz_0.to_u8(), Some(0u8));
        debug_assert_eq!(rug_fuzz_1.to_u8(), Some(255u8));
        debug_assert_eq!(rug_fuzz_2.to_u8(), None);
        debug_assert_eq!(u64::MAX.to_u8(), None);
        let _rug_ed_tests_llm_16_1714_llm_16_1714_rrrruuuugggg_test_u64_to_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1715_llm_16_1715 {
    use super::*;
    use crate::*;
    #[test]
    fn test_to_usize_within_bounds() {
        let _rug_st_tests_llm_16_1715_llm_16_1715_rrrruuuugggg_test_to_usize_within_bounds = 0;
        let rug_fuzz_0 = 2;
        let value: u64 = u64::MAX / rug_fuzz_0;
        let result = value.to_usize();
        debug_assert_eq!(result, Some(value as usize));
        let _rug_ed_tests_llm_16_1715_llm_16_1715_rrrruuuugggg_test_to_usize_within_bounds = 0;
    }
    #[test]
    fn test_to_usize_exceeding_bounds() {
        let _rug_st_tests_llm_16_1715_llm_16_1715_rrrruuuugggg_test_to_usize_exceeding_bounds = 0;
        let value: u64 = u64::MAX;
        if std::mem::size_of::<usize>() < std::mem::size_of::<u64>() {
            let result = value.to_usize();
            debug_assert_eq!(result, None);
        } else {
            let result = value.to_usize();
            debug_assert_eq!(result, Some(value as usize));
        }
        let _rug_ed_tests_llm_16_1715_llm_16_1715_rrrruuuugggg_test_to_usize_exceeding_bounds = 0;
    }
    #[test]
    fn test_to_usize_exact_bounds() {
        let _rug_st_tests_llm_16_1715_llm_16_1715_rrrruuuugggg_test_to_usize_exact_bounds = 0;
        let value: u64 = usize::MAX as u64;
        let result = value.to_usize();
        debug_assert_eq!(result, Some(value as usize));
        let _rug_ed_tests_llm_16_1715_llm_16_1715_rrrruuuugggg_test_to_usize_exact_bounds = 0;
    }
    #[test]
    fn test_to_usize_zero() {
        let _rug_st_tests_llm_16_1715_llm_16_1715_rrrruuuugggg_test_to_usize_zero = 0;
        let rug_fuzz_0 = 0;
        let value: u64 = rug_fuzz_0;
        let result = value.to_usize();
        debug_assert_eq!(result, Some(0));
        let _rug_ed_tests_llm_16_1715_llm_16_1715_rrrruuuugggg_test_to_usize_zero = 0;
    }
    #[test]
    fn test_to_usize_small_value() {
        let _rug_st_tests_llm_16_1715_llm_16_1715_rrrruuuugggg_test_to_usize_small_value = 0;
        let rug_fuzz_0 = 42;
        let value: u64 = rug_fuzz_0;
        let result = value.to_usize();
        debug_assert_eq!(result, Some(42));
        let _rug_ed_tests_llm_16_1715_llm_16_1715_rrrruuuugggg_test_to_usize_small_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1778_llm_16_1778 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_u8_to_char() {
        let _rug_st_tests_llm_16_1778_llm_16_1778_rrrruuuugggg_test_as_primitive_u8_to_char = 0;
        let rug_fuzz_0 = 65;
        let value_u8: u8 = rug_fuzz_0;
        let char_value: char = AsPrimitive::<char>::as_(value_u8);
        debug_assert_eq!(char_value, 'A');
        let _rug_ed_tests_llm_16_1778_llm_16_1778_rrrruuuugggg_test_as_primitive_u8_to_char = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1779_llm_16_1779 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_u8_to_f32() {
        let _rug_st_tests_llm_16_1779_llm_16_1779_rrrruuuugggg_test_as_primitive_u8_to_f32 = 0;
        let rug_fuzz_0 = 255;
        let value_u8: u8 = rug_fuzz_0;
        let value_f32: f32 = AsPrimitive::<f32>::as_(value_u8);
        debug_assert_eq!(value_f32, 255f32);
        let _rug_ed_tests_llm_16_1779_llm_16_1779_rrrruuuugggg_test_as_primitive_u8_to_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1780_llm_16_1780 {
    use crate::AsPrimitive;
    #[test]
    fn test_as_primitive_u8_to_f64() {
        let _rug_st_tests_llm_16_1780_llm_16_1780_rrrruuuugggg_test_as_primitive_u8_to_f64 = 0;
        let rug_fuzz_0 = 42;
        let x: u8 = rug_fuzz_0;
        let y: f64 = AsPrimitive::<f64>::as_(x);
        debug_assert_eq!(y as u8, x);
        debug_assert_eq!(y, 42.0_f64);
        let _rug_ed_tests_llm_16_1780_llm_16_1780_rrrruuuugggg_test_as_primitive_u8_to_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1781_llm_16_1781 {
    use crate::cast::AsPrimitive;
    #[test]
    fn as_primitive_u8_to_i128() {
        let _rug_st_tests_llm_16_1781_llm_16_1781_rrrruuuugggg_as_primitive_u8_to_i128 = 0;
        let rug_fuzz_0 = 42;
        let value_u8: u8 = rug_fuzz_0;
        let value_i128: i128 = AsPrimitive::<i128>::as_(value_u8);
        debug_assert_eq!(value_i128, i128::from(value_u8));
        let _rug_ed_tests_llm_16_1781_llm_16_1781_rrrruuuugggg_as_primitive_u8_to_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1782_llm_16_1782 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_u8_to_i16() {
        let _rug_st_tests_llm_16_1782_llm_16_1782_rrrruuuugggg_test_as_primitive_u8_to_i16 = 0;
        let rug_fuzz_0 = 100;
        let value_u8: u8 = rug_fuzz_0;
        let value_i16: i16 = AsPrimitive::<i16>::as_(value_u8);
        debug_assert_eq!(value_i16, 100i16);
        let _rug_ed_tests_llm_16_1782_llm_16_1782_rrrruuuugggg_test_as_primitive_u8_to_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1783_llm_16_1783 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_u8_to_i32() {
        let _rug_st_tests_llm_16_1783_llm_16_1783_rrrruuuugggg_test_as_primitive_u8_to_i32 = 0;
        let rug_fuzz_0 = 100;
        let value_u8: u8 = rug_fuzz_0;
        let value_i32: i32 = AsPrimitive::<i32>::as_(value_u8);
        debug_assert_eq!(value_i32, 100i32);
        let _rug_ed_tests_llm_16_1783_llm_16_1783_rrrruuuugggg_test_as_primitive_u8_to_i32 = 0;
    }
    #[test]
    fn test_as_primitive_u8_to_i32_max_value() {
        let _rug_st_tests_llm_16_1783_llm_16_1783_rrrruuuugggg_test_as_primitive_u8_to_i32_max_value = 0;
        let value_u8: u8 = u8::MAX;
        let value_i32: i32 = AsPrimitive::<i32>::as_(value_u8);
        debug_assert_eq!(value_i32, u8::MAX as i32);
        let _rug_ed_tests_llm_16_1783_llm_16_1783_rrrruuuugggg_test_as_primitive_u8_to_i32_max_value = 0;
    }
    #[test]
    fn test_as_primitive_u8_to_i32_min_value() {
        let _rug_st_tests_llm_16_1783_llm_16_1783_rrrruuuugggg_test_as_primitive_u8_to_i32_min_value = 0;
        let value_u8: u8 = u8::MIN;
        let value_i32: i32 = AsPrimitive::<i32>::as_(value_u8);
        debug_assert_eq!(value_i32, u8::MIN as i32);
        let _rug_ed_tests_llm_16_1783_llm_16_1783_rrrruuuugggg_test_as_primitive_u8_to_i32_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1784_llm_16_1784 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_u8_to_i64() {
        let _rug_st_tests_llm_16_1784_llm_16_1784_rrrruuuugggg_test_as_primitive_u8_to_i64 = 0;
        let rug_fuzz_0 = 100;
        let value_u8: u8 = rug_fuzz_0;
        let value_i64: i64 = AsPrimitive::<i64>::as_(value_u8);
        debug_assert_eq!(value_i64, 100i64);
        let _rug_ed_tests_llm_16_1784_llm_16_1784_rrrruuuugggg_test_as_primitive_u8_to_i64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1785_llm_16_1785 {
    use crate::AsPrimitive;
    #[test]
    fn test_as_primitive_u8_to_i8() {
        let _rug_st_tests_llm_16_1785_llm_16_1785_rrrruuuugggg_test_as_primitive_u8_to_i8 = 0;
        let rug_fuzz_0 = 10_u8;
        let val_u8: u8 = rug_fuzz_0;
        let val_i8: i8 = AsPrimitive::<i8>::as_(val_u8);
        debug_assert_eq!(val_i8, 10_i8);
        let val_u8_max: u8 = u8::MAX;
        let val_i8_max: i8 = AsPrimitive::<i8>::as_(val_u8_max);
        let _rug_ed_tests_llm_16_1785_llm_16_1785_rrrruuuugggg_test_as_primitive_u8_to_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1786_llm_16_1786 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_u8_to_isize() {
        let _rug_st_tests_llm_16_1786_llm_16_1786_rrrruuuugggg_test_as_primitive_u8_to_isize = 0;
        let rug_fuzz_0 = 100;
        let value_u8: u8 = rug_fuzz_0;
        let value_isize: isize = AsPrimitive::<isize>::as_(value_u8);
        debug_assert_eq!(value_isize, 100isize);
        let _rug_ed_tests_llm_16_1786_llm_16_1786_rrrruuuugggg_test_as_primitive_u8_to_isize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1787_llm_16_1787 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_u8_to_u128() {
        let _rug_st_tests_llm_16_1787_llm_16_1787_rrrruuuugggg_test_as_primitive_u8_to_u128 = 0;
        let rug_fuzz_0 = 100;
        let value_u8: u8 = rug_fuzz_0;
        let value_u128: u128 = <u8 as AsPrimitive<u128>>::as_(value_u8);
        debug_assert_eq!(value_u128, 100u128);
        let _rug_ed_tests_llm_16_1787_llm_16_1787_rrrruuuugggg_test_as_primitive_u8_to_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1788_llm_16_1788 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_u8_as_u16() {
        let _rug_st_tests_llm_16_1788_llm_16_1788_rrrruuuugggg_test_u8_as_u16 = 0;
        let rug_fuzz_0 = 123;
        let value: u8 = rug_fuzz_0;
        let result: u16 = AsPrimitive::<u16>::as_(value);
        debug_assert_eq!(result, 123u16);
        let _rug_ed_tests_llm_16_1788_llm_16_1788_rrrruuuugggg_test_u8_as_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1789_llm_16_1789 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_u8_as_u32() {
        let _rug_st_tests_llm_16_1789_llm_16_1789_rrrruuuugggg_test_u8_as_u32 = 0;
        let rug_fuzz_0 = 100;
        let value: u8 = rug_fuzz_0;
        let result: u32 = AsPrimitive::<u32>::as_(value);
        debug_assert_eq!(result, 100u32);
        let _rug_ed_tests_llm_16_1789_llm_16_1789_rrrruuuugggg_test_u8_as_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1790_llm_16_1790 {
    use crate::AsPrimitive;
    #[test]
    fn as_u64() {
        let _rug_st_tests_llm_16_1790_llm_16_1790_rrrruuuugggg_as_u64 = 0;
        let rug_fuzz_0 = 100;
        let value: u8 = rug_fuzz_0;
        let result: u64 = AsPrimitive::<u64>::as_(value);
        debug_assert_eq!(result, 100u64);
        let _rug_ed_tests_llm_16_1790_llm_16_1790_rrrruuuugggg_as_u64 = 0;
    }
    #[test]
    fn as_u64_max_value() {
        let _rug_st_tests_llm_16_1790_llm_16_1790_rrrruuuugggg_as_u64_max_value = 0;
        let value: u8 = u8::MAX;
        let result: u64 = AsPrimitive::<u64>::as_(value);
        debug_assert_eq!(result, u8::MAX as u64);
        let _rug_ed_tests_llm_16_1790_llm_16_1790_rrrruuuugggg_as_u64_max_value = 0;
    }
    #[test]
    fn as_u64_zero() {
        let _rug_st_tests_llm_16_1790_llm_16_1790_rrrruuuugggg_as_u64_zero = 0;
        let rug_fuzz_0 = 0;
        let value: u8 = rug_fuzz_0;
        let result: u64 = AsPrimitive::<u64>::as_(value);
        debug_assert_eq!(result, 0u64);
        let _rug_ed_tests_llm_16_1790_llm_16_1790_rrrruuuugggg_as_u64_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1791_llm_16_1791 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_u8_as_u8() {
        let _rug_st_tests_llm_16_1791_llm_16_1791_rrrruuuugggg_test_u8_as_u8 = 0;
        let rug_fuzz_0 = 100;
        let value: u8 = rug_fuzz_0;
        let result: u8 = AsPrimitive::<u8>::as_(value);
        debug_assert_eq!(result, 100);
        let _rug_ed_tests_llm_16_1791_llm_16_1791_rrrruuuugggg_test_u8_as_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1792_llm_16_1792 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_u8_as_usize() {
        let _rug_st_tests_llm_16_1792_llm_16_1792_rrrruuuugggg_test_u8_as_usize = 0;
        let rug_fuzz_0 = 100;
        let value: u8 = rug_fuzz_0;
        let result: usize = AsPrimitive::<usize>::as_(value);
        debug_assert_eq!(result, 100usize);
        let _rug_ed_tests_llm_16_1792_llm_16_1792_rrrruuuugggg_test_u8_as_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1793_llm_16_1793 {
    use crate::FromPrimitive;
    #[test]
    fn test_from_f32() {
        let _rug_st_tests_llm_16_1793_llm_16_1793_rrrruuuugggg_test_from_f32 = 0;
        let rug_fuzz_0 = 0.0_f32;
        let rug_fuzz_1 = 255.0_f32;
        let rug_fuzz_2 = 1.0_f32;
        let rug_fuzz_3 = 256.0_f32;
        debug_assert_eq!(< u8 as FromPrimitive > ::from_f32(rug_fuzz_0), Some(0));
        debug_assert_eq!(< u8 as FromPrimitive > ::from_f32(rug_fuzz_1), Some(255));
        debug_assert_eq!(< u8 as FromPrimitive > ::from_f32(- rug_fuzz_2), None);
        debug_assert_eq!(< u8 as FromPrimitive > ::from_f32(rug_fuzz_3), None);
        debug_assert_eq!(< u8 as FromPrimitive > ::from_f32(f32::NAN), None);
        debug_assert_eq!(< u8 as FromPrimitive > ::from_f32(f32::INFINITY), None);
        debug_assert_eq!(< u8 as FromPrimitive > ::from_f32(f32::NEG_INFINITY), None);
        let _rug_ed_tests_llm_16_1793_llm_16_1793_rrrruuuugggg_test_from_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1794_llm_16_1794 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_f64() {
        let _rug_st_tests_llm_16_1794_llm_16_1794_rrrruuuugggg_test_from_f64 = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 0;
        let values: Vec<f64> = vec![
            rug_fuzz_0, 1.0, 255.0, 255.999, 256.0, - 1.0, 0.999, 1.999, - 0.999,
            f64::NAN, f64::INFINITY, f64::NEG_INFINITY
        ];
        let expected: Vec<Option<u8>> = vec![
            Some(rug_fuzz_1), Some(1), Some(255), Some(255), None, None, Some(0),
            Some(1), None, None, None, None
        ];
        let results: Vec<Option<u8>> = values
            .into_iter()
            .map(|x| <u8 as FromPrimitive>::from_f64(x))
            .collect();
        debug_assert_eq!(results, expected);
        let _rug_ed_tests_llm_16_1794_llm_16_1794_rrrruuuugggg_test_from_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1795_llm_16_1795 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i128() {
        let _rug_st_tests_llm_16_1795_llm_16_1795_rrrruuuugggg_test_from_i128 = 0;
        let rug_fuzz_0 = 0_i128;
        let rug_fuzz_1 = 255_i128;
        let rug_fuzz_2 = 1_i128;
        let rug_fuzz_3 = 256_i128;
        debug_assert_eq!(< u8 as FromPrimitive > ::from_i128(rug_fuzz_0), Some(0));
        debug_assert_eq!(< u8 as FromPrimitive > ::from_i128(rug_fuzz_1), Some(255));
        debug_assert_eq!(< u8 as FromPrimitive > ::from_i128(- rug_fuzz_2), None);
        debug_assert_eq!(< u8 as FromPrimitive > ::from_i128(rug_fuzz_3), None);
        let _rug_ed_tests_llm_16_1795_llm_16_1795_rrrruuuugggg_test_from_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1796_llm_16_1796 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i16() {
        let values: Vec<i16> = vec![0, 1, 127, 128, 255, - 1, - 128, 256,];
        for &val in &values {
            let result = u8::from_i16(val);
            match val {
                0..=255 => assert_eq!(result, Some(val as u8)),
                _ => assert_eq!(result, None),
            }
        }
    }
}
#[cfg(test)]
mod tests_llm_16_1797_llm_16_1797 {
    use crate::FromPrimitive;
    #[test]
    fn test_from_i32() {
        let _rug_st_tests_llm_16_1797_llm_16_1797_rrrruuuugggg_test_from_i32 = 0;
        let rug_fuzz_0 = 0_i32;
        let rug_fuzz_1 = 255_i32;
        let rug_fuzz_2 = 1_i32;
        let rug_fuzz_3 = 256_i32;
        debug_assert_eq!(< u8 as FromPrimitive > ::from_i32(rug_fuzz_0), Some(0_u8));
        debug_assert_eq!(< u8 as FromPrimitive > ::from_i32(rug_fuzz_1), Some(255_u8));
        debug_assert_eq!(< u8 as FromPrimitive > ::from_i32(- rug_fuzz_2), None);
        debug_assert_eq!(< u8 as FromPrimitive > ::from_i32(rug_fuzz_3), None);
        debug_assert_eq!(< u8 as FromPrimitive > ::from_i32(i32::MIN), None);
        debug_assert_eq!(< u8 as FromPrimitive > ::from_i32(i32::MAX), None);
        let _rug_ed_tests_llm_16_1797_llm_16_1797_rrrruuuugggg_test_from_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1798_llm_16_1798 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i64_with_u8() {
        let _rug_st_tests_llm_16_1798_llm_16_1798_rrrruuuugggg_test_from_i64_with_u8 = 0;
        let rug_fuzz_0 = 0i64;
        let rug_fuzz_1 = 255i64;
        let rug_fuzz_2 = 1i64;
        let rug_fuzz_3 = 256i64;
        debug_assert_eq!(< u8 as FromPrimitive > ::from_i64(rug_fuzz_0), Some(0u8));
        debug_assert_eq!(< u8 as FromPrimitive > ::from_i64(rug_fuzz_1), Some(255u8));
        debug_assert_eq!(< u8 as FromPrimitive > ::from_i64(- rug_fuzz_2), None);
        debug_assert_eq!(< u8 as FromPrimitive > ::from_i64(rug_fuzz_3), None);
        let _rug_ed_tests_llm_16_1798_llm_16_1798_rrrruuuugggg_test_from_i64_with_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1799_llm_16_1799 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i8() {
        assert_eq!(< u8 as FromPrimitive >::from_i8(0), Some(0_u8));
        assert_eq!(< u8 as FromPrimitive >::from_i8(127), Some(127_u8));
        assert_eq!(< u8 as FromPrimitive >::from_i8(- 1), None);
        assert_eq!(< u8 as FromPrimitive >::from_i8(- 128), None);
    }
}
#[cfg(test)]
mod tests_llm_16_1800_llm_16_1800 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_isize() {
        let _rug_st_tests_llm_16_1800_llm_16_1800_rrrruuuugggg_test_from_isize = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 255;
        let rug_fuzz_2 = 256;
        let rug_fuzz_3 = 1;
        debug_assert_eq!(< u8 as FromPrimitive > ::from_isize(rug_fuzz_0), Some(0u8));
        debug_assert_eq!(< u8 as FromPrimitive > ::from_isize(rug_fuzz_1), Some(255u8));
        debug_assert_eq!(< u8 as FromPrimitive > ::from_isize(rug_fuzz_2), None);
        debug_assert_eq!(< u8 as FromPrimitive > ::from_isize(- rug_fuzz_3), None);
        #[cfg(target_pointer_width = "32")]
        {
            debug_assert_eq!(< u8 as FromPrimitive > ::from_isize(isize::MAX), None);
        }
        #[cfg(target_pointer_width = "64")]
        {
            debug_assert_eq!(< u8 as FromPrimitive > ::from_isize(isize::MAX), None);
        }
        debug_assert_eq!(< u8 as FromPrimitive > ::from_isize(isize::MIN), None);
        let _rug_ed_tests_llm_16_1800_llm_16_1800_rrrruuuugggg_test_from_isize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1801_llm_16_1801 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u128_within_range() {
        let _rug_st_tests_llm_16_1801_llm_16_1801_rrrruuuugggg_test_from_u128_within_range = 0;
        let rug_fuzz_0 = 255_u128;
        debug_assert_eq!(< u8 as FromPrimitive > ::from_u128(rug_fuzz_0), Some(255_u8));
        let _rug_ed_tests_llm_16_1801_llm_16_1801_rrrruuuugggg_test_from_u128_within_range = 0;
    }
    #[test]
    fn test_from_u128_out_of_range() {
        let _rug_st_tests_llm_16_1801_llm_16_1801_rrrruuuugggg_test_from_u128_out_of_range = 0;
        let rug_fuzz_0 = 256_u128;
        debug_assert_eq!(< u8 as FromPrimitive > ::from_u128(rug_fuzz_0), None);
        let _rug_ed_tests_llm_16_1801_llm_16_1801_rrrruuuugggg_test_from_u128_out_of_range = 0;
    }
    #[test]
    fn test_from_u128_zero() {
        let _rug_st_tests_llm_16_1801_llm_16_1801_rrrruuuugggg_test_from_u128_zero = 0;
        let rug_fuzz_0 = 0_u128;
        debug_assert_eq!(< u8 as FromPrimitive > ::from_u128(rug_fuzz_0), Some(0_u8));
        let _rug_ed_tests_llm_16_1801_llm_16_1801_rrrruuuugggg_test_from_u128_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1802_llm_16_1802 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u16() {
        let _rug_st_tests_llm_16_1802_llm_16_1802_rrrruuuugggg_test_from_u16 = 0;
        let rug_fuzz_0 = 255u16;
        let rug_fuzz_1 = 256u16;
        let rug_fuzz_2 = 0u16;
        debug_assert_eq!(< u8 as FromPrimitive > ::from_u16(rug_fuzz_0), Some(255u8));
        debug_assert_eq!(< u8 as FromPrimitive > ::from_u16(rug_fuzz_1), None);
        debug_assert_eq!(< u8 as FromPrimitive > ::from_u16(rug_fuzz_2), Some(0u8));
        let _rug_ed_tests_llm_16_1802_llm_16_1802_rrrruuuugggg_test_from_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1803_llm_16_1803 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u32() {
        let _rug_st_tests_llm_16_1803_llm_16_1803_rrrruuuugggg_test_from_u32 = 0;
        let rug_fuzz_0 = 0_u32;
        let rug_fuzz_1 = 255_u32;
        let rug_fuzz_2 = 256_u32;
        debug_assert_eq!(< u8 as FromPrimitive > ::from_u32(rug_fuzz_0), Some(0_u8));
        debug_assert_eq!(< u8 as FromPrimitive > ::from_u32(rug_fuzz_1), Some(255_u8));
        debug_assert_eq!(< u8 as FromPrimitive > ::from_u32(rug_fuzz_2), None);
        let _rug_ed_tests_llm_16_1803_llm_16_1803_rrrruuuugggg_test_from_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1804_llm_16_1804 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u64() {
        let _rug_st_tests_llm_16_1804_llm_16_1804_rrrruuuugggg_test_from_u64 = 0;
        let rug_fuzz_0 = 0u64;
        let rug_fuzz_1 = 0u8;
        let rug_fuzz_2 = 255u64;
        let rug_fuzz_3 = 255u8;
        let rug_fuzz_4 = 256u64;
        let values: [(u64, Option<u8>); 3] = [
            (rug_fuzz_0, Some(rug_fuzz_1)),
            (rug_fuzz_2, Some(rug_fuzz_3)),
            (rug_fuzz_4, None),
        ];
        for &(value, expected) in values.iter() {
            debug_assert_eq!(< u8 as FromPrimitive > ::from_u64(value), expected);
        }
        let _rug_ed_tests_llm_16_1804_llm_16_1804_rrrruuuugggg_test_from_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1805_llm_16_1805 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u8() {
        let _rug_st_tests_llm_16_1805_llm_16_1805_rrrruuuugggg_test_from_u8 = 0;
        let rug_fuzz_0 = 0_u8;
        let rug_fuzz_1 = 1_u8;
        let rug_fuzz_2 = 255_u8;
        debug_assert_eq!(< u8 as FromPrimitive > ::from_u8(rug_fuzz_0), Some(0u8));
        debug_assert_eq!(< u8 as FromPrimitive > ::from_u8(rug_fuzz_1), Some(1u8));
        debug_assert_eq!(< u8 as FromPrimitive > ::from_u8(rug_fuzz_2), Some(255u8));
        let _rug_ed_tests_llm_16_1805_llm_16_1805_rrrruuuugggg_test_from_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1806_llm_16_1806 {
    use crate::cast::FromPrimitive;
    #[test]
    fn from_usize_test() {
        let _rug_st_tests_llm_16_1806_llm_16_1806_rrrruuuugggg_from_usize_test = 0;
        let rug_fuzz_0 = 0_usize;
        let rug_fuzz_1 = 255_usize;
        let rug_fuzz_2 = 256_usize;
        debug_assert_eq!(< u8 as FromPrimitive > ::from_usize(rug_fuzz_0), Some(0_u8));
        debug_assert_eq!(< u8 as FromPrimitive > ::from_usize(rug_fuzz_1), Some(255_u8));
        debug_assert_eq!(< u8 as FromPrimitive > ::from_usize(rug_fuzz_2), None);
        let _rug_ed_tests_llm_16_1806_llm_16_1806_rrrruuuugggg_from_usize_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1807_llm_16_1807 {
    use crate::cast::NumCast;
    use std::num::Wrapping;
    #[test]
    fn test_num_cast_from_wrapping_to_u8() {
        let int_values = vec![- 128i8, - 1i8, 0i8, 1i8, 127i8];
        let uint_values = vec![0u8, 1u8, 255u8];
        for &int_val in &int_values {
            let wrapped_int = Wrapping(int_val);
            let casted = <u8 as NumCast>::from(wrapped_int);
            if int_val < 0 {
                assert_eq!(casted, None);
            } else {
                assert_eq!(casted, Some(int_val as u8));
            }
        }
        for &uint_val in &uint_values {
            let wrapped_uint = Wrapping(uint_val);
            let casted = <u8 as NumCast>::from(wrapped_uint);
            assert_eq!(casted, Some(uint_val));
        }
    }
}
#[cfg(test)]
mod tests_llm_16_1808 {
    use crate::ToPrimitive;
    #[test]
    fn u8_to_f32() {
        let _rug_st_tests_llm_16_1808_rrrruuuugggg_u8_to_f32 = 0;
        let rug_fuzz_0 = 0u8;
        let rug_fuzz_1 = 1u8;
        let rug_fuzz_2 = 255u8;
        debug_assert_eq!(rug_fuzz_0.to_f32(), Some(0.0f32));
        debug_assert_eq!(rug_fuzz_1.to_f32(), Some(1.0f32));
        debug_assert_eq!(rug_fuzz_2.to_f32(), Some(255.0f32));
        let _rug_ed_tests_llm_16_1808_rrrruuuugggg_u8_to_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1809_llm_16_1809 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_u8_to_f64_casting() {
        let _rug_st_tests_llm_16_1809_llm_16_1809_rrrruuuugggg_test_u8_to_f64_casting = 0;
        let rug_fuzz_0 = 0_u8;
        let rug_fuzz_1 = 1_u8;
        debug_assert_eq!(ToPrimitive::to_f64(& rug_fuzz_0), Some(0.0_f64));
        debug_assert_eq!(ToPrimitive::to_f64(& rug_fuzz_1), Some(1.0_f64));
        debug_assert_eq!(ToPrimitive::to_f64(& u8::MAX), Some(255.0_f64));
        let _rug_ed_tests_llm_16_1809_llm_16_1809_rrrruuuugggg_test_u8_to_f64_casting = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1810_llm_16_1810 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_u8_to_i128() {
        let _rug_st_tests_llm_16_1810_llm_16_1810_rrrruuuugggg_test_u8_to_i128 = 0;
        let rug_fuzz_0 = 0u8;
        let rug_fuzz_1 = 1u8;
        debug_assert_eq!(rug_fuzz_0.to_i128(), Some(0i128));
        debug_assert_eq!(rug_fuzz_1.to_i128(), Some(1i128));
        debug_assert_eq!(u8::MAX.to_i128(), Some(255i128));
        let _rug_ed_tests_llm_16_1810_llm_16_1810_rrrruuuugggg_test_u8_to_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1811_llm_16_1811 {
    use crate::ToPrimitive;
    #[test]
    fn test_u8_to_i16() {
        let _rug_st_tests_llm_16_1811_llm_16_1811_rrrruuuugggg_test_u8_to_i16 = 0;
        let rug_fuzz_0 = 0_u8;
        let rug_fuzz_1 = 1_u8;
        let rug_fuzz_2 = 127_u8;
        let rug_fuzz_3 = 255_u8;
        debug_assert_eq!(rug_fuzz_0.to_i16(), Some(0_i16));
        debug_assert_eq!(rug_fuzz_1.to_i16(), Some(1_i16));
        debug_assert_eq!(rug_fuzz_2.to_i16(), Some(127_i16));
        debug_assert_eq!(rug_fuzz_3.to_i16(), Some(255_i16));
        let _rug_ed_tests_llm_16_1811_llm_16_1811_rrrruuuugggg_test_u8_to_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1812_llm_16_1812 {
    use super::*;
    use crate::*;
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_i32_success() {
        let _rug_st_tests_llm_16_1812_llm_16_1812_rrrruuuugggg_test_to_i32_success = 0;
        let rug_fuzz_0 = 100;
        let value: u8 = rug_fuzz_0;
        let result: Option<i32> = ToPrimitive::to_i32(&value);
        debug_assert_eq!(result, Some(100i32));
        let _rug_ed_tests_llm_16_1812_llm_16_1812_rrrruuuugggg_test_to_i32_success = 0;
    }
    #[test]
    fn test_to_i32_none() {
        let _rug_st_tests_llm_16_1812_llm_16_1812_rrrruuuugggg_test_to_i32_none = 0;
        let rug_fuzz_0 = 255;
        let value: u8 = rug_fuzz_0;
        let result: Option<i32> = ToPrimitive::to_i32(&value);
        debug_assert!(result.is_some());
        let _rug_ed_tests_llm_16_1812_llm_16_1812_rrrruuuugggg_test_to_i32_none = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1813_llm_16_1813 {
    use crate::ToPrimitive;
    #[test]
    fn test_u8_to_i64() {
        let _rug_st_tests_llm_16_1813_llm_16_1813_rrrruuuugggg_test_u8_to_i64 = 0;
        let rug_fuzz_0 = 0u8;
        let rug_fuzz_1 = 1u8;
        debug_assert_eq!(rug_fuzz_0.to_i64(), Some(0i64));
        debug_assert_eq!(rug_fuzz_1.to_i64(), Some(1i64));
        debug_assert_eq!(u8::MAX.to_i64(), Some(255i64));
        let _rug_ed_tests_llm_16_1813_llm_16_1813_rrrruuuugggg_test_u8_to_i64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1814_llm_16_1814 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_u8_to_i8() {
        let _rug_st_tests_llm_16_1814_llm_16_1814_rrrruuuugggg_test_u8_to_i8 = 0;
        let rug_fuzz_0 = 0u8;
        let rug_fuzz_1 = 127u8;
        let rug_fuzz_2 = 128u8;
        let rug_fuzz_3 = 255u8;
        debug_assert_eq!(rug_fuzz_0.to_i8(), Some(0i8));
        debug_assert_eq!(rug_fuzz_1.to_i8(), Some(127i8));
        debug_assert_eq!(rug_fuzz_2.to_i8(), None);
        debug_assert_eq!(rug_fuzz_3.to_i8(), None);
        let _rug_ed_tests_llm_16_1814_llm_16_1814_rrrruuuugggg_test_u8_to_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1815_llm_16_1815 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_isize_within_range() {
        let _rug_st_tests_llm_16_1815_llm_16_1815_rrrruuuugggg_test_to_isize_within_range = 0;
        let rug_fuzz_0 = 100;
        let value: u8 = rug_fuzz_0;
        debug_assert_eq!(value.to_isize(), Some(100isize));
        let _rug_ed_tests_llm_16_1815_llm_16_1815_rrrruuuugggg_test_to_isize_within_range = 0;
    }
    #[test]
    fn test_to_isize_at_limit() {
        let _rug_st_tests_llm_16_1815_llm_16_1815_rrrruuuugggg_test_to_isize_at_limit = 0;
        let value: u8 = isize::MAX as u8;
        debug_assert!(value.to_isize().is_some());
        let _rug_ed_tests_llm_16_1815_llm_16_1815_rrrruuuugggg_test_to_isize_at_limit = 0;
    }
    #[test]
    fn test_to_isize_overflow() {
        let _rug_st_tests_llm_16_1815_llm_16_1815_rrrruuuugggg_test_to_isize_overflow = 0;
        let value: u8 = u8::MAX;
        debug_assert_eq!(
            value.to_isize(), if u8::MAX as isize <= isize::MAX { Some(u8::MAX as isize)
            } else { None }
        );
        let _rug_ed_tests_llm_16_1815_llm_16_1815_rrrruuuugggg_test_to_isize_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1816_llm_16_1816 {
    use crate::cast::ToPrimitive;
    #[test]
    fn u8_to_u128() {
        let _rug_st_tests_llm_16_1816_llm_16_1816_rrrruuuugggg_u8_to_u128 = 0;
        let rug_fuzz_0 = 0u8;
        let rug_fuzz_1 = 1u8;
        debug_assert_eq!(rug_fuzz_0.to_u128(), Some(0u128));
        debug_assert_eq!(rug_fuzz_1.to_u128(), Some(1u128));
        debug_assert_eq!(u8::max_value().to_u128(), Some(255u128));
        let _rug_ed_tests_llm_16_1816_llm_16_1816_rrrruuuugggg_u8_to_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1817_llm_16_1817 {
    use crate::cast::ToPrimitive;
    #[test]
    fn u8_to_u16_always_succeeds() {
        let _rug_st_tests_llm_16_1817_llm_16_1817_rrrruuuugggg_u8_to_u16_always_succeeds = 0;
        let rug_fuzz_0 = 0u8;
        for i in rug_fuzz_0..=u8::MAX {
            debug_assert_eq!(ToPrimitive::to_u16(& i), Some(i as u16));
        }
        let _rug_ed_tests_llm_16_1817_llm_16_1817_rrrruuuugggg_u8_to_u16_always_succeeds = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1818_llm_16_1818 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_u8_to_u32() {
        let _rug_st_tests_llm_16_1818_llm_16_1818_rrrruuuugggg_test_u8_to_u32 = 0;
        let rug_fuzz_0 = 100;
        let value: u8 = rug_fuzz_0;
        debug_assert_eq!(value.to_u32(), Some(100u32));
        let value: u8 = u8::MAX;
        debug_assert_eq!(value.to_u32(), Some(u32::from(u8::MAX)));
        let _rug_ed_tests_llm_16_1818_llm_16_1818_rrrruuuugggg_test_u8_to_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1819_llm_16_1819 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_u8_to_u64_within_bounds() {
        let _rug_st_tests_llm_16_1819_llm_16_1819_rrrruuuugggg_test_u8_to_u64_within_bounds = 0;
        let rug_fuzz_0 = 100;
        let value: u8 = rug_fuzz_0;
        let result = ToPrimitive::to_u64(&value);
        debug_assert_eq!(result, Some(100u64));
        let _rug_ed_tests_llm_16_1819_llm_16_1819_rrrruuuugggg_test_u8_to_u64_within_bounds = 0;
    }
    #[test]
    fn test_u8_to_u64_at_upper_bound() {
        let _rug_st_tests_llm_16_1819_llm_16_1819_rrrruuuugggg_test_u8_to_u64_at_upper_bound = 0;
        let value: u8 = u8::MAX;
        let result = ToPrimitive::to_u64(&value);
        debug_assert_eq!(result, Some(u64::from(u8::MAX)));
        let _rug_ed_tests_llm_16_1819_llm_16_1819_rrrruuuugggg_test_u8_to_u64_at_upper_bound = 0;
    }
    #[test]
    fn test_u8_to_u64_at_zero() {
        let _rug_st_tests_llm_16_1819_llm_16_1819_rrrruuuugggg_test_u8_to_u64_at_zero = 0;
        let rug_fuzz_0 = 0;
        let value: u8 = rug_fuzz_0;
        let result = ToPrimitive::to_u64(&value);
        debug_assert_eq!(result, Some(0u64));
        let _rug_ed_tests_llm_16_1819_llm_16_1819_rrrruuuugggg_test_u8_to_u64_at_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1820_llm_16_1820 {
    use super::*;
    use crate::*;
    #[test]
    fn test_u8_identity() {
        let _rug_st_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_identity = 0;
        let rug_fuzz_0 = 100;
        let value: u8 = rug_fuzz_0;
        debug_assert_eq!(< u8 as ToPrimitive > ::to_u8(& value), Some(100));
        let _rug_ed_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_identity = 0;
    }
    #[test]
    fn test_u8_max_value() {
        let _rug_st_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_max_value = 0;
        let value: u8 = u8::MAX;
        debug_assert_eq!(< u8 as ToPrimitive > ::to_u8(& value), Some(u8::MAX));
        let _rug_ed_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_max_value = 0;
    }
    #[test]
    fn test_u8_from_u16() {
        let _rug_st_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_u16 = 0;
        let rug_fuzz_0 = 100;
        let value: u16 = rug_fuzz_0;
        debug_assert_eq!(< u16 as ToPrimitive > ::to_u8(& value), Some(100));
        let _rug_ed_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_u16 = 0;
    }
    #[test]
    fn test_u8_from_u16_overflow() {
        let _rug_st_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_u16_overflow = 0;
        let value: u16 = u16::MAX;
        debug_assert_eq!(< u16 as ToPrimitive > ::to_u8(& value), None);
        let _rug_ed_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_u16_overflow = 0;
    }
    #[test]
    fn test_u8_from_u32() {
        let _rug_st_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_u32 = 0;
        let rug_fuzz_0 = 100;
        let value: u32 = rug_fuzz_0;
        debug_assert_eq!(< u32 as ToPrimitive > ::to_u8(& value), Some(100));
        let _rug_ed_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_u32 = 0;
    }
    #[test]
    fn test_u8_from_u32_overflow() {
        let _rug_st_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_u32_overflow = 0;
        let value: u32 = u32::MAX;
        debug_assert_eq!(< u32 as ToPrimitive > ::to_u8(& value), None);
        let _rug_ed_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_u32_overflow = 0;
    }
    #[test]
    fn test_u8_from_u64() {
        let _rug_st_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_u64 = 0;
        let rug_fuzz_0 = 100;
        let value: u64 = rug_fuzz_0;
        debug_assert_eq!(< u64 as ToPrimitive > ::to_u8(& value), Some(100));
        let _rug_ed_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_u64 = 0;
    }
    #[test]
    fn test_u8_from_u64_overflow() {
        let _rug_st_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_u64_overflow = 0;
        let value: u64 = u64::MAX;
        debug_assert_eq!(< u64 as ToPrimitive > ::to_u8(& value), None);
        let _rug_ed_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_u64_overflow = 0;
    }
    #[test]
    fn test_u8_from_usize() {
        let _rug_st_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_usize = 0;
        let rug_fuzz_0 = 100;
        let value: usize = rug_fuzz_0;
        debug_assert_eq!(< usize as ToPrimitive > ::to_u8(& value), Some(100));
        let _rug_ed_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_usize = 0;
    }
    #[test]
    fn test_u8_from_usize_overflow() {
        let _rug_st_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_usize_overflow = 0;
        let value: usize = usize::MAX;
        debug_assert_eq!(< usize as ToPrimitive > ::to_u8(& value), None);
        let _rug_ed_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_usize_overflow = 0;
    }
    #[test]
    fn test_u8_from_i8() {
        let _rug_st_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_i8 = 0;
        let rug_fuzz_0 = 100;
        let value: i8 = rug_fuzz_0;
        debug_assert_eq!(< i8 as ToPrimitive > ::to_u8(& value), Some(100));
        let _rug_ed_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_i8 = 0;
    }
    #[test]
    fn test_u8_from_negative_i8() {
        let _rug_st_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_negative_i8 = 0;
        let rug_fuzz_0 = 1;
        let value: i8 = -rug_fuzz_0;
        debug_assert_eq!(< i8 as ToPrimitive > ::to_u8(& value), None);
        let _rug_ed_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_negative_i8 = 0;
    }
    #[test]
    fn test_u8_from_i16() {
        let _rug_st_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_i16 = 0;
        let rug_fuzz_0 = 100;
        let value: i16 = rug_fuzz_0;
        debug_assert_eq!(< i16 as ToPrimitive > ::to_u8(& value), Some(100));
        let _rug_ed_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_i16 = 0;
    }
    #[test]
    fn test_u8_from_i16_overflow() {
        let _rug_st_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_i16_overflow = 0;
        let value: i16 = i16::MAX;
        debug_assert_eq!(< i16 as ToPrimitive > ::to_u8(& value), None);
        let _rug_ed_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_i16_overflow = 0;
    }
    #[test]
    fn test_u8_from_negative_i16() {
        let _rug_st_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_negative_i16 = 0;
        let rug_fuzz_0 = 1;
        let value: i16 = -rug_fuzz_0;
        debug_assert_eq!(< i16 as ToPrimitive > ::to_u8(& value), None);
        let _rug_ed_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_negative_i16 = 0;
    }
    #[test]
    fn test_u8_from_i32() {
        let _rug_st_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_i32 = 0;
        let rug_fuzz_0 = 100;
        let value: i32 = rug_fuzz_0;
        debug_assert_eq!(< i32 as ToPrimitive > ::to_u8(& value), Some(100));
        let _rug_ed_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_i32 = 0;
    }
    #[test]
    fn test_u8_from_i32_overflow() {
        let _rug_st_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_i32_overflow = 0;
        let value: i32 = i32::MAX;
        debug_assert_eq!(< i32 as ToPrimitive > ::to_u8(& value), None);
        let _rug_ed_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_i32_overflow = 0;
    }
    #[test]
    fn test_u8_from_negative_i32() {
        let _rug_st_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_negative_i32 = 0;
        let rug_fuzz_0 = 1;
        let value: i32 = -rug_fuzz_0;
        debug_assert_eq!(< i32 as ToPrimitive > ::to_u8(& value), None);
        let _rug_ed_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_negative_i32 = 0;
    }
    #[test]
    fn test_u8_from_i64() {
        let _rug_st_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_i64 = 0;
        let rug_fuzz_0 = 100;
        let value: i64 = rug_fuzz_0;
        debug_assert_eq!(< i64 as ToPrimitive > ::to_u8(& value), Some(100));
        let _rug_ed_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_i64 = 0;
    }
    #[test]
    fn test_u8_from_i64_overflow() {
        let _rug_st_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_i64_overflow = 0;
        let value: i64 = i64::MAX;
        debug_assert_eq!(< i64 as ToPrimitive > ::to_u8(& value), None);
        let _rug_ed_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_i64_overflow = 0;
    }
    #[test]
    fn test_u8_from_negative_i64() {
        let _rug_st_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_negative_i64 = 0;
        let rug_fuzz_0 = 1;
        let value: i64 = -rug_fuzz_0;
        debug_assert_eq!(< i64 as ToPrimitive > ::to_u8(& value), None);
        let _rug_ed_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_negative_i64 = 0;
    }
    #[test]
    fn test_u8_from_isize() {
        let _rug_st_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_isize = 0;
        let rug_fuzz_0 = 100;
        let value: isize = rug_fuzz_0;
        debug_assert_eq!(< isize as ToPrimitive > ::to_u8(& value), Some(100));
        let _rug_ed_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_isize = 0;
    }
    #[test]
    fn test_u8_from_isize_overflow() {
        let _rug_st_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_isize_overflow = 0;
        let value: isize = isize::MAX;
        debug_assert_eq!(< isize as ToPrimitive > ::to_u8(& value), None);
        let _rug_ed_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_isize_overflow = 0;
    }
    #[test]
    fn test_u8_from_negative_isize() {
        let _rug_st_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_negative_isize = 0;
        let rug_fuzz_0 = 1;
        let value: isize = -rug_fuzz_0;
        debug_assert_eq!(< isize as ToPrimitive > ::to_u8(& value), None);
        let _rug_ed_tests_llm_16_1820_llm_16_1820_rrrruuuugggg_test_u8_from_negative_isize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1821_llm_16_1821 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_usize() {
        let _rug_st_tests_llm_16_1821_llm_16_1821_rrrruuuugggg_test_to_usize = 0;
        let rug_fuzz_0 = 42;
        let small_value: u8 = rug_fuzz_0;
        let large_value: u8 = u8::MAX;
        let small_value_converted = small_value.to_usize();
        let large_value_converted = large_value.to_usize();
        debug_assert_eq!(small_value_converted, Some(42usize));
        debug_assert_eq!(large_value_converted, Some(u8::MAX as usize));
        let _rug_ed_tests_llm_16_1821_llm_16_1821_rrrruuuugggg_test_to_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1884_llm_16_1884 {
    use crate::AsPrimitive;
    #[test]
    fn usize_as_f32() {
        let _rug_st_tests_llm_16_1884_llm_16_1884_rrrruuuugggg_usize_as_f32 = 0;
        let rug_fuzz_0 = 42;
        let value: usize = rug_fuzz_0;
        let result: f32 = AsPrimitive::<f32>::as_(value);
        debug_assert_eq!(result, 42f32);
        let _rug_ed_tests_llm_16_1884_llm_16_1884_rrrruuuugggg_usize_as_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1885_llm_16_1885 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_usize_to_f64() {
        let _rug_st_tests_llm_16_1885_llm_16_1885_rrrruuuugggg_test_as_primitive_usize_to_f64 = 0;
        let rug_fuzz_0 = 42;
        let value: usize = rug_fuzz_0;
        let result: f64 = AsPrimitive::<f64>::as_(value);
        debug_assert_eq!(result, 42f64);
        let _rug_ed_tests_llm_16_1885_llm_16_1885_rrrruuuugggg_test_as_primitive_usize_to_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1886_llm_16_1886 {
    use crate::cast::AsPrimitive;
    #[test]
    fn usize_as_i128() {
        let _rug_st_tests_llm_16_1886_llm_16_1886_rrrruuuugggg_usize_as_i128 = 0;
        let rug_fuzz_0 = 42;
        let value: usize = rug_fuzz_0;
        let result: i128 = AsPrimitive::<i128>::as_(value);
        debug_assert_eq!(result, 42i128);
        let _rug_ed_tests_llm_16_1886_llm_16_1886_rrrruuuugggg_usize_as_i128 = 0;
    }
    #[test]
    fn usize_as_i128_max() {
        let _rug_st_tests_llm_16_1886_llm_16_1886_rrrruuuugggg_usize_as_i128_max = 0;
        let value: usize = usize::MAX;
        let result: i128 = AsPrimitive::<i128>::as_(value);
        debug_assert_eq!(result, usize::MAX as i128);
        let _rug_ed_tests_llm_16_1886_llm_16_1886_rrrruuuugggg_usize_as_i128_max = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1887_llm_16_1887 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_usize_to_i16() {
        let _rug_st_tests_llm_16_1887_llm_16_1887_rrrruuuugggg_test_as_primitive_usize_to_i16 = 0;
        let rug_fuzz_0 = 42;
        let val: usize = rug_fuzz_0;
        let result: i16 = AsPrimitive::<i16>::as_(val);
        debug_assert_eq!(result, 42i16);
        let _rug_ed_tests_llm_16_1887_llm_16_1887_rrrruuuugggg_test_as_primitive_usize_to_i16 = 0;
    }
    #[test]
    #[should_panic]
    fn test_as_primitive_usize_to_i16_overflow() {
        let _rug_st_tests_llm_16_1887_llm_16_1887_rrrruuuugggg_test_as_primitive_usize_to_i16_overflow = 0;
        let val: usize = usize::MAX;
        let _result: i16 = AsPrimitive::<i16>::as_(val);
        let _rug_ed_tests_llm_16_1887_llm_16_1887_rrrruuuugggg_test_as_primitive_usize_to_i16_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1888_llm_16_1888 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_usize_to_i32() {
        let _rug_st_tests_llm_16_1888_llm_16_1888_rrrruuuugggg_test_as_primitive_usize_to_i32 = 0;
        let rug_fuzz_0 = 42;
        let value: usize = rug_fuzz_0;
        let result: i32 = AsPrimitive::<i32>::as_(value);
        debug_assert_eq!(result, 42i32);
        let _rug_ed_tests_llm_16_1888_llm_16_1888_rrrruuuugggg_test_as_primitive_usize_to_i32 = 0;
    }
    #[test]
    #[should_panic(expected = "attempt to cast with overflow")]
    fn test_as_primitive_usize_to_i32_overflow() {
        let _rug_st_tests_llm_16_1888_llm_16_1888_rrrruuuugggg_test_as_primitive_usize_to_i32_overflow = 0;
        let value: usize = usize::MAX;
        let _result: i32 = AsPrimitive::<i32>::as_(value);
        let _rug_ed_tests_llm_16_1888_llm_16_1888_rrrruuuugggg_test_as_primitive_usize_to_i32_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1889_llm_16_1889 {
    use crate::cast::AsPrimitive;
    #[test]
    fn usize_as_i64() {
        let _rug_st_tests_llm_16_1889_llm_16_1889_rrrruuuugggg_usize_as_i64 = 0;
        let rug_fuzz_0 = 42;
        let value: usize = rug_fuzz_0;
        let casted_value: i64 = value.as_();
        debug_assert_eq!(casted_value, 42i64);
        let _rug_ed_tests_llm_16_1889_llm_16_1889_rrrruuuugggg_usize_as_i64 = 0;
    }
    #[test]
    fn usize_max_as_i64() {
        let _rug_st_tests_llm_16_1889_llm_16_1889_rrrruuuugggg_usize_max_as_i64 = 0;
        let rug_fuzz_0 = 0;
        let value: usize = usize::MAX;
        let value_as_i64 = i64::max_value() as usize;
        if value_as_i64 as usize == usize::MAX {
            let casted_value: i64 = value.as_();
            debug_assert_eq!(casted_value, i64::max_value());
        } else {
            let casted_value: i64 = value.as_();
            debug_assert!(casted_value < rug_fuzz_0);
        }
        let _rug_ed_tests_llm_16_1889_llm_16_1889_rrrruuuugggg_usize_max_as_i64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1890_llm_16_1890 {
    use crate::AsPrimitive;
    #[test]
    fn test_as_primitive_usize_to_i8() {
        let _rug_st_tests_llm_16_1890_llm_16_1890_rrrruuuugggg_test_as_primitive_usize_to_i8 = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 1;
        let value: usize = rug_fuzz_0;
        let result: i8 = value.as_();
        debug_assert_eq!(result, 42i8);
        let max_i8_as_usize: usize = i8::MAX as usize;
        let max_result: i8 = max_i8_as_usize.as_();
        debug_assert_eq!(max_result, i8::MAX);
        #[cfg(not(debug_assertions))]
        {
            let out_of_range_value: usize = (i8::MAX as usize) + rug_fuzz_1;
            let out_of_range_result: i8 = out_of_range_value.as_();
            debug_assert_eq!(out_of_range_result, i8::MIN);
        }
        let _rug_ed_tests_llm_16_1890_llm_16_1890_rrrruuuugggg_test_as_primitive_usize_to_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1891_llm_16_1891 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_usize_to_isize() {
        let _rug_st_tests_llm_16_1891_llm_16_1891_rrrruuuugggg_test_as_primitive_usize_to_isize = 0;
        let rug_fuzz_0 = 42;
        let value_usize: usize = rug_fuzz_0;
        let value_isize: isize = value_usize.as_();
        debug_assert_eq!(value_isize, 42isize);
        let _rug_ed_tests_llm_16_1891_llm_16_1891_rrrruuuugggg_test_as_primitive_usize_to_isize = 0;
    }
    #[test]
    fn test_as_primitive_usize_to_isize_edge_cases() {
        let _rug_st_tests_llm_16_1891_llm_16_1891_rrrruuuugggg_test_as_primitive_usize_to_isize_edge_cases = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0;
        if usize::BITS == isize::BITS {
            let max_usize: usize = usize::MAX;
            let max_isize: isize = max_usize.as_();
            let max_isize_expected = if isize::BITS < usize::BITS {
                (isize::MAX as usize).as_()
            } else {
                isize::MAX
            };
            debug_assert_eq!(
                max_isize, max_isize_expected,
                "usize::MAX to isize cast may not behave as expected if isize has fewer bits than usize"
            );
        }
        let zero_usize: usize = rug_fuzz_0;
        let zero_isize: isize = zero_usize.as_();
        debug_assert_eq!(zero_isize, 0isize);
        if (usize::MAX as isize) >= rug_fuzz_1 {
            let max_usize: usize = usize::MAX;
            let max_isize: isize = max_usize.as_();
            debug_assert_eq!(max_isize, isize::MAX);
        }
        let _rug_ed_tests_llm_16_1891_llm_16_1891_rrrruuuugggg_test_as_primitive_usize_to_isize_edge_cases = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1892_llm_16_1892 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_usize_to_u128() {
        let _rug_st_tests_llm_16_1892_llm_16_1892_rrrruuuugggg_test_as_usize_to_u128 = 0;
        let rug_fuzz_0 = 42;
        let value: usize = rug_fuzz_0;
        let result: u128 = AsPrimitive::<u128>::as_(value);
        debug_assert_eq!(result, 42u128);
        let _rug_ed_tests_llm_16_1892_llm_16_1892_rrrruuuugggg_test_as_usize_to_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1893_llm_16_1893 {
    use crate::AsPrimitive;
    #[test]
    fn test_as_primitive_usize_to_u16() {
        let _rug_st_tests_llm_16_1893_llm_16_1893_rrrruuuugggg_test_as_primitive_usize_to_u16 = 0;
        let rug_fuzz_0 = 42usize;
        let rug_fuzz_1 = 1;
        let x = rug_fuzz_0;
        let y: u16 = x.as_();
        debug_assert_eq!(y, 42u16);
        let max_value = u16::MAX as usize;
        let max_u16: u16 = max_value.as_();
        debug_assert_eq!(max_u16, u16::MAX);
        let over_u16 = (u16::MAX as usize) + rug_fuzz_1;
        let should_wrap: u16 = over_u16.as_();
        let _rug_ed_tests_llm_16_1893_llm_16_1893_rrrruuuugggg_test_as_primitive_usize_to_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1894_llm_16_1894 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_as_primitive_usize_to_u32() {
        let _rug_st_tests_llm_16_1894_llm_16_1894_rrrruuuugggg_test_as_primitive_usize_to_u32 = 0;
        let rug_fuzz_0 = 42;
        let value: usize = rug_fuzz_0;
        let result: u32 = AsPrimitive::<u32>::as_(value);
        debug_assert_eq!(result, 42u32);
        let _rug_ed_tests_llm_16_1894_llm_16_1894_rrrruuuugggg_test_as_primitive_usize_to_u32 = 0;
    }
    #[test]
    fn test_as_primitive_usize_to_u32_overflow() {
        let _rug_st_tests_llm_16_1894_llm_16_1894_rrrruuuugggg_test_as_primitive_usize_to_u32_overflow = 0;
        let value: usize = usize::max_value();
        if usize::BITS > u32::BITS {
            let _result: u32 = AsPrimitive::<u32>::as_(value);
        } else {
            let _result: u32 = AsPrimitive::<u32>::as_(value);
        }
        let _rug_ed_tests_llm_16_1894_llm_16_1894_rrrruuuugggg_test_as_primitive_usize_to_u32_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1895_llm_16_1895 {
    use crate::cast::AsPrimitive;
    #[test]
    fn usize_as_u64() {
        let _rug_st_tests_llm_16_1895_llm_16_1895_rrrruuuugggg_usize_as_u64 = 0;
        let rug_fuzz_0 = 42;
        let value: usize = rug_fuzz_0;
        let result: u64 = value.as_();
        debug_assert_eq!(result, 42u64);
        let _rug_ed_tests_llm_16_1895_llm_16_1895_rrrruuuugggg_usize_as_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1896_llm_16_1896 {
    use crate::cast::AsPrimitive;
    #[test]
    fn test_usize_as_u8() {
        let _rug_st_tests_llm_16_1896_llm_16_1896_rrrruuuugggg_test_usize_as_u8 = 0;
        let rug_fuzz_0 = 255;
        let rug_fuzz_1 = 256;
        let rug_fuzz_2 = 1;
        let value: usize = rug_fuzz_0;
        let result: u8 = AsPrimitive::<u8>::as_(value);
        debug_assert_eq!(result, 255u8);
        let value: usize = rug_fuzz_1;
        let result: u8 = AsPrimitive::<u8>::as_(value);
        debug_assert_eq!(result, 0u8);
        let value: usize = rug_fuzz_2;
        let result: u8 = AsPrimitive::<u8>::as_(value);
        debug_assert_eq!(result, 1u8);
        let _rug_ed_tests_llm_16_1896_llm_16_1896_rrrruuuugggg_test_usize_as_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1897_llm_16_1897 {
    use crate::AsPrimitive;
    #[test]
    fn test_as_primitive_usize() {
        let _rug_st_tests_llm_16_1897_llm_16_1897_rrrruuuugggg_test_as_primitive_usize = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 0;
        let x: usize = rug_fuzz_0;
        let y: usize = AsPrimitive::<usize>::as_(x);
        debug_assert_eq!(y, 42);
        let big_num: usize = usize::MAX;
        let big_num_as_usize: usize = AsPrimitive::<usize>::as_(big_num);
        debug_assert_eq!(big_num_as_usize, usize::MAX);
        let zero: usize = rug_fuzz_1;
        let zero_as_usize: usize = AsPrimitive::<usize>::as_(zero);
        debug_assert_eq!(zero_as_usize, 0);
        let _rug_ed_tests_llm_16_1897_llm_16_1897_rrrruuuugggg_test_as_primitive_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1898_llm_16_1898 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_f32() {
        let _rug_st_tests_llm_16_1898_llm_16_1898_rrrruuuugggg_test_from_f32 = 0;
        let rug_fuzz_0 = 0.0_f32;
        let rug_fuzz_1 = 0_usize;
        let test_values = vec![
            (rug_fuzz_0, Some(rug_fuzz_1)), (1.0_f32, Some(1_usize)), (1.5_f32, None), (-
            1.0_f32, None), (f32::MAX, None), (f32::MIN, None), (f32::EPSILON, None),
            (f32::INFINITY, None), (f32::NEG_INFINITY, None), (f32::NAN, None)
        ];
        for (value, expected) in test_values {
            let result = <usize as FromPrimitive>::from_f32(value);
            debug_assert_eq!(result, expected);
        }
        let _rug_ed_tests_llm_16_1898_llm_16_1898_rrrruuuugggg_test_from_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1899_llm_16_1899 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_f64() {
        let _rug_st_tests_llm_16_1899_llm_16_1899_rrrruuuugggg_test_from_f64 = 0;
        let rug_fuzz_0 = 42.0f64;
        let rug_fuzz_1 = 1e20f64;
        let rug_fuzz_2 = 1.0f64;
        let value_f64 = rug_fuzz_0;
        let value_usize = usize::from_f64(value_f64);
        debug_assert_eq!(value_usize, Some(42));
        let large_value_f64 = rug_fuzz_1;
        let large_value_usize = usize::from_f64(large_value_f64);
        #[cfg(target_pointer_width = "64")]
        debug_assert_eq!(large_value_usize, Some(1e20f64 as usize));
        #[cfg(not(target_pointer_width = "64"))]
        debug_assert_eq!(large_value_usize, None);
        let negative_value_f64 = -rug_fuzz_2;
        let negative_value_usize = usize::from_f64(negative_value_f64);
        debug_assert_eq!(negative_value_usize, None);
        let nan_value_f64 = f64::NAN;
        let nan_value_usize = usize::from_f64(nan_value_f64);
        debug_assert_eq!(nan_value_usize, None);
        let infinity_value_f64 = f64::INFINITY;
        let infinity_value_usize = usize::from_f64(infinity_value_f64);
        debug_assert_eq!(infinity_value_usize, None);
        let neg_infinity_value_f64 = f64::NEG_INFINITY;
        let neg_infinity_value_usize = usize::from_f64(neg_infinity_value_f64);
        debug_assert_eq!(neg_infinity_value_usize, None);
        let _rug_ed_tests_llm_16_1899_llm_16_1899_rrrruuuugggg_test_from_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1900_llm_16_1900 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i128_within_bounds() {
        let _rug_st_tests_llm_16_1900_llm_16_1900_rrrruuuugggg_test_from_i128_within_bounds = 0;
        let rug_fuzz_0 = 0_i128;
        debug_assert_eq!(
            < usize as FromPrimitive > ::from_i128(rug_fuzz_0), Some(0_usize)
        );
        debug_assert_eq!(
            < usize as FromPrimitive > ::from_i128(usize::MAX as i128), Some(usize::MAX)
        );
        let _rug_ed_tests_llm_16_1900_llm_16_1900_rrrruuuugggg_test_from_i128_within_bounds = 0;
    }
    #[test]
    fn test_from_i128_below_bounds() {
        let _rug_st_tests_llm_16_1900_llm_16_1900_rrrruuuugggg_test_from_i128_below_bounds = 0;
        let rug_fuzz_0 = 1_i128;
        debug_assert_eq!(< usize as FromPrimitive > ::from_i128(- rug_fuzz_0), None);
        let _rug_ed_tests_llm_16_1900_llm_16_1900_rrrruuuugggg_test_from_i128_below_bounds = 0;
    }
    #[test]
    fn test_from_i128_above_bounds() {
        let _rug_st_tests_llm_16_1900_llm_16_1900_rrrruuuugggg_test_from_i128_above_bounds = 0;
        let rug_fuzz_0 = 1;
        debug_assert_eq!(
            < usize as FromPrimitive > ::from_i128((usize::MAX as i128) + rug_fuzz_0),
            None
        );
        let _rug_ed_tests_llm_16_1900_llm_16_1900_rrrruuuugggg_test_from_i128_above_bounds = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1901_llm_16_1901 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i16() {
        let _rug_st_tests_llm_16_1901_llm_16_1901_rrrruuuugggg_test_from_i16 = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 42;
        let rug_fuzz_2 = 5;
        debug_assert_eq!(
            < usize as FromPrimitive > ::from_i16(rug_fuzz_0), Some(0usize)
        );
        debug_assert_eq!(
            < usize as FromPrimitive > ::from_i16(rug_fuzz_1), Some(42usize)
        );
        debug_assert_eq!(< usize as FromPrimitive > ::from_i16(- rug_fuzz_2), None);
        debug_assert_eq!(
            < usize as FromPrimitive > ::from_i16(i16::MAX), Some(i16::MAX as usize)
        );
        let _rug_ed_tests_llm_16_1901_llm_16_1901_rrrruuuugggg_test_from_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1902_llm_16_1902 {
    use super::*;
    use crate::*;
    #[test]
    fn test_from_i32_with_positive_value() {
        let result = <usize as FromPrimitive>::from_i32(123);
        assert_eq!(result, Some(123_usize));
    }
    #[test]
    fn test_from_i32_with_negative_value() {
        let result = <usize as FromPrimitive>::from_i32(-123);
        assert_eq!(result, None);
    }
    #[test]
    fn test_from_i32_with_zero() {
        let result = <usize as FromPrimitive>::from_i32(0);
        assert_eq!(result, Some(0_usize));
    }
    #[test]
    fn test_from_i32_with_max_value() {
        let result = <usize as FromPrimitive>::from_i32(i32::MAX);
        let expected = if cfg!(target_pointer_width = "32")
            || cfg!(target_pointer_width = "64")
        {
            Some(i32::MAX as usize)
        } else {
            None
        };
        assert_eq!(result, expected);
    }
    #[test]
    fn test_from_i32_with_min_value() {
        let result = <usize as FromPrimitive>::from_i32(i32::MIN);
        assert_eq!(result, None);
    }
}
#[cfg(test)]
mod tests_llm_16_1903_llm_16_1903 {
    use super::*;
    use crate::*;
    use crate::cast::FromPrimitive;
    use std::mem;
    #[test]
    fn test_from_i64() {
        let _rug_st_tests_llm_16_1903_llm_16_1903_rrrruuuugggg_test_from_i64 = 0;
        let rug_fuzz_0 = 0_i64;
        let rug_fuzz_1 = 42_i64;
        let rug_fuzz_2 = 1_i64;
        let rug_fuzz_3 = 8;
        debug_assert_eq!(
            < usize as FromPrimitive > ::from_i64(rug_fuzz_0), Some(0_usize)
        );
        debug_assert_eq!(
            < usize as FromPrimitive > ::from_i64(rug_fuzz_1), Some(42_usize)
        );
        debug_assert_eq!(< usize as FromPrimitive > ::from_i64(- rug_fuzz_2), None);
        debug_assert_eq!(
            < usize as FromPrimitive > ::from_i64(i64::MAX), Some(i64::MAX as usize)
        );
        if mem::size_of::<usize>() < rug_fuzz_3 {
            debug_assert_eq!(< usize as FromPrimitive > ::from_i64(i64::MIN), None);
        }
        let _rug_ed_tests_llm_16_1903_llm_16_1903_rrrruuuugggg_test_from_i64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1904 {
    use crate::FromPrimitive;
    #[test]
    fn from_i8_test() {
        assert_eq!(< usize as FromPrimitive >::from_i8(0), Some(0));
        assert_eq!(< usize as FromPrimitive >::from_i8(127), Some(127));
        assert_eq!(< usize as FromPrimitive >::from_i8(- 1), None);
        assert_eq!(< usize as FromPrimitive >::from_i8(- 128), None);
    }
}
#[cfg(test)]
mod tests_llm_16_1905_llm_16_1905 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_isize_within_bounds() {
        let _rug_st_tests_llm_16_1905_llm_16_1905_rrrruuuugggg_test_from_isize_within_bounds = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 10;
        debug_assert_eq!(< usize as FromPrimitive > ::from_isize(rug_fuzz_0), Some(0));
        debug_assert_eq!(< usize as FromPrimitive > ::from_isize(rug_fuzz_1), Some(10));
        debug_assert_eq!(
            < usize as FromPrimitive > ::from_isize(isize::MAX), Some(isize::MAX as
            usize)
        );
        let _rug_ed_tests_llm_16_1905_llm_16_1905_rrrruuuugggg_test_from_isize_within_bounds = 0;
    }
    #[test]
    fn test_from_isize_out_of_bounds_negative() {
        let _rug_st_tests_llm_16_1905_llm_16_1905_rrrruuuugggg_test_from_isize_out_of_bounds_negative = 0;
        let rug_fuzz_0 = 1;
        debug_assert_eq!(< usize as FromPrimitive > ::from_isize(- rug_fuzz_0), None);
        let _rug_ed_tests_llm_16_1905_llm_16_1905_rrrruuuugggg_test_from_isize_out_of_bounds_negative = 0;
    }
    #[test]
    fn test_from_isize_out_of_bounds_overflow() {
        let _rug_st_tests_llm_16_1905_llm_16_1905_rrrruuuugggg_test_from_isize_out_of_bounds_overflow = 0;
        let rug_fuzz_0 = 1;
        #[cfg(any(target_pointer_width = "32", target_pointer_width = "16"))]
        {
            debug_assert_eq!(
                < usize as FromPrimitive > ::from_isize(isize::MAX as i64 + rug_fuzz_0),
                None
            );
        }
        let _rug_ed_tests_llm_16_1905_llm_16_1905_rrrruuuugggg_test_from_isize_out_of_bounds_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1906_llm_16_1906 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u128() {
        let _rug_st_tests_llm_16_1906_llm_16_1906_rrrruuuugggg_test_from_u128 = 0;
        let rug_fuzz_0 = 1;
        let value_within_range: u128 = usize::MAX as u128;
        debug_assert_eq!(
            < usize as FromPrimitive > ::from_u128(value_within_range), Some(usize::MAX)
        );
        let value_out_of_range: u128 = (usize::MAX as u128).wrapping_add(rug_fuzz_0);
        debug_assert_eq!(
            < usize as FromPrimitive > ::from_u128(value_out_of_range), None
        );
        let _rug_ed_tests_llm_16_1906_llm_16_1906_rrrruuuugggg_test_from_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1907_llm_16_1907 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u16() {
        let _rug_st_tests_llm_16_1907_llm_16_1907_rrrruuuugggg_test_from_u16 = 0;
        let rug_fuzz_0 = 0_u16;
        debug_assert_eq!(
            < usize as FromPrimitive > ::from_u16(rug_fuzz_0), Some(0_usize)
        );
        debug_assert_eq!(
            < usize as FromPrimitive > ::from_u16(u16::MAX), Some(u16::MAX as usize)
        );
        let _rug_ed_tests_llm_16_1907_llm_16_1907_rrrruuuugggg_test_from_u16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1908_llm_16_1908 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u32() {
        let _rug_st_tests_llm_16_1908_llm_16_1908_rrrruuuugggg_test_from_u32 = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 1;
        let num: u32 = rug_fuzz_0;
        let converted: Option<usize> = <usize as FromPrimitive>::from_u32(num);
        debug_assert_eq!(converted, Some(42));
        let max_u32 = u32::MAX;
        let converted_max: Option<usize> = <usize as FromPrimitive>::from_u32(max_u32);
        debug_assert_eq!(converted_max, Some(max_u32 as usize));
        #[cfg(target_pointer_width = "32")]
        {
            let big_num: u64 = (u32::MAX as u64) + rug_fuzz_1;
            let converted: Option<usize> = <usize as FromPrimitive>::from_u32(
                big_num as u32,
            );
            debug_assert_eq!(converted, None);
        }
        let _rug_ed_tests_llm_16_1908_llm_16_1908_rrrruuuugggg_test_from_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1909 {
    use crate::FromPrimitive;
    #[test]
    fn test_from_u64() {
        let _rug_st_tests_llm_16_1909_rrrruuuugggg_test_from_u64 = 0;
        let rug_fuzz_0 = 42u64;
        debug_assert_eq!(
            < usize as FromPrimitive > ::from_u64(rug_fuzz_0), Some(42usize)
        );
        #[cfg(target_pointer_width = "32")]
        debug_assert_eq!(< usize as FromPrimitive > ::from_u64(u64::MAX), None);
        #[cfg(target_pointer_width = "64")]
        debug_assert_eq!(
            < usize as FromPrimitive > ::from_u64(u64::MAX), Some(u64::MAX as usize)
        );
        let _rug_ed_tests_llm_16_1909_rrrruuuugggg_test_from_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1910_llm_16_1910 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u8() {
        let _rug_st_tests_llm_16_1910_llm_16_1910_rrrruuuugggg_test_from_u8 = 0;
        let rug_fuzz_0 = 0u8;
        let rug_fuzz_1 = 255u8;
        debug_assert_eq!(< usize as FromPrimitive > ::from_u8(rug_fuzz_0), Some(0usize));
        debug_assert_eq!(
            < usize as FromPrimitive > ::from_u8(rug_fuzz_1), Some(255usize)
        );
        debug_assert_eq!(
            < usize as FromPrimitive > ::from_u8(u8::MAX), Some(usize::from(u8::MAX))
        );
        debug_assert_eq!(
            < usize as FromPrimitive > ::from_u8(u8::MIN), Some(usize::from(u8::MIN))
        );
        debug_assert_eq!(
            < usize as FromPrimitive > ::from_u8(u8::MAX), Some(usize::from(u8::MAX))
        );
        let _rug_ed_tests_llm_16_1910_llm_16_1910_rrrruuuugggg_test_from_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1911_llm_16_1911 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_usize() {
        let _rug_st_tests_llm_16_1911_llm_16_1911_rrrruuuugggg_test_from_usize = 0;
        let rug_fuzz_0 = 0_usize;
        let rug_fuzz_1 = 0_usize;
        let rug_fuzz_2 = 0_usize;
        let rug_fuzz_3 = 0_usize;
        let rug_fuzz_4 = 0_usize;
        let rug_fuzz_5 = 0_usize;
        let rug_fuzz_6 = 0_usize;
        debug_assert_eq!(< i32 as FromPrimitive > ::from_usize(rug_fuzz_0), Some(0_i32));
        debug_assert_eq!(< i32 as FromPrimitive > ::from_usize(usize::MAX), None);
        debug_assert_eq!(< u32 as FromPrimitive > ::from_usize(rug_fuzz_1), Some(0_u32));
        debug_assert_eq!(
            < u32 as FromPrimitive > ::from_usize(usize::MAX), Some(usize::MAX as u32)
        );
        debug_assert_eq!(< u64 as FromPrimitive > ::from_usize(rug_fuzz_2), Some(0_u64));
        debug_assert_eq!(
            < u64 as FromPrimitive > ::from_usize(usize::MAX), Some(usize::MAX as u64)
        );
        debug_assert_eq!(< u8 as FromPrimitive > ::from_usize(rug_fuzz_3), Some(0_u8));
        debug_assert_eq!(< u8 as FromPrimitive > ::from_usize(usize::MAX), None);
        debug_assert_eq!(< u16 as FromPrimitive > ::from_usize(rug_fuzz_4), Some(0_u16));
        debug_assert_eq!(< u16 as FromPrimitive > ::from_usize(usize::MAX), None);
        debug_assert_eq!(< f32 as FromPrimitive > ::from_usize(rug_fuzz_5), Some(0_f32));
        debug_assert_eq!(
            < f32 as FromPrimitive > ::from_usize(usize::MAX), Some(usize::MAX as f32)
        );
        debug_assert_eq!(< f64 as FromPrimitive > ::from_usize(rug_fuzz_6), Some(0_f64));
        debug_assert_eq!(
            < f64 as FromPrimitive > ::from_usize(usize::MAX), Some(usize::MAX as f64)
        );
        let _rug_ed_tests_llm_16_1911_llm_16_1911_rrrruuuugggg_test_from_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1912_llm_16_1912 {
    use crate::cast::NumCast;
    use std::num::Wrapping;
    #[test]
    fn test_numcast_from_to_primitive() {
        let _rug_st_tests_llm_16_1912_llm_16_1912_rrrruuuugggg_test_numcast_from_to_primitive = 0;
        let rug_fuzz_0 = 42;
        let value: isize = rug_fuzz_0;
        let wrapped_value = Wrapping(value);
        let numcast_value: Option<Wrapping<isize>> = NumCast::from(value);
        debug_assert_eq!(numcast_value, Some(wrapped_value));
        let value: i8 = i8::MAX;
        let wrapped_value = Wrapping(value);
        let numcast_value_max: Option<Wrapping<i8>> = NumCast::from(value);
        debug_assert_eq!(numcast_value_max, Some(wrapped_value));
        let value_i8: i8 = i8::MIN;
        let value_usize: Option<usize> = NumCast::from(value_i8);
        debug_assert_eq!(value_usize, None);
        let value_u64: u64 = u64::MAX;
        let wrapped_value = Wrapping(value_u64);
        let numcast_value_max_u64: Option<Wrapping<u64>> = NumCast::from(value_u64);
        debug_assert_eq!(numcast_value_max_u64, Some(wrapped_value));
        let value_usize: usize = usize::MAX;
        let wrapped_value = Wrapping(value_usize);
        let numcast_value_max_usize: Option<Wrapping<usize>> = NumCast::from(
            value_usize,
        );
        debug_assert_eq!(numcast_value_max_usize, Some(wrapped_value));
        let value_f64: f64 = f64::MAX;
        let wrapped_value = Wrapping(value_f64);
        let numcast_value_f64: Option<Wrapping<f64>> = NumCast::from(value_f64);
        debug_assert_eq!(numcast_value_f64, Some(wrapped_value));
        let _rug_ed_tests_llm_16_1912_llm_16_1912_rrrruuuugggg_test_numcast_from_to_primitive = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1913_llm_16_1913 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_usize_to_f32() {
        let _rug_st_tests_llm_16_1913_llm_16_1913_rrrruuuugggg_test_usize_to_f32 = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 42.0_f32;
        let value: usize = rug_fuzz_0;
        let expected = Some(rug_fuzz_1);
        let result = ToPrimitive::to_f32(&value);
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_1913_llm_16_1913_rrrruuuugggg_test_usize_to_f32 = 0;
    }
    #[test]
    fn test_usize_to_f32_large_number() {
        let _rug_st_tests_llm_16_1913_llm_16_1913_rrrruuuugggg_test_usize_to_f32_large_number = 0;
        let value: usize = usize::MAX;
        let expected = Some(usize::MAX as f32);
        let result = ToPrimitive::to_f32(&value);
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_1913_llm_16_1913_rrrruuuugggg_test_usize_to_f32_large_number = 0;
    }
    #[test]
    fn test_usize_to_f32_zero() {
        let _rug_st_tests_llm_16_1913_llm_16_1913_rrrruuuugggg_test_usize_to_f32_zero = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 0.0_f32;
        let value: usize = rug_fuzz_0;
        let expected = Some(rug_fuzz_1);
        let result = ToPrimitive::to_f32(&value);
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_1913_llm_16_1913_rrrruuuugggg_test_usize_to_f32_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1914_llm_16_1914 {
    use crate::cast::ToPrimitive;
    #[test]
    fn usize_to_f64_conversion() {
        let _rug_st_tests_llm_16_1914_llm_16_1914_rrrruuuugggg_usize_to_f64_conversion = 0;
        let rug_fuzz_0 = 42;
        let value: usize = rug_fuzz_0;
        let float_value: Option<f64> = value.to_f64();
        debug_assert_eq!(float_value, Some(42.0_f64));
        let _rug_ed_tests_llm_16_1914_llm_16_1914_rrrruuuugggg_usize_to_f64_conversion = 0;
    }
    #[test]
    fn usize_to_f64_conversion_max_value() {
        let _rug_st_tests_llm_16_1914_llm_16_1914_rrrruuuugggg_usize_to_f64_conversion_max_value = 0;
        let value: usize = usize::MAX;
        let float_value: Option<f64> = value.to_f64();
        debug_assert!(float_value.is_some());
        let _rug_ed_tests_llm_16_1914_llm_16_1914_rrrruuuugggg_usize_to_f64_conversion_max_value = 0;
    }
    #[test]
    fn usize_to_f64_conversion_zero() {
        let _rug_st_tests_llm_16_1914_llm_16_1914_rrrruuuugggg_usize_to_f64_conversion_zero = 0;
        let rug_fuzz_0 = 0;
        let value: usize = rug_fuzz_0;
        let float_value: Option<f64> = value.to_f64();
        debug_assert_eq!(float_value, Some(0.0_f64));
        let _rug_ed_tests_llm_16_1914_llm_16_1914_rrrruuuugggg_usize_to_f64_conversion_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1915_llm_16_1915 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_i128_max_value() {
        let _rug_st_tests_llm_16_1915_llm_16_1915_rrrruuuugggg_test_to_i128_max_value = 0;
        debug_assert_eq!(usize::MAX.to_i128(), Some(i128::MAX));
        let _rug_ed_tests_llm_16_1915_llm_16_1915_rrrruuuugggg_test_to_i128_max_value = 0;
    }
    #[test]
    fn test_to_i128_zero() {
        let _rug_st_tests_llm_16_1915_llm_16_1915_rrrruuuugggg_test_to_i128_zero = 0;
        let rug_fuzz_0 = 0usize;
        debug_assert_eq!(rug_fuzz_0.to_i128(), Some(0i128));
        let _rug_ed_tests_llm_16_1915_llm_16_1915_rrrruuuugggg_test_to_i128_zero = 0;
    }
    #[test]
    fn test_to_i128_typical_value() {
        let _rug_st_tests_llm_16_1915_llm_16_1915_rrrruuuugggg_test_to_i128_typical_value = 0;
        let rug_fuzz_0 = 42;
        let value: usize = rug_fuzz_0;
        debug_assert_eq!(value.to_i128(), Some(42i128));
        let _rug_ed_tests_llm_16_1915_llm_16_1915_rrrruuuugggg_test_to_i128_typical_value = 0;
    }
    #[test]
    fn test_to_i128_overflow() {
        let _rug_st_tests_llm_16_1915_llm_16_1915_rrrruuuugggg_test_to_i128_overflow = 0;
        let value: usize = usize::MAX;
        let max_i128 = i128::MAX as usize;
        if value > max_i128 {
            debug_assert_eq!(value.to_i128(), None);
        } else {
            debug_assert_eq!(value.to_i128(), Some(value as i128));
        }
        let _rug_ed_tests_llm_16_1915_llm_16_1915_rrrruuuugggg_test_to_i128_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1916_llm_16_1916 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_i16() {
        let _rug_st_tests_llm_16_1916_llm_16_1916_rrrruuuugggg_test_to_i16 = 0;
        let rug_fuzz_0 = 0_usize;
        let rug_fuzz_1 = 1;
        debug_assert_eq!(ToPrimitive::to_i16(& rug_fuzz_0), Some(0i16));
        debug_assert_eq!(ToPrimitive::to_i16(& (i16::MAX as usize)), Some(i16::MAX));
        debug_assert_eq!(ToPrimitive::to_i16(& (i16::MAX as usize + rug_fuzz_1)), None);
        debug_assert_eq!(ToPrimitive::to_i16(& usize::MAX), None);
        let _rug_ed_tests_llm_16_1916_llm_16_1916_rrrruuuugggg_test_to_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1917_llm_16_1917 {
    use crate::ToPrimitive;
    #[test]
    fn usize_to_i32_max_value() {
        let _rug_st_tests_llm_16_1917_llm_16_1917_rrrruuuugggg_usize_to_i32_max_value = 0;
        let max_usize: usize = i32::MAX as usize;
        debug_assert_eq!(max_usize.to_i32(), Some(i32::MAX));
        let _rug_ed_tests_llm_16_1917_llm_16_1917_rrrruuuugggg_usize_to_i32_max_value = 0;
    }
    #[test]
    fn usize_to_i32_within_bounds() {
        let _rug_st_tests_llm_16_1917_llm_16_1917_rrrruuuugggg_usize_to_i32_within_bounds = 0;
        let rug_fuzz_0 = 123;
        let value: usize = rug_fuzz_0;
        debug_assert_eq!(value.to_i32(), Some(123i32));
        let _rug_ed_tests_llm_16_1917_llm_16_1917_rrrruuuugggg_usize_to_i32_within_bounds = 0;
    }
    #[test]
    fn usize_to_i32_overflow() {
        let _rug_st_tests_llm_16_1917_llm_16_1917_rrrruuuugggg_usize_to_i32_overflow = 0;
        let rug_fuzz_0 = 1;
        let value: usize = (i32::MAX as usize) + rug_fuzz_0;
        debug_assert_eq!(value.to_i32(), None);
        let _rug_ed_tests_llm_16_1917_llm_16_1917_rrrruuuugggg_usize_to_i32_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1918_llm_16_1918 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_i64_max_value() {
        let _rug_st_tests_llm_16_1918_llm_16_1918_rrrruuuugggg_test_to_i64_max_value = 0;
        let value: usize = i64::MAX as usize;
        debug_assert_eq!(value.to_i64(), Some(i64::MAX));
        let _rug_ed_tests_llm_16_1918_llm_16_1918_rrrruuuugggg_test_to_i64_max_value = 0;
    }
    #[test]
    fn test_to_i64_within_bounds() {
        let _rug_st_tests_llm_16_1918_llm_16_1918_rrrruuuugggg_test_to_i64_within_bounds = 0;
        let rug_fuzz_0 = 42;
        let value: usize = rug_fuzz_0;
        debug_assert_eq!(value.to_i64(), Some(42));
        let _rug_ed_tests_llm_16_1918_llm_16_1918_rrrruuuugggg_test_to_i64_within_bounds = 0;
    }
    #[test]
    fn test_to_i64_overflow() {
        let _rug_st_tests_llm_16_1918_llm_16_1918_rrrruuuugggg_test_to_i64_overflow = 0;
        let rug_fuzz_0 = 1;
        let value: usize = (i64::MAX as usize).wrapping_add(rug_fuzz_0);
        debug_assert_eq!(value.to_i64(), None);
        let _rug_ed_tests_llm_16_1918_llm_16_1918_rrrruuuugggg_test_to_i64_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1919_llm_16_1919 {
    use crate::cast::ToPrimitive;
    #[test]
    fn usize_to_i8_cast_within_bounds() {
        let _rug_st_tests_llm_16_1919_llm_16_1919_rrrruuuugggg_usize_to_i8_cast_within_bounds = 0;
        let rug_fuzz_0 = 0_usize;
        let rug_fuzz_1 = 127_usize;
        debug_assert_eq!(ToPrimitive::to_i8(& rug_fuzz_0), Some(0_i8));
        debug_assert_eq!(ToPrimitive::to_i8(& rug_fuzz_1), Some(127_i8));
        let _rug_ed_tests_llm_16_1919_llm_16_1919_rrrruuuugggg_usize_to_i8_cast_within_bounds = 0;
    }
    #[test]
    fn usize_to_i8_cast_out_of_bounds() {
        let _rug_st_tests_llm_16_1919_llm_16_1919_rrrruuuugggg_usize_to_i8_cast_out_of_bounds = 0;
        let rug_fuzz_0 = 128_usize;
        debug_assert_eq!(ToPrimitive::to_i8(& rug_fuzz_0), None);
        debug_assert_eq!(ToPrimitive::to_i8(& usize::MAX), None);
        let _rug_ed_tests_llm_16_1919_llm_16_1919_rrrruuuugggg_usize_to_i8_cast_out_of_bounds = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1920_llm_16_1920 {
    use crate::cast::ToPrimitive;
    #[test]
    fn usize_to_isize_conversion_within_bounds() {
        let _rug_st_tests_llm_16_1920_llm_16_1920_rrrruuuugggg_usize_to_isize_conversion_within_bounds = 0;
        let small_usize: usize = isize::MAX as usize;
        debug_assert_eq!(small_usize.to_isize(), Some(isize::MAX));
        let _rug_ed_tests_llm_16_1920_llm_16_1920_rrrruuuugggg_usize_to_isize_conversion_within_bounds = 0;
    }
    #[test]
    fn usize_to_isize_conversion_out_of_bounds() {
        let _rug_st_tests_llm_16_1920_llm_16_1920_rrrruuuugggg_usize_to_isize_conversion_out_of_bounds = 0;
        let rug_fuzz_0 = 1;
        let big_usize: usize = (isize::MAX as usize).wrapping_add(rug_fuzz_0);
        debug_assert_eq!(big_usize.to_isize(), None);
        let _rug_ed_tests_llm_16_1920_llm_16_1920_rrrruuuugggg_usize_to_isize_conversion_out_of_bounds = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1921_llm_16_1921 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_u128_max_value() {
        let _rug_st_tests_llm_16_1921_llm_16_1921_rrrruuuugggg_test_to_u128_max_value = 0;
        let rug_fuzz_0 = 128;
        let max_usize = usize::MAX;
        let result = max_usize.to_u128();
        if usize::BITS as usize <= rug_fuzz_0 {
            debug_assert_eq!(result, Some(max_usize as u128));
        } else {
            debug_assert!(result.is_none());
        }
        let _rug_ed_tests_llm_16_1921_llm_16_1921_rrrruuuugggg_test_to_u128_max_value = 0;
    }
    #[test]
    fn test_to_u128_zero() {
        let _rug_st_tests_llm_16_1921_llm_16_1921_rrrruuuugggg_test_to_u128_zero = 0;
        let rug_fuzz_0 = 0;
        let value: usize = rug_fuzz_0;
        debug_assert_eq!(value.to_u128(), Some(0u128));
        let _rug_ed_tests_llm_16_1921_llm_16_1921_rrrruuuugggg_test_to_u128_zero = 0;
    }
    #[test]
    fn test_to_u128_typical() {
        let _rug_st_tests_llm_16_1921_llm_16_1921_rrrruuuugggg_test_to_u128_typical = 0;
        let rug_fuzz_0 = 42;
        let value: usize = rug_fuzz_0;
        debug_assert_eq!(value.to_u128(), Some(42u128));
        let _rug_ed_tests_llm_16_1921_llm_16_1921_rrrruuuugggg_test_to_u128_typical = 0;
    }
    #[test]
    fn test_to_u128_overflow() {
        let _rug_st_tests_llm_16_1921_llm_16_1921_rrrruuuugggg_test_to_u128_overflow = 0;
        let rug_fuzz_0 = 128;
        let value = usize::MAX;
        let result = value.to_u128();
        if usize::BITS as usize > rug_fuzz_0 {
            debug_assert!(result.is_none());
        } else {
            debug_assert_eq!(result, Some(value as u128));
        }
        let _rug_ed_tests_llm_16_1921_llm_16_1921_rrrruuuugggg_test_to_u128_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1922_llm_16_1922 {
    use super::*;
    use crate::*;
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_u16_within_bounds() {
        let _rug_st_tests_llm_16_1922_llm_16_1922_rrrruuuugggg_test_to_u16_within_bounds = 0;
        let rug_fuzz_0 = 42usize;
        debug_assert_eq!((rug_fuzz_0).to_u16(), Some(42u16));
        debug_assert_eq!((u16::MAX as usize).to_u16(), Some(u16::MAX));
        let _rug_ed_tests_llm_16_1922_llm_16_1922_rrrruuuugggg_test_to_u16_within_bounds = 0;
    }
    #[test]
    fn test_to_u16_out_of_bounds() {
        let _rug_st_tests_llm_16_1922_llm_16_1922_rrrruuuugggg_test_to_u16_out_of_bounds = 0;
        let rug_fuzz_0 = 1;
        debug_assert_eq!((u16::MAX as usize + rug_fuzz_0).to_u16(), None);
        let _rug_ed_tests_llm_16_1922_llm_16_1922_rrrruuuugggg_test_to_u16_out_of_bounds = 0;
    }
    #[test]
    fn test_to_u16_at_zero() {
        let _rug_st_tests_llm_16_1922_llm_16_1922_rrrruuuugggg_test_to_u16_at_zero = 0;
        let rug_fuzz_0 = 0usize;
        debug_assert_eq!((rug_fuzz_0).to_u16(), Some(0u16));
        let _rug_ed_tests_llm_16_1922_llm_16_1922_rrrruuuugggg_test_to_u16_at_zero = 0;
    }
    #[test]
    fn test_to_u16_at_max() {
        let _rug_st_tests_llm_16_1922_llm_16_1922_rrrruuuugggg_test_to_u16_at_max = 0;
        debug_assert_eq!(
            usize::MAX.to_u16(), if usize::MAX > u16::MAX as usize { None } else {
            Some(usize::MAX as u16) }
        );
        let _rug_ed_tests_llm_16_1922_llm_16_1922_rrrruuuugggg_test_to_u16_at_max = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1923_llm_16_1923 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_usize_to_u32_in_range() {
        let _rug_st_tests_llm_16_1923_llm_16_1923_rrrruuuugggg_test_usize_to_u32_in_range = 0;
        let value: usize = u32::MAX as usize;
        let result = ToPrimitive::to_u32(&value);
        debug_assert_eq!(result, Some(u32::MAX));
        let _rug_ed_tests_llm_16_1923_llm_16_1923_rrrruuuugggg_test_usize_to_u32_in_range = 0;
    }
    #[test]
    fn test_usize_to_u32_out_of_range() {
        let _rug_st_tests_llm_16_1923_llm_16_1923_rrrruuuugggg_test_usize_to_u32_out_of_range = 0;
        let rug_fuzz_0 = 1;
        let value: usize = (u32::MAX as usize).wrapping_add(rug_fuzz_0);
        let result = ToPrimitive::to_u32(&value);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1923_llm_16_1923_rrrruuuugggg_test_usize_to_u32_out_of_range = 0;
    }
    #[test]
    fn test_usize_to_u32_zero() {
        let _rug_st_tests_llm_16_1923_llm_16_1923_rrrruuuugggg_test_usize_to_u32_zero = 0;
        let rug_fuzz_0 = 0;
        let value: usize = rug_fuzz_0;
        let result = ToPrimitive::to_u32(&value);
        debug_assert_eq!(result, Some(0));
        let _rug_ed_tests_llm_16_1923_llm_16_1923_rrrruuuugggg_test_usize_to_u32_zero = 0;
    }
    #[test]
    fn test_usize_to_u32_positive() {
        let _rug_st_tests_llm_16_1923_llm_16_1923_rrrruuuugggg_test_usize_to_u32_positive = 0;
        let rug_fuzz_0 = 123;
        let value: usize = rug_fuzz_0;
        let result = ToPrimitive::to_u32(&value);
        debug_assert_eq!(result, Some(123));
        let _rug_ed_tests_llm_16_1923_llm_16_1923_rrrruuuugggg_test_usize_to_u32_positive = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1924_llm_16_1924 {
    use crate::ToPrimitive;
    use std::usize;
    #[test]
    fn test_to_u64_within_bounds() {
        let _rug_st_tests_llm_16_1924_llm_16_1924_rrrruuuugggg_test_to_u64_within_bounds = 0;
        let rug_fuzz_0 = 42;
        let small_usize: usize = rug_fuzz_0;
        let result: Option<u64> = small_usize.to_u64();
        debug_assert_eq!(result, Some(42u64));
        let _rug_ed_tests_llm_16_1924_llm_16_1924_rrrruuuugggg_test_to_u64_within_bounds = 0;
    }
    #[test]
    fn test_to_u64_at_bounds() {
        let _rug_st_tests_llm_16_1924_llm_16_1924_rrrruuuugggg_test_to_u64_at_bounds = 0;
        let max_u64_as_usize: usize = u64::MAX as usize;
        let result: Option<u64> = max_u64_as_usize.to_u64();
        if usize::MAX as u64 >= u64::MAX {
            debug_assert_eq!(result, Some(u64::MAX));
        } else {
            debug_assert_eq!(result, None);
        }
        let _rug_ed_tests_llm_16_1924_llm_16_1924_rrrruuuugggg_test_to_u64_at_bounds = 0;
    }
    #[cfg(target_pointer_width = "64")]
    #[test]
    fn test_to_u64_above_bounds() {
        let _rug_st_tests_llm_16_1924_llm_16_1924_rrrruuuugggg_test_to_u64_above_bounds = 0;
        let above_bounds_usize: usize = usize::MAX;
        debug_assert!(above_bounds_usize > (u64::MAX as usize));
        let result: Option<u64> = above_bounds_usize.to_u64();
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1924_llm_16_1924_rrrruuuugggg_test_to_u64_above_bounds = 0;
    }
    #[cfg(target_pointer_width = "64")]
    #[test]
    fn test_to_u64_max_usize() {
        let _rug_st_tests_llm_16_1924_llm_16_1924_rrrruuuugggg_test_to_u64_max_usize = 0;
        let max_usize: usize = usize::MAX;
        let result: Option<u64> = max_usize.to_u64();
        debug_assert_eq!(result, Some(usize::MAX as u64));
        let _rug_ed_tests_llm_16_1924_llm_16_1924_rrrruuuugggg_test_to_u64_max_usize = 0;
    }
    #[cfg(target_pointer_width = "32")]
    #[test]
    fn test_to_u64_max_usize_on_32bit() {
        let _rug_st_tests_llm_16_1924_llm_16_1924_rrrruuuugggg_test_to_u64_max_usize_on_32bit = 0;
        let max_usize: usize = usize::MAX;
        let result: Option<u64> = max_usize.to_u64();
        debug_assert_eq!(result, Some(usize::MAX as u64));
        let _rug_ed_tests_llm_16_1924_llm_16_1924_rrrruuuugggg_test_to_u64_max_usize_on_32bit = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1925_llm_16_1925 {
    use crate::cast::ToPrimitive;
    #[test]
    fn usize_to_u8_within_range() {
        let _rug_st_tests_llm_16_1925_llm_16_1925_rrrruuuugggg_usize_to_u8_within_range = 0;
        let val: usize = u8::MAX as usize;
        debug_assert_eq!(val.to_u8(), Some(u8::MAX));
        let _rug_ed_tests_llm_16_1925_llm_16_1925_rrrruuuugggg_usize_to_u8_within_range = 0;
    }
    #[test]
    fn usize_to_u8_out_of_range() {
        let _rug_st_tests_llm_16_1925_llm_16_1925_rrrruuuugggg_usize_to_u8_out_of_range = 0;
        let rug_fuzz_0 = 1;
        let val: usize = (u8::MAX as usize) + rug_fuzz_0;
        debug_assert_eq!(val.to_u8(), None);
        let _rug_ed_tests_llm_16_1925_llm_16_1925_rrrruuuugggg_usize_to_u8_out_of_range = 0;
    }
    #[test]
    fn usize_to_u8_zero() {
        let _rug_st_tests_llm_16_1925_llm_16_1925_rrrruuuugggg_usize_to_u8_zero = 0;
        let rug_fuzz_0 = 0;
        let val: usize = rug_fuzz_0;
        debug_assert_eq!(val.to_u8(), Some(0));
        let _rug_ed_tests_llm_16_1925_llm_16_1925_rrrruuuugggg_usize_to_u8_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1987_llm_16_1987 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_f64() {
        let _rug_st_tests_llm_16_1987_llm_16_1987_rrrruuuugggg_test_from_f64 = 0;
        let rug_fuzz_0 = 42.0;
        let rug_fuzz_1 = 42.0;
        let rug_fuzz_2 = 42.0;
        let rug_fuzz_3 = 42.0;
        let rug_fuzz_4 = 42.0;
        let rug_fuzz_5 = 42.0;
        let rug_fuzz_6 = 42.0;
        let rug_fuzz_7 = 42.0;
        let rug_fuzz_8 = 42.0;
        let rug_fuzz_9 = 42.0;
        let rug_fuzz_10 = 42.0;
        let rug_fuzz_11 = 42.0;
        let rug_fuzz_12 = 42.0;
        let rug_fuzz_13 = 42.0;
        let rug_fuzz_14 = 42.0;
        let rug_fuzz_15 = 42.0;
        let rug_fuzz_16 = 42.0;
        let rug_fuzz_17 = 42.0;
        let rug_fuzz_18 = 42.0;
        let rug_fuzz_19 = 42.0;
        debug_assert_eq!(< i8 as FromPrimitive > ::from_f64(rug_fuzz_0), Some(42i8));
        debug_assert_eq!(< i16 as FromPrimitive > ::from_f64(rug_fuzz_1), Some(42i16));
        debug_assert_eq!(< i32 as FromPrimitive > ::from_f64(rug_fuzz_2), Some(42i32));
        debug_assert_eq!(< i64 as FromPrimitive > ::from_f64(rug_fuzz_3), Some(42i64));
        debug_assert_eq!(< u8 as FromPrimitive > ::from_f64(rug_fuzz_4), Some(42u8));
        debug_assert_eq!(< u16 as FromPrimitive > ::from_f64(rug_fuzz_5), Some(42u16));
        debug_assert_eq!(< u32 as FromPrimitive > ::from_f64(rug_fuzz_6), Some(42u32));
        debug_assert_eq!(< u64 as FromPrimitive > ::from_f64(rug_fuzz_7), Some(42u64));
        debug_assert_eq!(< f32 as FromPrimitive > ::from_f64(rug_fuzz_8), Some(42.0f32));
        debug_assert_eq!(< f64 as FromPrimitive > ::from_f64(rug_fuzz_9), Some(42.0f64));
        debug_assert_eq!(< f32 as FromPrimitive > ::from_f64(f64::NAN), None);
        debug_assert_eq!(< f32 as FromPrimitive > ::from_f64(f64::INFINITY), None);
        debug_assert_eq!(< f32 as FromPrimitive > ::from_f64(f64::NEG_INFINITY), None);
        debug_assert_eq!(
            < i8 as FromPrimitive > ::from_f64(- rug_fuzz_10), Some(- 42i8)
        );
        debug_assert_eq!(
            < i16 as FromPrimitive > ::from_f64(- rug_fuzz_11), Some(- 42i16)
        );
        debug_assert_eq!(
            < i32 as FromPrimitive > ::from_f64(- rug_fuzz_12), Some(- 42i32)
        );
        debug_assert_eq!(
            < i64 as FromPrimitive > ::from_f64(- rug_fuzz_13), Some(- 42i64)
        );
        debug_assert_eq!(< u8 as FromPrimitive > ::from_f64(- rug_fuzz_14), None);
        debug_assert_eq!(< u16 as FromPrimitive > ::from_f64(- rug_fuzz_15), None);
        debug_assert_eq!(< u32 as FromPrimitive > ::from_f64(- rug_fuzz_16), None);
        debug_assert_eq!(< u64 as FromPrimitive > ::from_f64(- rug_fuzz_17), None);
        debug_assert_eq!(
            < f32 as FromPrimitive > ::from_f64(- rug_fuzz_18), Some(- 42.0f32)
        );
        debug_assert_eq!(
            < f64 as FromPrimitive > ::from_f64(- rug_fuzz_19), Some(- 42.0f64)
        );
        debug_assert_eq!(< i8 as FromPrimitive > ::from_f64(f64::MAX), None);
        debug_assert_eq!(< i16 as FromPrimitive > ::from_f64(f64::MAX), None);
        debug_assert_eq!(< i32 as FromPrimitive > ::from_f64(f64::MAX), None);
        debug_assert_eq!(< i64 as FromPrimitive > ::from_f64(f64::MAX), None);
        debug_assert_eq!(< u64 as FromPrimitive > ::from_f64(f64::MAX), Some(u64::MAX));
        debug_assert_eq!(< i8 as FromPrimitive > ::from_f64(f64::MIN), None);
        debug_assert_eq!(< i16 as FromPrimitive > ::from_f64(f64::MIN), None);
        debug_assert_eq!(< i32 as FromPrimitive > ::from_f64(f64::MIN), None);
        debug_assert_eq!(< i64 as FromPrimitive > ::from_f64(f64::MIN), Some(i64::MIN));
        debug_assert_eq!(< u64 as FromPrimitive > ::from_f64(f64::MIN), None);
        let _rug_ed_tests_llm_16_1987_llm_16_1987_rrrruuuugggg_test_from_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1988_llm_16_1988 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i128_within_bounds() {
        let _rug_st_tests_llm_16_1988_llm_16_1988_rrrruuuugggg_test_from_i128_within_bounds = 0;
        let rug_fuzz_0 = 123;
        let val_i128: i128 = rug_fuzz_0;
        let result = <i32 as FromPrimitive>::from_i128(val_i128);
        debug_assert_eq!(result, Some(123));
        let _rug_ed_tests_llm_16_1988_llm_16_1988_rrrruuuugggg_test_from_i128_within_bounds = 0;
    }
    #[test]
    fn test_from_i128_out_of_bounds_positive() {
        let _rug_st_tests_llm_16_1988_llm_16_1988_rrrruuuugggg_test_from_i128_out_of_bounds_positive = 0;
        let rug_fuzz_0 = 1;
        let val_i128: i128 = i64::MAX as i128 + rug_fuzz_0;
        let result = <i32 as FromPrimitive>::from_i128(val_i128);
        debug_assert!(result.is_none());
        let _rug_ed_tests_llm_16_1988_llm_16_1988_rrrruuuugggg_test_from_i128_out_of_bounds_positive = 0;
    }
    #[test]
    fn test_from_i128_out_of_bounds_negative() {
        let _rug_st_tests_llm_16_1988_llm_16_1988_rrrruuuugggg_test_from_i128_out_of_bounds_negative = 0;
        let rug_fuzz_0 = 1;
        let val_i128: i128 = i64::MIN as i128 - rug_fuzz_0;
        let result = <i32 as FromPrimitive>::from_i128(val_i128);
        debug_assert!(result.is_none());
        let _rug_ed_tests_llm_16_1988_llm_16_1988_rrrruuuugggg_test_from_i128_out_of_bounds_negative = 0;
    }
    #[test]
    fn test_from_i128_exact_bounds_positive() {
        let _rug_st_tests_llm_16_1988_llm_16_1988_rrrruuuugggg_test_from_i128_exact_bounds_positive = 0;
        let val_i128: i128 = i32::MAX as i128;
        let result = <i32 as FromPrimitive>::from_i128(val_i128);
        debug_assert_eq!(result, Some(i32::MAX));
        let _rug_ed_tests_llm_16_1988_llm_16_1988_rrrruuuugggg_test_from_i128_exact_bounds_positive = 0;
    }
    #[test]
    fn test_from_i128_exact_bounds_negative() {
        let _rug_st_tests_llm_16_1988_llm_16_1988_rrrruuuugggg_test_from_i128_exact_bounds_negative = 0;
        let val_i128: i128 = i32::MIN as i128;
        let result = <i32 as FromPrimitive>::from_i128(val_i128);
        debug_assert_eq!(result, Some(i32::MIN));
        let _rug_ed_tests_llm_16_1988_llm_16_1988_rrrruuuugggg_test_from_i128_exact_bounds_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1990_llm_16_1990 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i32_with_i32() {
        let _rug_st_tests_llm_16_1990_llm_16_1990_rrrruuuugggg_test_from_i32_with_i32 = 0;
        let rug_fuzz_0 = 123;
        let value: i32 = rug_fuzz_0;
        let result = <i32 as FromPrimitive>::from_i32(value);
        debug_assert_eq!(result, Some(123));
        let _rug_ed_tests_llm_16_1990_llm_16_1990_rrrruuuugggg_test_from_i32_with_i32 = 0;
    }
    #[test]
    fn test_from_i32_with_i64() {
        let _rug_st_tests_llm_16_1990_llm_16_1990_rrrruuuugggg_test_from_i32_with_i64 = 0;
        let rug_fuzz_0 = 123;
        let value: i32 = rug_fuzz_0;
        let result = <i64 as FromPrimitive>::from_i32(value);
        debug_assert_eq!(result, Some(123_i64));
        let _rug_ed_tests_llm_16_1990_llm_16_1990_rrrruuuugggg_test_from_i32_with_i64 = 0;
    }
    #[test]
    fn test_from_i32_with_u32() {
        let _rug_st_tests_llm_16_1990_llm_16_1990_rrrruuuugggg_test_from_i32_with_u32 = 0;
        let rug_fuzz_0 = 123;
        let value: i32 = rug_fuzz_0;
        let result = <u32 as FromPrimitive>::from_i32(value);
        debug_assert_eq!(result, Some(123_u32));
        let _rug_ed_tests_llm_16_1990_llm_16_1990_rrrruuuugggg_test_from_i32_with_u32 = 0;
    }
    #[test]
    fn test_from_i32_with_negative_to_unsigned() {
        let _rug_st_tests_llm_16_1990_llm_16_1990_rrrruuuugggg_test_from_i32_with_negative_to_unsigned = 0;
        let rug_fuzz_0 = 123;
        let value: i32 = -rug_fuzz_0;
        let result = <u32 as FromPrimitive>::from_i32(value);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1990_llm_16_1990_rrrruuuugggg_test_from_i32_with_negative_to_unsigned = 0;
    }
    #[test]
    fn test_from_i32_with_out_of_range_to_smaller_int() {
        let _rug_st_tests_llm_16_1990_llm_16_1990_rrrruuuugggg_test_from_i32_with_out_of_range_to_smaller_int = 0;
        let rug_fuzz_0 = 123_456_789;
        let value: i32 = rug_fuzz_0;
        let result = <i16 as FromPrimitive>::from_i32(value);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1990_llm_16_1990_rrrruuuugggg_test_from_i32_with_out_of_range_to_smaller_int = 0;
    }
    #[test]
    fn test_from_i32_with_f32() {
        let _rug_st_tests_llm_16_1990_llm_16_1990_rrrruuuugggg_test_from_i32_with_f32 = 0;
        let rug_fuzz_0 = 123;
        let rug_fuzz_1 = 123.0_f32;
        let value: i32 = rug_fuzz_0;
        let result = <f32 as FromPrimitive>::from_i32(value);
        debug_assert!(result.unwrap().eq(& rug_fuzz_1));
        let _rug_ed_tests_llm_16_1990_llm_16_1990_rrrruuuugggg_test_from_i32_with_f32 = 0;
    }
    #[test]
    fn test_from_i32_edge_cases() {
        let _rug_st_tests_llm_16_1990_llm_16_1990_rrrruuuugggg_test_from_i32_edge_cases = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 1;
        let smallest_i32: i32 = i32::MIN;
        let result = <i64 as FromPrimitive>::from_i32(smallest_i32);
        debug_assert_eq!(result, Some(smallest_i32 as i64));
        let largest_i32: i32 = i32::MAX;
        let result = <i64 as FromPrimitive>::from_i32(largest_i32);
        debug_assert_eq!(result, Some(largest_i32 as i64));
        let smallest_i64 = i64::from(i32::MIN) - rug_fuzz_0;
        let result = <i64 as FromPrimitive>::from_i32(smallest_i64 as i32);
        debug_assert_eq!(result, None);
        let largest_i64 = i64::from(i32::MAX) + rug_fuzz_1;
        let result = <i64 as FromPrimitive>::from_i32(largest_i64 as i32);
        debug_assert_eq!(result, None);
        let _rug_ed_tests_llm_16_1990_llm_16_1990_rrrruuuugggg_test_from_i32_edge_cases = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1991_llm_16_1991 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_i8() {
        let _rug_st_tests_llm_16_1991_llm_16_1991_rrrruuuugggg_test_from_i8 = 0;
        let rug_fuzz_0 = 42i8;
        let rug_fuzz_1 = 42i8;
        let rug_fuzz_2 = 42i8;
        let rug_fuzz_3 = 42i8;
        let rug_fuzz_4 = 42i8;
        let rug_fuzz_5 = 1i8;
        let rug_fuzz_6 = 1i8;
        let rug_fuzz_7 = 42i8;
        let rug_fuzz_8 = 42i8;
        debug_assert_eq!(FromPrimitive::from_i8(rug_fuzz_0), Some(42i8));
        debug_assert_eq!(FromPrimitive::from_i8(- rug_fuzz_1), Some(- 42i8));
        debug_assert_eq!(FromPrimitive::from_i8(i8::MAX), Some(i8::MAX));
        debug_assert_eq!(FromPrimitive::from_i8(i8::MIN), Some(i8::MIN));
        debug_assert_eq!(FromPrimitive::from_i8(rug_fuzz_2), Some(42u32));
        debug_assert_eq!(FromPrimitive::from_i8(- rug_fuzz_3), Some(- 42i32));
        debug_assert_eq!(FromPrimitive::from_i8(rug_fuzz_4), Some(42u8));
        debug_assert_eq!(FromPrimitive::from_i8(- rug_fuzz_5), None:: < u8 >);
        debug_assert_eq!(FromPrimitive::from_i8(- rug_fuzz_6), None:: < u64 >);
        #[derive(Debug, PartialEq)]
        struct MyType(i32);
        impl FromPrimitive for MyType {
            fn from_i8(n: i8) -> Option<Self> {
                Some(MyType(i32::from(n)))
            }
            fn from_u64(n: u64) -> Option<Self> {
                if n > i32::MAX as u64 { None } else { Some(MyType(n as i32)) }
            }
            fn from_i64(n: i64) -> Option<Self> {
                if n > i32::MAX as i64 || n < i32::MIN as i64 {
                    None
                } else {
                    Some(MyType(n as i32))
                }
            }
        }
        debug_assert_eq!(FromPrimitive::from_i8(rug_fuzz_7), Some(MyType(42)));
        debug_assert_eq!(FromPrimitive::from_i8(- rug_fuzz_8), Some(MyType(- 42)));
        let _rug_ed_tests_llm_16_1991_llm_16_1991_rrrruuuugggg_test_from_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1992_llm_16_1992 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_isize_within_bounds() {
        let _rug_st_tests_llm_16_1992_llm_16_1992_rrrruuuugggg_test_from_isize_within_bounds = 0;
        let rug_fuzz_0 = 42;
        let value: Option<i32> = FromPrimitive::from_isize(rug_fuzz_0);
        debug_assert_eq!(value, Some(42i32));
        let _rug_ed_tests_llm_16_1992_llm_16_1992_rrrruuuugggg_test_from_isize_within_bounds = 0;
    }
    #[test]
    fn test_from_isize_below_bounds() {
        let _rug_st_tests_llm_16_1992_llm_16_1992_rrrruuuugggg_test_from_isize_below_bounds = 0;
        let rug_fuzz_0 = 1;
        let value: Option<i32> = FromPrimitive::from_isize(-rug_fuzz_0);
        debug_assert!(value.is_some());
        let _rug_ed_tests_llm_16_1992_llm_16_1992_rrrruuuugggg_test_from_isize_below_bounds = 0;
    }
    #[test]
    fn test_from_isize_above_bounds() {
        let _rug_st_tests_llm_16_1992_llm_16_1992_rrrruuuugggg_test_from_isize_above_bounds = 0;
        let max_isize = isize::MAX;
        let value: Option<u8> = FromPrimitive::from_isize(max_isize);
        debug_assert!(value.is_none());
        let _rug_ed_tests_llm_16_1992_llm_16_1992_rrrruuuugggg_test_from_isize_above_bounds = 0;
    }
    #[test]
    fn test_from_isize_with_conversion() {
        let _rug_st_tests_llm_16_1992_llm_16_1992_rrrruuuugggg_test_from_isize_with_conversion = 0;
        let rug_fuzz_0 = 42;
        let value: Option<u32> = FromPrimitive::from_isize(rug_fuzz_0);
        debug_assert_eq!(value, Some(42u32));
        let _rug_ed_tests_llm_16_1992_llm_16_1992_rrrruuuugggg_test_from_isize_with_conversion = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1993 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u128() {
        let _rug_st_tests_llm_16_1993_rrrruuuugggg_test_from_u128 = 0;
        let rug_fuzz_0 = 0_u128;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 0_u128;
        let rug_fuzz_3 = 1;
        debug_assert_eq!(< u64 as FromPrimitive > ::from_u128(rug_fuzz_0), Some(0));
        debug_assert_eq!(
            < u64 as FromPrimitive > ::from_u128(u64::MAX as u128), Some(u64::MAX)
        );
        debug_assert_eq!(
            < u64 as FromPrimitive > ::from_u128((u64::MAX as u128) + rug_fuzz_1), None
        );
        debug_assert_eq!(< i32 as FromPrimitive > ::from_u128(rug_fuzz_2), Some(0));
        debug_assert_eq!(
            < i32 as FromPrimitive > ::from_u128(i32::MAX as u128), Some(i32::MAX)
        );
        debug_assert_eq!(
            < i32 as FromPrimitive > ::from_u128((i32::MAX as u128) + rug_fuzz_3), None
        );
        let _rug_ed_tests_llm_16_1993_rrrruuuugggg_test_from_u128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1994_llm_16_1994 {
    use crate::FromPrimitive;
    #[test]
    fn from_u16_to_types_within_bounds() {
        let _rug_st_tests_llm_16_1994_llm_16_1994_rrrruuuugggg_from_u16_to_types_within_bounds = 0;
        let rug_fuzz_0 = 255u16;
        let rug_fuzz_1 = 65535u16;
        let rug_fuzz_2 = 123u16;
        let rug_fuzz_3 = 999u16;
        debug_assert_eq!(FromPrimitive::from_u16(rug_fuzz_0), Some(255u8));
        debug_assert_eq!(FromPrimitive::from_u16(rug_fuzz_1), Some(65535u32));
        debug_assert_eq!(FromPrimitive::from_u16(rug_fuzz_2), Some(123usize));
        debug_assert_eq!(FromPrimitive::from_u16(rug_fuzz_3), Some(999f32));
        let _rug_ed_tests_llm_16_1994_llm_16_1994_rrrruuuugggg_from_u16_to_types_within_bounds = 0;
    }
    #[test]
    fn from_u16_to_types_outside_bounds() {
        let _rug_st_tests_llm_16_1994_llm_16_1994_rrrruuuugggg_from_u16_to_types_outside_bounds = 0;
        let rug_fuzz_0 = 256u16;
        debug_assert_eq!(FromPrimitive::from_u16(rug_fuzz_0), None:: < u8 >);
        let _rug_ed_tests_llm_16_1994_llm_16_1994_rrrruuuugggg_from_u16_to_types_outside_bounds = 0;
    }
    #[test]
    fn from_u16_edge_cases() {
        let _rug_st_tests_llm_16_1994_llm_16_1994_rrrruuuugggg_from_u16_edge_cases = 0;
        let rug_fuzz_0 = 0u16;
        debug_assert_eq!(FromPrimitive::from_u16(rug_fuzz_0), Some(0u8));
        debug_assert_eq!(FromPrimitive::from_u16(u16::MAX), Some(u16::MAX as usize));
        let _rug_ed_tests_llm_16_1994_llm_16_1994_rrrruuuugggg_from_u16_edge_cases = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1995_llm_16_1995 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u32_with_u32() {
        let _rug_st_tests_llm_16_1995_llm_16_1995_rrrruuuugggg_test_from_u32_with_u32 = 0;
        let value = u32::MAX;
        debug_assert_eq!(Some(value), < u32 as FromPrimitive > ::from_u32(value));
        let _rug_ed_tests_llm_16_1995_llm_16_1995_rrrruuuugggg_test_from_u32_with_u32 = 0;
    }
    #[test]
    fn test_from_u32_with_u64() {
        let _rug_st_tests_llm_16_1995_llm_16_1995_rrrruuuugggg_test_from_u32_with_u64 = 0;
        let value = u32::MAX as u64;
        debug_assert_eq!(Some(value), < u64 as FromPrimitive > ::from_u32(u32::MAX));
        let _rug_ed_tests_llm_16_1995_llm_16_1995_rrrruuuugggg_test_from_u32_with_u64 = 0;
    }
    #[test]
    fn test_from_u32_with_u16() {
        let _rug_st_tests_llm_16_1995_llm_16_1995_rrrruuuugggg_test_from_u32_with_u16 = 0;
        let value = u16::MAX as u32;
        debug_assert_eq!(Some(u16::MAX), < u16 as FromPrimitive > ::from_u32(value));
        debug_assert_eq!(None, < u16 as FromPrimitive > ::from_u32(u32::MAX));
        let _rug_ed_tests_llm_16_1995_llm_16_1995_rrrruuuugggg_test_from_u32_with_u16 = 0;
    }
    #[test]
    fn test_from_u32_with_u8() {
        let _rug_st_tests_llm_16_1995_llm_16_1995_rrrruuuugggg_test_from_u32_with_u8 = 0;
        let value = u8::MAX as u32;
        debug_assert_eq!(Some(u8::MAX), < u8 as FromPrimitive > ::from_u32(value));
        debug_assert_eq!(None, < u8 as FromPrimitive > ::from_u32(u32::MAX));
        let _rug_ed_tests_llm_16_1995_llm_16_1995_rrrruuuugggg_test_from_u32_with_u8 = 0;
    }
    #[test]
    fn test_from_u32_with_i32() {
        let _rug_st_tests_llm_16_1995_llm_16_1995_rrrruuuugggg_test_from_u32_with_i32 = 0;
        let value = i32::MAX as u32;
        debug_assert_eq!(Some(i32::MAX), < i32 as FromPrimitive > ::from_u32(value));
        debug_assert_eq!(None, < i32 as FromPrimitive > ::from_u32(u32::MAX));
        let _rug_ed_tests_llm_16_1995_llm_16_1995_rrrruuuugggg_test_from_u32_with_i32 = 0;
    }
    #[test]
    fn test_from_u32_with_i16() {
        let _rug_st_tests_llm_16_1995_llm_16_1995_rrrruuuugggg_test_from_u32_with_i16 = 0;
        let value = u16::MAX as u32;
        debug_assert_eq!(Some(i16::MAX), < i16 as FromPrimitive > ::from_u32(value));
        debug_assert_eq!(None, < i16 as FromPrimitive > ::from_u32(u32::MAX));
        let _rug_ed_tests_llm_16_1995_llm_16_1995_rrrruuuugggg_test_from_u32_with_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1996_llm_16_1996 {
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_u8() {
        let _rug_st_tests_llm_16_1996_llm_16_1996_rrrruuuugggg_test_from_u8 = 0;
        let rug_fuzz_0 = 0_u8;
        let rug_fuzz_1 = 255_u8;
        let rug_fuzz_2 = 127_u8;
        let rug_fuzz_3 = 255_u8;
        let rug_fuzz_4 = 0_u8;
        let rug_fuzz_5 = 255_u8;
        let rug_fuzz_6 = 0_u8;
        let rug_fuzz_7 = 255_u8;
        debug_assert_eq!(< i32 as FromPrimitive > ::from_u8(rug_fuzz_0), Some(0_i32));
        debug_assert_eq!(< i32 as FromPrimitive > ::from_u8(rug_fuzz_1), Some(255_i32));
        debug_assert_eq!(< i8 as FromPrimitive > ::from_u8(rug_fuzz_2), Some(127_i8));
        debug_assert_eq!(< i8 as FromPrimitive > ::from_u8(rug_fuzz_3), None);
        debug_assert_eq!(< u32 as FromPrimitive > ::from_u8(rug_fuzz_4), Some(0_u32));
        debug_assert_eq!(< u32 as FromPrimitive > ::from_u8(rug_fuzz_5), Some(255_u32));
        debug_assert_eq!(< f32 as FromPrimitive > ::from_u8(rug_fuzz_6), Some(0.0_f32));
        debug_assert_eq!(
            < f32 as FromPrimitive > ::from_u8(rug_fuzz_7), Some(255.0_f32)
        );
        let _rug_ed_tests_llm_16_1996_llm_16_1996_rrrruuuugggg_test_from_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1997_llm_16_1997 {
    use crate::FromPrimitive;
    #[test]
    fn test_from_usize() {
        let _rug_st_tests_llm_16_1997_llm_16_1997_rrrruuuugggg_test_from_usize = 0;
        let rug_fuzz_0 = 0_usize;
        let rug_fuzz_1 = 256_usize;
        let rug_fuzz_2 = 0_usize;
        let rug_fuzz_3 = 0;
        let rug_fuzz_4 = 0;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 0;
        let rug_fuzz_7 = 42_usize;
        debug_assert_eq!(FromPrimitive::from_usize(rug_fuzz_0), Some(0_usize));
        debug_assert_eq!(FromPrimitive::from_usize(usize::MAX), Some(usize::MAX));
        debug_assert_eq!(FromPrimitive::from_usize(rug_fuzz_1), Some(256_u16));
        debug_assert_eq!(FromPrimitive::from_usize(usize::MAX), None:: < u8 >);
        debug_assert_eq!(FromPrimitive::from_usize(usize::MAX), None:: < u16 >);
        debug_assert_eq!(FromPrimitive::from_usize(usize::MAX), None:: < u32 >);
        debug_assert_eq!(FromPrimitive::from_usize(rug_fuzz_2), Some(0_i64));
        debug_assert_eq!(FromPrimitive::from_usize(usize::MAX), None:: < i16 >);
        debug_assert_eq!(FromPrimitive::from_usize(usize::MAX), None:: < i32 >);
        debug_assert_eq!(FromPrimitive::from_usize(usize::MAX), None:: < i64 >);
        debug_assert!(usize::MAX as i64 >= rug_fuzz_3);
        debug_assert!(usize::MAX as i32 >= rug_fuzz_4);
        debug_assert!(usize::MAX as i16 >= rug_fuzz_5);
        debug_assert!(usize::MAX as i8 >= rug_fuzz_6);
        debug_assert_eq!(FromPrimitive::from_usize(rug_fuzz_7), Some(42.0_f64));
        debug_assert_eq!(FromPrimitive::from_usize(usize::MAX), None:: < f32 >);
        debug_assert_eq!(FromPrimitive::from_usize(usize::MAX), None:: < f64 >);
        let _rug_ed_tests_llm_16_1997_llm_16_1997_rrrruuuugggg_test_from_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_1998_llm_16_1998 {
    use crate::ToPrimitive;
    #[test]
    fn test_to_f32_with_i32() {
        let _rug_st_tests_llm_16_1998_llm_16_1998_rrrruuuugggg_test_to_f32_with_i32 = 0;
        let rug_fuzz_0 = 42;
        let val: i32 = rug_fuzz_0;
        debug_assert_eq!(val.to_f32(), Some(42.0f32));
        let _rug_ed_tests_llm_16_1998_llm_16_1998_rrrruuuugggg_test_to_f32_with_i32 = 0;
    }
    #[test]
    fn test_to_f32_with_i64() {
        let _rug_st_tests_llm_16_1998_llm_16_1998_rrrruuuugggg_test_to_f32_with_i64 = 0;
        let rug_fuzz_0 = 42;
        let val: i64 = rug_fuzz_0;
        debug_assert_eq!(val.to_f32(), Some(42.0f32));
        let _rug_ed_tests_llm_16_1998_llm_16_1998_rrrruuuugggg_test_to_f32_with_i64 = 0;
    }
    #[test]
    fn test_to_f32_with_u32() {
        let _rug_st_tests_llm_16_1998_llm_16_1998_rrrruuuugggg_test_to_f32_with_u32 = 0;
        let rug_fuzz_0 = 42;
        let val: u32 = rug_fuzz_0;
        debug_assert_eq!(val.to_f32(), Some(42.0f32));
        let _rug_ed_tests_llm_16_1998_llm_16_1998_rrrruuuugggg_test_to_f32_with_u32 = 0;
    }
    #[test]
    fn test_to_f32_with_u64() {
        let _rug_st_tests_llm_16_1998_llm_16_1998_rrrruuuugggg_test_to_f32_with_u64 = 0;
        let rug_fuzz_0 = 42;
        let val: u64 = rug_fuzz_0;
        debug_assert_eq!(val.to_f32(), Some(42.0f32));
        let _rug_ed_tests_llm_16_1998_llm_16_1998_rrrruuuugggg_test_to_f32_with_u64 = 0;
    }
    #[test]
    fn test_to_f32_with_f64() {
        let _rug_st_tests_llm_16_1998_llm_16_1998_rrrruuuugggg_test_to_f32_with_f64 = 0;
        let rug_fuzz_0 = 42.0;
        let val: f64 = rug_fuzz_0;
        debug_assert_eq!(val.to_f32(), Some(42.0f32));
        let _rug_ed_tests_llm_16_1998_llm_16_1998_rrrruuuugggg_test_to_f32_with_f64 = 0;
    }
    #[test]
    fn test_to_f32_with_large_i64() {
        let _rug_st_tests_llm_16_1998_llm_16_1998_rrrruuuugggg_test_to_f32_with_large_i64 = 0;
        let rug_fuzz_0 = 0.0;
        let val: i64 = i64::MAX;
        let result = val.to_f32();
        debug_assert!(
            result.is_some() && result.unwrap().is_infinite() && result.unwrap() >
            rug_fuzz_0
        );
        let _rug_ed_tests_llm_16_1998_llm_16_1998_rrrruuuugggg_test_to_f32_with_large_i64 = 0;
    }
    #[test]
    fn test_to_f32_with_small_i64() {
        let _rug_st_tests_llm_16_1998_llm_16_1998_rrrruuuugggg_test_to_f32_with_small_i64 = 0;
        let rug_fuzz_0 = 0.0;
        let val: i64 = i64::MIN;
        let result = val.to_f32();
        debug_assert!(
            result.is_some() && result.unwrap().is_infinite() && result.unwrap() <
            rug_fuzz_0
        );
        let _rug_ed_tests_llm_16_1998_llm_16_1998_rrrruuuugggg_test_to_f32_with_small_i64 = 0;
    }
    #[test]
    fn test_to_f32_with_large_u64() {
        let _rug_st_tests_llm_16_1998_llm_16_1998_rrrruuuugggg_test_to_f32_with_large_u64 = 0;
        let rug_fuzz_0 = 0.0;
        let val: u64 = u64::MAX;
        let result = val.to_f32();
        debug_assert!(
            result.is_some() && result.unwrap().is_infinite() && result.unwrap() >
            rug_fuzz_0
        );
        let _rug_ed_tests_llm_16_1998_llm_16_1998_rrrruuuugggg_test_to_f32_with_large_u64 = 0;
    }
    #[test]
    fn test_to_f32_with_large_f64() {
        let _rug_st_tests_llm_16_1998_llm_16_1998_rrrruuuugggg_test_to_f32_with_large_f64 = 0;
        let val: f64 = f64::MAX;
        let result = val.to_f32();
        debug_assert!(result.is_some() && result.unwrap().is_infinite());
        let _rug_ed_tests_llm_16_1998_llm_16_1998_rrrruuuugggg_test_to_f32_with_large_f64 = 0;
    }
    #[test]
    fn test_to_f32_with_small_f64() {
        let _rug_st_tests_llm_16_1998_llm_16_1998_rrrruuuugggg_test_to_f32_with_small_f64 = 0;
        let val: f64 = f64::MIN;
        let result = val.to_f32();
        debug_assert!(result.is_some() && result.unwrap().is_infinite());
        let _rug_ed_tests_llm_16_1998_llm_16_1998_rrrruuuugggg_test_to_f32_with_small_f64 = 0;
    }
    #[test]
    fn test_to_f32_with_nan_f64() {
        let _rug_st_tests_llm_16_1998_llm_16_1998_rrrruuuugggg_test_to_f32_with_nan_f64 = 0;
        let val: f64 = f64::NAN;
        let result = val.to_f32();
        debug_assert!(result.is_some() && result.unwrap().is_nan());
        let _rug_ed_tests_llm_16_1998_llm_16_1998_rrrruuuugggg_test_to_f32_with_nan_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2000 {
    use super::*;
    use crate::*;
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_i128() {
        let _rug_st_tests_llm_16_2000_rrrruuuugggg_test_to_i128 = 0;
        let rug_fuzz_0 = 123_i128;
        let rug_fuzz_1 = 123_i128;
        let rug_fuzz_2 = 123_i128;
        let rug_fuzz_3 = 123_i128;
        let rug_fuzz_4 = 123_i128;
        let rug_fuzz_5 = 123_i128;
        debug_assert_eq!(Some(rug_fuzz_0), 123_i32.to_i128());
        debug_assert_eq!(Some(- rug_fuzz_1), (- 123_i32).to_i128());
        debug_assert_eq!(Some(rug_fuzz_2), 123_i64.to_i128());
        debug_assert_eq!(Some(- rug_fuzz_3), (- 123_i64).to_i128());
        debug_assert_eq!(Some(rug_fuzz_4), 123_u32.to_i128());
        debug_assert_eq!(Some(rug_fuzz_5), 123_u64.to_i128());
        debug_assert_eq!(None, (- 1_i32).to_i128());
        debug_assert_eq!(Some(i128::MAX), i128::MAX.to_i128());
        debug_assert_eq!(Some(i128::MIN), i128::MIN.to_i128());
        let _rug_ed_tests_llm_16_2000_rrrruuuugggg_test_to_i128 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2001_llm_16_2001 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_i16_with_i16() {
        let _rug_st_tests_llm_16_2001_llm_16_2001_rrrruuuugggg_test_to_i16_with_i16 = 0;
        let rug_fuzz_0 = 123;
        let num: i16 = rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_i16(& num), Some(123));
        let _rug_ed_tests_llm_16_2001_llm_16_2001_rrrruuuugggg_test_to_i16_with_i16 = 0;
    }
    #[test]
    fn test_to_i16_with_i32() {
        let _rug_st_tests_llm_16_2001_llm_16_2001_rrrruuuugggg_test_to_i16_with_i32 = 0;
        let rug_fuzz_0 = 123;
        let num: i32 = rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_i16(& num), Some(123));
        let _rug_ed_tests_llm_16_2001_llm_16_2001_rrrruuuugggg_test_to_i16_with_i32 = 0;
    }
    #[test]
    fn test_to_i16_with_i32_overflow() {
        let _rug_st_tests_llm_16_2001_llm_16_2001_rrrruuuugggg_test_to_i16_with_i32_overflow = 0;
        let num: i32 = i32::MAX;
        debug_assert_eq!(ToPrimitive::to_i16(& num), None);
        let _rug_ed_tests_llm_16_2001_llm_16_2001_rrrruuuugggg_test_to_i16_with_i32_overflow = 0;
    }
    #[test]
    fn test_to_i16_with_u32() {
        let _rug_st_tests_llm_16_2001_llm_16_2001_rrrruuuugggg_test_to_i16_with_u32 = 0;
        let rug_fuzz_0 = 123;
        let num: u32 = rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_i16(& num), Some(123));
        let _rug_ed_tests_llm_16_2001_llm_16_2001_rrrruuuugggg_test_to_i16_with_u32 = 0;
    }
    #[test]
    fn test_to_i16_with_u32_overflow() {
        let _rug_st_tests_llm_16_2001_llm_16_2001_rrrruuuugggg_test_to_i16_with_u32_overflow = 0;
        let num: u32 = u32::MAX;
        debug_assert_eq!(ToPrimitive::to_i16(& num), None);
        let _rug_ed_tests_llm_16_2001_llm_16_2001_rrrruuuugggg_test_to_i16_with_u32_overflow = 0;
    }
    #[test]
    fn test_to_i16_with_i64() {
        let _rug_st_tests_llm_16_2001_llm_16_2001_rrrruuuugggg_test_to_i16_with_i64 = 0;
        let rug_fuzz_0 = 123;
        let num: i64 = rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_i16(& num), Some(123));
        let _rug_ed_tests_llm_16_2001_llm_16_2001_rrrruuuugggg_test_to_i16_with_i64 = 0;
    }
    #[test]
    fn test_to_i16_with_i64_overflow() {
        let _rug_st_tests_llm_16_2001_llm_16_2001_rrrruuuugggg_test_to_i16_with_i64_overflow = 0;
        let num: i64 = i64::MAX;
        debug_assert_eq!(ToPrimitive::to_i16(& num), None);
        let _rug_ed_tests_llm_16_2001_llm_16_2001_rrrruuuugggg_test_to_i16_with_i64_overflow = 0;
    }
    #[test]
    fn test_to_i16_with_f32() {
        let _rug_st_tests_llm_16_2001_llm_16_2001_rrrruuuugggg_test_to_i16_with_f32 = 0;
        let rug_fuzz_0 = 123.0;
        let num: f32 = rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_i16(& num), Some(123));
        let _rug_ed_tests_llm_16_2001_llm_16_2001_rrrruuuugggg_test_to_i16_with_f32 = 0;
    }
    #[test]
    fn test_to_i16_with_f32_overflow() {
        let _rug_st_tests_llm_16_2001_llm_16_2001_rrrruuuugggg_test_to_i16_with_f32_overflow = 0;
        let num: f32 = f32::MAX;
        debug_assert_eq!(ToPrimitive::to_i16(& num), None);
        let _rug_ed_tests_llm_16_2001_llm_16_2001_rrrruuuugggg_test_to_i16_with_f32_overflow = 0;
    }
    #[test]
    fn test_to_i16_with_f64() {
        let _rug_st_tests_llm_16_2001_llm_16_2001_rrrruuuugggg_test_to_i16_with_f64 = 0;
        let rug_fuzz_0 = 123.0;
        let num: f64 = rug_fuzz_0;
        debug_assert_eq!(ToPrimitive::to_i16(& num), Some(123));
        let _rug_ed_tests_llm_16_2001_llm_16_2001_rrrruuuugggg_test_to_i16_with_f64 = 0;
    }
    #[test]
    fn test_to_i16_with_f64_overflow() {
        let _rug_st_tests_llm_16_2001_llm_16_2001_rrrruuuugggg_test_to_i16_with_f64_overflow = 0;
        let num: f64 = f64::MAX;
        debug_assert_eq!(ToPrimitive::to_i16(& num), None);
        let _rug_ed_tests_llm_16_2001_llm_16_2001_rrrruuuugggg_test_to_i16_with_f64_overflow = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2002_llm_16_2002 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_i32_with_i32() {
        let _rug_st_tests_llm_16_2002_llm_16_2002_rrrruuuugggg_test_to_i32_with_i32 = 0;
        let rug_fuzz_0 = 5;
        let x: i32 = rug_fuzz_0;
        debug_assert_eq!(x.to_i32(), Some(5));
        let _rug_ed_tests_llm_16_2002_llm_16_2002_rrrruuuugggg_test_to_i32_with_i32 = 0;
    }
    #[test]
    fn test_to_i32_with_u32() {
        let _rug_st_tests_llm_16_2002_llm_16_2002_rrrruuuugggg_test_to_i32_with_u32 = 0;
        let rug_fuzz_0 = 5;
        let x: u32 = rug_fuzz_0;
        debug_assert_eq!(x.to_i32(), Some(5));
        let _rug_ed_tests_llm_16_2002_llm_16_2002_rrrruuuugggg_test_to_i32_with_u32 = 0;
    }
    #[test]
    fn test_to_i32_with_large_u32() {
        let _rug_st_tests_llm_16_2002_llm_16_2002_rrrruuuugggg_test_to_i32_with_large_u32 = 0;
        let x: u32 = u32::MAX;
        debug_assert_eq!(x.to_i32(), None);
        let _rug_ed_tests_llm_16_2002_llm_16_2002_rrrruuuugggg_test_to_i32_with_large_u32 = 0;
    }
    #[test]
    fn test_to_i32_with_i64() {
        let _rug_st_tests_llm_16_2002_llm_16_2002_rrrruuuugggg_test_to_i32_with_i64 = 0;
        let rug_fuzz_0 = 5;
        let x: i64 = rug_fuzz_0;
        debug_assert_eq!(x.to_i32(), Some(5));
        let _rug_ed_tests_llm_16_2002_llm_16_2002_rrrruuuugggg_test_to_i32_with_i64 = 0;
    }
    #[test]
    fn test_to_i32_with_large_i64() {
        let _rug_st_tests_llm_16_2002_llm_16_2002_rrrruuuugggg_test_to_i32_with_large_i64 = 0;
        let x: i64 = i64::MAX;
        debug_assert_eq!(x.to_i32(), None);
        let _rug_ed_tests_llm_16_2002_llm_16_2002_rrrruuuugggg_test_to_i32_with_large_i64 = 0;
    }
    #[test]
    fn test_to_i32_with_small_i64() {
        let _rug_st_tests_llm_16_2002_llm_16_2002_rrrruuuugggg_test_to_i32_with_small_i64 = 0;
        let x: i64 = i64::MIN;
        debug_assert_eq!(x.to_i32(), None);
        let _rug_ed_tests_llm_16_2002_llm_16_2002_rrrruuuugggg_test_to_i32_with_small_i64 = 0;
    }
    #[test]
    fn test_to_i32_with_f32() {
        let _rug_st_tests_llm_16_2002_llm_16_2002_rrrruuuugggg_test_to_i32_with_f32 = 0;
        let rug_fuzz_0 = 5.0;
        let x: f32 = rug_fuzz_0;
        debug_assert_eq!(x.to_i32(), Some(5));
        let _rug_ed_tests_llm_16_2002_llm_16_2002_rrrruuuugggg_test_to_i32_with_f32 = 0;
    }
    #[test]
    fn test_to_i32_with_large_f32() {
        let _rug_st_tests_llm_16_2002_llm_16_2002_rrrruuuugggg_test_to_i32_with_large_f32 = 0;
        let rug_fuzz_0 = 1e10;
        let x: f32 = rug_fuzz_0;
        debug_assert_eq!(x.to_i32(), None);
        let _rug_ed_tests_llm_16_2002_llm_16_2002_rrrruuuugggg_test_to_i32_with_large_f32 = 0;
    }
    #[test]
    fn test_to_i32_with_negative_f32() {
        let _rug_st_tests_llm_16_2002_llm_16_2002_rrrruuuugggg_test_to_i32_with_negative_f32 = 0;
        let rug_fuzz_0 = 5.0;
        let x: f32 = -rug_fuzz_0;
        debug_assert_eq!(x.to_i32(), Some(- 5));
        let _rug_ed_tests_llm_16_2002_llm_16_2002_rrrruuuugggg_test_to_i32_with_negative_f32 = 0;
    }
    #[test]
    fn test_to_i32_with_f64() {
        let _rug_st_tests_llm_16_2002_llm_16_2002_rrrruuuugggg_test_to_i32_with_f64 = 0;
        let rug_fuzz_0 = 5.0;
        let x: f64 = rug_fuzz_0;
        debug_assert_eq!(x.to_i32(), Some(5));
        let _rug_ed_tests_llm_16_2002_llm_16_2002_rrrruuuugggg_test_to_i32_with_f64 = 0;
    }
    #[test]
    fn test_to_i32_with_large_f64() {
        let _rug_st_tests_llm_16_2002_llm_16_2002_rrrruuuugggg_test_to_i32_with_large_f64 = 0;
        let rug_fuzz_0 = 1e10;
        let x: f64 = rug_fuzz_0;
        debug_assert_eq!(x.to_i32(), None);
        let _rug_ed_tests_llm_16_2002_llm_16_2002_rrrruuuugggg_test_to_i32_with_large_f64 = 0;
    }
    #[test]
    fn test_to_i32_with_negative_f64() {
        let _rug_st_tests_llm_16_2002_llm_16_2002_rrrruuuugggg_test_to_i32_with_negative_f64 = 0;
        let rug_fuzz_0 = 5.0;
        let x: f64 = -rug_fuzz_0;
        debug_assert_eq!(x.to_i32(), Some(- 5));
        let _rug_ed_tests_llm_16_2002_llm_16_2002_rrrruuuugggg_test_to_i32_with_negative_f64 = 0;
    }
    #[test]
    fn test_to_i32_with_u64() {
        let _rug_st_tests_llm_16_2002_llm_16_2002_rrrruuuugggg_test_to_i32_with_u64 = 0;
        let rug_fuzz_0 = 5;
        let x: u64 = rug_fuzz_0;
        debug_assert_eq!(x.to_i32(), Some(5));
        let _rug_ed_tests_llm_16_2002_llm_16_2002_rrrruuuugggg_test_to_i32_with_u64 = 0;
    }
    #[test]
    fn test_to_i32_with_large_u64() {
        let _rug_st_tests_llm_16_2002_llm_16_2002_rrrruuuugggg_test_to_i32_with_large_u64 = 0;
        let x: u64 = u64::MAX;
        debug_assert_eq!(x.to_i32(), None);
        let _rug_ed_tests_llm_16_2002_llm_16_2002_rrrruuuugggg_test_to_i32_with_large_u64 = 0;
    }
}
#[cfg(test)]
mod test {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_i8() {
        assert_eq!(0i8.to_i8(), Some(0i8));
        assert_eq!(127i16.to_i8(), Some(127i8));
        assert_eq!((- 128i16).to_i8(), Some(- 128i8));
        assert_eq!(128i16.to_i8(), None);
        assert_eq!((- 129i16).to_i8(), None);
        assert_eq!(0i32.to_i8(), Some(0i8));
        assert_eq!(127i32.to_i8(), Some(127i8));
        assert_eq!((- 128i32).to_i8(), Some(- 128i8));
        assert_eq!(128i32.to_i8(), None);
        assert_eq!((- 129i32).to_i8(), None);
        assert_eq!(0i64.to_i8(), Some(0i8));
        assert_eq!(127i64.to_i8(), Some(127i8));
        assert_eq!((- 128i64).to_i8(), Some(- 128i8));
        assert_eq!(128i64.to_i8(), None);
        assert_eq!((- 129i64).to_i8(), None);
        assert_eq!(0u8.to_i8(), Some(0i8));
        assert_eq!(127u8.to_i8(), Some(127i8));
        assert_eq!(128u8.to_i8(), None);
        assert_eq!(0u16.to_i8(), Some(0i8));
        assert_eq!(127u16.to_i8(), Some(127i8));
        assert_eq!(128u16.to_i8(), None);
        assert_eq!(0u32.to_i8(), Some(0i8));
        assert_eq!(127u32.to_i8(), Some(127i8));
        assert_eq!(128u32.to_i8(), None);
        assert_eq!(0u64.to_i8(), Some(0i8));
        assert_eq!(127u64.to_i8(), Some(127i8));
        assert_eq!(128u64.to_i8(), None);
        assert_eq!(0.0f32.to_i8(), Some(0i8));
        assert_eq!(127.0f32.to_i8(), Some(127i8));
        assert_eq!((- 128.0f32).to_i8(), Some(- 128i8));
        assert_eq!(128.0f32.to_i8(), None);
        assert_eq!((- 129.0f32).to_i8(), None);
        assert_eq!(f32::MAX.to_i8(), None);
        assert_eq!(f32::MIN.to_i8(), None);
        assert_eq!(0.0f64.to_i8(), Some(0i8));
        assert_eq!(127.0f64.to_i8(), Some(127i8));
        assert_eq!((- 128.0f64).to_i8(), Some(- 128i8));
        assert_eq!(128.0f64.to_i8(), None);
        assert_eq!((- 129.0f64).to_i8(), None);
        assert_eq!(f64::MAX.to_i8(), None);
        assert_eq!(f64::MIN.to_i8(), None);
    }
}
#[cfg(test)]
mod tests_llm_16_2004 {
    use crate::ToPrimitive;
    #[test]
    fn test_to_isize_with_i32() {
        let _rug_st_tests_llm_16_2004_rrrruuuugggg_test_to_isize_with_i32 = 0;
        let rug_fuzz_0 = 42;
        let value: i32 = rug_fuzz_0;
        debug_assert_eq!(value.to_isize(), Some(42));
        let _rug_ed_tests_llm_16_2004_rrrruuuugggg_test_to_isize_with_i32 = 0;
    }
    #[test]
    fn test_to_isize_with_i64() {
        let _rug_st_tests_llm_16_2004_rrrruuuugggg_test_to_isize_with_i64 = 0;
        let rug_fuzz_0 = 42;
        let value: i64 = rug_fuzz_0;
        debug_assert_eq!(value.to_isize(), Some(42));
        let _rug_ed_tests_llm_16_2004_rrrruuuugggg_test_to_isize_with_i64 = 0;
    }
    #[test]
    fn test_to_isize_with_u64_too_large() {
        let _rug_st_tests_llm_16_2004_rrrruuuugggg_test_to_isize_with_u64_too_large = 0;
        let value: u64 = u64::MAX;
        debug_assert_eq!(value.to_isize(), None);
        let _rug_ed_tests_llm_16_2004_rrrruuuugggg_test_to_isize_with_u64_too_large = 0;
    }
    #[test]
    fn test_to_isize_with_u32() {
        let _rug_st_tests_llm_16_2004_rrrruuuugggg_test_to_isize_with_u32 = 0;
        let rug_fuzz_0 = 42;
        let value: u32 = rug_fuzz_0;
        debug_assert_eq!(value.to_isize(), Some(42));
        let _rug_ed_tests_llm_16_2004_rrrruuuugggg_test_to_isize_with_u32 = 0;
    }
    #[test]
    fn test_to_isize_with_u32_too_large() {
        let _rug_st_tests_llm_16_2004_rrrruuuugggg_test_to_isize_with_u32_too_large = 0;
        let value: u32 = u32::MAX;
        if value as u64 > isize::MAX as u64 {
            debug_assert_eq!(value.to_isize(), None);
        } else {
            debug_assert_eq!(value.to_isize(), Some(value as isize));
        }
        let _rug_ed_tests_llm_16_2004_rrrruuuugggg_test_to_isize_with_u32_too_large = 0;
    }
    #[test]
    fn test_to_isize_with_f64() {
        let _rug_st_tests_llm_16_2004_rrrruuuugggg_test_to_isize_with_f64 = 0;
        let rug_fuzz_0 = 42.0;
        let value: f64 = rug_fuzz_0;
        debug_assert_eq!(value.to_isize(), Some(42));
        let _rug_ed_tests_llm_16_2004_rrrruuuugggg_test_to_isize_with_f64 = 0;
    }
    #[test]
    fn test_to_isize_with_f64_too_large() {
        let _rug_st_tests_llm_16_2004_rrrruuuugggg_test_to_isize_with_f64_too_large = 0;
        let value: f64 = f64::MAX;
        debug_assert_eq!(value.to_isize(), None);
        let _rug_ed_tests_llm_16_2004_rrrruuuugggg_test_to_isize_with_f64_too_large = 0;
    }
    #[test]
    fn test_to_isize_with_f64_negative() {
        let _rug_st_tests_llm_16_2004_rrrruuuugggg_test_to_isize_with_f64_negative = 0;
        let rug_fuzz_0 = 42.0;
        let value: f64 = -rug_fuzz_0;
        debug_assert_eq!(value.to_isize(), Some(- 42));
        let _rug_ed_tests_llm_16_2004_rrrruuuugggg_test_to_isize_with_f64_negative = 0;
    }
    #[test]
    fn test_to_isize_with_f64_non_integer() {
        let _rug_st_tests_llm_16_2004_rrrruuuugggg_test_to_isize_with_f64_non_integer = 0;
        let rug_fuzz_0 = 42.5;
        let value: f64 = rug_fuzz_0;
        debug_assert_eq!(value.to_isize(), None);
        let _rug_ed_tests_llm_16_2004_rrrruuuugggg_test_to_isize_with_f64_non_integer = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2005_llm_16_2005 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_u128_within_u64_range() {
        let _rug_st_tests_llm_16_2005_llm_16_2005_rrrruuuugggg_test_to_u128_within_u64_range = 0;
        let rug_fuzz_0 = 42u64;
        debug_assert_eq!(ToPrimitive::to_u128(& rug_fuzz_0), Some(42u128));
        debug_assert_eq!(ToPrimitive::to_u128(& u64::MAX), Some(u128::from(u64::MAX)));
        let _rug_ed_tests_llm_16_2005_llm_16_2005_rrrruuuugggg_test_to_u128_within_u64_range = 0;
    }
    #[test]
    fn test_to_u128_for_u64_max_plus_one() {
        let _rug_st_tests_llm_16_2005_llm_16_2005_rrrruuuugggg_test_to_u128_for_u64_max_plus_one = 0;
        let _rug_ed_tests_llm_16_2005_llm_16_2005_rrrruuuugggg_test_to_u128_for_u64_max_plus_one = 0;
    }
    #[test]
    fn test_to_u128_for_negative_values() {
        let _rug_st_tests_llm_16_2005_llm_16_2005_rrrruuuugggg_test_to_u128_for_negative_values = 0;
        let rug_fuzz_0 = 1i64;
        debug_assert_eq!(ToPrimitive::to_u128(& (- rug_fuzz_0)), None);
        let _rug_ed_tests_llm_16_2005_llm_16_2005_rrrruuuugggg_test_to_u128_for_negative_values = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2006 {
    use crate::ToPrimitive;
    #[test]
    fn test_to_u16_with_u16() {
        let _rug_st_tests_llm_16_2006_rrrruuuugggg_test_to_u16_with_u16 = 0;
        let rug_fuzz_0 = 123;
        let value: u16 = rug_fuzz_0;
        debug_assert_eq!(value.to_u16(), Some(123));
        let _rug_ed_tests_llm_16_2006_rrrruuuugggg_test_to_u16_with_u16 = 0;
    }
    #[test]
    fn test_to_u16_with_i32_within_range() {
        let _rug_st_tests_llm_16_2006_rrrruuuugggg_test_to_u16_with_i32_within_range = 0;
        let rug_fuzz_0 = 100;
        let value: i32 = rug_fuzz_0;
        debug_assert_eq!(value.to_u16(), Some(100));
        let _rug_ed_tests_llm_16_2006_rrrruuuugggg_test_to_u16_with_i32_within_range = 0;
    }
    #[test]
    fn test_to_u16_with_i32_out_of_range_negative() {
        let _rug_st_tests_llm_16_2006_rrrruuuugggg_test_to_u16_with_i32_out_of_range_negative = 0;
        let rug_fuzz_0 = 100;
        let value: i32 = -rug_fuzz_0;
        debug_assert_eq!(value.to_u16(), None);
        let _rug_ed_tests_llm_16_2006_rrrruuuugggg_test_to_u16_with_i32_out_of_range_negative = 0;
    }
    #[test]
    fn test_to_u16_with_i32_out_of_range_positive() {
        let _rug_st_tests_llm_16_2006_rrrruuuugggg_test_to_u16_with_i32_out_of_range_positive = 0;
        let rug_fuzz_0 = 100_000;
        let value: i32 = rug_fuzz_0;
        debug_assert_eq!(value.to_u16(), None);
        let _rug_ed_tests_llm_16_2006_rrrruuuugggg_test_to_u16_with_i32_out_of_range_positive = 0;
    }
    #[test]
    fn test_to_u16_with_u64_within_range() {
        let _rug_st_tests_llm_16_2006_rrrruuuugggg_test_to_u16_with_u64_within_range = 0;
        let rug_fuzz_0 = 255;
        let value: u64 = rug_fuzz_0;
        debug_assert_eq!(value.to_u16(), Some(255));
        let _rug_ed_tests_llm_16_2006_rrrruuuugggg_test_to_u16_with_u64_within_range = 0;
    }
    #[test]
    fn test_to_u16_with_u64_out_of_range() {
        let _rug_st_tests_llm_16_2006_rrrruuuugggg_test_to_u16_with_u64_out_of_range = 0;
        let rug_fuzz_0 = 100_000;
        let value: u64 = rug_fuzz_0;
        debug_assert_eq!(value.to_u16(), None);
        let _rug_ed_tests_llm_16_2006_rrrruuuugggg_test_to_u16_with_u64_out_of_range = 0;
    }
    #[test]
    fn test_to_u16_with_f64_within_range() {
        let _rug_st_tests_llm_16_2006_rrrruuuugggg_test_to_u16_with_f64_within_range = 0;
        let rug_fuzz_0 = 255.0;
        let value: f64 = rug_fuzz_0;
        debug_assert_eq!(value.to_u16(), Some(255));
        let _rug_ed_tests_llm_16_2006_rrrruuuugggg_test_to_u16_with_f64_within_range = 0;
    }
    #[test]
    fn test_to_u16_with_f64_out_of_range() {
        let _rug_st_tests_llm_16_2006_rrrruuuugggg_test_to_u16_with_f64_out_of_range = 0;
        let rug_fuzz_0 = 100_000.0;
        let value: f64 = rug_fuzz_0;
        debug_assert_eq!(value.to_u16(), None);
        let _rug_ed_tests_llm_16_2006_rrrruuuugggg_test_to_u16_with_f64_out_of_range = 0;
    }
    #[test]
    fn test_to_u16_with_f64_fractional() {
        let _rug_st_tests_llm_16_2006_rrrruuuugggg_test_to_u16_with_f64_fractional = 0;
        let rug_fuzz_0 = 255.99;
        let value: f64 = rug_fuzz_0;
        debug_assert_eq!(value.to_u16(), Some(255));
        let _rug_ed_tests_llm_16_2006_rrrruuuugggg_test_to_u16_with_f64_fractional = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2007_llm_16_2007 {
    use crate::ToPrimitive;
    #[test]
    fn test_to_u32_with_u32_max() {
        let _rug_st_tests_llm_16_2007_llm_16_2007_rrrruuuugggg_test_to_u32_with_u32_max = 0;
        let value: u32 = u32::MAX;
        debug_assert_eq!(value.to_u32(), Some(u32::MAX));
        let _rug_ed_tests_llm_16_2007_llm_16_2007_rrrruuuugggg_test_to_u32_with_u32_max = 0;
    }
    #[test]
    fn test_to_u32_with_u64_max() {
        let _rug_st_tests_llm_16_2007_llm_16_2007_rrrruuuugggg_test_to_u32_with_u64_max = 0;
        let value: u64 = u64::MAX;
        debug_assert_eq!(value.to_u32(), None);
        let _rug_ed_tests_llm_16_2007_llm_16_2007_rrrruuuugggg_test_to_u32_with_u64_max = 0;
    }
    #[test]
    fn test_to_u32_with_i32_max() {
        let _rug_st_tests_llm_16_2007_llm_16_2007_rrrruuuugggg_test_to_u32_with_i32_max = 0;
        let value: i32 = i32::MAX;
        debug_assert_eq!(value.to_u32(), Some(i32::MAX as u32));
        let _rug_ed_tests_llm_16_2007_llm_16_2007_rrrruuuugggg_test_to_u32_with_i32_max = 0;
    }
    #[test]
    fn test_to_u32_with_i32_min() {
        let _rug_st_tests_llm_16_2007_llm_16_2007_rrrruuuugggg_test_to_u32_with_i32_min = 0;
        let value: i32 = i32::MIN;
        debug_assert_eq!(value.to_u32(), None);
        let _rug_ed_tests_llm_16_2007_llm_16_2007_rrrruuuugggg_test_to_u32_with_i32_min = 0;
    }
    #[test]
    fn test_to_u32_with_i64_max() {
        let _rug_st_tests_llm_16_2007_llm_16_2007_rrrruuuugggg_test_to_u32_with_i64_max = 0;
        let value: i64 = i64::MAX;
        debug_assert_eq!(value.to_u32(), None);
        let _rug_ed_tests_llm_16_2007_llm_16_2007_rrrruuuugggg_test_to_u32_with_i64_max = 0;
    }
    #[test]
    fn test_to_u32_with_i64_min() {
        let _rug_st_tests_llm_16_2007_llm_16_2007_rrrruuuugggg_test_to_u32_with_i64_min = 0;
        let value: i64 = i64::MIN;
        debug_assert_eq!(value.to_u32(), None);
        let _rug_ed_tests_llm_16_2007_llm_16_2007_rrrruuuugggg_test_to_u32_with_i64_min = 0;
    }
    #[test]
    fn test_to_u32_with_f64_max() {
        let _rug_st_tests_llm_16_2007_llm_16_2007_rrrruuuugggg_test_to_u32_with_f64_max = 0;
        let value: f64 = f64::MAX;
        debug_assert_eq!(value.to_u32(), None);
        let _rug_ed_tests_llm_16_2007_llm_16_2007_rrrruuuugggg_test_to_u32_with_f64_max = 0;
    }
    #[test]
    fn test_to_u32_with_f64_min() {
        let _rug_st_tests_llm_16_2007_llm_16_2007_rrrruuuugggg_test_to_u32_with_f64_min = 0;
        let value: f64 = f64::MIN;
        debug_assert_eq!(value.to_u32(), None);
        let _rug_ed_tests_llm_16_2007_llm_16_2007_rrrruuuugggg_test_to_u32_with_f64_min = 0;
    }
    #[test]
    fn test_to_u32_with_f64_zero() {
        let _rug_st_tests_llm_16_2007_llm_16_2007_rrrruuuugggg_test_to_u32_with_f64_zero = 0;
        let rug_fuzz_0 = 0.0;
        let value: f64 = rug_fuzz_0;
        debug_assert_eq!(value.to_u32(), Some(0));
        let _rug_ed_tests_llm_16_2007_llm_16_2007_rrrruuuugggg_test_to_u32_with_f64_zero = 0;
    }
    #[test]
    fn test_to_u32_with_f64_positive() {
        let _rug_st_tests_llm_16_2007_llm_16_2007_rrrruuuugggg_test_to_u32_with_f64_positive = 0;
        let rug_fuzz_0 = 12345.678;
        let value = rug_fuzz_0;
        debug_assert_eq!(value.to_u32(), Some(12345));
        let _rug_ed_tests_llm_16_2007_llm_16_2007_rrrruuuugggg_test_to_u32_with_f64_positive = 0;
    }
    #[test]
    fn test_to_u32_with_f64_negative() {
        let _rug_st_tests_llm_16_2007_llm_16_2007_rrrruuuugggg_test_to_u32_with_f64_negative = 0;
        let rug_fuzz_0 = 12345.678;
        let value = -rug_fuzz_0;
        debug_assert_eq!(value.to_u32(), None);
        let _rug_ed_tests_llm_16_2007_llm_16_2007_rrrruuuugggg_test_to_u32_with_f64_negative = 0;
    }
    #[test]
    fn test_to_u32_with_f64_large() {
        let _rug_st_tests_llm_16_2007_llm_16_2007_rrrruuuugggg_test_to_u32_with_f64_large = 0;
        let rug_fuzz_0 = 1e10;
        let value = rug_fuzz_0;
        debug_assert_eq!(value.to_u32(), None);
        let _rug_ed_tests_llm_16_2007_llm_16_2007_rrrruuuugggg_test_to_u32_with_f64_large = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2008_llm_16_2008 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_u8_within_bounds() {
        let _rug_st_tests_llm_16_2008_llm_16_2008_rrrruuuugggg_test_to_u8_within_bounds = 0;
        let rug_fuzz_0 = 5i32;
        let rug_fuzz_1 = 0i32;
        debug_assert_eq!(rug_fuzz_0.to_u8(), Some(5u8));
        debug_assert_eq!(rug_fuzz_1.to_u8(), Some(0u8));
        debug_assert_eq!((u8::MAX as i32).to_u8(), Some(u8::MAX));
        let _rug_ed_tests_llm_16_2008_llm_16_2008_rrrruuuugggg_test_to_u8_within_bounds = 0;
    }
    #[test]
    fn test_to_u8_out_of_bounds() {
        let _rug_st_tests_llm_16_2008_llm_16_2008_rrrruuuugggg_test_to_u8_out_of_bounds = 0;
        let rug_fuzz_0 = 1i32;
        let rug_fuzz_1 = 1;
        debug_assert_eq!((- rug_fuzz_0).to_u8(), None);
        debug_assert_eq!((u8::MAX as i32 + rug_fuzz_1).to_u8(), None);
        debug_assert_eq!((i32::MAX).to_u8(), None);
        let _rug_ed_tests_llm_16_2008_llm_16_2008_rrrruuuugggg_test_to_u8_out_of_bounds = 0;
    }
    #[test]
    fn test_to_u8_with_floats() {
        let _rug_st_tests_llm_16_2008_llm_16_2008_rrrruuuugggg_test_to_u8_with_floats = 0;
        let rug_fuzz_0 = 5.0f32;
        let rug_fuzz_1 = 1.0f32;
        let rug_fuzz_2 = 1.0;
        let rug_fuzz_3 = 256.999f32;
        debug_assert_eq!(rug_fuzz_0.to_u8(), Some(5u8));
        debug_assert_eq!((- rug_fuzz_1).to_u8(), None);
        debug_assert_eq!((u8::MAX as f32 + rug_fuzz_2).to_u8(), None);
        debug_assert_eq!(rug_fuzz_3.to_u8(), None);
        let _rug_ed_tests_llm_16_2008_llm_16_2008_rrrruuuugggg_test_to_u8_with_floats = 0;
    }
    #[test]
    fn test_to_u8_with_large_integers() {
        let _rug_st_tests_llm_16_2008_llm_16_2008_rrrruuuugggg_test_to_u8_with_large_integers = 0;
        let rug_fuzz_0 = 1i64;
        let rug_fuzz_1 = 40;
        let rug_fuzz_2 = 1i64;
        debug_assert_eq!((rug_fuzz_0 << rug_fuzz_1).to_u8(), None);
        debug_assert_eq!((- rug_fuzz_2).to_u8(), None);
        let _rug_ed_tests_llm_16_2008_llm_16_2008_rrrruuuugggg_test_to_u8_with_large_integers = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2009_llm_16_2009 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_usize() {
        let _rug_st_tests_llm_16_2009_llm_16_2009_rrrruuuugggg_test_to_usize = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 1i32;
        debug_assert_eq!(rug_fuzz_0.to_usize(), Some(5_usize));
        debug_assert_eq!((- rug_fuzz_1).to_usize(), None);
        debug_assert_eq!(i32::MAX.to_usize(), Some(i32::MAX as usize));
        #[cfg(target_pointer_width = "64")]
        {
            debug_assert_eq!((i64::MAX).to_usize(), None);
        }
        #[cfg(target_pointer_width = "32")]
        {
            debug_assert_eq!((i64::MAX).to_usize(), Some(i64::MAX as usize));
        }
        let _rug_ed_tests_llm_16_2009_llm_16_2009_rrrruuuugggg_test_to_usize = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_2010_llm_16_2010 {
    use crate::cast::cast;
    use crate::cast::NumCast;
    use std::num::Wrapping;
    #[test]
    fn cast_wrapping() {
        let _rug_st_tests_llm_16_2010_llm_16_2010_rrrruuuugggg_cast_wrapping = 0;
        let rug_fuzz_0 = 100;
        let rug_fuzz_1 = 1.5;
        let rug_fuzz_2 = 1e20;
        let rug_fuzz_3 = 1;
        let x: Wrapping<i32> = Wrapping(rug_fuzz_0);
        let y: Option<Wrapping<i64>> = cast(x);
        debug_assert_eq!(y, Some(Wrapping(100i64)));
        let x: Wrapping<u32> = Wrapping(u32::MAX);
        let y: Option<Wrapping<i64>> = cast(x);
        debug_assert_eq!(y, Some(Wrapping(u32::MAX as i64)));
        let x: Wrapping<f32> = Wrapping(rug_fuzz_1);
        let y: Option<Wrapping<i32>> = cast(x);
        debug_assert_eq!(y, Some(Wrapping(1)));
        let x: Wrapping<f64> = Wrapping(rug_fuzz_2);
        let y: Option<Wrapping<f32>> = cast(x);
        debug_assert!(y.is_none() || y.unwrap().0.is_infinite());
        let x: Wrapping<i64> = Wrapping(-rug_fuzz_3);
        let y: Option<Wrapping<u32>> = cast(x);
        debug_assert_eq!(y, None);
        let _rug_ed_tests_llm_16_2010_llm_16_2010_rrrruuuugggg_cast_wrapping = 0;
    }
}
#[cfg(test)]
mod tests_rug_1 {
    use super::*;
    use std::num::Wrapping;
    #[test]
    fn test_to_f64() {
        let _rug_st_tests_rug_1_rrrruuuugggg_test_to_f64 = 0;
        let rug_fuzz_0 = 0_i64;
        let p0 = Wrapping(rug_fuzz_0);
        let result = crate::cast::ToPrimitive::to_f64(&p0);
        debug_assert_eq!(result, Some(0.0_f64));
        let _rug_ed_tests_rug_1_rrrruuuugggg_test_to_f64 = 0;
    }
}
#[cfg(test)]
mod tests_rug_2 {
    use super::*;
    use crate::cast::FromPrimitive;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_2_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 42;
        let mut p0: i16 = rug_fuzz_0;
        let result = <i32 as FromPrimitive>::from_i16(p0);
        debug_assert_eq!(result, Some(42i32));
        let _rug_ed_tests_rug_2_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_4 {
    use super::*;
    use crate::ToPrimitive;
    #[test]
    fn test_to_isize() {
        let _rug_st_tests_rug_4_rrrruuuugggg_test_to_isize = 0;
        let rug_fuzz_0 = 42;
        let mut p0: isize = rug_fuzz_0;
        debug_assert_eq!(< isize as ToPrimitive > ::to_isize(& p0), Some(42));
        let _rug_ed_tests_rug_4_rrrruuuugggg_test_to_isize = 0;
    }
}
#[cfg(test)]
mod tests_rug_5 {
    use super::*;
    use crate::ToPrimitive;
    #[test]
    fn test_to_isize() {
        let _rug_st_tests_rug_5_rrrruuuugggg_test_to_isize = 0;
        let rug_fuzz_0 = 123;
        let p0: i32 = rug_fuzz_0;
        debug_assert_eq!(< i32 as ToPrimitive > ::to_isize(& p0), Some(123_isize));
        let _rug_ed_tests_rug_5_rrrruuuugggg_test_to_isize = 0;
    }
}
#[cfg(test)]
mod tests_rug_6 {
    use super::*;
    use crate::ToPrimitive;
    #[test]
    fn test_to_u128() {
        let _rug_st_tests_rug_6_rrrruuuugggg_test_to_u128 = 0;
        let rug_fuzz_0 = 42;
        let p0: i32 = rug_fuzz_0;
        debug_assert_eq!(< i32 as ToPrimitive > ::to_u128(& p0), Some(42_u128));
        let _rug_ed_tests_rug_6_rrrruuuugggg_test_to_u128 = 0;
    }
}
#[cfg(test)]
mod tests_rug_7 {
    use super::*;
    use crate::ToPrimitive;
    use std::mem::size_of;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_7_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 42;
        let mut p0: i64 = rug_fuzz_0;
        debug_assert_eq!(< i64 as ToPrimitive > ::to_isize(& p0), Some(42));
        let mut p0: i64 = i64::MAX;
        debug_assert_eq!(
            < i64 as ToPrimitive > ::to_isize(& p0), Some(i64::MAX as isize)
        );
        let mut p0: i64 = i64::MIN;
        debug_assert_eq!(
            < i64 as ToPrimitive > ::to_isize(& p0), if size_of:: < i64 > () <= size_of::
            < isize > () { Some(i64::MIN as isize) } else { None }
        );
        let _rug_ed_tests_rug_7_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_8 {
    use crate::cast::ToPrimitive;
    #[test]
    fn test_to_f64() {
        let _rug_st_tests_rug_8_rrrruuuugggg_test_to_f64 = 0;
        let rug_fuzz_0 = 42;
        let mut p0: i64 = rug_fuzz_0;
        debug_assert_eq!(< i64 as ToPrimitive > ::to_f64(& p0), Some(42.0_f64));
        let _rug_ed_tests_rug_8_rrrruuuugggg_test_to_f64 = 0;
    }
}
#[cfg(test)]
mod tests_rug_9 {
    use super::*;
    use crate::ToPrimitive;
    #[test]
    fn test_to_i128() {
        let _rug_st_tests_rug_9_rrrruuuugggg_test_to_i128 = 0;
        let rug_fuzz_0 = 1234567890123456789i128;
        let p0: i128 = rug_fuzz_0;
        debug_assert_eq!(
            < i128 as ToPrimitive > ::to_i128(& p0), Some(1234567890123456789i128)
        );
        let _rug_ed_tests_rug_9_rrrruuuugggg_test_to_i128 = 0;
    }
}
#[cfg(test)]
mod tests_rug_10 {
    use super::*;
    use crate::ToPrimitive;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_10_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 12;
        let p0: i128 = rug_fuzz_0;
        debug_assert_eq!(< i128 as ToPrimitive > ::to_usize(& p0), Some(12_usize));
        let _rug_ed_tests_rug_10_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_11 {
    use super::*;
    use crate::ToPrimitive;
    #[test]
    fn test_to_usize() {
        let _rug_st_tests_rug_11_rrrruuuugggg_test_to_usize = 0;
        let rug_fuzz_0 = 12345;
        let p0: usize = rug_fuzz_0;
        debug_assert_eq!(< usize as ToPrimitive > ::to_usize(& p0), Some(12345));
        let _rug_ed_tests_rug_11_rrrruuuugggg_test_to_usize = 0;
    }
}
#[cfg(test)]
mod tests_rug_12 {
    use super::*;
    use crate::ToPrimitive;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_12_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 42;
        let p0: u16 = rug_fuzz_0;
        debug_assert_eq!(< u16 as ToPrimitive > ::to_u32(& p0), Some(42_u32));
        let _rug_ed_tests_rug_12_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_13 {
    use super::*;
    use crate::ToPrimitive;
    #[test]
    fn test_to_i16() {
        let _rug_st_tests_rug_13_rrrruuuugggg_test_to_i16 = 0;
        let rug_fuzz_0 = 32.0;
        let p0: f32 = rug_fuzz_0;
        debug_assert_eq!(< f32 as ToPrimitive > ::to_i16(& p0), Some(32));
        let _rug_ed_tests_rug_13_rrrruuuugggg_test_to_i16 = 0;
    }
}
#[cfg(test)]
mod tests_rug_14 {
    use super::*;
    use crate::ToPrimitive;
    #[test]
    fn test_to_usize() {
        let _rug_st_tests_rug_14_rrrruuuugggg_test_to_usize = 0;
        let rug_fuzz_0 = 42.0;
        let p0: f32 = rug_fuzz_0;
        debug_assert_eq!(< f32 as ToPrimitive > ::to_usize(& p0), Some(42));
        let _rug_ed_tests_rug_14_rrrruuuugggg_test_to_usize = 0;
    }
}
#[cfg(test)]
mod tests_rug_15 {
    use super::*;
    use crate::FromPrimitive;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_15_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 42;
        let mut p0: i64 = rug_fuzz_0;
        debug_assert_eq!(< isize > ::from_i64(p0), Some(42isize));
        let _rug_ed_tests_rug_15_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_16 {
    use super::*;
    use crate::FromPrimitive;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_16_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 42;
        let mut p0: u32 = rug_fuzz_0;
        <isize>::from_u32(p0);
        let _rug_ed_tests_rug_16_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_17 {
    use super::*;
    use crate::cast::FromPrimitive;
    #[test]
    fn test_from_f32() {
        let _rug_st_tests_rug_17_rrrruuuugggg_test_from_f32 = 0;
        let rug_fuzz_0 = 42.0;
        let p0: f32 = rug_fuzz_0;
        debug_assert_eq!(< i16 > ::from_f32(p0), Some(42));
        let _rug_ed_tests_rug_17_rrrruuuugggg_test_from_f32 = 0;
    }
}
#[cfg(test)]
mod tests_rug_18 {
    use super::*;
    use crate::FromPrimitive;
    #[test]
    fn test_from_u64() {
        let _rug_st_tests_rug_18_rrrruuuugggg_test_from_u64 = 0;
        let rug_fuzz_0 = 42u64;
        let p0: u64 = rug_fuzz_0;
        debug_assert_eq!(< u64 as FromPrimitive > ::from_u64(p0), Some(42u64));
        let _rug_ed_tests_rug_18_rrrruuuugggg_test_from_u64 = 0;
    }
}
#[cfg(test)]
mod tests_rug_19 {
    use std::num::Wrapping;
    use crate::FromPrimitive;
    #[test]
    fn test_from_i8() {
        let _rug_st_tests_rug_19_rrrruuuugggg_test_from_i8 = 0;
        let rug_fuzz_0 = 42;
        let p0: i8 = rug_fuzz_0;
        debug_assert_eq!(
            < Wrapping < u32 > as FromPrimitive > ::from_i8(p0), Some(Wrapping(42u32))
        );
        let _rug_ed_tests_rug_19_rrrruuuugggg_test_from_i8 = 0;
    }
}
#[cfg(test)]
mod tests_rug_20 {
    use crate::NumCast;
    use std::num::Wrapping;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_20_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 0;
        let mut p0 = Wrapping(rug_fuzz_0);
        let _result: Option<i8> = <i8 as NumCast>::from(p0);
        let _rug_ed_tests_rug_20_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_21 {
    use std::num::Wrapping;
    use crate::NumCast;
    #[test]
    fn test_from() {
        let _rug_st_tests_rug_21_rrrruuuugggg_test_from = 0;
        let rug_fuzz_0 = 0;
        let mut p0 = Wrapping(rug_fuzz_0);
        debug_assert_eq!(< isize as NumCast > ::from(p0), Some(0));
        let _rug_ed_tests_rug_21_rrrruuuugggg_test_from = 0;
    }
}
#[cfg(test)]
mod tests_rug_22 {
    use super::*;
    use crate::AsPrimitive;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_22_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 42u32;
        let p0: u32 = rug_fuzz_0;
        debug_assert_eq!(< u32 as AsPrimitive < u16 > > ::as_(p0), p0 as u16);
        let _rug_ed_tests_rug_22_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_23 {
    use super::*;
    use crate::AsPrimitive;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_23_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 123;
        let mut p0: i32 = rug_fuzz_0;
        let result: u8 = <i32 as AsPrimitive<u8>>::as_(p0);
        debug_assert_eq!(result, p0 as u8);
        let _rug_ed_tests_rug_23_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_24 {
    use crate::AsPrimitive;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_24_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 12345;
        let mut p0: i32 = rug_fuzz_0;
        let _: u16 = <i32 as AsPrimitive<u16>>::as_(p0);
        let _rug_ed_tests_rug_24_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_25 {
    use super::*;
    use crate::AsPrimitive;
    #[test]
    fn test_as_primitive_i64_to_u128() {
        let _rug_st_tests_rug_25_rrrruuuugggg_test_as_primitive_i64_to_u128 = 0;
        let rug_fuzz_0 = 1234_i64;
        let mut p0: i64 = rug_fuzz_0;
        let result: u128 = <i64 as AsPrimitive<u128>>::as_(p0);
        debug_assert_eq!(result, 1234_u128);
        let _rug_ed_tests_rug_25_rrrruuuugggg_test_as_primitive_i64_to_u128 = 0;
    }
}
#[cfg(test)]
mod tests_rug_26 {
    use super::*;
    use crate::AsPrimitive;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_26_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 12345678901234567890;
        let mut p0: u128 = rug_fuzz_0;
        let _result: i128 = <u128 as AsPrimitive<i128>>::as_(p0);
        let _rug_ed_tests_rug_26_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_27 {
    use super::*;
    use crate::AsPrimitive;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_27_rrrruuuugggg_test_rug = 0;
        let mut p0: i128 = i128::MAX;
        let result: u16 = <i128 as AsPrimitive<u16>>::as_(p0);
        debug_assert_eq!(result, u16::MAX);
        let _rug_ed_tests_rug_27_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_28 {
    use super::*;
    use crate::AsPrimitive;
    #[test]
    fn test_as_primitive_i128_to_isize() {
        let _rug_st_tests_rug_28_rrrruuuugggg_test_as_primitive_i128_to_isize = 0;
        let rug_fuzz_0 = 1234_i128;
        let p0: i128 = rug_fuzz_0;
        <i128 as AsPrimitive<isize>>::as_(p0);
        let _rug_ed_tests_rug_28_rrrruuuugggg_test_as_primitive_i128_to_isize = 0;
    }
}
#[cfg(test)]
mod tests_rug_29 {
    use super::*;
    use crate::AsPrimitive;
    #[test]
    fn test_as_primitive() {
        let _rug_st_tests_rug_29_rrrruuuugggg_test_as_primitive = 0;
        let rug_fuzz_0 = 123;
        let p0: isize = rug_fuzz_0;
        debug_assert_eq!(
            < isize as crate ::cast::AsPrimitive < u8 > > ::as_(p0), 123 as u8
        );
        let _rug_ed_tests_rug_29_rrrruuuugggg_test_as_primitive = 0;
    }
}
#[cfg(test)]
mod tests_rug_30 {
    use super::*;
    use crate::AsPrimitive;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_30_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 123;
        let mut p0: isize = rug_fuzz_0;
        let result: u128 = <isize as AsPrimitive<u128>>::as_(p0);
        debug_assert_eq!(result, 123u128);
        let _rug_ed_tests_rug_30_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_31 {
    use super::*;
    use crate::AsPrimitive;
    #[test]
    fn test_as_primitive_f32_to_i8() {
        let _rug_st_tests_rug_31_rrrruuuugggg_test_as_primitive_f32_to_i8 = 0;
        let rug_fuzz_0 = 123.456;
        let p0: f32 = rug_fuzz_0;
        debug_assert_eq!(< f32 as AsPrimitive < i8 > > ::as_(p0), 123i8);
        let _rug_ed_tests_rug_31_rrrruuuugggg_test_as_primitive_f32_to_i8 = 0;
    }
}
#[cfg(test)]
mod tests_rug_32 {
    use super::*;
    use crate::AsPrimitive;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_32_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 3.141592;
        let p0: f32 = rug_fuzz_0;
        let result: isize = <f32 as AsPrimitive<isize>>::as_(p0);
        debug_assert_eq!(result, 3);
        let _rug_ed_tests_rug_32_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_33 {
    use super::*;
    use crate::AsPrimitive;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_33_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 42.3;
        let p0: f64 = rug_fuzz_0;
        debug_assert_eq!(< f64 as AsPrimitive < u8 > > ::as_(p0), p0 as u8);
        let _rug_ed_tests_rug_33_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_34 {
    use super::*;
    use crate::AsPrimitive;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_34_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 'A';
        let mut p0: char = rug_fuzz_0;
        let result: u32 = <char as AsPrimitive<u32>>::as_(p0);
        debug_assert_eq!(result, 'A' as u32);
        let _rug_ed_tests_rug_34_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_35 {
    use super::*;
    use crate::AsPrimitive;
    #[test]
    fn test_as_primitive_bool_to_u128() {
        let _rug_st_tests_rug_35_rrrruuuugggg_test_as_primitive_bool_to_u128 = 0;
        let rug_fuzz_0 = true;
        let p0: bool = rug_fuzz_0;
        debug_assert_eq!(< bool as AsPrimitive < u128 > > ::as_(p0), 1u128);
        debug_assert_eq!(< bool as AsPrimitive < u128 > > ::as_(! p0), 0u128);
        let _rug_ed_tests_rug_35_rrrruuuugggg_test_as_primitive_bool_to_u128 = 0;
    }
}
#[cfg(test)]
mod tests_rug_36 {
    use super::*;
    use crate::AsPrimitive;
    #[test]
    fn test_as_primitive_bool_to_i16() {
        let _rug_st_tests_rug_36_rrrruuuugggg_test_as_primitive_bool_to_i16 = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = false;
        let p0: bool = rug_fuzz_0;
        debug_assert_eq!(< bool as AsPrimitive < i16 > > ::as_(p0), 1_i16);
        let p0: bool = rug_fuzz_1;
        debug_assert_eq!(< bool as AsPrimitive < i16 > > ::as_(p0), 0_i16);
        let _rug_ed_tests_rug_36_rrrruuuugggg_test_as_primitive_bool_to_i16 = 0;
    }
}
