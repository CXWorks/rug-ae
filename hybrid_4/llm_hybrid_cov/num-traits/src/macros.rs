#![allow(unused)]
/// Forward a method to an inherent method or a base trait method.
macro_rules! forward {
    ($(Self:: $method:ident (self $(, $arg:ident : $ty:ty)*) -> $ret:ty;)*) => {
        $(#[inline] fn $method (self $(, $arg : $ty)*) -> $ret { Self::$method (self $(,
        $arg)*) })*
    };
    ($($base:ident :: $method:ident (self $(, $arg:ident : $ty:ty)*) -> $ret:ty;)*) => {
        $(#[inline] fn $method (self $(, $arg : $ty)*) -> $ret { < Self as $base
        >::$method (self $(, $arg)*) })*
    };
    ($($base:ident :: $method:ident ($($arg:ident : $ty:ty),*) -> $ret:ty;)*) => {
        $(#[inline] fn $method ($($arg : $ty),*) -> $ret { < Self as $base >::$method
        ($($arg),*) })*
    };
    ($($imp:path as $method:ident (self $(, $arg:ident : $ty:ty)*) -> $ret:ty;)*) => {
        $(#[inline] fn $method (self $(, $arg : $ty)*) -> $ret { $imp (self $(, $arg)*)
        })*
    };
}
macro_rules! constant {
    ($($method:ident () -> $ret:expr;)*) => {
        $(#[inline] fn $method () -> Self { $ret })*
    };
}
#[cfg(test)]
mod tests_llm_16_191_llm_16_191 {
    use crate::real::Real;
    #[test]
    fn test_abs() {
        let _rug_st_tests_llm_16_191_llm_16_191_rrrruuuugggg_test_abs = 0;
        let rug_fuzz_0 = 5i32;
        let rug_fuzz_1 = 5i32;
        let rug_fuzz_2 = 0i32;
        let rug_fuzz_3 = 5.0f32;
        let rug_fuzz_4 = 5.0f32;
        let rug_fuzz_5 = 0.0f32;
        let rug_fuzz_6 = 5.0f64;
        let rug_fuzz_7 = 5.0f64;
        let rug_fuzz_8 = 0.0f64;
        debug_assert_eq!(rug_fuzz_0.abs(), 5);
        debug_assert_eq!((- rug_fuzz_1).abs(), 5);
        debug_assert_eq!(rug_fuzz_2.abs(), 0);
        debug_assert_eq!(rug_fuzz_3.abs(), 5.0);
        debug_assert_eq!((- rug_fuzz_4).abs(), 5.0);
        debug_assert_eq!(rug_fuzz_5.abs(), 0.0);
        debug_assert_eq!(rug_fuzz_6.abs(), 5.0);
        debug_assert_eq!((- rug_fuzz_7).abs(), 5.0);
        debug_assert_eq!(rug_fuzz_8.abs(), 0.0);
        let _rug_ed_tests_llm_16_191_llm_16_191_rrrruuuugggg_test_abs = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_192_llm_16_192 {
    use crate::real::Real;
    #[test]
    fn test_abs_sub() {
        let _rug_st_tests_llm_16_192_llm_16_192_rrrruuuugggg_test_abs_sub = 0;
        let rug_fuzz_0 = 6.0;
        let rug_fuzz_1 = 3.0;
        let rug_fuzz_2 = 3.0;
        let rug_fuzz_3 = 6.0;
        let rug_fuzz_4 = 0.0;
        let rug_fuzz_5 = 0.0;
        let rug_fuzz_6 = 3.0;
        let rug_fuzz_7 = 6.0;
        let rug_fuzz_8 = 6.0;
        let rug_fuzz_9 = 3.0;
        let x: f64 = rug_fuzz_0;
        let y: f64 = rug_fuzz_1;
        let z: f64 = x.abs_sub(y);
        debug_assert_eq!(z, 3.0);
        let a: f64 = rug_fuzz_2;
        let b: f64 = rug_fuzz_3;
        let c: f64 = a.abs_sub(b);
        debug_assert_eq!(c, 0.0);
        let p: f32 = rug_fuzz_4;
        let q: f32 = rug_fuzz_5;
        let r: f32 = p.abs_sub(q);
        debug_assert_eq!(r, 0.0);
        let m: f64 = -rug_fuzz_6;
        let n: f64 = -rug_fuzz_7;
        let o: f64 = m.abs_sub(n);
        debug_assert_eq!(o, 0.0);
        let i: f64 = -rug_fuzz_8;
        let j: f64 = -rug_fuzz_9;
        let k: f64 = i.abs_sub(j);
        debug_assert_eq!(k, 3.0);
        let _rug_ed_tests_llm_16_192_llm_16_192_rrrruuuugggg_test_abs_sub = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_193_llm_16_193 {
    use crate::real::Real;
    #[test]
    fn test_acos() {
        let _rug_st_tests_llm_16_193_llm_16_193_rrrruuuugggg_test_acos = 0;
        let rug_fuzz_0 = 0.5;
        let rug_fuzz_1 = 1e-10;
        let x: f64 = rug_fuzz_0;
        let result = Real::acos(x);
        let expected = f64::acos(x);
        debug_assert!((result - expected).abs() < rug_fuzz_1);
        let _rug_ed_tests_llm_16_193_llm_16_193_rrrruuuugggg_test_acos = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_195_llm_16_195 {
    use crate::real::Real;
    #[test]
    fn test_asin() {
        let _rug_st_tests_llm_16_195_llm_16_195_rrrruuuugggg_test_asin = 0;
        let rug_fuzz_0 = 0.0f64;
        let rug_fuzz_1 = 0.5f64;
        let rug_fuzz_2 = 0.5235987755982988f64;
        let rug_fuzz_3 = 1e-10;
        let rug_fuzz_4 = 0.5f64;
        let rug_fuzz_5 = 0.5235987755982988f64;
        let rug_fuzz_6 = 1e-10;
        debug_assert_eq!(rug_fuzz_0.asin(), 0.0f64);
        debug_assert!((rug_fuzz_1.asin() - rug_fuzz_2).abs() < rug_fuzz_3);
        debug_assert!((- rug_fuzz_4.asin() + rug_fuzz_5).abs() < rug_fuzz_6);
        debug_assert!(f64::NAN.asin().is_nan());
        let _rug_ed_tests_llm_16_195_llm_16_195_rrrruuuugggg_test_asin = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_196_llm_16_196 {
    use crate::real::Real;
    #[test]
    fn test_asinh() {
        let _rug_st_tests_llm_16_196_llm_16_196_rrrruuuugggg_test_asinh = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 0.0;
        let rug_fuzz_2 = 1.0;
        let rug_fuzz_3 = 0.881373587019543;
        let rug_fuzz_4 = 1.0;
        let rug_fuzz_5 = 0.881373587019543;
        let rug_fuzz_6 = 1e-15;
        let values: [(f64, f64); 5] = [
            (rug_fuzz_0, rug_fuzz_1),
            (rug_fuzz_2, rug_fuzz_3),
            (-rug_fuzz_4, -rug_fuzz_5),
            (f64::INFINITY, f64::INFINITY),
            (f64::NEG_INFINITY, f64::NEG_INFINITY),
        ];
        for &(input, expected) in values.iter() {
            let result = input.asinh();
            debug_assert!(
                (result - expected).abs() < rug_fuzz_6, "Testing value: {}", input
            );
        }
        let _rug_ed_tests_llm_16_196_llm_16_196_rrrruuuugggg_test_asinh = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_197_llm_16_197 {
    use crate::real::Real;
    #[test]
    fn test_atan() {
        let _rug_st_tests_llm_16_197_llm_16_197_rrrruuuugggg_test_atan = 0;
        let rug_fuzz_0 = 1.0;
        let rug_fuzz_1 = 1e-10;
        let value: f64 = rug_fuzz_0;
        let result = value.atan();
        debug_assert!((result - std::f64::consts::FRAC_PI_4).abs() < rug_fuzz_1);
        let _rug_ed_tests_llm_16_197_llm_16_197_rrrruuuugggg_test_atan = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_198_llm_16_198 {
    use crate::real::Real;
    #[test]
    fn test_atan2() {
        let _rug_st_tests_llm_16_198_llm_16_198_rrrruuuugggg_test_atan2 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 2.0;
        let rug_fuzz_2 = 1e-10;
        let y: f64 = rug_fuzz_0;
        let x: f64 = rug_fuzz_1;
        let result = <f64 as Real>::atan2(y, x);
        let expected = y.atan2(x);
        debug_assert!((result - expected).abs() < rug_fuzz_2);
        let _rug_ed_tests_llm_16_198_llm_16_198_rrrruuuugggg_test_atan2 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_199_llm_16_199 {
    use crate::float::FloatCore;
    use crate::real::Real;
    #[test]
    fn atanh_test_basic() {
        let _rug_st_tests_llm_16_199_llm_16_199_rrrruuuugggg_atanh_test_basic = 0;
        let rug_fuzz_0 = 0.5f64;
        let rug_fuzz_1 = 1.0;
        let rug_fuzz_2 = 2.0;
        let rug_fuzz_3 = 1.0;
        let rug_fuzz_4 = 2.0;
        let rug_fuzz_5 = 1e-10;
        let x = rug_fuzz_0;
        let result = x.atanh();
        let expected = (x + rug_fuzz_1).ln() / rug_fuzz_2
            - (rug_fuzz_3 - x).ln() / rug_fuzz_4;
        debug_assert!((result - expected).abs() < rug_fuzz_5);
        let _rug_ed_tests_llm_16_199_llm_16_199_rrrruuuugggg_atanh_test_basic = 0;
    }
    #[test]
    fn atanh_test_zero() {
        let _rug_st_tests_llm_16_199_llm_16_199_rrrruuuugggg_atanh_test_zero = 0;
        let rug_fuzz_0 = 0.0f64;
        let x = rug_fuzz_0;
        let result = x.atanh();
        debug_assert_eq!(result, 0.0);
        let _rug_ed_tests_llm_16_199_llm_16_199_rrrruuuugggg_atanh_test_zero = 0;
    }
    #[test]
    #[should_panic]
    fn atanh_test_greater_than_one() {
        let _rug_st_tests_llm_16_199_llm_16_199_rrrruuuugggg_atanh_test_greater_than_one = 0;
        let rug_fuzz_0 = 1.1f64;
        let x = rug_fuzz_0;
        let _ = x.atanh();
        let _rug_ed_tests_llm_16_199_llm_16_199_rrrruuuugggg_atanh_test_greater_than_one = 0;
    }
    #[test]
    #[should_panic]
    fn atanh_test_less_than_neg_one() {
        let _rug_st_tests_llm_16_199_llm_16_199_rrrruuuugggg_atanh_test_less_than_neg_one = 0;
        let rug_fuzz_0 = 1.1f64;
        let x = -rug_fuzz_0;
        let _ = x.atanh();
        let _rug_ed_tests_llm_16_199_llm_16_199_rrrruuuugggg_atanh_test_less_than_neg_one = 0;
    }
    #[test]
    fn atanh_test_one() {
        let _rug_st_tests_llm_16_199_llm_16_199_rrrruuuugggg_atanh_test_one = 0;
        let rug_fuzz_0 = 1.0f64;
        let x = rug_fuzz_0;
        debug_assert!(x.atanh().is_infinite());
        let _rug_ed_tests_llm_16_199_llm_16_199_rrrruuuugggg_atanh_test_one = 0;
    }
    #[test]
    fn atanh_test_neg_one() {
        let _rug_st_tests_llm_16_199_llm_16_199_rrrruuuugggg_atanh_test_neg_one = 0;
        let rug_fuzz_0 = 1.0f64;
        let x = -rug_fuzz_0;
        debug_assert!(x.atanh().is_infinite());
        let _rug_ed_tests_llm_16_199_llm_16_199_rrrruuuugggg_atanh_test_neg_one = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_201_llm_16_201 {
    use crate::float::FloatCore;
    #[test]
    fn test_ceil() {
        let _rug_st_tests_llm_16_201_llm_16_201_rrrruuuugggg_test_ceil = 0;
        let rug_fuzz_0 = 1.2f32;
        let rug_fuzz_1 = 1.0f32;
        let rug_fuzz_2 = 1.2f32;
        let rug_fuzz_3 = 1.0f32;
        let rug_fuzz_4 = 0.0f32;
        let rug_fuzz_5 = 1.2f64;
        let rug_fuzz_6 = 1.0f64;
        let rug_fuzz_7 = 1.2f64;
        let rug_fuzz_8 = 1.0f64;
        let rug_fuzz_9 = 0.0f64;
        debug_assert_eq!(rug_fuzz_0.ceil(), 2.0);
        debug_assert_eq!(rug_fuzz_1.ceil(), 1.0);
        debug_assert_eq!((- rug_fuzz_2).ceil(), - 1.0);
        debug_assert_eq!((- rug_fuzz_3).ceil(), - 1.0);
        debug_assert_eq!(rug_fuzz_4.ceil(), 0.0);
        debug_assert_eq!(rug_fuzz_5.ceil(), 2.0);
        debug_assert_eq!(rug_fuzz_6.ceil(), 1.0);
        debug_assert_eq!((- rug_fuzz_7).ceil(), - 1.0);
        debug_assert_eq!((- rug_fuzz_8).ceil(), - 1.0);
        debug_assert_eq!(rug_fuzz_9.ceil(), 0.0);
        let _rug_ed_tests_llm_16_201_llm_16_201_rrrruuuugggg_test_ceil = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_202_llm_16_202 {
    use crate::real::Real;
    #[test]
    fn test_cos() {
        let _rug_st_tests_llm_16_202_llm_16_202_rrrruuuugggg_test_cos = 0;
        let rug_fuzz_0 = 1.0;
        let rug_fuzz_1 = 1e-10;
        let angle_rad: f64 = std::f64::consts::PI;
        let cos_value = angle_rad.cos();
        let expected_value: f64 = -rug_fuzz_0;
        let tolerance: f64 = rug_fuzz_1;
        debug_assert!(
            (cos_value - expected_value).abs() < tolerance,
            "The cos of PI should be -1.0, instead got {}", cos_value
        );
        let _rug_ed_tests_llm_16_202_llm_16_202_rrrruuuugggg_test_cos = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_203_llm_16_203 {
    use crate::real::Real;
    #[test]
    fn cosh_test() {
        let _rug_st_tests_llm_16_203_llm_16_203_rrrruuuugggg_cosh_test = 0;
        let rug_fuzz_0 = 0.0_f64;
        let rug_fuzz_1 = 1.0_f64;
        let rug_fuzz_2 = 0.0_f64;
        let rug_fuzz_3 = 1.0_f64;
        let rug_fuzz_4 = 1.0_f64;
        let rug_fuzz_5 = 1.0_f64;
        let rug_fuzz_6 = 1.0_f64;
        let rug_fuzz_7 = 1.0_f64;
        let rug_fuzz_8 = 0.5_f64;
        let rug_fuzz_9 = 0.5_f64;
        let rug_fuzz_10 = 0.5_f64;
        let rug_fuzz_11 = 0.5_f64;
        let values = [
            (rug_fuzz_0, rug_fuzz_1),
            (-rug_fuzz_2, rug_fuzz_3),
            (rug_fuzz_4, f64::cosh(rug_fuzz_5)),
            (-rug_fuzz_6, f64::cosh(-rug_fuzz_7)),
            (rug_fuzz_8, f64::cosh(rug_fuzz_9)),
            (-rug_fuzz_10, f64::cosh(-rug_fuzz_11)),
        ];
        for &(x, expected) in &values {
            let result = <f64 as Real>::cosh(x);
            let epsilon = f64::EPSILON;
            debug_assert!(
                (result - expected).abs() <= epsilon,
                "cosh({}) failed: got {}, expected {}", x, result, expected
            );
        }
        let _rug_ed_tests_llm_16_203_llm_16_203_rrrruuuugggg_cosh_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_204 {
    use crate::real::Real;
    #[test]
    fn epsilon_test() {
        let _rug_st_tests_llm_16_204_rrrruuuugggg_epsilon_test = 0;
        let eps_f32: f32 = <f32 as Real>::epsilon();
        debug_assert_eq!(eps_f32, f32::EPSILON);
        let eps_f64: f64 = <f64 as Real>::epsilon();
        debug_assert_eq!(eps_f64, f64::EPSILON);
        let _rug_ed_tests_llm_16_204_rrrruuuugggg_epsilon_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_205_llm_16_205 {
    use crate::real::Real;
    #[test]
    fn test_exp() {
        let _rug_st_tests_llm_16_205_llm_16_205_rrrruuuugggg_test_exp = 0;
        let rug_fuzz_0 = 2.0f64;
        let value = rug_fuzz_0;
        let expected = value.exp();
        let result = <f64 as Real>::exp(value);
        debug_assert_eq!(expected, result);
        let _rug_ed_tests_llm_16_205_llm_16_205_rrrruuuugggg_test_exp = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_206_llm_16_206 {
    use crate::real::Real;
    #[test]
    fn test_exp2() {
        let _rug_st_tests_llm_16_206_llm_16_206_rrrruuuugggg_test_exp2 = 0;
        let rug_fuzz_0 = 0.0f32;
        let rug_fuzz_1 = 1.0f32;
        let rug_fuzz_2 = 2.0f32;
        let rug_fuzz_3 = 1.0f32;
        let rug_fuzz_4 = 0.0f64;
        let rug_fuzz_5 = 1.0f64;
        let rug_fuzz_6 = 2.0f64;
        let rug_fuzz_7 = 1.0f64;
        debug_assert_eq!(Real::exp2(rug_fuzz_0), 1.0);
        debug_assert_eq!(Real::exp2(rug_fuzz_1), 2.0);
        debug_assert_eq!(Real::exp2(rug_fuzz_2), 4.0);
        debug_assert_eq!(Real::exp2(- rug_fuzz_3), 0.5);
        debug_assert_eq!(Real::exp2(rug_fuzz_4), 1.0);
        debug_assert_eq!(Real::exp2(rug_fuzz_5), 2.0);
        debug_assert_eq!(Real::exp2(rug_fuzz_6), 4.0);
        debug_assert_eq!(Real::exp2(- rug_fuzz_7), 0.5);
        let _rug_ed_tests_llm_16_206_llm_16_206_rrrruuuugggg_test_exp2 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_207_llm_16_207 {
    use crate::real::Real;
    #[test]
    fn test_exp_m1() {
        let _rug_st_tests_llm_16_207_llm_16_207_rrrruuuugggg_test_exp_m1 = 0;
        let rug_fuzz_0 = 1.0;
        let rug_fuzz_1 = 1.0;
        let rug_fuzz_2 = 1.0;
        let rug_fuzz_3 = 1.0;
        let rug_fuzz_4 = 1.0;
        let rug_fuzz_5 = 1.0;
        let rug_fuzz_6 = 1.0;
        let rug_fuzz_7 = 1.0;
        let rug_fuzz_8 = 0.0;
        let rug_fuzz_9 = 0.0;
        let value_f32: f32 = rug_fuzz_0;
        debug_assert!(
            (value_f32.exp_m1() - (value_f32.exp() - rug_fuzz_1)).abs() <
            std::f32::EPSILON
        );
        let value_f64: f64 = rug_fuzz_2;
        debug_assert!(
            (value_f64.exp_m1() - (value_f64.exp() - rug_fuzz_3)).abs() <
            std::f64::EPSILON
        );
        let value_f32_neg: f32 = -rug_fuzz_4;
        debug_assert!(
            (value_f32_neg.exp_m1() - (value_f32_neg.exp() - rug_fuzz_5)).abs() <
            std::f32::EPSILON
        );
        let value_f64_neg: f64 = -rug_fuzz_6;
        debug_assert!(
            (value_f64_neg.exp_m1() - (value_f64_neg.exp() - rug_fuzz_7)).abs() <
            std::f64::EPSILON
        );
        let value_f32_zero: f32 = rug_fuzz_8;
        debug_assert_eq!(value_f32_zero.exp_m1(), 0.0);
        let value_f64_zero: f64 = rug_fuzz_9;
        debug_assert_eq!(value_f64_zero.exp_m1(), 0.0);
        let _rug_ed_tests_llm_16_207_llm_16_207_rrrruuuugggg_test_exp_m1 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_208_llm_16_208 {
    use crate::real::Real;
    #[test]
    fn test_floor() {
        let _rug_st_tests_llm_16_208_llm_16_208_rrrruuuugggg_test_floor = 0;
        let rug_fuzz_0 = 3.7;
        let rug_fuzz_1 = 3.7;
        let rug_fuzz_2 = 3.7;
        let rug_fuzz_3 = 3.7;
        let rug_fuzz_4 = 0.0;
        let rug_fuzz_5 = 0.0;
        let rug_fuzz_6 = 0.0;
        let rug_fuzz_7 = 0.0;
        debug_assert_eq!(< f32 as Real > ::floor(rug_fuzz_0), 3.0);
        debug_assert_eq!(< f32 as Real > ::floor(- rug_fuzz_1), - 4.0);
        debug_assert_eq!(< f64 as Real > ::floor(rug_fuzz_2), 3.0);
        debug_assert_eq!(< f64 as Real > ::floor(- rug_fuzz_3), - 4.0);
        debug_assert_eq!(< f32 as Real > ::floor(rug_fuzz_4), 0.0);
        debug_assert_eq!(< f32 as Real > ::floor(- rug_fuzz_5), - 0.0);
        debug_assert_eq!(< f64 as Real > ::floor(rug_fuzz_6), 0.0);
        debug_assert_eq!(< f64 as Real > ::floor(- rug_fuzz_7), - 0.0);
        let _rug_ed_tests_llm_16_208_llm_16_208_rrrruuuugggg_test_floor = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_210_llm_16_210 {
    use crate::real::Real;
    #[test]
    fn hypot_test() {
        let _rug_st_tests_llm_16_210_llm_16_210_rrrruuuugggg_hypot_test = 0;
        let rug_fuzz_0 = 3.0;
        let rug_fuzz_1 = 4.0;
        let rug_fuzz_2 = 5.0;
        let rug_fuzz_3 = 1e-10;
        let a: f64 = rug_fuzz_0;
        let b: f64 = rug_fuzz_1;
        let result = a.hypot(b);
        let expected = rug_fuzz_2;
        debug_assert!((result - expected).abs() < rug_fuzz_3);
        let _rug_ed_tests_llm_16_210_llm_16_210_rrrruuuugggg_hypot_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_211_llm_16_211 {
    use crate::real::Real;
    #[test]
    fn test_is_sign_negative() {
        let _rug_st_tests_llm_16_211_llm_16_211_rrrruuuugggg_test_is_sign_negative = 0;
        let rug_fuzz_0 = 1.0f32;
        let rug_fuzz_1 = 1.0f32;
        let rug_fuzz_2 = 0.0f32;
        let rug_fuzz_3 = 1.0f64;
        let rug_fuzz_4 = 1.0f64;
        let rug_fuzz_5 = 0.0f64;
        debug_assert!((- rug_fuzz_0).is_sign_negative());
        debug_assert!(! rug_fuzz_1.is_sign_negative());
        debug_assert!(! rug_fuzz_2.is_sign_negative());
        debug_assert!((- rug_fuzz_3).is_sign_negative());
        debug_assert!(! rug_fuzz_4.is_sign_negative());
        debug_assert!(! rug_fuzz_5.is_sign_negative());
        let _rug_ed_tests_llm_16_211_llm_16_211_rrrruuuugggg_test_is_sign_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_212_llm_16_212 {
    use crate::real::Real;
    #[test]
    fn test_is_sign_positive() {
        let _rug_st_tests_llm_16_212_llm_16_212_rrrruuuugggg_test_is_sign_positive = 0;
        let rug_fuzz_0 = 1.0f32;
        let rug_fuzz_1 = 1.0f64;
        let rug_fuzz_2 = 0.0f32;
        let rug_fuzz_3 = 0.0f64;
        let rug_fuzz_4 = 1.0f32;
        let rug_fuzz_5 = 1.0f64;
        debug_assert!(rug_fuzz_0.is_sign_positive());
        debug_assert!(rug_fuzz_1.is_sign_positive());
        debug_assert!(rug_fuzz_2.is_sign_positive());
        debug_assert!(rug_fuzz_3.is_sign_positive());
        debug_assert!(! (- rug_fuzz_4).is_sign_positive());
        debug_assert!(! (- rug_fuzz_5).is_sign_positive());
        let _rug_ed_tests_llm_16_212_llm_16_212_rrrruuuugggg_test_is_sign_positive = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_213_llm_16_213 {
    use crate::real::Real;
    #[test]
    fn test_ln() {
        let _rug_st_tests_llm_16_213_llm_16_213_rrrruuuugggg_test_ln = 0;
        let rug_fuzz_0 = 1.0f64;
        let rug_fuzz_1 = 0.0f64;
        let rug_fuzz_2 = 2.718281828459045f64;
        let rug_fuzz_3 = 1.0f64;
        let rug_fuzz_4 = 0.0f64;
        let rug_fuzz_5 = 1.0f64;
        let a = rug_fuzz_0;
        let result = a.ln();
        let expected = rug_fuzz_1;
        debug_assert!((result - expected).abs() < f64::EPSILON);
        let b = rug_fuzz_2;
        let result = b.ln();
        let expected = rug_fuzz_3;
        debug_assert!((result - expected).abs() < f64::EPSILON);
        let c = rug_fuzz_4;
        debug_assert!(c.ln().is_infinite() && c.ln().is_sign_negative());
        let d = -rug_fuzz_5;
        debug_assert!(d.ln().is_nan());
        let _rug_ed_tests_llm_16_213_llm_16_213_rrrruuuugggg_test_ln = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_214_llm_16_214 {
    use crate::real::Real;
    #[test]
    fn test_ln_1p() {
        let _rug_st_tests_llm_16_214_llm_16_214_rrrruuuugggg_test_ln_1p = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 1.0;
        let rug_fuzz_2 = 2.0;
        let rug_fuzz_3 = 1e-10;
        let rug_fuzz_4 = 1e-10;
        let x: f64 = rug_fuzz_0;
        debug_assert_eq!(x.ln_1p(), 0.0);
        let x: f64 = rug_fuzz_1;
        let expected = f64::ln(rug_fuzz_2);
        debug_assert!((x.ln_1p() - expected).abs() < rug_fuzz_3);
        let x: f64 = f64::EPSILON;
        debug_assert!((x.ln_1p() - f64::EPSILON).abs() < rug_fuzz_4);
        let _rug_ed_tests_llm_16_214_llm_16_214_rrrruuuugggg_test_ln_1p = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_215_llm_16_215 {
    use crate::real::Real;
    #[test]
    fn test_log() {
        let _rug_st_tests_llm_16_215_llm_16_215_rrrruuuugggg_test_log = 0;
        let rug_fuzz_0 = 10.0;
        let rug_fuzz_1 = 2.0;
        let rug_fuzz_2 = 3.321928094887362;
        let rug_fuzz_3 = 1e-15;
        let value: f64 = rug_fuzz_0;
        let base: f64 = rug_fuzz_1;
        let result = value.log(base);
        let expected = rug_fuzz_2;
        let epsilon = rug_fuzz_3;
        debug_assert!(
            (result - expected).abs() < epsilon,
            "The log function did not return the expected result: expected {} but got {}",
            expected, result
        );
        let _rug_ed_tests_llm_16_215_llm_16_215_rrrruuuugggg_test_log = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_216_llm_16_216 {
    use crate::real::Real;
    #[test]
    fn test_log10() {
        let _rug_st_tests_llm_16_216_llm_16_216_rrrruuuugggg_test_log10 = 0;
        let rug_fuzz_0 = 1.0f32;
        let rug_fuzz_1 = 10.0f32;
        let rug_fuzz_2 = 1.0f64;
        let rug_fuzz_3 = 10.0f64;
        let rug_fuzz_4 = 100.0f64;
        let rug_fuzz_5 = 10.0f32;
        let rug_fuzz_6 = 10.0f64;
        debug_assert_eq!(rug_fuzz_0.log10(), 0.0);
        debug_assert_eq!(rug_fuzz_1.log10(), 1.0);
        debug_assert_eq!(rug_fuzz_2.log10(), 0.0);
        debug_assert_eq!(rug_fuzz_3.log10(), 1.0);
        debug_assert_eq!(rug_fuzz_4.log10(), 2.0);
        debug_assert!(f32::NAN.log10().is_nan());
        debug_assert!(f64::NAN.log10().is_nan());
        debug_assert!(f32::INFINITY.log10().is_infinite());
        debug_assert!(f64::INFINITY.log10().is_infinite());
        debug_assert!(f32::NEG_INFINITY.log10().is_nan());
        debug_assert!(f64::NEG_INFINITY.log10().is_nan());
        debug_assert!((- rug_fuzz_5).log10().is_nan());
        debug_assert!((- rug_fuzz_6).log10().is_nan());
        let _rug_ed_tests_llm_16_216_llm_16_216_rrrruuuugggg_test_log10 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_217_llm_16_217 {
    use crate::real::Real;
    #[test]
    fn test_log2() {
        let _rug_st_tests_llm_16_217_llm_16_217_rrrruuuugggg_test_log2 = 0;
        let rug_fuzz_0 = 2.0f32;
        let rug_fuzz_1 = 1.0f64;
        let rug_fuzz_2 = 32.0f32;
        let rug_fuzz_3 = 64.0f64;
        let rug_fuzz_4 = 1.0f32;
        let rug_fuzz_5 = 1.0f64;
        let rug_fuzz_6 = 0.0f32;
        let rug_fuzz_7 = 0.0f64;
        debug_assert_eq!(rug_fuzz_0.log2(), 1.0);
        debug_assert_eq!(rug_fuzz_1.log2(), 0.0);
        debug_assert!(f32::EPSILON.log2().is_finite());
        debug_assert!(f64::EPSILON.log2().is_finite());
        debug_assert!(f32::MAX.log2().is_finite());
        debug_assert!(f64::MAX.log2().is_finite());
        debug_assert!(f32::MIN_POSITIVE.log2().is_finite());
        debug_assert!(f64::MIN_POSITIVE.log2().is_finite());
        debug_assert_eq!(rug_fuzz_2.log2(), 5.0);
        debug_assert_eq!(rug_fuzz_3.log2(), 6.0);
        debug_assert!(f32::NAN.log2().is_nan());
        debug_assert!(f64::NAN.log2().is_nan());
        debug_assert!(f32::INFINITY.log2().is_infinite());
        debug_assert!(f64::INFINITY.log2().is_infinite());
        debug_assert!(f32::NEG_INFINITY.log2().is_nan());
        debug_assert!(f64::NEG_INFINITY.log2().is_nan());
        debug_assert!((- rug_fuzz_4).log2().is_nan());
        debug_assert!((- rug_fuzz_5).log2().is_nan());
        debug_assert_eq!(rug_fuzz_6.log2(), f32::NEG_INFINITY);
        debug_assert_eq!(rug_fuzz_7.log2(), f64::NEG_INFINITY);
        let _rug_ed_tests_llm_16_217_llm_16_217_rrrruuuugggg_test_log2 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_218_llm_16_218 {
    use crate::real::Real;
    #[test]
    fn test_max() {
        let _rug_st_tests_llm_16_218_llm_16_218_rrrruuuugggg_test_max = 0;
        let rug_fuzz_0 = 5.0_f32;
        let rug_fuzz_1 = 3.0_f32;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 10;
        let rug_fuzz_5 = 20;
        let rug_fuzz_6 = 0.0_f32;
        let rug_fuzz_7 = 0.0_f32;
        let rug_fuzz_8 = 0.0_f32;
        let rug_fuzz_9 = 0.0_f32;
        debug_assert_eq!(rug_fuzz_0.max(rug_fuzz_1), 5.0_f32);
        debug_assert_eq!(rug_fuzz_2.max(rug_fuzz_3), 4);
        debug_assert_eq!((- rug_fuzz_4).max(- rug_fuzz_5), - 10);
        debug_assert_eq!(rug_fuzz_6.max(- rug_fuzz_7), 0.0_f32);
        debug_assert!(f32::NAN.max(rug_fuzz_8).is_nan());
        debug_assert_eq!(rug_fuzz_9.max(f32::NAN), 0.0_f32);
        debug_assert!(f32::NAN.max(f32::NAN).is_nan());
        debug_assert_eq!((- f32::INFINITY).max(f32::INFINITY), f32::INFINITY);
        debug_assert_eq!(f32::INFINITY.max(f32::INFINITY), f32::INFINITY);
        debug_assert_eq!(f32::INFINITY.max(- f32::INFINITY), f32::INFINITY);
        debug_assert_eq!((- f32::INFINITY).max(- f32::INFINITY), - f32::INFINITY);
        let _rug_ed_tests_llm_16_218_llm_16_218_rrrruuuugggg_test_max = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_219_llm_16_219 {
    use crate::real::Real;
    use crate::bounds::Bounded;
    #[test]
    fn test_max_value() {
        let _rug_st_tests_llm_16_219_llm_16_219_rrrruuuugggg_test_max_value = 0;
        debug_assert_eq!(
            < f32 as Bounded > ::max_value(), < f32 as Real > ::max_value()
        );
        debug_assert_eq!(
            < f64 as Bounded > ::max_value(), < f64 as Real > ::max_value()
        );
        let _rug_ed_tests_llm_16_219_llm_16_219_rrrruuuugggg_test_max_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_222_llm_16_222 {
    use crate::real::Real;
    #[test]
    fn test_min_value() {
        let _rug_st_tests_llm_16_222_llm_16_222_rrrruuuugggg_test_min_value = 0;
        debug_assert_eq!(< f32 as Real > ::min_value(), f32::MIN);
        debug_assert_eq!(< f64 as Real > ::min_value(), f64::MIN);
        let _rug_ed_tests_llm_16_222_llm_16_222_rrrruuuugggg_test_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_223_llm_16_223 {
    use crate::real::Real;
    #[test]
    fn test_mul_add() {
        let _rug_st_tests_llm_16_223_llm_16_223_rrrruuuugggg_test_mul_add = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3.0;
        let rug_fuzz_2 = 4.0;
        let a: f64 = rug_fuzz_0;
        let b: f64 = rug_fuzz_1;
        let c: f64 = rug_fuzz_2;
        let result = <f64 as Real>::mul_add(a, b, c);
        debug_assert_eq!(result, 2.0 * 3.0 + 4.0);
        let _rug_ed_tests_llm_16_223_llm_16_223_rrrruuuugggg_test_mul_add = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_224_llm_16_224 {
    use crate::real::Real;
    #[test]
    fn powf_test() {
        let _rug_st_tests_llm_16_224_llm_16_224_rrrruuuugggg_powf_test = 0;
        let rug_fuzz_0 = 2.0f32;
        let rug_fuzz_1 = 8.0f32;
        let rug_fuzz_2 = 3.0f32;
        let rug_fuzz_3 = 1.0;
        let rug_fuzz_4 = 3.0;
        let rug_fuzz_5 = 2.0f32;
        let rug_fuzz_6 = 1e-6;
        let rug_fuzz_7 = 1.0f32;
        let rug_fuzz_8 = 5.5f32;
        let x: f32 = rug_fuzz_0;
        let y: f32 = rug_fuzz_1;
        debug_assert_eq!(x.powf(rug_fuzz_2), 8.0f32);
        debug_assert!((y.powf(rug_fuzz_3 / rug_fuzz_4) - rug_fuzz_5).abs() < rug_fuzz_6);
        debug_assert!((rug_fuzz_7).powf(rug_fuzz_8).is_normal());
        let _rug_ed_tests_llm_16_224_llm_16_224_rrrruuuugggg_powf_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_225_llm_16_225 {
    use crate::real::Real;
    #[test]
    fn powi_test() {
        let _rug_st_tests_llm_16_225_llm_16_225_rrrruuuugggg_powi_test = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 2.0;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 2.0;
        let rug_fuzz_5 = 3;
        let rug_fuzz_6 = 2.0;
        let rug_fuzz_7 = 0;
        let rug_fuzz_8 = 2.0;
        let rug_fuzz_9 = 1;
        let x: f32 = rug_fuzz_0;
        let y = x.powi(rug_fuzz_1);
        debug_assert_eq!(y, 8.0);
        let x: f64 = rug_fuzz_2;
        let y = x.powi(rug_fuzz_3);
        debug_assert_eq!(y, 8.0);
        let x: f32 = -rug_fuzz_4;
        let y = x.powi(rug_fuzz_5);
        debug_assert_eq!(y, - 8.0);
        let x: f32 = rug_fuzz_6;
        let y = x.powi(rug_fuzz_7);
        debug_assert_eq!(y, 1.0);
        let x: f32 = rug_fuzz_8;
        let y = x.powi(rug_fuzz_9);
        debug_assert_eq!(y, 2.0);
        let _rug_ed_tests_llm_16_225_llm_16_225_rrrruuuugggg_powi_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_226_llm_16_226 {
    use crate::real::Real;
    #[test]
    fn recip_test() {
        let _rug_st_tests_llm_16_226_llm_16_226_rrrruuuugggg_recip_test = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 2.0;
        let rug_fuzz_2 = 0.0;
        let rug_fuzz_3 = 2.0;
        let rug_fuzz_4 = 1.0;
        let rug_fuzz_5 = 5.0;
        let value: f32 = rug_fuzz_0;
        let result = value.recip();
        debug_assert_eq!(result, 0.5);
        let value: f64 = rug_fuzz_1;
        let result = value.recip();
        debug_assert_eq!(result, 0.5);
        let value: f32 = rug_fuzz_2;
        let result = value.recip();
        debug_assert!(result.is_infinite());
        let value: f64 = -rug_fuzz_3;
        let result = value.recip();
        debug_assert_eq!(result, - 0.5);
        let value: f32 = rug_fuzz_4;
        let result = value.recip();
        debug_assert_eq!(result, 1.0);
        let value: f64 = rug_fuzz_5;
        let result = value.recip().recip();
        debug_assert_eq!(result, value);
        let _rug_ed_tests_llm_16_226_llm_16_226_rrrruuuugggg_recip_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_227_llm_16_227 {
    use crate::float::FloatCore;
    #[test]
    fn test_round() {
        let _rug_st_tests_llm_16_227_llm_16_227_rrrruuuugggg_test_round = 0;
        let rug_fuzz_0 = 3.0f32;
        let rug_fuzz_1 = 3.3f32;
        let rug_fuzz_2 = 3.5f32;
        let rug_fuzz_3 = 3.7f32;
        let rug_fuzz_4 = 3.3f32;
        let rug_fuzz_5 = 3.5f32;
        let rug_fuzz_6 = 3.7f32;
        let rug_fuzz_7 = 3.0f64;
        let rug_fuzz_8 = 3.3f64;
        let rug_fuzz_9 = 3.5f64;
        let rug_fuzz_10 = 3.7f64;
        let rug_fuzz_11 = 3.3f64;
        let rug_fuzz_12 = 3.5f64;
        let rug_fuzz_13 = 3.7f64;
        debug_assert_eq!(rug_fuzz_0.round(), 3.0f32);
        debug_assert_eq!(rug_fuzz_1.round(), 3.0f32);
        debug_assert_eq!(rug_fuzz_2.round(), 4.0f32);
        debug_assert_eq!(rug_fuzz_3.round(), 4.0f32);
        debug_assert_eq!((- rug_fuzz_4).round(), - 3.0f32);
        debug_assert_eq!((- rug_fuzz_5).round(), - 4.0f32);
        debug_assert_eq!((- rug_fuzz_6).round(), - 4.0f32);
        debug_assert_eq!(rug_fuzz_7.round(), 3.0f64);
        debug_assert_eq!(rug_fuzz_8.round(), 3.0f64);
        debug_assert_eq!(rug_fuzz_9.round(), 4.0f64);
        debug_assert_eq!(rug_fuzz_10.round(), 4.0f64);
        debug_assert_eq!((- rug_fuzz_11).round(), - 3.0f64);
        debug_assert_eq!((- rug_fuzz_12).round(), - 4.0f64);
        debug_assert_eq!((- rug_fuzz_13).round(), - 4.0f64);
        let _rug_ed_tests_llm_16_227_llm_16_227_rrrruuuugggg_test_round = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_228_llm_16_228 {
    use crate::real::Real;
    #[test]
    fn test_signum() {
        let _rug_st_tests_llm_16_228_llm_16_228_rrrruuuugggg_test_signum = 0;
        let rug_fuzz_0 = 5f32;
        let rug_fuzz_1 = 0f32;
        let rug_fuzz_2 = 5f32;
        let rug_fuzz_3 = 5f64;
        let rug_fuzz_4 = 0f64;
        let rug_fuzz_5 = 5f64;
        debug_assert_eq!(rug_fuzz_0.signum(), 1f32);
        debug_assert_eq!(rug_fuzz_1.signum(), 0f32);
        debug_assert_eq!((- rug_fuzz_2).signum(), - 1f32);
        debug_assert_eq!(rug_fuzz_3.signum(), 1f64);
        debug_assert_eq!(rug_fuzz_4.signum(), 0f64);
        debug_assert_eq!((- rug_fuzz_5).signum(), - 1f64);
        let _rug_ed_tests_llm_16_228_llm_16_228_rrrruuuugggg_test_signum = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_229_llm_16_229 {
    use crate::real::Real;
    #[test]
    fn test_sin() {
        let _rug_st_tests_llm_16_229_llm_16_229_rrrruuuugggg_test_sin = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 0.0;
        let rug_fuzz_2 = 0.0;
        let rug_fuzz_3 = 2.0;
        let rug_fuzz_4 = 1.0;
        let rug_fuzz_5 = 2.0;
        let rug_fuzz_6 = 1.0;
        let rug_fuzz_7 = 2.0;
        let rug_fuzz_8 = 0.0;
        let value: f64 = rug_fuzz_0;
        let sin_value = Real::sin(value);
        debug_assert!((sin_value - rug_fuzz_1).abs() < f64::EPSILON);
        let value: f64 = std::f64::consts::PI;
        let sin_value = Real::sin(value);
        debug_assert!((sin_value - rug_fuzz_2).abs() < f64::EPSILON);
        let value: f64 = std::f64::consts::PI / rug_fuzz_3;
        let sin_value = Real::sin(value);
        debug_assert!((sin_value - rug_fuzz_4).abs() < f64::EPSILON);
        let value: f64 = -std::f64::consts::PI / rug_fuzz_5;
        let sin_value = Real::sin(value);
        debug_assert!((sin_value - (- rug_fuzz_6)).abs() < f64::EPSILON);
        let value: f64 = rug_fuzz_7 * std::f64::consts::PI;
        let sin_value = Real::sin(value);
        debug_assert!((sin_value - rug_fuzz_8).abs() < f64::EPSILON);
        let _rug_ed_tests_llm_16_229_llm_16_229_rrrruuuugggg_test_sin = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_230_llm_16_230 {
    use crate::real::Real;
    #[test]
    fn test_sin_cos() {
        let _rug_st_tests_llm_16_230_llm_16_230_rrrruuuugggg_test_sin_cos = 0;
        let rug_fuzz_0 = 1.0_f64;
        let rug_fuzz_1 = 1e-10;
        let angle = rug_fuzz_0;
        let (sin_value, cos_value) = angle.sin_cos();
        let epsilon = rug_fuzz_1;
        debug_assert!((sin_value - angle.sin()).abs() < epsilon);
        debug_assert!((cos_value - angle.cos()).abs() < epsilon);
        let _rug_ed_tests_llm_16_230_llm_16_230_rrrruuuugggg_test_sin_cos = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_231_llm_16_231 {
    use crate::real::Real;
    #[test]
    fn sinh_test() {
        let _rug_st_tests_llm_16_231_llm_16_231_rrrruuuugggg_sinh_test = 0;
        let rug_fuzz_0 = 1.0;
        let rug_fuzz_1 = 1e-10;
        let value: f64 = rug_fuzz_0;
        let result = <f64 as Real>::sinh(value);
        let expected = value.sinh();
        debug_assert!((result - expected).abs() < rug_fuzz_1);
        let _rug_ed_tests_llm_16_231_llm_16_231_rrrruuuugggg_sinh_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_232_llm_16_232 {
    use crate::real::Real;
    #[test]
    fn test_sqrt() {
        let _rug_st_tests_llm_16_232_llm_16_232_rrrruuuugggg_test_sqrt = 0;
        let rug_fuzz_0 = 4.0;
        let num = rug_fuzz_0;
        let result = <f64 as Real>::sqrt(num);
        debug_assert_eq!(result, 2.0);
        let _rug_ed_tests_llm_16_232_llm_16_232_rrrruuuugggg_test_sqrt = 0;
    }
    #[test]
    #[should_panic(
        expected = "attempt to calculate the square root of a negative number"
    )]
    fn test_sqrt_negative() {
        let _rug_st_tests_llm_16_232_llm_16_232_rrrruuuugggg_test_sqrt_negative = 0;
        let rug_fuzz_0 = 4.0;
        let num = -rug_fuzz_0;
        let _result = <f64 as Real>::sqrt(num);
        let _rug_ed_tests_llm_16_232_llm_16_232_rrrruuuugggg_test_sqrt_negative = 0;
    }
    #[test]
    fn test_sqrt_zero() {
        let _rug_st_tests_llm_16_232_llm_16_232_rrrruuuugggg_test_sqrt_zero = 0;
        let rug_fuzz_0 = 0.0;
        let num = rug_fuzz_0;
        let result = <f64 as Real>::sqrt(num);
        debug_assert_eq!(result, 0.0);
        let _rug_ed_tests_llm_16_232_llm_16_232_rrrruuuugggg_test_sqrt_zero = 0;
    }
    #[test]
    fn test_sqrt_one() {
        let _rug_st_tests_llm_16_232_llm_16_232_rrrruuuugggg_test_sqrt_one = 0;
        let rug_fuzz_0 = 1.0;
        let num = rug_fuzz_0;
        let result = <f64 as Real>::sqrt(num);
        debug_assert_eq!(result, 1.0);
        let _rug_ed_tests_llm_16_232_llm_16_232_rrrruuuugggg_test_sqrt_one = 0;
    }
    #[test]
    fn test_sqrt_fraction() {
        let _rug_st_tests_llm_16_232_llm_16_232_rrrruuuugggg_test_sqrt_fraction = 0;
        let rug_fuzz_0 = 0.25;
        let rug_fuzz_1 = 0.5;
        let num = rug_fuzz_0;
        let result = <f64 as Real>::sqrt(num);
        debug_assert!((result - rug_fuzz_1).abs() < f64::EPSILON);
        let _rug_ed_tests_llm_16_232_llm_16_232_rrrruuuugggg_test_sqrt_fraction = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_233_llm_16_233 {
    use crate::real::Real;
    #[test]
    fn test_tan() {
        let _rug_st_tests_llm_16_233_llm_16_233_rrrruuuugggg_test_tan = 0;
        let rug_fuzz_0 = 0_f64;
        let rug_fuzz_1 = 1e-10;
        let rug_fuzz_2 = 4.0;
        let rug_fuzz_3 = 1e-10;
        let value = rug_fuzz_0;
        let result = f64::tan(value);
        let expected = value.tan();
        debug_assert!((result - expected).abs() < rug_fuzz_1);
        let value = std::f64::consts::PI / rug_fuzz_2;
        let result = f64::tan(value);
        let expected = value.tan();
        debug_assert!((result - expected).abs() < rug_fuzz_3);
        let _rug_ed_tests_llm_16_233_llm_16_233_rrrruuuugggg_test_tan = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_234_llm_16_234 {
    use crate::real::Real;
    #[test]
    fn test_tanh() {
        let _rug_st_tests_llm_16_234_llm_16_234_rrrruuuugggg_test_tanh = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 1e-10;
        let values: Vec<f64> = vec![- rug_fuzz_0, - 1.0, 0.0, 1.0, 2.0];
        for &val in &values {
            let result = Real::tanh(val);
            let expected = val.tanh();
            debug_assert!(
                (result - expected).abs() < rug_fuzz_1, "Testing value: {}", val
            );
        }
        let _rug_ed_tests_llm_16_234_llm_16_234_rrrruuuugggg_test_tanh = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_235_llm_16_235 {
    use crate::real::Real;
    #[test]
    fn test_to_degrees() {
        let _rug_st_tests_llm_16_235_llm_16_235_rrrruuuugggg_test_to_degrees = 0;
        let rug_fuzz_0 = 1.0f64;
        let rug_fuzz_1 = 180.0;
        let rug_fuzz_2 = 1e-10;
        let radians = rug_fuzz_0;
        let degrees = Real::to_degrees(radians);
        let expected_degrees = radians * rug_fuzz_1 / std::f64::consts::PI;
        debug_assert!((degrees - expected_degrees).abs() < rug_fuzz_2);
        let _rug_ed_tests_llm_16_235_llm_16_235_rrrruuuugggg_test_to_degrees = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_236_llm_16_236 {
    use crate::real::Real;
    #[test]
    fn test_to_radians() {
        let _rug_st_tests_llm_16_236_llm_16_236_rrrruuuugggg_test_to_radians = 0;
        let rug_fuzz_0 = 180.0;
        let rug_fuzz_1 = 1e-10;
        let rug_fuzz_2 = 90.0;
        let rug_fuzz_3 = 2.0;
        let degrees: f64 = rug_fuzz_0;
        let radians = degrees.to_radians();
        let expected_radians: f64 = std::f64::consts::PI;
        let epsilon = rug_fuzz_1;
        debug_assert!(
            (radians - expected_radians).abs() < epsilon,
            "Conversion to radians is incorrect: expected {}, got {}", expected_radians,
            radians
        );
        let degrees: f32 = rug_fuzz_2;
        let radians = degrees.to_radians();
        let expected_radians: f32 = std::f32::consts::PI / rug_fuzz_3;
        debug_assert!(
            (radians - expected_radians).abs() < epsilon as f32,
            "Conversion to radians is incorrect: expected {}, got {}", expected_radians,
            radians
        );
        let _rug_ed_tests_llm_16_236_llm_16_236_rrrruuuugggg_test_to_radians = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_237_llm_16_237 {
    use crate::real::Real;
    #[test]
    fn test_trunc() {
        let _rug_st_tests_llm_16_237_llm_16_237_rrrruuuugggg_test_trunc = 0;
        let rug_fuzz_0 = 3.9999_f64;
        let rug_fuzz_1 = 3.0_f64;
        let rug_fuzz_2 = 2.9999_f64;
        let rug_fuzz_3 = 2.0_f64;
        let rug_fuzz_4 = 0.0_f64;
        let a = rug_fuzz_0;
        let b = rug_fuzz_1;
        debug_assert_eq!(a.trunc(), b);
        let c = -rug_fuzz_2;
        let d = -rug_fuzz_3;
        debug_assert_eq!(c.trunc(), d);
        let e = rug_fuzz_4;
        debug_assert_eq!(e.trunc(), e);
        let _rug_ed_tests_llm_16_237_llm_16_237_rrrruuuugggg_test_trunc = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_311_llm_16_311 {
    use crate::float::Float;
    #[test]
    fn test_abs() {
        let _rug_st_tests_llm_16_311_llm_16_311_rrrruuuugggg_test_abs = 0;
        let rug_fuzz_0 = 1.0;
        let rug_fuzz_1 = 0.0;
        let rug_fuzz_2 = 1.0;
        let rug_fuzz_3 = 1.0f32;
        debug_assert_eq!(< f32 as Float > ::abs(- rug_fuzz_0), 1.0);
        debug_assert_eq!(< f32 as Float > ::abs(rug_fuzz_1), 0.0);
        debug_assert_eq!(< f32 as Float > ::abs(rug_fuzz_2), 1.0);
        debug_assert_eq!(< f32 as Float > ::abs(- rug_fuzz_3), 1.0f32);
        let _rug_ed_tests_llm_16_311_llm_16_311_rrrruuuugggg_test_abs = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_313_llm_16_313 {
    use crate::Float;
    #[test]
    fn acos_test() {
        let _rug_st_tests_llm_16_313_llm_16_313_rrrruuuugggg_acos_test = 0;
        let rug_fuzz_0 = 1.0;
        let rug_fuzz_1 = 0.0;
        let rug_fuzz_2 = 1.0;
        let rug_fuzz_3 = 2.0;
        let x: f32 = rug_fuzz_0;
        let acos_x = <f32 as Float>::acos(x);
        debug_assert_eq!(acos_x, 0.0);
        let x: f32 = rug_fuzz_1;
        let acos_x = <f32 as Float>::acos(x);
        debug_assert!((acos_x - std::f32::consts::FRAC_PI_2).abs() < f32::EPSILON);
        let x: f32 = -rug_fuzz_2;
        let acos_x = <f32 as Float>::acos(x);
        debug_assert!((acos_x - std::f32::consts::PI).abs() < f32::EPSILON);
        let x: f32 = rug_fuzz_3;
        let acos_x = <f32 as Float>::acos(x);
        debug_assert!(acos_x.is_nan());
        let _rug_ed_tests_llm_16_313_llm_16_313_rrrruuuugggg_acos_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_314_llm_16_314 {
    use crate::float::Float;
    #[test]
    fn acosh_test() {
        let _rug_st_tests_llm_16_314_llm_16_314_rrrruuuugggg_acosh_test = 0;
        let rug_fuzz_0 = 2f32;
        let value = rug_fuzz_0;
        let result = <f32 as Float>::acosh(value);
        let expected = value.acosh();
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_314_llm_16_314_rrrruuuugggg_acosh_test = 0;
    }
    #[test]
    #[should_panic(expected = "acosh domain error")]
    fn acosh_test_out_of_domain() {
        let _rug_st_tests_llm_16_314_llm_16_314_rrrruuuugggg_acosh_test_out_of_domain = 0;
        let rug_fuzz_0 = 0.5f32;
        let value = rug_fuzz_0;
        let _ = <f32 as Float>::acosh(value);
        let _rug_ed_tests_llm_16_314_llm_16_314_rrrruuuugggg_acosh_test_out_of_domain = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_315_llm_16_315 {
    use crate::float::Float;
    #[test]
    fn test_asin() {
        let _rug_st_tests_llm_16_315_llm_16_315_rrrruuuugggg_test_asin = 0;
        let rug_fuzz_0 = 0.0_f32;
        let rug_fuzz_1 = 0.5_f32;
        let rug_fuzz_2 = 1.0_f32;
        let rug_fuzz_3 = 0.5_f32;
        let rug_fuzz_4 = 1.1_f32;
        let rug_fuzz_5 = 1.1_f32;
        let rug_fuzz_6 = 0.0;
        let rug_fuzz_7 = 0.0;
        let x = rug_fuzz_0;
        let y = rug_fuzz_1;
        let z = rug_fuzz_2;
        let w = -rug_fuzz_3;
        let out_of_domain_pos = rug_fuzz_4;
        let out_of_domain_neg = -rug_fuzz_5;
        let result_x = x.asin();
        let result_y = y.asin();
        let result_z = z.asin();
        let result_w = w.asin();
        let result_out_of_domain_pos = out_of_domain_pos.asin();
        let result_out_of_domain_neg = out_of_domain_neg.asin();
        debug_assert_eq!(result_x, 0.0);
        debug_assert!(result_y > rug_fuzz_6);
        debug_assert_eq!(result_z, std::f32::consts::FRAC_PI_2);
        debug_assert!(result_w < rug_fuzz_7);
        debug_assert!(result_out_of_domain_pos.is_nan());
        debug_assert!(result_out_of_domain_neg.is_nan());
        let _rug_ed_tests_llm_16_315_llm_16_315_rrrruuuugggg_test_asin = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_316_llm_16_316 {
    use crate::float::Float;
    #[test]
    fn asinh_test() {
        let _rug_st_tests_llm_16_316_llm_16_316_rrrruuuugggg_asinh_test = 0;
        let rug_fuzz_0 = 0.0_f32;
        let rug_fuzz_1 = 0.0_f32;
        let rug_fuzz_2 = 1.0_f32;
        let rug_fuzz_3 = 0.881373587019543_f32;
        let rug_fuzz_4 = 1.0_f32;
        let rug_fuzz_5 = 0.881373587019543_f32;
        let rug_fuzz_6 = 1.725382558852315_f32;
        let values = [
            (rug_fuzz_0, rug_fuzz_1),
            (rug_fuzz_2, rug_fuzz_3),
            (-rug_fuzz_4, -rug_fuzz_5),
            (std::f32::consts::E, rug_fuzz_6),
        ];
        for (input, expected) in values.iter() {
            let result = input.asinh();
            debug_assert!(
                (result - expected).abs() <= std::f32::EPSILON,
                "asinh({}) = {}, expected {}", input, result, expected
            );
        }
        let _rug_ed_tests_llm_16_316_llm_16_316_rrrruuuugggg_asinh_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_317_llm_16_317 {
    use crate::float::Float;
    #[test]
    fn test_atan() {
        let _rug_st_tests_llm_16_317_llm_16_317_rrrruuuugggg_test_atan = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 1.0;
        let rug_fuzz_2 = 1.0;
        debug_assert_eq!(< f32 as Float > ::atan(rug_fuzz_0), 0.0);
        debug_assert!(
            (< f32 as Float > ::atan(rug_fuzz_1) - std::f32::consts::FRAC_PI_4).abs() <
            f32::EPSILON
        );
        debug_assert!(
            (< f32 as Float > ::atan(- rug_fuzz_2) + std::f32::consts::FRAC_PI_4).abs() <
            f32::EPSILON
        );
        let _rug_ed_tests_llm_16_317_llm_16_317_rrrruuuugggg_test_atan = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_318_llm_16_318 {
    use crate::float::Float;
    #[test]
    fn test_f32_atan2() {
        let _rug_st_tests_llm_16_318_llm_16_318_rrrruuuugggg_test_f32_atan2 = 0;
        let rug_fuzz_0 = 2.0f32;
        let rug_fuzz_1 = 2.0f32;
        let rug_fuzz_2 = 1e-6;
        let rug_fuzz_3 = 0.0f32;
        let rug_fuzz_4 = 1.0f32;
        let rug_fuzz_5 = 1.0f32;
        let rug_fuzz_6 = 0.0f32;
        let rug_fuzz_7 = 0.0f32;
        let rug_fuzz_8 = 1.0f32;
        let rug_fuzz_9 = 1.0f32;
        let rug_fuzz_10 = 0.0f32;
        let rug_fuzz_11 = 0.0f32;
        let rug_fuzz_12 = 0.0f32;
        let y = rug_fuzz_0;
        let x = rug_fuzz_1;
        let result = <f32 as Float>::atan2(y, x);
        let expected = y.atan2(x);
        debug_assert!(
            (result - expected).abs() < rug_fuzz_2, "atan2 did not match expected value"
        );
        let y = rug_fuzz_3;
        let x = rug_fuzz_4;
        debug_assert_eq!(< f32 as Float > ::atan2(y, x), 0.0);
        let y = rug_fuzz_5;
        let x = rug_fuzz_6;
        debug_assert_eq!(< f32 as Float > ::atan2(y, x), std::f32::consts::FRAC_PI_2);
        let y = rug_fuzz_7;
        let x = -rug_fuzz_8;
        debug_assert_eq!(< f32 as Float > ::atan2(y, x), std::f32::consts::PI);
        let y = -rug_fuzz_9;
        let x = rug_fuzz_10;
        debug_assert_eq!(< f32 as Float > ::atan2(y, x), - std::f32::consts::FRAC_PI_2);
        let y = f32::INFINITY;
        let x = f32::INFINITY;
        debug_assert_eq!(< f32 as Float > ::atan2(y, x), std::f32::consts::FRAC_PI_4);
        let y = f32::INFINITY;
        let x = -f32::INFINITY;
        debug_assert_eq!(
            < f32 as Float > ::atan2(y, x), 3.0 * std::f32::consts::FRAC_PI_4
        );
        let y = -f32::INFINITY;
        let x = f32::INFINITY;
        debug_assert_eq!(< f32 as Float > ::atan2(y, x), - std::f32::consts::FRAC_PI_4);
        let y = -f32::INFINITY;
        let x = -f32::INFINITY;
        debug_assert_eq!(
            < f32 as Float > ::atan2(y, x), - 3.0 * std::f32::consts::FRAC_PI_4
        );
        let y = rug_fuzz_11;
        let x = f32::NAN;
        debug_assert!(< f32 as Float > ::atan2(y, x).is_nan());
        let y = f32::NAN;
        let x = rug_fuzz_12;
        debug_assert!(< f32 as Float > ::atan2(y, x).is_nan());
        let _rug_ed_tests_llm_16_318_llm_16_318_rrrruuuugggg_test_f32_atan2 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_319_llm_16_319 {
    use crate::float::Float;
    #[test]
    fn atanh_test() {
        let _rug_st_tests_llm_16_319_llm_16_319_rrrruuuugggg_atanh_test = 0;
        let rug_fuzz_0 = 0.0f32;
        let rug_fuzz_1 = 0.5f32;
        let rug_fuzz_2 = 0.5f32;
        let value1 = rug_fuzz_0;
        let value2 = rug_fuzz_1;
        let value3 = -rug_fuzz_2;
        let result1 = <f32 as Float>::atanh(value1);
        let result2 = <f32 as Float>::atanh(value2);
        let result3 = <f32 as Float>::atanh(value3);
        let expected_result1 = value1.atanh();
        let expected_result2 = value2.atanh();
        let expected_result3 = value3.atanh();
        debug_assert!((result1 - expected_result1).abs() < f32::EPSILON);
        debug_assert!((result2 - expected_result2).abs() < f32::EPSILON);
        debug_assert!((result3 - expected_result3).abs() < f32::EPSILON);
        let _rug_ed_tests_llm_16_319_llm_16_319_rrrruuuugggg_atanh_test = 0;
    }
    #[test]
    #[should_panic]
    fn atanh_test_panic1() {
        let _rug_st_tests_llm_16_319_llm_16_319_rrrruuuugggg_atanh_test_panic1 = 0;
        let rug_fuzz_0 = 2.0f32;
        let value = rug_fuzz_0;
        let _result = <f32 as Float>::atanh(value);
        let _rug_ed_tests_llm_16_319_llm_16_319_rrrruuuugggg_atanh_test_panic1 = 0;
    }
    #[test]
    #[should_panic]
    fn atanh_test_panic2() {
        let _rug_st_tests_llm_16_319_llm_16_319_rrrruuuugggg_atanh_test_panic2 = 0;
        let rug_fuzz_0 = 2.0f32;
        let value = -rug_fuzz_0;
        let _result = <f32 as Float>::atanh(value);
        let _rug_ed_tests_llm_16_319_llm_16_319_rrrruuuugggg_atanh_test_panic2 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_320_llm_16_320 {
    use crate::float::FloatCore;
    #[test]
    fn test_cbrt_positive() {
        let _rug_st_tests_llm_16_320_llm_16_320_rrrruuuugggg_test_cbrt_positive = 0;
        let rug_fuzz_0 = 8.0;
        let x: f32 = rug_fuzz_0;
        debug_assert_eq!(x.cbrt(), 2.0);
        let _rug_ed_tests_llm_16_320_llm_16_320_rrrruuuugggg_test_cbrt_positive = 0;
    }
    #[test]
    fn test_cbrt_negative() {
        let _rug_st_tests_llm_16_320_llm_16_320_rrrruuuugggg_test_cbrt_negative = 0;
        let rug_fuzz_0 = 8.0;
        let x: f32 = -rug_fuzz_0;
        debug_assert_eq!(x.cbrt(), - 2.0);
        let _rug_ed_tests_llm_16_320_llm_16_320_rrrruuuugggg_test_cbrt_negative = 0;
    }
    #[test]
    fn test_cbrt_zero() {
        let _rug_st_tests_llm_16_320_llm_16_320_rrrruuuugggg_test_cbrt_zero = 0;
        let rug_fuzz_0 = 0.0;
        let x: f32 = rug_fuzz_0;
        debug_assert_eq!(x.cbrt(), 0.0);
        let _rug_ed_tests_llm_16_320_llm_16_320_rrrruuuugggg_test_cbrt_zero = 0;
    }
    #[test]
    fn test_cbrt_one() {
        let _rug_st_tests_llm_16_320_llm_16_320_rrrruuuugggg_test_cbrt_one = 0;
        let rug_fuzz_0 = 1.0;
        let x: f32 = rug_fuzz_0;
        debug_assert_eq!(x.cbrt(), 1.0);
        let _rug_ed_tests_llm_16_320_llm_16_320_rrrruuuugggg_test_cbrt_one = 0;
    }
    #[test]
    fn test_cbrt_subunitary() {
        let _rug_st_tests_llm_16_320_llm_16_320_rrrruuuugggg_test_cbrt_subunitary = 0;
        let rug_fuzz_0 = 0.125;
        let x: f32 = rug_fuzz_0;
        debug_assert_eq!(x.cbrt(), 0.5);
        let _rug_ed_tests_llm_16_320_llm_16_320_rrrruuuugggg_test_cbrt_subunitary = 0;
    }
    #[test]
    fn test_cbrt_very_small() {
        let _rug_st_tests_llm_16_320_llm_16_320_rrrruuuugggg_test_cbrt_very_small = 0;
        let rug_fuzz_0 = 1e-9;
        let rug_fuzz_1 = 0.0;
        let x: f32 = rug_fuzz_0;
        let cbrt_x = x.cbrt();
        debug_assert!(
            cbrt_x > rug_fuzz_1,
            "Cbrt of a very small positive number should be positive"
        );
        let _rug_ed_tests_llm_16_320_llm_16_320_rrrruuuugggg_test_cbrt_very_small = 0;
    }
    #[test]
    fn test_cbrt_very_large() {
        let _rug_st_tests_llm_16_320_llm_16_320_rrrruuuugggg_test_cbrt_very_large = 0;
        let rug_fuzz_0 = 1e9;
        let rug_fuzz_1 = 0.0;
        let x: f32 = rug_fuzz_0;
        let cbrt_x = x.cbrt();
        debug_assert!(
            cbrt_x > rug_fuzz_1,
            "Cbrt of a very large positive number should be positive"
        );
        let _rug_ed_tests_llm_16_320_llm_16_320_rrrruuuugggg_test_cbrt_very_large = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_321_llm_16_321 {
    use crate::float::Float;
    #[test]
    fn test_ceil() {
        let _rug_st_tests_llm_16_321_llm_16_321_rrrruuuugggg_test_ceil = 0;
        let rug_fuzz_0 = 3.7;
        let rug_fuzz_1 = 3.3;
        let rug_fuzz_2 = 4.0;
        let rug_fuzz_3 = 0.0;
        let rug_fuzz_4 = 0.0;
        debug_assert_eq!(< f32 as Float > ::ceil(- rug_fuzz_0), - 3.0);
        debug_assert_eq!(< f32 as Float > ::ceil(rug_fuzz_1), 4.0);
        debug_assert_eq!(< f32 as Float > ::ceil(rug_fuzz_2), 4.0);
        debug_assert_eq!(< f32 as Float > ::ceil(rug_fuzz_3), 0.0);
        debug_assert_eq!(< f32 as Float > ::ceil(- rug_fuzz_4), - 0.0);
        debug_assert_eq!(< f32 as Float > ::ceil(f32::INFINITY), f32::INFINITY);
        debug_assert_eq!(< f32 as Float > ::ceil(f32::NEG_INFINITY), f32::NEG_INFINITY);
        debug_assert!(< f32 as Float > ::ceil(f32::NAN).is_nan());
        let _rug_ed_tests_llm_16_321_llm_16_321_rrrruuuugggg_test_ceil = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_322_llm_16_322 {
    use crate::float::Float;
    use std::num::FpCategory::*;
    #[test]
    fn test_classify() {
        let _rug_st_tests_llm_16_322_llm_16_322_rrrruuuugggg_test_classify = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 0.0;
        let rug_fuzz_2 = 1.0;
        let rug_fuzz_3 = 1.0;
        let rug_fuzz_4 = 1.0e-45;
        let rug_fuzz_5 = 1.0e-45;
        debug_assert_eq!(< f32 as Float > ::classify(rug_fuzz_0), Zero);
        debug_assert_eq!(< f32 as Float > ::classify(- rug_fuzz_1), Zero);
        debug_assert_eq!(< f32 as Float > ::classify(rug_fuzz_2), Normal);
        debug_assert_eq!(< f32 as Float > ::classify(- rug_fuzz_3), Normal);
        debug_assert_eq!(< f32 as Float > ::classify(rug_fuzz_4), Subnormal);
        debug_assert_eq!(< f32 as Float > ::classify(- rug_fuzz_5), Subnormal);
        debug_assert_eq!(< f32 as Float > ::classify(f32::INFINITY), Infinite);
        debug_assert_eq!(< f32 as Float > ::classify(f32::NEG_INFINITY), Infinite);
        debug_assert_eq!(< f32 as Float > ::classify(f32::NAN), Nan);
        let _rug_ed_tests_llm_16_322_llm_16_322_rrrruuuugggg_test_classify = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_324_llm_16_324 {
    use crate::float::Float;
    #[test]
    fn test_cos() {
        let _rug_st_tests_llm_16_324_llm_16_324_rrrruuuugggg_test_cos = 0;
        let rug_fuzz_0 = 1.0;
        let angle_rad = std::f32::consts::PI;
        let cos_value = <f32 as Float>::cos(angle_rad);
        debug_assert!((cos_value - (- rug_fuzz_0)).abs() < std::f32::EPSILON);
        let _rug_ed_tests_llm_16_324_llm_16_324_rrrruuuugggg_test_cos = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_325_llm_16_325 {
    use crate::float::Float;
    #[test]
    fn test_cosh() {
        let _rug_st_tests_llm_16_325_llm_16_325_rrrruuuugggg_test_cosh = 0;
        let rug_fuzz_0 = 1f32;
        let value = rug_fuzz_0;
        let result = <f32 as Float>::cosh(value);
        let expected = value.cosh();
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_325_llm_16_325_rrrruuuugggg_test_cosh = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_326_llm_16_326 {
    use crate::float::Float;
    #[test]
    fn test_epsilon_f32() {
        let _rug_st_tests_llm_16_326_llm_16_326_rrrruuuugggg_test_epsilon_f32 = 0;
        let eps = f32::epsilon();
        debug_assert_eq!(eps, std::f32::EPSILON);
        let _rug_ed_tests_llm_16_326_llm_16_326_rrrruuuugggg_test_epsilon_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_327_llm_16_327 {
    use crate::float::Float;
    #[test]
    fn exp_test() {
        let _rug_st_tests_llm_16_327_llm_16_327_rrrruuuugggg_exp_test = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 0.0;
        let rug_fuzz_2 = 1.0;
        let rug_fuzz_3 = 1.0;
        let rug_fuzz_4 = 1.0;
        let rug_fuzz_5 = 0.0;
        let value: f32 = rug_fuzz_0;
        let result = value.exp();
        let expected = value.exp();
        debug_assert_eq!(result, expected);
        let zero: f32 = rug_fuzz_1;
        debug_assert_eq!(zero.exp(), 1.0);
        let one: f32 = rug_fuzz_2;
        debug_assert_eq!(one.exp(), one.exp());
        let neg: f32 = -rug_fuzz_3;
        let exp_neg = neg.exp();
        debug_assert!(exp_neg < rug_fuzz_4 && exp_neg > rug_fuzz_5);
        let inf: f32 = f32::INFINITY;
        debug_assert_eq!(inf.exp(), inf);
        let neg_inf: f32 = f32::NEG_INFINITY;
        debug_assert_eq!(neg_inf.exp(), 0.0);
        let nan: f32 = f32::NAN;
        debug_assert!(nan.exp().is_nan());
        let _rug_ed_tests_llm_16_327_llm_16_327_rrrruuuugggg_exp_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_328_llm_16_328 {
    use crate::float::Float;
    #[test]
    fn exp2_test() {
        let _rug_st_tests_llm_16_328_llm_16_328_rrrruuuugggg_exp2_test = 0;
        let rug_fuzz_0 = 2.0;
        let value: f32 = rug_fuzz_0;
        let result = <f32 as Float>::exp2(value);
        let expected = f32::exp2(value);
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_328_llm_16_328_rrrruuuugggg_exp2_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_329_llm_16_329 {
    use crate::float::Float;
    #[test]
    fn test_f32_exp_m1() {
        let _rug_st_tests_llm_16_329_llm_16_329_rrrruuuugggg_test_f32_exp_m1 = 0;
        let rug_fuzz_0 = 1.0;
        let value: f32 = rug_fuzz_0;
        let result = <f32 as Float>::exp_m1(value);
        let expected = value.exp_m1();
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_329_llm_16_329_rrrruuuugggg_test_f32_exp_m1 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_330_llm_16_330 {
    use crate::float::Float;
    #[test]
    fn floor_test() {
        let _rug_st_tests_llm_16_330_llm_16_330_rrrruuuugggg_floor_test = 0;
        let rug_fuzz_0 = 3.6f32;
        let num = rug_fuzz_0;
        let result = <f32 as Float>::floor(num);
        debug_assert_eq!(result, 3.0f32);
        let _rug_ed_tests_llm_16_330_llm_16_330_rrrruuuugggg_floor_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_331_llm_16_331 {
    use crate::float::Float;
    #[test]
    fn fract_test() {
        let _rug_st_tests_llm_16_331_llm_16_331_rrrruuuugggg_fract_test = 0;
        let rug_fuzz_0 = 3.5f32;
        let rug_fuzz_1 = 3.5f32;
        let rug_fuzz_2 = 0.0f32;
        let rug_fuzz_3 = 0.0f32;
        let rug_fuzz_4 = 1.0f32;
        let rug_fuzz_5 = 1.0f32;
        let rug_fuzz_6 = 1.0f32;
        let rug_fuzz_7 = 0.0;
        let rug_fuzz_8 = 1.0f32;
        let rug_fuzz_9 = 0.0;
        let rug_fuzz_10 = 12345678.0f32;
        debug_assert_eq!(rug_fuzz_0.fract(), 0.5);
        debug_assert_eq!((- rug_fuzz_1).fract(), - 0.5);
        debug_assert_eq!(rug_fuzz_2.fract(), 0.0);
        debug_assert_eq!((- rug_fuzz_3).fract(), - 0.0);
        debug_assert_eq!(rug_fuzz_4.fract(), 0.0);
        debug_assert_eq!((- rug_fuzz_5).fract(), - 0.0);
        debug_assert!((rug_fuzz_6 / rug_fuzz_7).fract().is_nan());
        debug_assert!(((- rug_fuzz_8) / rug_fuzz_9).fract().is_nan());
        debug_assert_eq!(rug_fuzz_10.fract(), 0.0);
        let _rug_ed_tests_llm_16_331_llm_16_331_rrrruuuugggg_fract_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_332_llm_16_332 {
    use crate::float::Float;
    #[test]
    fn hypot_test() {
        let _rug_st_tests_llm_16_332_llm_16_332_rrrruuuugggg_hypot_test = 0;
        let rug_fuzz_0 = 3.0;
        let rug_fuzz_1 = 4.0;
        let x: f32 = rug_fuzz_0;
        let y: f32 = rug_fuzz_1;
        let result = <f32 as Float>::hypot(x, y);
        debug_assert_eq!(result, 5.0);
        let _rug_ed_tests_llm_16_332_llm_16_332_rrrruuuugggg_hypot_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_333 {
    use super::*;
    use crate::*;
    #[test]
    fn test_infinity() {
        let _rug_st_tests_llm_16_333_rrrruuuugggg_test_infinity = 0;
        let inf: f32 = Float::infinity();
        debug_assert!(inf.is_infinite());
        debug_assert!(! inf.is_finite());
        debug_assert!(inf.is_sign_positive());
        debug_assert!(! inf.is_sign_negative());
        let _rug_ed_tests_llm_16_333_rrrruuuugggg_test_infinity = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_335_llm_16_335 {
    #[test]
    fn test_f32_is_finite() {
        let _rug_st_tests_llm_16_335_llm_16_335_rrrruuuugggg_test_f32_is_finite = 0;
        let rug_fuzz_0 = 0.0_f32;
        let rug_fuzz_1 = 1.0_f32;
        let rug_fuzz_2 = 1.0_f32;
        debug_assert!((rug_fuzz_0).is_finite());
        debug_assert!((rug_fuzz_1).is_finite());
        debug_assert!((- rug_fuzz_2).is_finite());
        debug_assert!((f32::MIN).is_finite());
        debug_assert!((f32::MAX).is_finite());
        debug_assert!((f32::EPSILON).is_finite());
        debug_assert!(! (f32::NAN).is_finite());
        debug_assert!(! (f32::INFINITY).is_finite());
        debug_assert!(! (f32::NEG_INFINITY).is_finite());
        let _rug_ed_tests_llm_16_335_llm_16_335_rrrruuuugggg_test_f32_is_finite = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_336_llm_16_336 {
    use crate::float::Float;
    #[test]
    fn test_is_infinite() {
        let _rug_st_tests_llm_16_336_llm_16_336_rrrruuuugggg_test_is_infinite = 0;
        let rug_fuzz_0 = 42f32;
        let rug_fuzz_1 = 0f32;
        let pos_inf = std::f32::INFINITY;
        let neg_inf = std::f32::NEG_INFINITY;
        let nan = std::f32::NAN;
        let normal = rug_fuzz_0;
        let zero = rug_fuzz_1;
        debug_assert!(pos_inf.is_infinite());
        debug_assert!(neg_inf.is_infinite());
        debug_assert!(! nan.is_infinite());
        debug_assert!(! normal.is_infinite());
        debug_assert!(! zero.is_infinite());
        let _rug_ed_tests_llm_16_336_llm_16_336_rrrruuuugggg_test_is_infinite = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_337_llm_16_337 {
    use crate::float::Float;
    #[test]
    fn test_is_nan() {
        let _rug_st_tests_llm_16_337_llm_16_337_rrrruuuugggg_test_is_nan = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 1.0;
        let rug_fuzz_2 = 1.0;
        debug_assert!(! < f32 as Float > ::is_nan(rug_fuzz_0));
        debug_assert!(! < f32 as Float > ::is_nan(rug_fuzz_1));
        debug_assert!(! < f32 as Float > ::is_nan(- rug_fuzz_2));
        debug_assert!(! < f32 as Float > ::is_nan(f32::INFINITY));
        debug_assert!(! < f32 as Float > ::is_nan(f32::NEG_INFINITY));
        debug_assert!(< f32 as Float > ::is_nan(f32::NAN));
        let _rug_ed_tests_llm_16_337_llm_16_337_rrrruuuugggg_test_is_nan = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_338_llm_16_338 {
    use crate::float::Float;
    #[test]
    fn test_is_normal() {
        let _rug_st_tests_llm_16_338_llm_16_338_rrrruuuugggg_test_is_normal = 0;
        let rug_fuzz_0 = 0.0_f32;
        let rug_fuzz_1 = 0.0_f32;
        let rug_fuzz_2 = 1.0_f32;
        let rug_fuzz_3 = 1.0e-40_f32;
        let nan: f32 = f32::NAN;
        let infinity: f32 = f32::INFINITY;
        let negative_infinity: f32 = f32::NEG_INFINITY;
        let zero: f32 = rug_fuzz_0;
        let negative_zero: f32 = -rug_fuzz_1;
        let normal_number: f32 = rug_fuzz_2;
        let subnormal_number: f32 = rug_fuzz_3;
        debug_assert!(! nan.is_normal(), "NaN should not be normal");
        debug_assert!(! infinity.is_normal(), "Infinity should not be normal");
        debug_assert!(
            ! negative_infinity.is_normal(), "Negative infinity should not be normal"
        );
        debug_assert!(! zero.is_normal(), "Zero should not be normal");
        debug_assert!(! negative_zero.is_normal(), "Negative zero should not be normal");
        debug_assert!(
            normal_number.is_normal(), "Regular floating numbers should be normal"
        );
        debug_assert!(
            ! subnormal_number.is_normal(), "Subnormal numbers should not be normal"
        );
        let _rug_ed_tests_llm_16_338_llm_16_338_rrrruuuugggg_test_is_normal = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_339_llm_16_339 {
    use crate::float::Float;
    #[test]
    fn test_is_sign_negative() {
        let _rug_st_tests_llm_16_339_llm_16_339_rrrruuuugggg_test_is_sign_negative = 0;
        let rug_fuzz_0 = 1.0;
        let rug_fuzz_1 = 0.0;
        let rug_fuzz_2 = 0.0;
        let rug_fuzz_3 = 1.0;
        debug_assert_eq!(< f32 as Float > ::is_sign_negative(- rug_fuzz_0), true);
        debug_assert_eq!(< f32 as Float > ::is_sign_negative(- rug_fuzz_1), true);
        debug_assert_eq!(< f32 as Float > ::is_sign_negative(rug_fuzz_2), false);
        debug_assert_eq!(< f32 as Float > ::is_sign_negative(rug_fuzz_3), false);
        debug_assert_eq!(< f32 as Float > ::is_sign_negative(f32::NAN), false);
        debug_assert_eq!(< f32 as Float > ::is_sign_negative(f32::INFINITY), false);
        debug_assert_eq!(< f32 as Float > ::is_sign_negative(f32::NEG_INFINITY), true);
        let _rug_ed_tests_llm_16_339_llm_16_339_rrrruuuugggg_test_is_sign_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_340_llm_16_340 {
    use crate::float::Float;
    #[test]
    fn test_is_sign_positive() {
        let _rug_st_tests_llm_16_340_llm_16_340_rrrruuuugggg_test_is_sign_positive = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 3.14;
        let rug_fuzz_2 = 3.14;
        let rug_fuzz_3 = 0.0;
        debug_assert!(f32::is_sign_positive(rug_fuzz_0));
        debug_assert!(f32::is_sign_positive(rug_fuzz_1));
        debug_assert!(f32::is_sign_positive(f32::INFINITY));
        debug_assert!(! f32::is_sign_positive(- rug_fuzz_2));
        debug_assert!(! f32::is_sign_positive(- rug_fuzz_3));
        debug_assert!(! f32::is_sign_positive(f32::NEG_INFINITY));
        let _rug_ed_tests_llm_16_340_llm_16_340_rrrruuuugggg_test_is_sign_positive = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_341_llm_16_341 {
    use crate::float::Float;
    #[test]
    fn test_ln() {
        let _rug_st_tests_llm_16_341_llm_16_341_rrrruuuugggg_test_ln = 0;
        let rug_fuzz_0 = 2.71828;
        let rug_fuzz_1 = 1.0;
        let rug_fuzz_2 = 1.0e-5;
        let value: f32 = rug_fuzz_0;
        let result = <f32 as Float>::ln(value);
        let expected = rug_fuzz_1;
        let tolerance = rug_fuzz_2;
        debug_assert!((result - expected).abs() < tolerance);
        let _rug_ed_tests_llm_16_341_llm_16_341_rrrruuuugggg_test_ln = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_342_llm_16_342 {
    use crate::float::Float;
    #[test]
    fn ln_1p_positive() {
        let _rug_st_tests_llm_16_342_llm_16_342_rrrruuuugggg_ln_1p_positive = 0;
        let rug_fuzz_0 = 0.5f32;
        let rug_fuzz_1 = 1.0;
        let rug_fuzz_2 = 1e-6;
        let x = rug_fuzz_0;
        let result = <f32 as Float>::ln_1p(x);
        let expected = (rug_fuzz_1 + x).ln();
        debug_assert!((result - expected).abs() < rug_fuzz_2);
        let _rug_ed_tests_llm_16_342_llm_16_342_rrrruuuugggg_ln_1p_positive = 0;
    }
    #[test]
    fn ln_1p_zero() {
        let _rug_st_tests_llm_16_342_llm_16_342_rrrruuuugggg_ln_1p_zero = 0;
        let rug_fuzz_0 = 0.0f32;
        let rug_fuzz_1 = 1.0;
        let x = rug_fuzz_0;
        let result = <f32 as Float>::ln_1p(x);
        let expected = (rug_fuzz_1 + x).ln();
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_342_llm_16_342_rrrruuuugggg_ln_1p_zero = 0;
    }
    #[test]
    fn ln_1p_negative() {
        let _rug_st_tests_llm_16_342_llm_16_342_rrrruuuugggg_ln_1p_negative = 0;
        let rug_fuzz_0 = 0.5f32;
        let rug_fuzz_1 = 1.0;
        let rug_fuzz_2 = 1e-6;
        let x = -rug_fuzz_0;
        let result = <f32 as Float>::ln_1p(x);
        let expected = (rug_fuzz_1 + x).ln();
        debug_assert!((result - expected).abs() < rug_fuzz_2);
        let _rug_ed_tests_llm_16_342_llm_16_342_rrrruuuugggg_ln_1p_negative = 0;
    }
    #[test]
    #[should_panic]
    fn ln_1p_edge_case() {
        let _rug_st_tests_llm_16_342_llm_16_342_rrrruuuugggg_ln_1p_edge_case = 0;
        let rug_fuzz_0 = 1.0f32;
        let x = -rug_fuzz_0;
        let result = <f32 as Float>::ln_1p(x);
        debug_assert!(result.is_infinite());
        let _rug_ed_tests_llm_16_342_llm_16_342_rrrruuuugggg_ln_1p_edge_case = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_343_llm_16_343 {
    use super::*;
    use crate::*;
    #[test]
    fn log_base_10() {
        let _rug_st_tests_llm_16_343_llm_16_343_rrrruuuugggg_log_base_10 = 0;
        let rug_fuzz_0 = 10.0;
        let rug_fuzz_1 = 10.0;
        let rug_fuzz_2 = 1.0;
        let value: f32 = rug_fuzz_0;
        let result = <f32 as Float>::log(value, rug_fuzz_1);
        debug_assert!((result - rug_fuzz_2).abs() < f32::EPSILON);
        let _rug_ed_tests_llm_16_343_llm_16_343_rrrruuuugggg_log_base_10 = 0;
    }
    #[test]
    fn log_base_e() {
        let _rug_st_tests_llm_16_343_llm_16_343_rrrruuuugggg_log_base_e = 0;
        let rug_fuzz_0 = 1.0;
        let value: f32 = std::f32::consts::E;
        let result = <f32 as Float>::log(value, std::f32::consts::E);
        debug_assert!((result - rug_fuzz_0).abs() < f32::EPSILON);
        let _rug_ed_tests_llm_16_343_llm_16_343_rrrruuuugggg_log_base_e = 0;
    }
    #[test]
    fn log_base_2() {
        let _rug_st_tests_llm_16_343_llm_16_343_rrrruuuugggg_log_base_2 = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 2.0;
        let rug_fuzz_2 = 1.0;
        let value: f32 = rug_fuzz_0;
        let result = <f32 as Float>::log(value, rug_fuzz_1);
        debug_assert!((result - rug_fuzz_2).abs() < f32::EPSILON);
        let _rug_ed_tests_llm_16_343_llm_16_343_rrrruuuugggg_log_base_2 = 0;
    }
    #[test]
    #[should_panic]
    fn log_zero() {
        let _rug_st_tests_llm_16_343_llm_16_343_rrrruuuugggg_log_zero = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 2.0;
        let value: f32 = rug_fuzz_0;
        let _ = <f32 as Float>::log(value, rug_fuzz_1);
        let _rug_ed_tests_llm_16_343_llm_16_343_rrrruuuugggg_log_zero = 0;
    }
    #[test]
    #[should_panic]
    fn log_negative() {
        let _rug_st_tests_llm_16_343_llm_16_343_rrrruuuugggg_log_negative = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 2.0;
        let value: f32 = -rug_fuzz_0;
        let _ = <f32 as Float>::log(value, rug_fuzz_1);
        let _rug_ed_tests_llm_16_343_llm_16_343_rrrruuuugggg_log_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_344_llm_16_344 {
    use crate::float::Float;
    #[test]
    fn test_log10() {
        let _rug_st_tests_llm_16_344_llm_16_344_rrrruuuugggg_test_log10 = 0;
        let rug_fuzz_0 = 1.0;
        let rug_fuzz_1 = 10.0;
        let rug_fuzz_2 = 100.0;
        let rug_fuzz_3 = 1000.0;
        let rug_fuzz_4 = 0.1;
        let rug_fuzz_5 = 1.0;
        let rug_fuzz_6 = 0.01;
        let rug_fuzz_7 = 2.0;
        let rug_fuzz_8 = 0.001;
        let rug_fuzz_9 = 3.0;
        debug_assert_eq!(< f32 as Float > ::log10(rug_fuzz_0), 0.0);
        debug_assert_eq!(< f32 as Float > ::log10(rug_fuzz_1), 1.0);
        debug_assert_eq!(< f32 as Float > ::log10(rug_fuzz_2), 2.0);
        debug_assert_eq!(< f32 as Float > ::log10(rug_fuzz_3), 3.0);
        debug_assert!(
            (< f32 as Float > ::log10(rug_fuzz_4) - (- rug_fuzz_5)).abs() < f32::EPSILON
        );
        debug_assert!(
            (< f32 as Float > ::log10(rug_fuzz_6) - (- rug_fuzz_7)).abs() < f32::EPSILON
        );
        debug_assert!(
            (< f32 as Float > ::log10(rug_fuzz_8) - (- rug_fuzz_9)).abs() < f32::EPSILON
        );
        let _rug_ed_tests_llm_16_344_llm_16_344_rrrruuuugggg_test_log10 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_345 {
    use crate::float::Float;
    #[test]
    fn test_log2() {
        let _rug_st_tests_llm_16_345_rrrruuuugggg_test_log2 = 0;
        let rug_fuzz_0 = 8.0;
        let value: f32 = rug_fuzz_0;
        let result = f32::log2(value);
        debug_assert_eq!(result, 3.0);
        let _rug_ed_tests_llm_16_345_rrrruuuugggg_test_log2 = 0;
    }
    #[test]
    fn test_log2_one() {
        let _rug_st_tests_llm_16_345_rrrruuuugggg_test_log2_one = 0;
        let rug_fuzz_0 = 1.0;
        let value: f32 = rug_fuzz_0;
        let result = f32::log2(value);
        debug_assert_eq!(result, 0.0);
        let _rug_ed_tests_llm_16_345_rrrruuuugggg_test_log2_one = 0;
    }
    #[test]
    fn test_log2_sub_one() {
        let _rug_st_tests_llm_16_345_rrrruuuugggg_test_log2_sub_one = 0;
        let rug_fuzz_0 = 0.5;
        let rug_fuzz_1 = 1.0;
        let value: f32 = rug_fuzz_0;
        let result = f32::log2(value);
        debug_assert!((result - - rug_fuzz_1).abs() < f32::EPSILON);
        let _rug_ed_tests_llm_16_345_rrrruuuugggg_test_log2_sub_one = 0;
    }
    #[test]
    fn test_log2_nan() {
        let _rug_st_tests_llm_16_345_rrrruuuugggg_test_log2_nan = 0;
        let value: f32 = f32::NAN;
        let result = f32::log2(value);
        debug_assert!(result.is_nan());
        let _rug_ed_tests_llm_16_345_rrrruuuugggg_test_log2_nan = 0;
    }
    #[test]
    fn test_log2_infinity() {
        let _rug_st_tests_llm_16_345_rrrruuuugggg_test_log2_infinity = 0;
        let rug_fuzz_0 = 0.0;
        let value: f32 = f32::INFINITY;
        let result = f32::log2(value);
        debug_assert!(result.is_infinite() && result > rug_fuzz_0);
        let _rug_ed_tests_llm_16_345_rrrruuuugggg_test_log2_infinity = 0;
    }
    #[test]
    fn test_log2_negative() {
        let _rug_st_tests_llm_16_345_rrrruuuugggg_test_log2_negative = 0;
        let rug_fuzz_0 = 1.0;
        let value: f32 = -rug_fuzz_0;
        let result = f32::log2(value);
        debug_assert!(result.is_nan());
        let _rug_ed_tests_llm_16_345_rrrruuuugggg_test_log2_negative = 0;
    }
    #[test]
    fn test_log2_zero() {
        let _rug_st_tests_llm_16_345_rrrruuuugggg_test_log2_zero = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 0.0;
        let value: f32 = rug_fuzz_0;
        let result = f32::log2(value);
        debug_assert!(result.is_infinite() && result < rug_fuzz_1);
        let _rug_ed_tests_llm_16_345_rrrruuuugggg_test_log2_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_346_llm_16_346 {
    use crate::float::Float;
    #[test]
    fn test_max() {
        let _rug_st_tests_llm_16_346_llm_16_346_rrrruuuugggg_test_max = 0;
        let rug_fuzz_0 = 1.0;
        let rug_fuzz_1 = 2.0;
        let rug_fuzz_2 = 2.0;
        let rug_fuzz_3 = 1.0;
        let rug_fuzz_4 = 1.5;
        let rug_fuzz_5 = 1.5;
        let rug_fuzz_6 = 1.0;
        let rug_fuzz_7 = 1.0;
        let rug_fuzz_8 = 1.0;
        let rug_fuzz_9 = 1.0;
        let rug_fuzz_10 = 1.0;
        let rug_fuzz_11 = 1.0;
        debug_assert_eq!(< f32 as Float > ::max(rug_fuzz_0, rug_fuzz_1), 2.0);
        debug_assert_eq!(< f32 as Float > ::max(rug_fuzz_2, rug_fuzz_3), 2.0);
        debug_assert_eq!(< f32 as Float > ::max(rug_fuzz_4, rug_fuzz_5), 1.5);
        debug_assert_eq!(< f32 as Float > ::max(- rug_fuzz_6, rug_fuzz_7), 1.0);
        debug_assert!(< f32 as Float > ::max(f32::NAN, rug_fuzz_8).is_nan());
        debug_assert_eq!(
            < f32 as Float > ::max(rug_fuzz_9, f32::INFINITY), f32::INFINITY
        );
        debug_assert_eq!(
            < f32 as Float > ::max(f32::INFINITY, rug_fuzz_10), f32::INFINITY
        );
        debug_assert_eq!(< f32 as Float > ::max(f32::NEG_INFINITY, rug_fuzz_11), 1.0);
        let _rug_ed_tests_llm_16_346_llm_16_346_rrrruuuugggg_test_max = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_347_llm_16_347 {
    use crate::float::Float;
    #[test]
    fn test_max_value() {
        let _rug_st_tests_llm_16_347_llm_16_347_rrrruuuugggg_test_max_value = 0;
        let max_value = <f32 as Float>::max_value();
        debug_assert_eq!(max_value, std::f32::MAX);
        let _rug_ed_tests_llm_16_347_llm_16_347_rrrruuuugggg_test_max_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_348_llm_16_348 {
    use crate::float::Float;
    #[test]
    fn test_min() {
        let _rug_st_tests_llm_16_348_llm_16_348_rrrruuuugggg_test_min = 0;
        let rug_fuzz_0 = 3.0f32;
        let rug_fuzz_1 = 2.0f32;
        let a = rug_fuzz_0;
        let b = rug_fuzz_1;
        debug_assert_eq!(a.min(b), b);
        debug_assert_eq!(b.min(a), b);
        debug_assert_eq!(a.min(a), a);
        let _rug_ed_tests_llm_16_348_llm_16_348_rrrruuuugggg_test_min = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_349_llm_16_349 {
    use crate::float::Float;
    #[test]
    fn test_min_positive_value() {
        let _rug_st_tests_llm_16_349_llm_16_349_rrrruuuugggg_test_min_positive_value = 0;
        let min_val = <f32 as Float>::min_positive_value();
        debug_assert_eq!(min_val, std::f32::MIN_POSITIVE);
        let _rug_ed_tests_llm_16_349_llm_16_349_rrrruuuugggg_test_min_positive_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_350_llm_16_350 {
    use crate::float::Float;
    #[test]
    fn test_min_value() {
        let _rug_st_tests_llm_16_350_llm_16_350_rrrruuuugggg_test_min_value = 0;
        let min_val: f32 = <f32 as Float>::min_value();
        debug_assert_eq!(min_val, f32::MIN);
        let _rug_ed_tests_llm_16_350_llm_16_350_rrrruuuugggg_test_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_351_llm_16_351 {
    use crate::float::Float;
    #[test]
    fn test_mul_add() {
        let _rug_st_tests_llm_16_351_llm_16_351_rrrruuuugggg_test_mul_add = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3.0;
        let rug_fuzz_2 = 4.0;
        let a: f32 = rug_fuzz_0;
        let b: f32 = rug_fuzz_1;
        let c: f32 = rug_fuzz_2;
        let expected = a * b + c;
        let result = <f32 as Float>::mul_add(a, b, c);
        debug_assert_eq!(expected, result);
        let _rug_ed_tests_llm_16_351_llm_16_351_rrrruuuugggg_test_mul_add = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_352_llm_16_352 {
    use crate::float::Float;
    #[test]
    fn nan_test() {
        let _rug_st_tests_llm_16_352_llm_16_352_rrrruuuugggg_nan_test = 0;
        let nan = <f32 as Float>::nan();
        debug_assert!(nan.is_nan());
        let _rug_ed_tests_llm_16_352_llm_16_352_rrrruuuugggg_nan_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_353_llm_16_353 {
    use crate::float::Float;
    #[test]
    fn neg_infinity_test() {
        let _rug_st_tests_llm_16_353_llm_16_353_rrrruuuugggg_neg_infinity_test = 0;
        debug_assert_eq!(< f32 as Float > ::neg_infinity(), f32::NEG_INFINITY);
        let _rug_ed_tests_llm_16_353_llm_16_353_rrrruuuugggg_neg_infinity_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_354_llm_16_354 {
    use crate::float::Float;
    #[test]
    fn test_neg_zero() {
        let _rug_st_tests_llm_16_354_llm_16_354_rrrruuuugggg_test_neg_zero = 0;
        let rug_fuzz_0 = 0.0f32;
        let neg_zero = -rug_fuzz_0;
        debug_assert!(neg_zero.is_sign_negative());
        debug_assert_eq!(< f32 as Float > ::neg_zero(), neg_zero);
        let _rug_ed_tests_llm_16_354_llm_16_354_rrrruuuugggg_test_neg_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_355_llm_16_355 {
    use crate::float::Float;
    #[test]
    fn powf_test() {
        let _rug_st_tests_llm_16_355_llm_16_355_rrrruuuugggg_powf_test = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3.0;
        let base: f32 = rug_fuzz_0;
        let exponent: f32 = rug_fuzz_1;
        let result = <f32 as Float>::powf(base, exponent);
        let expected = base.powf(exponent);
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_355_llm_16_355_rrrruuuugggg_powf_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_356_llm_16_356 {
    use crate::float::Float;
    #[test]
    fn powi_test() {
        let _rug_st_tests_llm_16_356_llm_16_356_rrrruuuugggg_powi_test = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let x: f32 = rug_fuzz_0;
        let result = <f32 as Float>::powi(x, rug_fuzz_1);
        debug_assert_eq!(result, 8.0);
        let _rug_ed_tests_llm_16_356_llm_16_356_rrrruuuugggg_powi_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_357_llm_16_357 {
    use super::*;
    use crate::*;
    #[test]
    fn test_recip() {
        let _rug_st_tests_llm_16_357_llm_16_357_rrrruuuugggg_test_recip = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 0.5;
        let rug_fuzz_2 = 1.0;
        let rug_fuzz_3 = 1.0;
        let rug_fuzz_4 = 1.0;
        let rug_fuzz_5 = 1.0;
        let rug_fuzz_6 = 0.0;
        let rug_fuzz_7 = 0.0;
        let value: f32 = rug_fuzz_0;
        let expected: f32 = rug_fuzz_1;
        debug_assert_eq!(< f32 as crate ::float::Float > ::recip(value), expected);
        let value: f32 = rug_fuzz_2;
        let expected: f32 = rug_fuzz_3;
        debug_assert_eq!(< f32 as crate ::float::Float > ::recip(value), expected);
        let value: f32 = -rug_fuzz_4;
        let expected: f32 = -rug_fuzz_5;
        debug_assert_eq!(< f32 as crate ::float::Float > ::recip(value), expected);
        let value: f32 = rug_fuzz_6;
        debug_assert!(< f32 as crate ::float::Float > ::recip(value).is_infinite());
        let value: f32 = std::f32::INFINITY;
        let expected: f32 = rug_fuzz_7;
        debug_assert_eq!(< f32 as crate ::float::Float > ::recip(value), expected);
        let _rug_ed_tests_llm_16_357_llm_16_357_rrrruuuugggg_test_recip = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_358_llm_16_358 {
    use crate::float::Float;
    #[test]
    fn test_round() {
        let _rug_st_tests_llm_16_358_llm_16_358_rrrruuuugggg_test_round = 0;
        let rug_fuzz_0 = 3.3;
        let rug_fuzz_1 = 3.5;
        let rug_fuzz_2 = 3.7;
        let rug_fuzz_3 = 3.3;
        let rug_fuzz_4 = 3.5;
        let rug_fuzz_5 = 3.7;
        let rug_fuzz_6 = 0.0;
        let rug_fuzz_7 = 0.0;
        debug_assert_eq!(< f32 as Float > ::round(rug_fuzz_0), 3.0);
        debug_assert_eq!(< f32 as Float > ::round(rug_fuzz_1), 4.0);
        debug_assert_eq!(< f32 as Float > ::round(rug_fuzz_2), 4.0);
        debug_assert_eq!(< f32 as Float > ::round(- rug_fuzz_3), - 3.0);
        debug_assert_eq!(< f32 as Float > ::round(- rug_fuzz_4), - 4.0);
        debug_assert_eq!(< f32 as Float > ::round(- rug_fuzz_5), - 4.0);
        debug_assert_eq!(< f32 as Float > ::round(rug_fuzz_6), 0.0);
        debug_assert_eq!(< f32 as Float > ::round(- rug_fuzz_7), - 0.0);
        let _rug_ed_tests_llm_16_358_llm_16_358_rrrruuuugggg_test_round = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_359_llm_16_359 {
    use crate::float::Float;
    #[test]
    fn test_signum() {
        let _rug_st_tests_llm_16_359_llm_16_359_rrrruuuugggg_test_signum = 0;
        let rug_fuzz_0 = 0.0f32;
        let rug_fuzz_1 = 1.0f32;
        let rug_fuzz_2 = 1.0f32;
        let rug_fuzz_3 = 42.0f32;
        let rug_fuzz_4 = 42.0f32;
        let rug_fuzz_5 = 0.0f32;
        let rug_fuzz_6 = 0.0f32;
        debug_assert_eq!(rug_fuzz_0.signum(), 0.0f32);
        debug_assert_eq!(rug_fuzz_1.signum(), 1.0f32);
        debug_assert_eq!((- rug_fuzz_2).signum(), - 1.0f32);
        debug_assert_eq!(rug_fuzz_3.signum(), 1.0f32);
        debug_assert_eq!((- rug_fuzz_4).signum(), - 1.0f32);
        debug_assert!((- rug_fuzz_5).signum().is_sign_negative());
        debug_assert!(rug_fuzz_6.signum().is_sign_positive());
        let _rug_ed_tests_llm_16_359_llm_16_359_rrrruuuugggg_test_signum = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_360_llm_16_360 {
    use crate::float::Float;
    #[test]
    fn test_sin() {
        let _rug_st_tests_llm_16_360_llm_16_360_rrrruuuugggg_test_sin = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 1.0f32;
        let angle_rad = std::f32::consts::PI / rug_fuzz_0;
        let result = <f32 as Float>::sin(angle_rad);
        let expected = rug_fuzz_1;
        debug_assert!((result - expected).abs() < f32::EPSILON);
        let _rug_ed_tests_llm_16_360_llm_16_360_rrrruuuugggg_test_sin = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_361 {
    use super::*;
    use crate::*;
    #[test]
    fn test_sin_cos() {
        let _rug_st_tests_llm_16_361_rrrruuuugggg_test_sin_cos = 0;
        let rug_fuzz_0 = 4.0;
        let rug_fuzz_1 = 0.70710678118;
        let rug_fuzz_2 = 0.70710678118;
        let rug_fuzz_3 = 1e-5;
        let angle = std::f32::consts::PI / rug_fuzz_0;
        let (sin_val, cos_val) = angle.sin_cos();
        let expected_sin = rug_fuzz_1;
        let expected_cos = rug_fuzz_2;
        let sin_diff = (sin_val - expected_sin).abs();
        let cos_diff = (cos_val - expected_cos).abs();
        let tolerance = rug_fuzz_3;
        debug_assert!(sin_diff < tolerance, "Sin value out of tolerance: {}", sin_diff);
        debug_assert!(cos_diff < tolerance, "Cos value out of tolerance: {}", cos_diff);
        let _rug_ed_tests_llm_16_361_rrrruuuugggg_test_sin_cos = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_362_llm_16_362 {
    use crate::float::Float;
    #[test]
    fn test_sinh() {
        let _rug_st_tests_llm_16_362_llm_16_362_rrrruuuugggg_test_sinh = 0;
        let rug_fuzz_0 = 1.0;
        let value: f32 = rug_fuzz_0;
        let result = <f32 as Float>::sinh(value);
        let expected = value.sinh();
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_362_llm_16_362_rrrruuuugggg_test_sinh = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_363_llm_16_363 {
    use crate::float::Float;
    #[test]
    fn test_sqrt() {
        let _rug_st_tests_llm_16_363_llm_16_363_rrrruuuugggg_test_sqrt = 0;
        let rug_fuzz_0 = 4.0;
        let rug_fuzz_1 = 2.0;
        let num: f32 = rug_fuzz_0;
        let result = num.sqrt();
        let expected = rug_fuzz_1;
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_363_llm_16_363_rrrruuuugggg_test_sqrt = 0;
    }
    #[test]
    #[should_panic]
    fn test_sqrt_negative() {
        let _rug_st_tests_llm_16_363_llm_16_363_rrrruuuugggg_test_sqrt_negative = 0;
        let rug_fuzz_0 = 4.0;
        let num: f32 = -rug_fuzz_0;
        let _ = num.sqrt();
        let _rug_ed_tests_llm_16_363_llm_16_363_rrrruuuugggg_test_sqrt_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_364_llm_16_364 {
    use crate::float::Float;
    #[test]
    fn test_tan() {
        let _rug_st_tests_llm_16_364_llm_16_364_rrrruuuugggg_test_tan = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 2.0;
        let angle_zero: f32 = rug_fuzz_0;
        let angle_pi: f32 = std::f32::consts::PI;
        let angle_pi_2: f32 = std::f32::consts::PI / rug_fuzz_1;
        let tan_zero = angle_zero.tan();
        let tan_pi = angle_pi.tan();
        let tan_pi_2 = angle_pi_2.tan();
        debug_assert!(tan_zero.abs() < std::f32::EPSILON);
        debug_assert!(tan_pi.abs() < std::f32::EPSILON);
        debug_assert!(tan_pi_2.is_infinite());
        let _rug_ed_tests_llm_16_364_llm_16_364_rrrruuuugggg_test_tan = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_365_llm_16_365 {
    use crate::float::Float;
    #[test]
    fn tanh_test() {
        let _rug_st_tests_llm_16_365_llm_16_365_rrrruuuugggg_tanh_test = 0;
        let rug_fuzz_0 = 0.0_f32;
        let rug_fuzz_1 = 0.0_f32;
        let rug_fuzz_2 = 1.0_f32;
        let rug_fuzz_3 = 1.0_f32;
        let rug_fuzz_4 = 1.0_f32;
        let rug_fuzz_5 = 1.0_f32;
        let rug_fuzz_6 = 0.5_f32;
        let rug_fuzz_7 = 0.5_f32;
        let rug_fuzz_8 = 0.5_f32;
        let rug_fuzz_9 = 0.5_f32;
        let values = [
            (rug_fuzz_0, rug_fuzz_1),
            (rug_fuzz_2, rug_fuzz_3.tanh()),
            (-rug_fuzz_4, (-rug_fuzz_5).tanh()),
            (rug_fuzz_6, rug_fuzz_7.tanh()),
            (-rug_fuzz_8, (-rug_fuzz_9).tanh()),
        ];
        for &(value, expected) in &values {
            let result = <f32 as Float>::tanh(value);
            debug_assert!(
                (result - expected).abs() < f32::EPSILON,
                "value: {}, result: {}, expected: {}", value, result, expected
            );
        }
        let _rug_ed_tests_llm_16_365_llm_16_365_rrrruuuugggg_tanh_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_366_llm_16_366 {
    use crate::float::Float;
    #[test]
    fn to_degrees_test() {
        let _rug_st_tests_llm_16_366_llm_16_366_rrrruuuugggg_to_degrees_test = 0;
        let rug_fuzz_0 = 180.0;
        let rug_fuzz_1 = 1e-5;
        let rug_fuzz_2 = 0.0_f32;
        let rug_fuzz_3 = 0.0;
        let rug_fuzz_4 = 1e-5;
        let rug_fuzz_5 = 90.0;
        let rug_fuzz_6 = 1e-5;
        let rug_fuzz_7 = 2.0;
        let rug_fuzz_8 = 360.0;
        let rug_fuzz_9 = 1e-5;
        let rug_fuzz_10 = 90.0;
        let rug_fuzz_11 = 1e-5;
        let radians = std::f32::consts::PI;
        let degrees = <f32 as Float>::to_degrees(radians);
        debug_assert!((degrees - rug_fuzz_0).abs() < rug_fuzz_1);
        let radians = rug_fuzz_2;
        let degrees = <f32 as Float>::to_degrees(radians);
        debug_assert!((degrees - rug_fuzz_3).abs() < rug_fuzz_4);
        let radians = std::f32::consts::FRAC_PI_2;
        let degrees = <f32 as Float>::to_degrees(radians);
        debug_assert!((degrees - rug_fuzz_5).abs() < rug_fuzz_6);
        let radians = std::f32::consts::PI * rug_fuzz_7;
        let degrees = <f32 as Float>::to_degrees(radians);
        debug_assert!((degrees - rug_fuzz_8).abs() < rug_fuzz_9);
        let radians = -std::f32::consts::FRAC_PI_2;
        let degrees = <f32 as Float>::to_degrees(radians);
        debug_assert!((degrees - (- rug_fuzz_10)).abs() < rug_fuzz_11);
        let _rug_ed_tests_llm_16_366_llm_16_366_rrrruuuugggg_to_degrees_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_367 {
    use super::*;
    use crate::*;
    #[test]
    fn test_to_radians() {
        let _rug_st_tests_llm_16_367_rrrruuuugggg_test_to_radians = 0;
        let rug_fuzz_0 = 180.0;
        let rug_fuzz_1 = 1e-6;
        let degrees: f32 = rug_fuzz_0;
        let expected_radians: f32 = std::f32::consts::PI;
        let radians = degrees.to_radians();
        debug_assert!(
            (radians - expected_radians).abs() < rug_fuzz_1,
            "Conversion to radians did not produce expected result. Got: {}, Expected: {}",
            radians, expected_radians
        );
        let _rug_ed_tests_llm_16_367_rrrruuuugggg_test_to_radians = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_368_llm_16_368 {
    use crate::float::Float;
    #[test]
    fn test_trunc() {
        let _rug_st_tests_llm_16_368_llm_16_368_rrrruuuugggg_test_trunc = 0;
        let rug_fuzz_0 = 3.99_f32;
        let rug_fuzz_1 = 3.01_f32;
        let rug_fuzz_2 = 3.99_f32;
        let rug_fuzz_3 = 3.01_f32;
        let rug_fuzz_4 = 0.0_f32;
        let rug_fuzz_5 = 0.0_f32;
        debug_assert_eq!(< f32 as Float > ::trunc(rug_fuzz_0), 3.0_f32);
        debug_assert_eq!(< f32 as Float > ::trunc(rug_fuzz_1), 3.0_f32);
        debug_assert_eq!(< f32 as Float > ::trunc(- rug_fuzz_2), - 3.0_f32);
        debug_assert_eq!(< f32 as Float > ::trunc(- rug_fuzz_3), - 3.0_f32);
        debug_assert_eq!(< f32 as Float > ::trunc(rug_fuzz_4), 0.0_f32);
        debug_assert_eq!(< f32 as Float > ::trunc(- rug_fuzz_5), - 0.0_f32);
        debug_assert_eq!(< f32 as Float > ::trunc(f32::INFINITY), f32::INFINITY);
        debug_assert_eq!(< f32 as Float > ::trunc(f32::NEG_INFINITY), f32::NEG_INFINITY);
        debug_assert_eq!(< f32 as Float > ::trunc(f32::NAN).is_nan(), true);
        let _rug_ed_tests_llm_16_368_llm_16_368_rrrruuuugggg_test_trunc = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_369_llm_16_369 {
    use crate::float::FloatConst;
    #[test]
    fn test_float_const_e() {
        let _rug_st_tests_llm_16_369_llm_16_369_rrrruuuugggg_test_float_const_e = 0;
        let rug_fuzz_0 = 2.7182817_f32;
        let rug_fuzz_1 = 1e-6_f32;
        let e = f32::E();
        let expected_e = rug_fuzz_0;
        let epsilon = rug_fuzz_1;
        debug_assert!(
            (e - expected_e).abs() < epsilon,
            "The value of e is not within the expected range"
        );
        let _rug_ed_tests_llm_16_369_llm_16_369_rrrruuuugggg_test_float_const_e = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_371_llm_16_371 {
    use crate::float::FloatConst;
    #[test]
    fn test_frac_1_sqrt_2() {
        let _rug_st_tests_llm_16_371_llm_16_371_rrrruuuugggg_test_frac_1_sqrt_2 = 0;
        let rug_fuzz_0 = 1.0;
        let rug_fuzz_1 = 2.0;
        let value = f32::FRAC_1_SQRT_2();
        let expected = rug_fuzz_0 / f32::sqrt(rug_fuzz_1);
        debug_assert!((value - expected).abs() < f32::EPSILON);
        let _rug_ed_tests_llm_16_371_llm_16_371_rrrruuuugggg_test_frac_1_sqrt_2 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_375_llm_16_375 {
    use crate::float::FloatConst;
    #[test]
    fn test_frac_pi_3() {
        let _rug_st_tests_llm_16_375_llm_16_375_rrrruuuugggg_test_frac_pi_3 = 0;
        let rug_fuzz_0 = 3.0;
        let result = <f32 as FloatConst>::FRAC_PI_3();
        let expected = std::f32::consts::PI / rug_fuzz_0;
        debug_assert!((result - expected).abs() < f32::EPSILON);
        let _rug_ed_tests_llm_16_375_llm_16_375_rrrruuuugggg_test_frac_pi_3 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_376_llm_16_376 {
    use crate::float::FloatConst;
    #[test]
    fn test_frac_pi_4() {
        let _rug_st_tests_llm_16_376_llm_16_376_rrrruuuugggg_test_frac_pi_4 = 0;
        let frac_pi_4 = <f32 as FloatConst>::FRAC_PI_4();
        let expected = std::f32::consts::FRAC_PI_4;
        debug_assert!((frac_pi_4 - expected).abs() < std::f32::EPSILON);
        let _rug_ed_tests_llm_16_376_llm_16_376_rrrruuuugggg_test_frac_pi_4 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_377_llm_16_377 {
    use crate::float::FloatConst;
    #[test]
    fn test_frac_pi_6() {
        let _rug_st_tests_llm_16_377_llm_16_377_rrrruuuugggg_test_frac_pi_6 = 0;
        let rug_fuzz_0 = 6.0;
        let result = <f32 as FloatConst>::FRAC_PI_6();
        let expected = std::f32::consts::PI / rug_fuzz_0;
        debug_assert!((result - expected).abs() < f32::EPSILON);
        let _rug_ed_tests_llm_16_377_llm_16_377_rrrruuuugggg_test_frac_pi_6 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_384_llm_16_384 {
    use crate::float::FloatConst;
    #[test]
    fn log2_e_test() {
        let _rug_st_tests_llm_16_384_llm_16_384_rrrruuuugggg_log2_e_test = 0;
        let rug_fuzz_0 = 1.44269504089f32;
        let log2_e = <f32 as FloatConst>::LOG2_E();
        let expected = rug_fuzz_0;
        let epsilon = f32::EPSILON;
        debug_assert!(
            (log2_e - expected).abs() < epsilon,
            "LOG2_E did not match the expected value."
        );
        let _rug_ed_tests_llm_16_384_llm_16_384_rrrruuuugggg_log2_e_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_388_llm_16_388 {
    use crate::float::FloatCore;
    #[test]
    fn test_abs() {
        let _rug_st_tests_llm_16_388_llm_16_388_rrrruuuugggg_test_abs = 0;
        let rug_fuzz_0 = 1.0;
        let rug_fuzz_1 = 0.0;
        let rug_fuzz_2 = 1.0;
        let rug_fuzz_3 = 1.234;
        debug_assert_eq!(< f32 as FloatCore > ::abs(- rug_fuzz_0), 1.0);
        debug_assert_eq!(< f32 as FloatCore > ::abs(rug_fuzz_1), 0.0);
        debug_assert_eq!(< f32 as FloatCore > ::abs(rug_fuzz_2), 1.0);
        debug_assert_eq!(< f32 as FloatCore > ::abs(- rug_fuzz_3), 1.234);
        debug_assert!(< f32 as FloatCore > ::abs(f32::NAN).is_nan());
        let _rug_ed_tests_llm_16_388_llm_16_388_rrrruuuugggg_test_abs = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_389_llm_16_389 {
    use crate::float::FloatCore;
    #[test]
    fn test_ceil() {
        let _rug_st_tests_llm_16_389_llm_16_389_rrrruuuugggg_test_ceil = 0;
        let rug_fuzz_0 = 1.0f32;
        let rug_fuzz_1 = 1.1f32;
        let rug_fuzz_2 = 1.1f32;
        let rug_fuzz_3 = 0.0f32;
        let rug_fuzz_4 = 0.0f32;
        debug_assert_eq!(rug_fuzz_0.ceil(), 1.0f32);
        debug_assert_eq!(rug_fuzz_1.ceil(), 2.0f32);
        debug_assert_eq!(- rug_fuzz_2.ceil(), - 1.0f32);
        debug_assert_eq!(rug_fuzz_3.ceil(), 0.0f32);
        debug_assert_eq!(- rug_fuzz_4.ceil(), - 0.0f32);
        debug_assert_eq!(f32::INFINITY.ceil(), f32::INFINITY);
        debug_assert_eq!(f32::NEG_INFINITY.ceil(), f32::NEG_INFINITY);
        debug_assert!(f32::NAN.ceil().is_nan());
        let _rug_ed_tests_llm_16_389_llm_16_389_rrrruuuugggg_test_ceil = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_390_llm_16_390 {
    use crate::float::FloatCore;
    #[test]
    fn test_classify() {
        let _rug_st_tests_llm_16_390_llm_16_390_rrrruuuugggg_test_classify = 0;
        let rug_fuzz_0 = 0f32;
        let rug_fuzz_1 = 0f32;
        let rug_fuzz_2 = 1f32;
        let rug_fuzz_3 = 1f32;
        debug_assert_eq!(f32::INFINITY.classify(), std::num::FpCategory::Infinite);
        debug_assert_eq!((- f32::INFINITY).classify(), std::num::FpCategory::Infinite);
        debug_assert_eq!(f32::NAN.classify(), std::num::FpCategory::Nan);
        debug_assert_eq!(rug_fuzz_0.classify(), std::num::FpCategory::Zero);
        debug_assert_eq!((- rug_fuzz_1).classify(), std::num::FpCategory::Zero);
        debug_assert_eq!(rug_fuzz_2.classify(), std::num::FpCategory::Normal);
        debug_assert_eq!((- rug_fuzz_3).classify(), std::num::FpCategory::Normal);
        debug_assert_eq!(f32::MIN_POSITIVE.classify(), std::num::FpCategory::Subnormal);
        debug_assert_eq!(
            (- f32::MIN_POSITIVE).classify(), std::num::FpCategory::Subnormal
        );
        let _rug_ed_tests_llm_16_390_llm_16_390_rrrruuuugggg_test_classify = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_391_llm_16_391 {
    use crate::float::FloatCore;
    #[test]
    fn epsilon_f32() {
        let _rug_st_tests_llm_16_391_llm_16_391_rrrruuuugggg_epsilon_f32 = 0;
        let eps = f32::epsilon();
        debug_assert_eq!(eps, std::f32::EPSILON);
        let _rug_ed_tests_llm_16_391_llm_16_391_rrrruuuugggg_epsilon_f32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_392_llm_16_392 {
    use crate::float::FloatCore;
    #[test]
    fn floor_test() {
        let _rug_st_tests_llm_16_392_llm_16_392_rrrruuuugggg_floor_test = 0;
        let rug_fuzz_0 = 3.0;
        let rug_fuzz_1 = 3.3;
        let rug_fuzz_2 = 3.7;
        let rug_fuzz_3 = 3.3;
        let rug_fuzz_4 = 3.7;
        let rug_fuzz_5 = 0.0;
        let rug_fuzz_6 = 0.0;
        debug_assert_eq!(< f32 as FloatCore > ::floor(rug_fuzz_0), 3.0);
        debug_assert_eq!(< f32 as FloatCore > ::floor(rug_fuzz_1), 3.0);
        debug_assert_eq!(< f32 as FloatCore > ::floor(rug_fuzz_2), 3.0);
        debug_assert_eq!(< f32 as FloatCore > ::floor(- rug_fuzz_3), - 4.0);
        debug_assert_eq!(< f32 as FloatCore > ::floor(- rug_fuzz_4), - 4.0);
        debug_assert_eq!(< f32 as FloatCore > ::floor(rug_fuzz_5), 0.0);
        debug_assert_eq!(< f32 as FloatCore > ::floor(- rug_fuzz_6), - 0.0);
        debug_assert_eq!(< f32 as FloatCore > ::floor(f32::INFINITY), f32::INFINITY);
        debug_assert_eq!(
            < f32 as FloatCore > ::floor(f32::NEG_INFINITY), f32::NEG_INFINITY
        );
        debug_assert!(< f32 as FloatCore > ::floor(f32::NAN).is_nan());
        let _rug_ed_tests_llm_16_392_llm_16_392_rrrruuuugggg_floor_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_393_llm_16_393 {
    use crate::float::FloatCore;
    #[test]
    fn test_fract() {
        let _rug_st_tests_llm_16_393_llm_16_393_rrrruuuugggg_test_fract = 0;
        let rug_fuzz_0 = 3.5f32;
        let rug_fuzz_1 = 4.0f32;
        let rug_fuzz_2 = 3.75f32;
        let rug_fuzz_3 = 0.0f32;
        let rug_fuzz_4 = 0.0f32;
        let rug_fuzz_5 = 0.0;
        let a = rug_fuzz_0;
        let b = rug_fuzz_1;
        let c = -rug_fuzz_2;
        let d = rug_fuzz_3;
        let e = -rug_fuzz_4;
        debug_assert_eq!(a.fract(), 0.5);
        debug_assert_eq!(b.fract(), 0.0);
        debug_assert_eq!(c.fract(), - 0.75);
        debug_assert_eq!(d.fract(), 0.0);
        debug_assert!(e.fract() == - rug_fuzz_5);
        let _rug_ed_tests_llm_16_393_llm_16_393_rrrruuuugggg_test_fract = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_396_llm_16_396 {
    use crate::float::FloatCore;
    #[test]
    fn test_is_finite() {
        let _rug_st_tests_llm_16_396_llm_16_396_rrrruuuugggg_test_is_finite = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 1.0;
        let rug_fuzz_2 = 1.0;
        debug_assert!(< f32 as FloatCore > ::is_finite(rug_fuzz_0));
        debug_assert!(< f32 as FloatCore > ::is_finite(rug_fuzz_1));
        debug_assert!(< f32 as FloatCore > ::is_finite(- rug_fuzz_2));
        debug_assert!(< f32 as FloatCore > ::is_finite(f32::MIN));
        debug_assert!(< f32 as FloatCore > ::is_finite(f32::MAX));
        debug_assert!(! < f32 as FloatCore > ::is_finite(f32::NAN));
        debug_assert!(! < f32 as FloatCore > ::is_finite(f32::INFINITY));
        debug_assert!(! < f32 as FloatCore > ::is_finite(f32::NEG_INFINITY));
        let _rug_ed_tests_llm_16_396_llm_16_396_rrrruuuugggg_test_is_finite = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_397_llm_16_397 {
    use crate::float::FloatCore;
    #[test]
    fn test_is_infinite() {
        let _rug_st_tests_llm_16_397_llm_16_397_rrrruuuugggg_test_is_infinite = 0;
        let rug_fuzz_0 = 0.0f32;
        let rug_fuzz_1 = 1.0f32;
        let rug_fuzz_2 = 1.0f32;
        debug_assert!(f32::INFINITY.is_infinite());
        debug_assert!(f32::NEG_INFINITY.is_infinite());
        debug_assert!(! f32::NAN.is_infinite());
        debug_assert!(! rug_fuzz_0.is_infinite());
        debug_assert!(! rug_fuzz_1.is_infinite());
        debug_assert!(! (- rug_fuzz_2).is_infinite());
        let _rug_ed_tests_llm_16_397_llm_16_397_rrrruuuugggg_test_is_infinite = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_398_llm_16_398 {
    use crate::float::FloatCore;
    #[test]
    fn test_is_nan() {
        let _rug_st_tests_llm_16_398_llm_16_398_rrrruuuugggg_test_is_nan = 0;
        let rug_fuzz_0 = 0f32;
        let rug_fuzz_1 = 1f32;
        let rug_fuzz_2 = 1f32;
        debug_assert!(f32::NAN.is_nan());
        debug_assert!(! f32::INFINITY.is_nan());
        debug_assert!(! (- f32::INFINITY).is_nan());
        debug_assert!(! rug_fuzz_0.is_nan());
        debug_assert!(! rug_fuzz_1.is_nan());
        debug_assert!(! (- rug_fuzz_2).is_nan());
        debug_assert!(! f32::MIN.is_nan());
        debug_assert!(! f32::MAX.is_nan());
        let _rug_ed_tests_llm_16_398_llm_16_398_rrrruuuugggg_test_is_nan = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_399_llm_16_399 {
    use crate::float::FloatCore;
    #[test]
    fn test_is_normal() {
        let _rug_st_tests_llm_16_399_llm_16_399_rrrruuuugggg_test_is_normal = 0;
        let rug_fuzz_0 = 1.0f32;
        let rug_fuzz_1 = 0.0f32;
        let rug_fuzz_2 = 1.0e-38f32;
        let rug_fuzz_3 = 1.0e-40f32;
        debug_assert!(rug_fuzz_0.is_normal());
        debug_assert!(! rug_fuzz_1.is_normal());
        debug_assert!(! f32::NAN.is_normal());
        debug_assert!(! f32::INFINITY.is_normal());
        debug_assert!(! f32::NEG_INFINITY.is_normal());
        debug_assert!(! f32::MIN_POSITIVE.is_normal());
        debug_assert!((rug_fuzz_2).is_normal());
        debug_assert!(! (rug_fuzz_3).is_normal());
        let _rug_ed_tests_llm_16_399_llm_16_399_rrrruuuugggg_test_is_normal = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_400_llm_16_400 {
    use crate::float::FloatCore;
    #[test]
    fn test_is_sign_negative() {
        let _rug_st_tests_llm_16_400_llm_16_400_rrrruuuugggg_test_is_sign_negative = 0;
        let rug_fuzz_0 = 1.0;
        let rug_fuzz_1 = 1.0;
        let rug_fuzz_2 = 0.0;
        let rug_fuzz_3 = 0.0;
        debug_assert_eq!(< f32 as FloatCore > ::is_sign_negative(- rug_fuzz_0), true);
        debug_assert_eq!(< f32 as FloatCore > ::is_sign_negative(rug_fuzz_1), false);
        debug_assert_eq!(< f32 as FloatCore > ::is_sign_negative(rug_fuzz_2), false);
        debug_assert_eq!(< f32 as FloatCore > ::is_sign_negative(- rug_fuzz_3), true);
        debug_assert_eq!(< f32 as FloatCore > ::is_sign_negative(f32::INFINITY), false);
        debug_assert_eq!(
            < f32 as FloatCore > ::is_sign_negative(f32::NEG_INFINITY), true
        );
        debug_assert!(< f32 as FloatCore > ::is_sign_negative(f32::NAN));
        let _rug_ed_tests_llm_16_400_llm_16_400_rrrruuuugggg_test_is_sign_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_401_llm_16_401 {
    use crate::float::FloatCore;
    #[test]
    fn test_is_sign_positive() {
        let _rug_st_tests_llm_16_401_llm_16_401_rrrruuuugggg_test_is_sign_positive = 0;
        let rug_fuzz_0 = 3.14;
        let rug_fuzz_1 = 0.0;
        let rug_fuzz_2 = 3.14;
        let rug_fuzz_3 = 0.0;
        debug_assert!(< f32 as FloatCore > ::is_sign_positive(rug_fuzz_0));
        debug_assert!(< f32 as FloatCore > ::is_sign_positive(rug_fuzz_1));
        debug_assert!(! < f32 as FloatCore > ::is_sign_positive(- rug_fuzz_2));
        debug_assert!(! < f32 as FloatCore > ::is_sign_positive(- rug_fuzz_3));
        let _rug_ed_tests_llm_16_401_llm_16_401_rrrruuuugggg_test_is_sign_positive = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_402_llm_16_402 {
    use crate::float::FloatCore;
    #[test]
    fn test_f32_max() {
        let _rug_st_tests_llm_16_402_llm_16_402_rrrruuuugggg_test_f32_max = 0;
        let rug_fuzz_0 = 1.0;
        let rug_fuzz_1 = 2.0;
        let a: f32 = rug_fuzz_0;
        let b: f32 = rug_fuzz_1;
        let c: f32 = f32::NAN;
        debug_assert!((a.max(b) - b).abs() < f32::EPSILON);
        debug_assert!((b.max(a) - b).abs() < f32::EPSILON);
        debug_assert!((a.max(c) - a).abs() < f32::EPSILON);
        debug_assert!((c.max(a) - a).abs() < f32::EPSILON);
        debug_assert!((b.max(b) - b).abs() < f32::EPSILON);
        let _rug_ed_tests_llm_16_402_llm_16_402_rrrruuuugggg_test_f32_max = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_404_llm_16_404 {
    use crate::float::FloatCore;
    #[test]
    fn test_min() {
        let _rug_st_tests_llm_16_404_llm_16_404_rrrruuuugggg_test_min = 0;
        let rug_fuzz_0 = 3.5;
        let rug_fuzz_1 = 2.5;
        let a: f32 = rug_fuzz_0;
        let b: f32 = rug_fuzz_1;
        debug_assert_eq!(< f32 as FloatCore > ::min(a, b), 2.5);
        let _rug_ed_tests_llm_16_404_llm_16_404_rrrruuuugggg_test_min = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_406_llm_16_406 {
    use crate::float::FloatCore;
    #[test]
    fn test_f32_min_value() {
        let _rug_st_tests_llm_16_406_llm_16_406_rrrruuuugggg_test_f32_min_value = 0;
        let min_val: f32 = <f32 as FloatCore>::min_value();
        debug_assert_eq!(min_val, f32::MIN);
        let _rug_ed_tests_llm_16_406_llm_16_406_rrrruuuugggg_test_f32_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_407_llm_16_407 {
    use crate::float::FloatCore;
    #[test]
    fn nan_test() {
        let _rug_st_tests_llm_16_407_llm_16_407_rrrruuuugggg_nan_test = 0;
        let nan_value: f32 = <f32 as FloatCore>::nan();
        debug_assert!(nan_value.is_nan());
        let _rug_ed_tests_llm_16_407_llm_16_407_rrrruuuugggg_nan_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_408_llm_16_408 {
    use crate::float::FloatCore;
    #[test]
    fn neg_infinity_test() {
        let _rug_st_tests_llm_16_408_llm_16_408_rrrruuuugggg_neg_infinity_test = 0;
        let neg_inf: f32 = <f32 as FloatCore>::neg_infinity();
        debug_assert!(neg_inf.is_infinite());
        debug_assert!(neg_inf.is_sign_negative());
        debug_assert!(! neg_inf.is_nan());
        let _rug_ed_tests_llm_16_408_llm_16_408_rrrruuuugggg_neg_infinity_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_409_llm_16_409 {
    use crate::float::FloatCore;
    #[test]
    fn test_neg_zero() {
        let _rug_st_tests_llm_16_409_llm_16_409_rrrruuuugggg_test_neg_zero = 0;
        let neg_zero = <f32 as FloatCore>::neg_zero();
        debug_assert!(neg_zero.is_sign_negative());
        debug_assert_eq!(neg_zero, - 0.0_f32);
        let _rug_ed_tests_llm_16_409_llm_16_409_rrrruuuugggg_test_neg_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_410_llm_16_410 {
    use crate::float::FloatCore;
    #[test]
    fn test_powi() {
        let _rug_st_tests_llm_16_410_llm_16_410_rrrruuuugggg_test_powi = 0;
        let rug_fuzz_0 = 2.0f32;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 2.0f32;
        let rug_fuzz_3 = 3;
        let rug_fuzz_4 = 2.0f32;
        let rug_fuzz_5 = 0;
        let rug_fuzz_6 = 2.0f32;
        let rug_fuzz_7 = 3;
        let rug_fuzz_8 = 2.0f32;
        let rug_fuzz_9 = 2;
        let num = rug_fuzz_0;
        let power = rug_fuzz_1;
        let result = num.powi(power);
        debug_assert_eq!(result, 8.0f32);
        let num = rug_fuzz_2;
        let power = -rug_fuzz_3;
        let result = num.powi(power);
        debug_assert_eq!(result, 0.125f32);
        let num = rug_fuzz_4;
        let power = rug_fuzz_5;
        let result = num.powi(power);
        debug_assert_eq!(result, 1.0f32);
        let num = -rug_fuzz_6;
        let power = rug_fuzz_7;
        let result = num.powi(power);
        debug_assert_eq!(result, - 8.0f32);
        let num = -rug_fuzz_8;
        let power = rug_fuzz_9;
        let result = num.powi(power);
        debug_assert_eq!(result, 4.0f32);
        let _rug_ed_tests_llm_16_410_llm_16_410_rrrruuuugggg_test_powi = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_411_llm_16_411 {
    use crate::float::FloatCore;
    #[test]
    fn recip_test() {
        let _rug_st_tests_llm_16_411_llm_16_411_rrrruuuugggg_recip_test = 0;
        let rug_fuzz_0 = 2.0f32;
        let rug_fuzz_1 = 0.5f32;
        let value = rug_fuzz_0;
        let expected = rug_fuzz_1;
        let result = <f32 as FloatCore>::recip(value);
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_411_llm_16_411_rrrruuuugggg_recip_test = 0;
    }
    #[test]
    fn recip_test_nonzero() {
        let _rug_st_tests_llm_16_411_llm_16_411_rrrruuuugggg_recip_test_nonzero = 0;
        let rug_fuzz_0 = 1.0f32;
        let rug_fuzz_1 = 1.0f32;
        let value = rug_fuzz_0;
        let expected = rug_fuzz_1;
        let result = <f32 as FloatCore>::recip(value);
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_411_llm_16_411_rrrruuuugggg_recip_test_nonzero = 0;
    }
    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn recip_test_zero() {
        let _rug_st_tests_llm_16_411_llm_16_411_rrrruuuugggg_recip_test_zero = 0;
        let rug_fuzz_0 = 0.0f32;
        let value = rug_fuzz_0;
        let _result = <f32 as FloatCore>::recip(value);
        let _rug_ed_tests_llm_16_411_llm_16_411_rrrruuuugggg_recip_test_zero = 0;
    }
    #[test]
    fn recip_test_infinity() {
        let _rug_st_tests_llm_16_411_llm_16_411_rrrruuuugggg_recip_test_infinity = 0;
        let rug_fuzz_0 = 0.0f32;
        let value = f32::INFINITY;
        let expected = rug_fuzz_0;
        let result = <f32 as FloatCore>::recip(value);
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_411_llm_16_411_rrrruuuugggg_recip_test_infinity = 0;
    }
    #[test]
    fn recip_test_negative_infinity() {
        let _rug_st_tests_llm_16_411_llm_16_411_rrrruuuugggg_recip_test_negative_infinity = 0;
        let rug_fuzz_0 = 0.0f32;
        let value = f32::NEG_INFINITY;
        let expected = rug_fuzz_0;
        let result = <f32 as FloatCore>::recip(value);
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_411_llm_16_411_rrrruuuugggg_recip_test_negative_infinity = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_412_llm_16_412 {
    use crate::float::FloatCore;
    #[test]
    fn test_round() {
        let _rug_st_tests_llm_16_412_llm_16_412_rrrruuuugggg_test_round = 0;
        let rug_fuzz_0 = 3.3;
        let rug_fuzz_1 = 3.5;
        let rug_fuzz_2 = 3.7;
        let rug_fuzz_3 = 3.3;
        let rug_fuzz_4 = 3.5;
        let rug_fuzz_5 = 3.7;
        let num: f32 = rug_fuzz_0;
        debug_assert_eq!(num.round(), 3.0);
        let num: f32 = rug_fuzz_1;
        debug_assert_eq!(num.round(), 4.0);
        let num: f32 = rug_fuzz_2;
        debug_assert_eq!(num.round(), 4.0);
        let num: f32 = -rug_fuzz_3;
        debug_assert_eq!(num.round(), - 3.0);
        let num: f32 = -rug_fuzz_4;
        debug_assert_eq!(num.round(), - 4.0);
        let num: f32 = -rug_fuzz_5;
        debug_assert_eq!(num.round(), - 4.0);
        let _rug_ed_tests_llm_16_412_llm_16_412_rrrruuuugggg_test_round = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_413_llm_16_413 {
    use crate::float::FloatCore;
    #[test]
    fn f32_signum_positive() {
        let _rug_st_tests_llm_16_413_llm_16_413_rrrruuuugggg_f32_signum_positive = 0;
        let rug_fuzz_0 = 1.0f32;
        debug_assert_eq!(rug_fuzz_0.signum(), 1.0);
        let _rug_ed_tests_llm_16_413_llm_16_413_rrrruuuugggg_f32_signum_positive = 0;
    }
    #[test]
    fn f32_signum_negative() {
        let _rug_st_tests_llm_16_413_llm_16_413_rrrruuuugggg_f32_signum_negative = 0;
        let rug_fuzz_0 = 1.0f32;
        debug_assert_eq!((- rug_fuzz_0).signum(), - 1.0);
        let _rug_ed_tests_llm_16_413_llm_16_413_rrrruuuugggg_f32_signum_negative = 0;
    }
    #[test]
    fn f32_signum_zero() {
        let _rug_st_tests_llm_16_413_llm_16_413_rrrruuuugggg_f32_signum_zero = 0;
        let rug_fuzz_0 = 0.0f32;
        let rug_fuzz_1 = 0.0f32;
        debug_assert_eq!(rug_fuzz_0.signum(), 1.0);
        debug_assert_eq!((- rug_fuzz_1).signum(), - 1.0);
        let _rug_ed_tests_llm_16_413_llm_16_413_rrrruuuugggg_f32_signum_zero = 0;
    }
    #[test]
    fn f32_signum_nan() {
        let _rug_st_tests_llm_16_413_llm_16_413_rrrruuuugggg_f32_signum_nan = 0;
        debug_assert!(f32::NAN.signum().is_nan());
        let _rug_ed_tests_llm_16_413_llm_16_413_rrrruuuugggg_f32_signum_nan = 0;
    }
    #[test]
    fn f32_signum_inf() {
        let _rug_st_tests_llm_16_413_llm_16_413_rrrruuuugggg_f32_signum_inf = 0;
        debug_assert_eq!(f32::INFINITY.signum(), 1.0);
        debug_assert_eq!(f32::NEG_INFINITY.signum(), - 1.0);
        let _rug_ed_tests_llm_16_413_llm_16_413_rrrruuuugggg_f32_signum_inf = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_414_llm_16_414 {
    use crate::float::FloatCore;
    #[test]
    fn test_to_degrees() {
        let _rug_st_tests_llm_16_414_llm_16_414_rrrruuuugggg_test_to_degrees = 0;
        let rug_fuzz_0 = 180.0;
        let rad = std::f32::consts::PI;
        let deg = <f32 as FloatCore>::to_degrees(rad);
        debug_assert!((deg - rug_fuzz_0).abs() < f32::EPSILON);
        let _rug_ed_tests_llm_16_414_llm_16_414_rrrruuuugggg_test_to_degrees = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_416_llm_16_416 {
    use crate::float::FloatCore;
    #[test]
    fn trunc_test() {
        let _rug_st_tests_llm_16_416_llm_16_416_rrrruuuugggg_trunc_test = 0;
        let rug_fuzz_0 = 3.9;
        let rug_fuzz_1 = 3.9;
        let rug_fuzz_2 = 0.0;
        let rug_fuzz_3 = 0.0;
        debug_assert_eq!(< f32 as FloatCore > ::trunc(rug_fuzz_0), 3.0);
        debug_assert_eq!(< f32 as FloatCore > ::trunc(- rug_fuzz_1), - 3.0);
        debug_assert_eq!(< f32 as FloatCore > ::trunc(rug_fuzz_2), 0.0);
        debug_assert_eq!(< f32 as FloatCore > ::trunc(- rug_fuzz_3), - 0.0);
        debug_assert_eq!(< f32 as FloatCore > ::trunc(f32::INFINITY), f32::INFINITY);
        debug_assert_eq!(
            < f32 as FloatCore > ::trunc(f32::NEG_INFINITY), f32::NEG_INFINITY
        );
        debug_assert!(< f32 as FloatCore > ::trunc(f32::NAN).is_nan());
        let _rug_ed_tests_llm_16_416_llm_16_416_rrrruuuugggg_trunc_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_479_llm_16_479 {
    use crate::float::Float;
    #[test]
    fn abs_test() {
        let _rug_st_tests_llm_16_479_llm_16_479_rrrruuuugggg_abs_test = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 1.0;
        let rug_fuzz_2 = 1.0;
        let rug_fuzz_3 = 1.23;
        let rug_fuzz_4 = 1.23;
        debug_assert_eq!(< f64 as Float > ::abs(- rug_fuzz_0), 0.0);
        debug_assert_eq!(< f64 as Float > ::abs(- rug_fuzz_1), 1.0);
        debug_assert_eq!(< f64 as Float > ::abs(rug_fuzz_2), 1.0);
        debug_assert_eq!(< f64 as Float > ::abs(- rug_fuzz_3), 1.23);
        debug_assert_eq!(< f64 as Float > ::abs(rug_fuzz_4), 1.23);
        debug_assert!(< f64 as Float > ::abs(f64::NAN).is_nan());
        let _rug_ed_tests_llm_16_479_llm_16_479_rrrruuuugggg_abs_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_481 {
    #[test]
    fn acos_test() {
        let _rug_st_tests_llm_16_481_rrrruuuugggg_acos_test = 0;
        let rug_fuzz_0 = 1.0f64;
        let rug_fuzz_1 = 0.0;
        let rug_fuzz_2 = 1e-15;
        let rug_fuzz_3 = 0.0f64;
        let rug_fuzz_4 = 1e-15;
        let rug_fuzz_5 = 1.0f64;
        let rug_fuzz_6 = 1e-15;
        let rug_fuzz_7 = 2.0f64;
        let result = rug_fuzz_0.acos();
        debug_assert!((result - rug_fuzz_1).abs() < rug_fuzz_2);
        let result = rug_fuzz_3.acos();
        debug_assert!((result - std::f64::consts::FRAC_PI_2).abs() < rug_fuzz_4);
        let result = (-rug_fuzz_5).acos();
        debug_assert!((result - std::f64::consts::PI).abs() < rug_fuzz_6);
        let result = rug_fuzz_7.acos();
        debug_assert!(result.is_nan());
        let _rug_ed_tests_llm_16_481_rrrruuuugggg_acos_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_482_llm_16_482 {
    use crate::float::Float;
    #[test]
    fn acosh_test() {
        let _rug_st_tests_llm_16_482_llm_16_482_rrrruuuugggg_acosh_test = 0;
        let rug_fuzz_0 = 2.0f64;
        let x = rug_fuzz_0;
        let result = <f64 as Float>::acosh(x);
        let expected = x.acosh();
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_482_llm_16_482_rrrruuuugggg_acosh_test = 0;
    }
    #[test]
    #[should_panic]
    fn acosh_test_invalid_input() {
        let _rug_st_tests_llm_16_482_llm_16_482_rrrruuuugggg_acosh_test_invalid_input = 0;
        let rug_fuzz_0 = 0.5f64;
        let x = rug_fuzz_0;
        let _ = <f64 as Float>::acosh(x);
        let _rug_ed_tests_llm_16_482_llm_16_482_rrrruuuugggg_acosh_test_invalid_input = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_483_llm_16_483 {
    use crate::float::Float;
    #[test]
    fn asin_test() {
        let _rug_st_tests_llm_16_483_llm_16_483_rrrruuuugggg_asin_test = 0;
        let rug_fuzz_0 = 0.5f64;
        let rug_fuzz_1 = 1e-10;
        let x = rug_fuzz_0;
        let result = <f64 as Float>::asin(x);
        let expected = x.asin();
        debug_assert!((result - expected).abs() < rug_fuzz_1);
        let _rug_ed_tests_llm_16_483_llm_16_483_rrrruuuugggg_asin_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_484_llm_16_484 {
    use crate::Float;
    #[test]
    fn asinh_test() {
        let _rug_st_tests_llm_16_484_llm_16_484_rrrruuuugggg_asinh_test = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 0.0f64;
        let rug_fuzz_2 = 1e-10;
        let values: Vec<f64> = vec![
            rug_fuzz_0, 1.0, - 1.0, f64::INFINITY, f64::NEG_INFINITY
        ];
        let expected: Vec<f64> = vec![
            rug_fuzz_1.asinh(), 1.0f64.asinh(), (- 1.0f64).asinh(), f64::INFINITY,
            f64::NEG_INFINITY
        ];
        for (i, &value) in values.iter().enumerate() {
            let result = <f64 as Float>::asinh(value);
            if value.is_finite() {
                debug_assert!(
                    (result - expected[i]).abs() < rug_fuzz_2,
                    "asinh({}) = {}, expected: {}", value, result, expected[i]
                );
            } else {
                debug_assert_eq!(
                    result, expected[i], "asinh({}) = {}, expected: {}", value, result,
                    expected[i]
                );
            }
        }
        let _rug_ed_tests_llm_16_484_llm_16_484_rrrruuuugggg_asinh_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_485 {
    use super::*;
    use crate::*;
    use float::Float;
    #[test]
    fn test_atan() {
        let _rug_st_tests_llm_16_485_rrrruuuugggg_test_atan = 0;
        let rug_fuzz_0 = 0.0_f64;
        let rug_fuzz_1 = 0.0_f64;
        let rug_fuzz_2 = 1.0_f64;
        let rug_fuzz_3 = 1.0_f64;
        let test_cases = [
            (rug_fuzz_0, rug_fuzz_1),
            (rug_fuzz_2, std::f64::consts::FRAC_PI_4),
            (-rug_fuzz_3, -std::f64::consts::FRAC_PI_4),
            (std::f64::INFINITY, std::f64::consts::FRAC_PI_2),
            (std::f64::NEG_INFINITY, -std::f64::consts::FRAC_PI_2),
        ];
        for &(input, expected) in test_cases.iter() {
            let result = input.atan();
            debug_assert!(
                (result - expected).abs() < std::f64::EPSILON,
                "atan({}) = {}, expected {}", input, result, expected
            );
        }
        let _rug_ed_tests_llm_16_485_rrrruuuugggg_test_atan = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_486_llm_16_486 {
    use crate::float::Float;
    #[test]
    fn atan2_test() {
        let _rug_st_tests_llm_16_486_llm_16_486_rrrruuuugggg_atan2_test = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 2.0;
        let rug_fuzz_2 = 1e-10;
        let y: f64 = rug_fuzz_0;
        let x: f64 = rug_fuzz_1;
        let atan2_result = <f64 as Float>::atan2(y, x);
        let expected = f64::atan2(y, x);
        debug_assert!((atan2_result - expected).abs() < rug_fuzz_2);
        let _rug_ed_tests_llm_16_486_llm_16_486_rrrruuuugggg_atan2_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_487_llm_16_487 {
    use crate::float::Float;
    #[test]
    fn atanh_test() {
        let _rug_st_tests_llm_16_487_llm_16_487_rrrruuuugggg_atanh_test = 0;
        let rug_fuzz_0 = 0.5f64;
        let rug_fuzz_1 = 1e-10;
        let rug_fuzz_2 = 1.0f64;
        let rug_fuzz_3 = 1.0f64;
        let rug_fuzz_4 = 2.0f64;
        let rug_fuzz_5 = 2.0f64;
        let x = rug_fuzz_0;
        let result = <f64 as Float>::atanh(x);
        let expected = x.tanh().atanh();
        debug_assert!(
            (result - expected).abs() < rug_fuzz_1,
            "atanh(tanh(x)) should be approximately x"
        );
        let x = rug_fuzz_2;
        debug_assert!(
            < f64 as Float > ::atanh(x).is_infinite(), "atanh(1.0) should be infinity"
        );
        let x = -rug_fuzz_3;
        debug_assert!(
            < f64 as Float > ::atanh(x).is_infinite(), "atanh(-1.0) should be -infinity"
        );
        let x = rug_fuzz_4;
        let result = std::panic::catch_unwind(|| <f64 as Float>::atanh(x));
        debug_assert!(result.is_err(), "atanh(2.0) should panic as it is out of domain");
        let x = -rug_fuzz_5;
        let result = std::panic::catch_unwind(|| <f64 as Float>::atanh(x));
        debug_assert!(
            result.is_err(), "atanh(-2.0) should panic as it is out of domain"
        );
        let _rug_ed_tests_llm_16_487_llm_16_487_rrrruuuugggg_atanh_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_488_llm_16_488 {
    use super::*;
    use crate::*;
    #[test]
    fn test_cbrt() {
        let _rug_st_tests_llm_16_488_llm_16_488_rrrruuuugggg_test_cbrt = 0;
        let rug_fuzz_0 = 8.0;
        let rug_fuzz_1 = 8.0;
        let rug_fuzz_2 = 0.0;
        let rug_fuzz_3 = 1.0;
        let rug_fuzz_4 = 0.0;
        let rug_fuzz_5 = 1.0;
        let rug_fuzz_6 = 0.0;
        let rug_fuzz_7 = 1.0;
        let rug_fuzz_8 = 0.0;
        let rug_fuzz_9 = 1.0;
        let rug_fuzz_10 = 0.0;
        let x: f64 = rug_fuzz_0;
        let result = <f64 as Float>::cbrt(x);
        debug_assert_eq!(result, 2.0);
        let x: f64 = -rug_fuzz_1;
        let result = <f64 as Float>::cbrt(x);
        debug_assert_eq!(result, - 2.0);
        let x: f64 = rug_fuzz_2;
        let result = <f64 as Float>::cbrt(x);
        debug_assert_eq!(result, 0.0);
        let x: f64 = rug_fuzz_3 / rug_fuzz_4;
        let result = <f64 as Float>::cbrt(x);
        debug_assert!(result.is_infinite() && result.is_sign_positive());
        let x: f64 = -rug_fuzz_5 / rug_fuzz_6;
        let result = <f64 as Float>::cbrt(x);
        debug_assert!(result.is_infinite() && result.is_sign_negative());
        let x: f64 = rug_fuzz_7 / rug_fuzz_8;
        let result = <f64 as Float>::cbrt(x);
        debug_assert_eq!(result, x);
        let x: f64 = -rug_fuzz_9 / rug_fuzz_10;
        let result = <f64 as Float>::cbrt(x);
        debug_assert_eq!(result, x);
        let x: f64 = f64::NAN;
        let result = <f64 as Float>::cbrt(x);
        debug_assert!(result.is_nan());
        let _rug_ed_tests_llm_16_488_llm_16_488_rrrruuuugggg_test_cbrt = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_489 {
    use super::*;
    use crate::*;
    #[test]
    fn test_ceil() {
        let _rug_st_tests_llm_16_489_rrrruuuugggg_test_ceil = 0;
        let rug_fuzz_0 = 1.0f64;
        let rug_fuzz_1 = 1.1f64;
        let rug_fuzz_2 = 1.1f64;
        let rug_fuzz_3 = 0.0f64;
        debug_assert_eq!(rug_fuzz_0.ceil(), 1.0);
        debug_assert_eq!(rug_fuzz_1.ceil(), 2.0);
        debug_assert_eq!((- rug_fuzz_2).ceil(), - 1.0);
        debug_assert_eq!(rug_fuzz_3.ceil(), 0.0);
        let _rug_ed_tests_llm_16_489_rrrruuuugggg_test_ceil = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_490_llm_16_490 {
    use super::*;
    use crate::*;
    use core::num::FpCategory;
    #[test]
    fn test_classify() {
        let _rug_st_tests_llm_16_490_llm_16_490_rrrruuuugggg_test_classify = 0;
        let rug_fuzz_0 = 0.0f64;
        let rug_fuzz_1 = 0.0;
        let rug_fuzz_2 = 42.0;
        let rug_fuzz_3 = 42.0;
        let rug_fuzz_4 = 1e-308;
        let rug_fuzz_5 = 1e-308;
        let nan = f64::NAN;
        let inf = f64::INFINITY;
        let neginf = f64::NEG_INFINITY;
        let zero = rug_fuzz_0;
        let neg_zero = -rug_fuzz_1;
        let pos_num = rug_fuzz_2;
        let neg_num = -rug_fuzz_3;
        debug_assert_eq!(nan.classify(), FpCategory::Nan);
        debug_assert_eq!(inf.classify(), FpCategory::Infinite);
        debug_assert_eq!(neginf.classify(), FpCategory::Infinite);
        debug_assert_eq!(zero.classify(), FpCategory::Zero);
        debug_assert_eq!(neg_zero.classify(), FpCategory::Zero);
        debug_assert_eq!(pos_num.classify(), FpCategory::Normal);
        debug_assert_eq!(neg_num.classify(), FpCategory::Normal);
        let subnormal_pos = rug_fuzz_4;
        let subnormal_neg = -rug_fuzz_5;
        debug_assert_eq!(subnormal_pos.classify(), FpCategory::Subnormal);
        debug_assert_eq!(subnormal_neg.classify(), FpCategory::Subnormal);
        let _rug_ed_tests_llm_16_490_llm_16_490_rrrruuuugggg_test_classify = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_492_llm_16_492 {
    use crate::float::Float;
    #[test]
    fn test_cos() {
        let _rug_st_tests_llm_16_492_llm_16_492_rrrruuuugggg_test_cos = 0;
        let rug_fuzz_0 = 0.0_f64;
        let rug_fuzz_1 = 1.0;
        let rug_fuzz_2 = 1.0;
        let rug_fuzz_3 = 2.0;
        let rug_fuzz_4 = 0.0;
        let rug_fuzz_5 = 4.0;
        let rug_fuzz_6 = 1.0;
        let rug_fuzz_7 = 2.0;
        let value = rug_fuzz_0;
        let result = <f64 as Float>::cos(value);
        debug_assert!((result - rug_fuzz_1).abs() < f64::EPSILON);
        let value = std::f64::consts::PI;
        let result = <f64 as Float>::cos(value);
        debug_assert!((result - - rug_fuzz_2).abs() < f64::EPSILON);
        let value = std::f64::consts::PI / rug_fuzz_3;
        let result = <f64 as Float>::cos(value);
        debug_assert!((result - rug_fuzz_4).abs() < f64::EPSILON);
        let value = std::f64::consts::PI / rug_fuzz_5;
        let result = <f64 as Float>::cos(value);
        let expected = rug_fuzz_6 / f64::sqrt(rug_fuzz_7);
        debug_assert!((result - expected).abs() < f64::EPSILON);
        let _rug_ed_tests_llm_16_492_llm_16_492_rrrruuuugggg_test_cos = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_493_llm_16_493 {
    use crate::float::Float;
    #[test]
    fn test_cosh() {
        let _rug_st_tests_llm_16_493_llm_16_493_rrrruuuugggg_test_cosh = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 1.0;
        let rug_fuzz_2 = 1.0;
        let rug_fuzz_3 = 1.0;
        let rug_fuzz_4 = 10.0;
        let rug_fuzz_5 = 1e-10;
        let rug_fuzz_6 = 1e-10;
        let rug_fuzz_7 = 1e-10;
        let value = rug_fuzz_0;
        let expected = rug_fuzz_1;
        debug_assert_eq!(< f64 as Float > ::cosh(value), expected);
        let value = rug_fuzz_2;
        let expected = value.cosh();
        debug_assert_eq!(< f64 as Float > ::cosh(value), expected);
        let value = -rug_fuzz_3;
        let expected = value.cosh();
        debug_assert_eq!(< f64 as Float > ::cosh(value), expected);
        let value = rug_fuzz_4;
        let expected = value.cosh();
        debug_assert!((< f64 as Float > ::cosh(value) - expected).abs() < rug_fuzz_5);
        let value = rug_fuzz_6;
        let expected = value.cosh();
        debug_assert!((< f64 as Float > ::cosh(value) - expected).abs() < rug_fuzz_7);
        let _rug_ed_tests_llm_16_493_llm_16_493_rrrruuuugggg_test_cosh = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_494_llm_16_494 {
    use crate::float::Float;
    #[test]
    fn epsilon_for_f64() {
        let _rug_st_tests_llm_16_494_llm_16_494_rrrruuuugggg_epsilon_for_f64 = 0;
        let eps = <f64 as Float>::epsilon();
        debug_assert_eq!(eps, f64::EPSILON);
        let _rug_ed_tests_llm_16_494_llm_16_494_rrrruuuugggg_epsilon_for_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_495 {
    use crate::float::Float;
    #[test]
    fn exp_test() {
        let _rug_st_tests_llm_16_495_rrrruuuugggg_exp_test = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 1e-10;
        let value: f64 = rug_fuzz_0;
        let result = value.exp();
        let expected = value.exp();
        debug_assert!((result - expected).abs() < rug_fuzz_1);
        let _rug_ed_tests_llm_16_495_rrrruuuugggg_exp_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_496_llm_16_496 {
    use crate::float::Float;
    #[test]
    fn exp2_test() {
        let _rug_st_tests_llm_16_496_llm_16_496_rrrruuuugggg_exp2_test = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 1.0;
        let rug_fuzz_2 = 2.0;
        let rug_fuzz_3 = 3.0;
        let rug_fuzz_4 = 1.0;
        let rug_fuzz_5 = 2.0;
        let rug_fuzz_6 = 3.0;
        let rug_fuzz_7 = 1.5;
        let rug_fuzz_8 = 2.8284271247461903;
        let rug_fuzz_9 = 1e-15;
        debug_assert_eq!(< f64 as Float > ::exp2(rug_fuzz_0), 1.0);
        debug_assert_eq!(< f64 as Float > ::exp2(rug_fuzz_1), 2.0);
        debug_assert_eq!(< f64 as Float > ::exp2(rug_fuzz_2), 4.0);
        debug_assert_eq!(< f64 as Float > ::exp2(rug_fuzz_3), 8.0);
        debug_assert_eq!(< f64 as Float > ::exp2(- rug_fuzz_4), 0.5);
        debug_assert_eq!(< f64 as Float > ::exp2(- rug_fuzz_5), 0.25);
        debug_assert_eq!(< f64 as Float > ::exp2(- rug_fuzz_6), 0.125);
        debug_assert!(
            (< f64 as Float > ::exp2(rug_fuzz_7) - rug_fuzz_8).abs() < rug_fuzz_9
        );
        let _rug_ed_tests_llm_16_496_llm_16_496_rrrruuuugggg_exp2_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_497_llm_16_497 {
    use crate::float::Float;
    #[test]
    fn exp_m1_test() {
        let _rug_st_tests_llm_16_497_llm_16_497_rrrruuuugggg_exp_m1_test = 0;
        let rug_fuzz_0 = 0.0_f64;
        let rug_fuzz_1 = 0.5_f64;
        let rug_fuzz_2 = 1.0_f64;
        let rug_fuzz_3 = 1.0_f64;
        let rug_fuzz_4 = 2.0_f64;
        let rug_fuzz_5 = 1.0;
        let rug_fuzz_6 = 1e-10;
        let values = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, -rug_fuzz_3, rug_fuzz_4];
        for &val in &values {
            let expected = val.exp() - rug_fuzz_5;
            let result = <f64 as Float>::exp_m1(val);
            let diff = (result - expected).abs();
            debug_assert!(
                diff < rug_fuzz_6, "Value: {}, Expected: {}, Result: {}, Difference: {}",
                val, expected, result, diff
            );
        }
        let _rug_ed_tests_llm_16_497_llm_16_497_rrrruuuugggg_exp_m1_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_498_llm_16_498 {
    use crate::float::Float;
    #[test]
    fn test_floor() {
        let _rug_st_tests_llm_16_498_llm_16_498_rrrruuuugggg_test_floor = 0;
        let rug_fuzz_0 = 3.7;
        let rug_fuzz_1 = 3.7;
        let rug_fuzz_2 = 3.0;
        let rug_fuzz_3 = 3.0;
        let rug_fuzz_4 = 0.0;
        let a: f64 = rug_fuzz_0;
        let b: f64 = -rug_fuzz_1;
        let c: f64 = rug_fuzz_2;
        let d: f64 = -rug_fuzz_3;
        let e: f64 = rug_fuzz_4;
        debug_assert_eq!(a.floor(), 3.0);
        debug_assert_eq!(b.floor(), - 4.0);
        debug_assert_eq!(c.floor(), 3.0);
        debug_assert_eq!(d.floor(), - 3.0);
        debug_assert_eq!(e.floor(), 0.0);
        let _rug_ed_tests_llm_16_498_llm_16_498_rrrruuuugggg_test_floor = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_499_llm_16_499 {
    use crate::float::Float;
    #[test]
    fn test_fract() {
        let _rug_st_tests_llm_16_499_llm_16_499_rrrruuuugggg_test_fract = 0;
        let rug_fuzz_0 = 3.14;
        let rug_fuzz_1 = 3.14;
        let rug_fuzz_2 = 0.0;
        let rug_fuzz_3 = 0.0;
        let rug_fuzz_4 = 3.0;
        let rug_fuzz_5 = 3.0;
        let rug_fuzz_6 = 1.0;
        let rug_fuzz_7 = 0.0;
        let rug_fuzz_8 = 1.0;
        let rug_fuzz_9 = 0.0;
        let rug_fuzz_10 = 0.0;
        let rug_fuzz_11 = 0.0;
        debug_assert_eq!(rug_fuzz_0.fract(), 0.14);
        debug_assert_eq!((- rug_fuzz_1).fract(), - 0.14);
        debug_assert_eq!(rug_fuzz_2.fract(), 0.0);
        debug_assert_eq!((- rug_fuzz_3).fract(), - 0.0);
        debug_assert_eq!(rug_fuzz_4.fract(), 0.0);
        debug_assert_eq!((- rug_fuzz_5).fract(), - 0.0);
        debug_assert_eq!((rug_fuzz_6 / rug_fuzz_7).fract(), 0.0);
        debug_assert!((- rug_fuzz_8 / rug_fuzz_9).fract().is_nan());
        debug_assert!((rug_fuzz_10 / rug_fuzz_11).fract().is_nan());
        let _rug_ed_tests_llm_16_499_llm_16_499_rrrruuuugggg_test_fract = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_500 {
    use crate::float::Float;
    #[test]
    fn hypot_test() {
        let _rug_st_tests_llm_16_500_rrrruuuugggg_hypot_test = 0;
        let rug_fuzz_0 = 3.0;
        let rug_fuzz_1 = 4.0;
        let rug_fuzz_2 = 5.0;
        let rug_fuzz_3 = 1e-10;
        let x: f64 = rug_fuzz_0;
        let y: f64 = rug_fuzz_1;
        let result = <f64 as Float>::hypot(x, y);
        let expected = rug_fuzz_2;
        let tolerance = rug_fuzz_3;
        debug_assert!((result - expected).abs() < tolerance);
        let _rug_ed_tests_llm_16_500_rrrruuuugggg_hypot_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_501_llm_16_501 {
    use crate::float::Float;
    #[test]
    fn test_infinity() {
        let _rug_st_tests_llm_16_501_llm_16_501_rrrruuuugggg_test_infinity = 0;
        let inf: f64 = <f64 as Float>::infinity();
        debug_assert!(inf.is_infinite());
        debug_assert!(inf.is_sign_positive());
        let _rug_ed_tests_llm_16_501_llm_16_501_rrrruuuugggg_test_infinity = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_503_llm_16_503 {
    use crate::float::Float;
    #[test]
    fn test_is_finite() {
        let _rug_st_tests_llm_16_503_llm_16_503_rrrruuuugggg_test_is_finite = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 0.0;
        let rug_fuzz_2 = 1.0;
        let rug_fuzz_3 = 1.0;
        debug_assert!(< f64 as Float > ::is_finite(rug_fuzz_0));
        debug_assert!(< f64 as Float > ::is_finite(- rug_fuzz_1));
        debug_assert!(< f64 as Float > ::is_finite(rug_fuzz_2));
        debug_assert!(< f64 as Float > ::is_finite(- rug_fuzz_3));
        debug_assert!(< f64 as Float > ::is_finite(f64::MIN));
        debug_assert!(< f64 as Float > ::is_finite(f64::MAX));
        debug_assert!(! < f64 as Float > ::is_finite(f64::NAN));
        debug_assert!(! < f64 as Float > ::is_finite(f64::INFINITY));
        debug_assert!(! < f64 as Float > ::is_finite(f64::NEG_INFINITY));
        let _rug_ed_tests_llm_16_503_llm_16_503_rrrruuuugggg_test_is_finite = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_504_llm_16_504 {
    use crate::float::Float;
    #[test]
    fn test_is_infinite() {
        let _rug_st_tests_llm_16_504_llm_16_504_rrrruuuugggg_test_is_infinite = 0;
        let rug_fuzz_0 = 0f64;
        let rug_fuzz_1 = 0f64;
        let rug_fuzz_2 = 1f64;
        let rug_fuzz_3 = 1f64;
        debug_assert!(f64::INFINITY.is_infinite());
        debug_assert!(f64::NEG_INFINITY.is_infinite());
        debug_assert!(! f64::NAN.is_infinite());
        debug_assert!(! f64::MAX.is_infinite());
        debug_assert!(! rug_fuzz_0.is_infinite());
        debug_assert!(! (- rug_fuzz_1).is_infinite());
        debug_assert!(! rug_fuzz_2.is_infinite());
        debug_assert!(! (- rug_fuzz_3).is_infinite());
        let _rug_ed_tests_llm_16_504_llm_16_504_rrrruuuugggg_test_is_infinite = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_505_llm_16_505 {
    use crate::float::Float;
    #[test]
    fn test_is_nan() {
        let _rug_st_tests_llm_16_505_llm_16_505_rrrruuuugggg_test_is_nan = 0;
        let rug_fuzz_0 = 42.0f64;
        let nan = f64::NAN;
        let not_nan = rug_fuzz_0;
        debug_assert!(nan.is_nan());
        debug_assert!(! not_nan.is_nan());
        let _rug_ed_tests_llm_16_505_llm_16_505_rrrruuuugggg_test_is_nan = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_506_llm_16_506 {
    use crate::float::Float;
    #[test]
    fn test_is_normal() {
        let _rug_st_tests_llm_16_506_llm_16_506_rrrruuuugggg_test_is_normal = 0;
        let rug_fuzz_0 = 1.23;
        let rug_fuzz_1 = 4.56e123;
        let rug_fuzz_2 = 7.89;
        let rug_fuzz_3 = 0.0;
        let rug_fuzz_4 = 0.0;
        let rug_fuzz_5 = 2.0;
        debug_assert!(f64::is_normal(rug_fuzz_0));
        debug_assert!(f64::is_normal(rug_fuzz_1));
        debug_assert!(f64::is_normal(- rug_fuzz_2));
        debug_assert!(! f64::is_normal(rug_fuzz_3));
        debug_assert!(! f64::is_normal(- rug_fuzz_4));
        debug_assert!(! f64::is_normal(f64::INFINITY));
        debug_assert!(! f64::is_normal(f64::NEG_INFINITY));
        debug_assert!(! f64::is_normal(f64::NAN));
        debug_assert!(! f64::is_normal(f64::MIN_POSITIVE / rug_fuzz_5));
        let _rug_ed_tests_llm_16_506_llm_16_506_rrrruuuugggg_test_is_normal = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_507_llm_16_507 {
    use crate::float::Float;
    #[test]
    fn test_is_sign_negative() {
        let _rug_st_tests_llm_16_507_llm_16_507_rrrruuuugggg_test_is_sign_negative = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 1.0;
        let rug_fuzz_2 = 23.5;
        let rug_fuzz_3 = 0.0;
        let rug_fuzz_4 = 1.0;
        let rug_fuzz_5 = 23.5;
        debug_assert_eq!(< f64 as Float > ::is_sign_negative(- rug_fuzz_0), true);
        debug_assert_eq!(< f64 as Float > ::is_sign_negative(- rug_fuzz_1), true);
        debug_assert_eq!(< f64 as Float > ::is_sign_negative(- rug_fuzz_2), true);
        debug_assert_eq!(< f64 as Float > ::is_sign_negative(rug_fuzz_3), false);
        debug_assert_eq!(< f64 as Float > ::is_sign_negative(rug_fuzz_4), false);
        debug_assert_eq!(< f64 as Float > ::is_sign_negative(rug_fuzz_5), false);
        let _rug_ed_tests_llm_16_507_llm_16_507_rrrruuuugggg_test_is_sign_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_508_llm_16_508 {
    use crate::float::Float;
    #[test]
    fn test_is_sign_positive() {
        let _rug_st_tests_llm_16_508_llm_16_508_rrrruuuugggg_test_is_sign_positive = 0;
        let rug_fuzz_0 = 3.0;
        let rug_fuzz_1 = 0.0;
        let rug_fuzz_2 = 1.0;
        let rug_fuzz_3 = 0.0;
        debug_assert!(< f64 as Float > ::is_sign_positive(rug_fuzz_0));
        debug_assert!(< f64 as Float > ::is_sign_positive(rug_fuzz_1));
        debug_assert!(! < f64 as Float > ::is_sign_positive(- rug_fuzz_2));
        debug_assert!(! < f64 as Float > ::is_sign_positive(- rug_fuzz_3));
        debug_assert!(! < f64 as Float > ::is_sign_positive(f64::NEG_INFINITY));
        debug_assert!(< f64 as Float > ::is_sign_positive(f64::INFINITY));
        debug_assert!(! < f64 as Float > ::is_sign_positive(f64::NAN));
        let _rug_ed_tests_llm_16_508_llm_16_508_rrrruuuugggg_test_is_sign_positive = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_509_llm_16_509 {
    use super::*;
    use crate::*;
    #[test]
    fn test_ln() {
        let _rug_st_tests_llm_16_509_llm_16_509_rrrruuuugggg_test_ln = 0;
        let rug_fuzz_0 = 2.718282_f64;
        let rug_fuzz_1 = 1.0;
        let rug_fuzz_2 = 1e-6;
        let value = rug_fuzz_0;
        let result = value.ln();
        debug_assert!((result - rug_fuzz_1).abs() < rug_fuzz_2);
        let _rug_ed_tests_llm_16_509_llm_16_509_rrrruuuugggg_test_ln = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_510_llm_16_510 {
    use crate::float::Float;
    #[test]
    fn ln_1p_test() {
        let _rug_st_tests_llm_16_510_llm_16_510_rrrruuuugggg_ln_1p_test = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 0.5;
        let rug_fuzz_2 = 1.0;
        let rug_fuzz_3 = 0.5;
        let rug_fuzz_4 = 1e-10;
        let rug_fuzz_5 = 1e-10_f64;
        let rug_fuzz_6 = 1e-12;
        let rug_fuzz_7 = 0.9;
        let rug_fuzz_8 = 1.0;
        let x: f64 = rug_fuzz_0;
        debug_assert_eq!(< f64 as Float > ::ln_1p(x), 0.0_f64.ln_1p());
        let x: f64 = rug_fuzz_1;
        debug_assert_eq!(< f64 as Float > ::ln_1p(x), 0.5_f64.ln_1p());
        let x: f64 = rug_fuzz_2;
        debug_assert_eq!(< f64 as Float > ::ln_1p(x), 1.0_f64.ln_1p());
        let x: f64 = -rug_fuzz_3;
        debug_assert_eq!(< f64 as Float > ::ln_1p(x), (- 0.5_f64).ln_1p());
        let x: f64 = rug_fuzz_4;
        debug_assert!(
            (< f64 as Float > ::ln_1p(x) - rug_fuzz_5.ln_1p()).abs() < rug_fuzz_6
        );
        let x: f64 = -rug_fuzz_7;
        debug_assert_eq!(< f64 as Float > ::ln_1p(x), (- 0.9_f64).ln_1p());
        let x = f64::MAX;
        debug_assert!(< f64 as Float > ::ln_1p(x).is_finite());
        let x = -rug_fuzz_8;
        debug_assert!(< f64 as Float > ::ln_1p(x).is_nan());
        let _rug_ed_tests_llm_16_510_llm_16_510_rrrruuuugggg_ln_1p_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_511_llm_16_511 {
    use crate::float::Float;
    #[test]
    fn test_log() {
        let _rug_st_tests_llm_16_511_llm_16_511_rrrruuuugggg_test_log = 0;
        let rug_fuzz_0 = 10f64;
        let rug_fuzz_1 = 2f64;
        let rug_fuzz_2 = 3.321928094887362;
        let rug_fuzz_3 = 1e-15;
        let value = rug_fuzz_0;
        let base = rug_fuzz_1;
        let log_value = value.log(base);
        let expected = rug_fuzz_2;
        let epsilon = rug_fuzz_3;
        debug_assert!(
            (log_value - expected).abs() <= epsilon,
            "Value of log({}, {}) is incorrect, expected approximately {}, got {}",
            value, base, expected, log_value
        );
        let _rug_ed_tests_llm_16_511_llm_16_511_rrrruuuugggg_test_log = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_512 {
    use super::*;
    use crate::*;
    #[test]
    fn test_log10() {
        let _rug_st_tests_llm_16_512_rrrruuuugggg_test_log10 = 0;
        let rug_fuzz_0 = 1f64;
        let rug_fuzz_1 = 10f64;
        let rug_fuzz_2 = 100f64;
        let rug_fuzz_3 = 1e-10f64;
        let rug_fuzz_4 = 10.0;
        let rug_fuzz_5 = 1e-10;
        let rug_fuzz_6 = 1f64;
        let num1 = rug_fuzz_0;
        let num2 = rug_fuzz_1;
        let num3 = rug_fuzz_2;
        let num4 = rug_fuzz_3;
        debug_assert_eq!(num1.log10(), 0.0);
        debug_assert_eq!(num2.log10(), 1.0);
        debug_assert_eq!(num3.log10(), 2.0);
        debug_assert!((num4.log10() - (- rug_fuzz_4)).abs() < rug_fuzz_5);
        let num5 = -rug_fuzz_6;
        let num6 = f64::NEG_INFINITY;
        debug_assert!(num5.log10().is_nan());
        debug_assert!(num6.log10().is_nan());
        let _rug_ed_tests_llm_16_512_rrrruuuugggg_test_log10 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_514_llm_16_514 {
    use crate::float::Float;
    #[test]
    fn test_max() {
        let _rug_st_tests_llm_16_514_llm_16_514_rrrruuuugggg_test_max = 0;
        let rug_fuzz_0 = 1.5;
        let rug_fuzz_1 = 2.5;
        let a: f64 = rug_fuzz_0;
        let b: f64 = rug_fuzz_1;
        let result = <f64 as Float>::max(a, b);
        debug_assert_eq!(result, b);
        let _rug_ed_tests_llm_16_514_llm_16_514_rrrruuuugggg_test_max = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_515_llm_16_515 {
    use crate::float::Float;
    #[test]
    fn test_max_value() {
        let _rug_st_tests_llm_16_515_llm_16_515_rrrruuuugggg_test_max_value = 0;
        let max_val = <f64 as Float>::max_value();
        debug_assert_eq!(max_val, f64::MAX);
        let _rug_ed_tests_llm_16_515_llm_16_515_rrrruuuugggg_test_max_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_516_llm_16_516 {
    use crate::float::Float;
    #[test]
    fn test_min() {
        let _rug_st_tests_llm_16_516_llm_16_516_rrrruuuugggg_test_min = 0;
        let rug_fuzz_0 = 3.5f64;
        let rug_fuzz_1 = 2.2f64;
        let a = rug_fuzz_0;
        let b = rug_fuzz_1;
        let c = f64::min(a, b);
        debug_assert_eq!(c, b);
        let _rug_ed_tests_llm_16_516_llm_16_516_rrrruuuugggg_test_min = 0;
    }
    #[test]
    fn test_min_with_nan() {
        let _rug_st_tests_llm_16_516_llm_16_516_rrrruuuugggg_test_min_with_nan = 0;
        let rug_fuzz_0 = 2.2f64;
        let a = f64::NAN;
        let b = rug_fuzz_0;
        let c = f64::min(a, b);
        debug_assert_eq!(c, b);
        let _rug_ed_tests_llm_16_516_llm_16_516_rrrruuuugggg_test_min_with_nan = 0;
    }
    #[test]
    fn test_min_with_infinity() {
        let _rug_st_tests_llm_16_516_llm_16_516_rrrruuuugggg_test_min_with_infinity = 0;
        let rug_fuzz_0 = 2.2f64;
        let a = f64::INFINITY;
        let b = rug_fuzz_0;
        let c = f64::min(a, b);
        debug_assert_eq!(c, b);
        let _rug_ed_tests_llm_16_516_llm_16_516_rrrruuuugggg_test_min_with_infinity = 0;
    }
    #[test]
    fn test_min_with_neg_infinity() {
        let _rug_st_tests_llm_16_516_llm_16_516_rrrruuuugggg_test_min_with_neg_infinity = 0;
        let rug_fuzz_0 = 2.2f64;
        let a = f64::NEG_INFINITY;
        let b = rug_fuzz_0;
        let c = f64::min(a, b);
        debug_assert_eq!(c, a);
        let _rug_ed_tests_llm_16_516_llm_16_516_rrrruuuugggg_test_min_with_neg_infinity = 0;
    }
    #[test]
    fn test_min_with_equal_values() {
        let _rug_st_tests_llm_16_516_llm_16_516_rrrruuuugggg_test_min_with_equal_values = 0;
        let rug_fuzz_0 = 2.2f64;
        let rug_fuzz_1 = 2.2f64;
        let a = rug_fuzz_0;
        let b = rug_fuzz_1;
        let c = f64::min(a, b);
        debug_assert_eq!(c, a);
        debug_assert_eq!(c, b);
        let _rug_ed_tests_llm_16_516_llm_16_516_rrrruuuugggg_test_min_with_equal_values = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_517_llm_16_517 {
    use crate::float::Float;
    #[test]
    fn test_min_positive_value() {
        let _rug_st_tests_llm_16_517_llm_16_517_rrrruuuugggg_test_min_positive_value = 0;
        let min_val = <f64 as Float>::min_positive_value();
        debug_assert_eq!(min_val, f64::MIN_POSITIVE);
        let _rug_ed_tests_llm_16_517_llm_16_517_rrrruuuugggg_test_min_positive_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_518_llm_16_518 {
    use crate::Float;
    #[test]
    fn test_min_value() {
        let _rug_st_tests_llm_16_518_llm_16_518_rrrruuuugggg_test_min_value = 0;
        let rug_fuzz_0 = 0.0;
        let min_val: f64 = <f64 as Float>::min_value();
        debug_assert!(min_val.is_finite());
        debug_assert!(min_val < rug_fuzz_0);
        let _rug_ed_tests_llm_16_518_llm_16_518_rrrruuuugggg_test_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_519_llm_16_519 {
    use crate::float::Float;
    #[test]
    fn test_mul_add() {
        let _rug_st_tests_llm_16_519_llm_16_519_rrrruuuugggg_test_mul_add = 0;
        let rug_fuzz_0 = 1.0f64;
        let rug_fuzz_1 = 2.0f64;
        let rug_fuzz_2 = 3.0f64;
        let a = rug_fuzz_0;
        let b = rug_fuzz_1;
        let c = rug_fuzz_2;
        let result = <f64 as Float>::mul_add(a, b, c);
        debug_assert_eq!(result, 5.0f64);
        let _rug_ed_tests_llm_16_519_llm_16_519_rrrruuuugggg_test_mul_add = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_520_llm_16_520 {
    use crate::float::Float;
    #[test]
    fn nan_test() {
        let _rug_st_tests_llm_16_520_llm_16_520_rrrruuuugggg_nan_test = 0;
        let nan_value = <f64 as Float>::nan();
        debug_assert!(nan_value.is_nan());
        let _rug_ed_tests_llm_16_520_llm_16_520_rrrruuuugggg_nan_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_522_llm_16_522 {
    use crate::float::Float;
    #[test]
    fn test_neg_zero() {
        let _rug_st_tests_llm_16_522_llm_16_522_rrrruuuugggg_test_neg_zero = 0;
        let neg_zero = <f64 as Float>::neg_zero();
        debug_assert!(neg_zero.is_sign_negative());
        debug_assert_eq!(neg_zero, - 0.0_f64);
        let _rug_ed_tests_llm_16_522_llm_16_522_rrrruuuugggg_test_neg_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_523_llm_16_523 {
    use crate::Float;
    #[test]
    fn test_powf() {
        let _rug_st_tests_llm_16_523_llm_16_523_rrrruuuugggg_test_powf = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3.0;
        let rug_fuzz_2 = 8.0;
        let base: f64 = rug_fuzz_0;
        let exponent: f64 = rug_fuzz_1;
        let result = <f64 as Float>::powf(base, exponent);
        let expected = rug_fuzz_2;
        debug_assert_eq!(
            result, expected, "powf did not calculate {} ^ {} correctly", base, exponent
        );
        let _rug_ed_tests_llm_16_523_llm_16_523_rrrruuuugggg_test_powf = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_524_llm_16_524 {
    use crate::float::Float;
    #[test]
    fn test_powi() {
        let _rug_st_tests_llm_16_524_llm_16_524_rrrruuuugggg_test_powi = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 3;
        let value: f64 = rug_fuzz_0;
        let result = <f64 as Float>::powi(value, rug_fuzz_1);
        debug_assert_eq!(result, 8.0);
        let _rug_ed_tests_llm_16_524_llm_16_524_rrrruuuugggg_test_powi = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_525_llm_16_525 {
    use crate::float::Float;
    #[test]
    fn test_recip() {
        let _rug_st_tests_llm_16_525_llm_16_525_rrrruuuugggg_test_recip = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 0.5;
        let value: f64 = rug_fuzz_0;
        let expected_recip: f64 = rug_fuzz_1;
        let result_recip = <f64 as Float>::recip(value);
        debug_assert_eq!(expected_recip, result_recip);
        let _rug_ed_tests_llm_16_525_llm_16_525_rrrruuuugggg_test_recip = 0;
    }
    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn test_recip_zero() {
        let _rug_st_tests_llm_16_525_llm_16_525_rrrruuuugggg_test_recip_zero = 0;
        let rug_fuzz_0 = 0.0;
        let value: f64 = rug_fuzz_0;
        let _ = <f64 as Float>::recip(value);
        let _rug_ed_tests_llm_16_525_llm_16_525_rrrruuuugggg_test_recip_zero = 0;
    }
    #[test]
    fn test_recip_negative() {
        let _rug_st_tests_llm_16_525_llm_16_525_rrrruuuugggg_test_recip_negative = 0;
        let rug_fuzz_0 = 2.0;
        let rug_fuzz_1 = 0.5;
        let value: f64 = -rug_fuzz_0;
        let expected_recip: f64 = -rug_fuzz_1;
        let result_recip = <f64 as Float>::recip(value);
        debug_assert_eq!(expected_recip, result_recip);
        let _rug_ed_tests_llm_16_525_llm_16_525_rrrruuuugggg_test_recip_negative = 0;
    }
    #[test]
    fn test_recip_one() {
        let _rug_st_tests_llm_16_525_llm_16_525_rrrruuuugggg_test_recip_one = 0;
        let rug_fuzz_0 = 1.0;
        let rug_fuzz_1 = 1.0;
        let value: f64 = rug_fuzz_0;
        let expected_recip: f64 = rug_fuzz_1;
        let result_recip = <f64 as Float>::recip(value);
        debug_assert_eq!(expected_recip, result_recip);
        let _rug_ed_tests_llm_16_525_llm_16_525_rrrruuuugggg_test_recip_one = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_526_llm_16_526 {
    use super::*;
    use crate::*;
    #[test]
    fn test_round() {
        let _rug_st_tests_llm_16_526_llm_16_526_rrrruuuugggg_test_round = 0;
        let rug_fuzz_0 = 3.3_f64;
        let rug_fuzz_1 = 3.5_f64;
        let rug_fuzz_2 = 3.3_f64;
        let rug_fuzz_3 = 3.5_f64;
        let rug_fuzz_4 = 0.0_f64;
        let num = rug_fuzz_0;
        let rounded = num.round();
        debug_assert_eq!(rounded, 3.0_f64);
        let num = rug_fuzz_1;
        let rounded = num.round();
        debug_assert_eq!(rounded, 4.0_f64);
        let num = -rug_fuzz_2;
        let rounded = num.round();
        debug_assert_eq!(rounded, - 3.0_f64);
        let num = -rug_fuzz_3;
        let rounded = num.round();
        debug_assert_eq!(rounded, - 4.0_f64);
        let num = rug_fuzz_4;
        let rounded = num.round();
        debug_assert_eq!(rounded, 0.0_f64);
        let _rug_ed_tests_llm_16_526_llm_16_526_rrrruuuugggg_test_round = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_527_llm_16_527 {
    use crate::Float;
    #[test]
    fn test_signum_positive() {
        let _rug_st_tests_llm_16_527_llm_16_527_rrrruuuugggg_test_signum_positive = 0;
        let rug_fuzz_0 = 3.14;
        let pos_value: f64 = rug_fuzz_0;
        debug_assert_eq!(pos_value.signum(), 1.0);
        let _rug_ed_tests_llm_16_527_llm_16_527_rrrruuuugggg_test_signum_positive = 0;
    }
    #[test]
    fn test_signum_negative() {
        let _rug_st_tests_llm_16_527_llm_16_527_rrrruuuugggg_test_signum_negative = 0;
        let rug_fuzz_0 = 3.14;
        let neg_value: f64 = -rug_fuzz_0;
        debug_assert_eq!(neg_value.signum(), - 1.0);
        let _rug_ed_tests_llm_16_527_llm_16_527_rrrruuuugggg_test_signum_negative = 0;
    }
    #[test]
    fn test_signum_zero_positive() {
        let _rug_st_tests_llm_16_527_llm_16_527_rrrruuuugggg_test_signum_zero_positive = 0;
        let rug_fuzz_0 = 0.0;
        let zero_pos: f64 = rug_fuzz_0;
        debug_assert_eq!(zero_pos.signum(), 0.0);
        let _rug_ed_tests_llm_16_527_llm_16_527_rrrruuuugggg_test_signum_zero_positive = 0;
    }
    #[test]
    fn test_signum_zero_negative() {
        let _rug_st_tests_llm_16_527_llm_16_527_rrrruuuugggg_test_signum_zero_negative = 0;
        let rug_fuzz_0 = 0.0;
        let zero_neg: f64 = -rug_fuzz_0;
        debug_assert_eq!(zero_neg.signum(), 0.0);
        let _rug_ed_tests_llm_16_527_llm_16_527_rrrruuuugggg_test_signum_zero_negative = 0;
    }
    #[test]
    fn test_signum_nan() {
        let _rug_st_tests_llm_16_527_llm_16_527_rrrruuuugggg_test_signum_nan = 0;
        let nan: f64 = f64::NAN;
        debug_assert!(nan.signum().is_nan());
        let _rug_ed_tests_llm_16_527_llm_16_527_rrrruuuugggg_test_signum_nan = 0;
    }
    #[test]
    fn test_signum_infinity_positive() {
        let _rug_st_tests_llm_16_527_llm_16_527_rrrruuuugggg_test_signum_infinity_positive = 0;
        let infinity_pos: f64 = f64::INFINITY;
        debug_assert_eq!(infinity_pos.signum(), 1.0);
        let _rug_ed_tests_llm_16_527_llm_16_527_rrrruuuugggg_test_signum_infinity_positive = 0;
    }
    #[test]
    fn test_signum_infinity_negative() {
        let _rug_st_tests_llm_16_527_llm_16_527_rrrruuuugggg_test_signum_infinity_negative = 0;
        let infinity_neg: f64 = f64::NEG_INFINITY;
        debug_assert_eq!(infinity_neg.signum(), - 1.0);
        let _rug_ed_tests_llm_16_527_llm_16_527_rrrruuuugggg_test_signum_infinity_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_528_llm_16_528 {
    use crate::float::Float;
    #[test]
    fn test_sin() {
        let _rug_st_tests_llm_16_528_llm_16_528_rrrruuuugggg_test_sin = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 0.0f64;
        let rug_fuzz_2 = 2.0;
        let rug_fuzz_3 = 2.0;
        let rug_fuzz_4 = 2.0;
        let rug_fuzz_5 = 2.0;
        let value: f64 = rug_fuzz_0;
        let result = <f64 as Float>::sin(value);
        let expected = rug_fuzz_1.sin();
        debug_assert_eq!(result, expected);
        let value: f64 = std::f64::consts::PI;
        let result = <f64 as Float>::sin(value);
        let expected = std::f64::consts::PI.sin();
        debug_assert_eq!(result, expected);
        let value: f64 = std::f64::consts::PI / rug_fuzz_2;
        let result = <f64 as Float>::sin(value);
        let expected = (std::f64::consts::PI / rug_fuzz_3).sin();
        debug_assert_eq!(result, expected);
        let value: f64 = -std::f64::consts::PI / rug_fuzz_4;
        let result = <f64 as Float>::sin(value);
        let expected = (-std::f64::consts::PI / rug_fuzz_5).sin();
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_528_llm_16_528_rrrruuuugggg_test_sin = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_529_llm_16_529 {
    use crate::float::Float;
    #[test]
    fn sin_cos_test() {
        let _rug_st_tests_llm_16_529_llm_16_529_rrrruuuugggg_sin_cos_test = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 0.0;
        let rug_fuzz_2 = 1.0;
        let rug_fuzz_3 = 2.0;
        let rug_fuzz_4 = 1.0;
        let rug_fuzz_5 = 1.0;
        let input: f64 = rug_fuzz_0;
        let (sin, cos) = <f64 as Float>::sin_cos(input);
        debug_assert!((sin - rug_fuzz_1).abs() < f64::EPSILON);
        debug_assert!((cos - rug_fuzz_2).abs() < f64::EPSILON);
        let input: f64 = std::f64::consts::PI / rug_fuzz_3;
        let (sin, cos) = <f64 as Float>::sin_cos(input);
        debug_assert!((sin - rug_fuzz_4).abs() < f64::EPSILON);
        debug_assert!(cos.abs() < f64::EPSILON);
        let input: f64 = std::f64::consts::PI;
        let (sin, cos) = <f64 as Float>::sin_cos(input);
        debug_assert!(sin.abs() < f64::EPSILON);
        debug_assert!((cos - - rug_fuzz_5).abs() < f64::EPSILON);
        let _rug_ed_tests_llm_16_529_llm_16_529_rrrruuuugggg_sin_cos_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_530_llm_16_530 {
    use super::*;
    use crate::*;
    #[test]
    fn sinh_test() {
        let _rug_st_tests_llm_16_530_llm_16_530_rrrruuuugggg_sinh_test = 0;
        let rug_fuzz_0 = 1.0;
        let x: f64 = rug_fuzz_0;
        let expected = x.sinh();
        let result = <f64 as Float>::sinh(x);
        debug_assert_eq!(expected, result);
        let _rug_ed_tests_llm_16_530_llm_16_530_rrrruuuugggg_sinh_test = 0;
    }
    #[test]
    fn sinh_test_negative() {
        let _rug_st_tests_llm_16_530_llm_16_530_rrrruuuugggg_sinh_test_negative = 0;
        let rug_fuzz_0 = 1.0;
        let x: f64 = -rug_fuzz_0;
        let expected = x.sinh();
        let result = <f64 as Float>::sinh(x);
        debug_assert_eq!(expected, result);
        let _rug_ed_tests_llm_16_530_llm_16_530_rrrruuuugggg_sinh_test_negative = 0;
    }
    #[test]
    fn sinh_test_zero() {
        let _rug_st_tests_llm_16_530_llm_16_530_rrrruuuugggg_sinh_test_zero = 0;
        let rug_fuzz_0 = 0.0;
        let x: f64 = rug_fuzz_0;
        let expected = x.sinh();
        let result = <f64 as Float>::sinh(x);
        debug_assert_eq!(expected, result);
        let _rug_ed_tests_llm_16_530_llm_16_530_rrrruuuugggg_sinh_test_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_531_llm_16_531 {
    use crate::float::Float;
    #[test]
    fn test_sqrt() {
        let _rug_st_tests_llm_16_531_llm_16_531_rrrruuuugggg_test_sqrt = 0;
        let rug_fuzz_0 = 4.0_f64;
        let rug_fuzz_1 = 4.0_f64;
        let rug_fuzz_2 = 0.0_f64;
        let rug_fuzz_3 = 1.0_f64;
        let num_pos = rug_fuzz_0;
        let num_neg = -rug_fuzz_1;
        let zero = rug_fuzz_2;
        let one = rug_fuzz_3;
        let sqrt_pos = num_pos.sqrt();
        let sqrt_neg = num_neg.sqrt();
        let sqrt_zero = zero.sqrt();
        let sqrt_one = one.sqrt();
        debug_assert_eq!(sqrt_pos, 2.0_f64);
        debug_assert!(sqrt_neg.is_nan());
        debug_assert_eq!(sqrt_zero, 0.0_f64);
        debug_assert_eq!(sqrt_one, 1.0_f64);
        let _rug_ed_tests_llm_16_531_llm_16_531_rrrruuuugggg_test_sqrt = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_533_llm_16_533 {
    use crate::float::Float;
    #[test]
    fn tanh_test() {
        let _rug_st_tests_llm_16_533_llm_16_533_rrrruuuugggg_tanh_test = 0;
        let rug_fuzz_0 = 0.5;
        let rug_fuzz_1 = 1e-10;
        let value: f64 = rug_fuzz_0;
        let result = <f64 as Float>::tanh(value);
        let expected = value.tanh();
        debug_assert!((result - expected).abs() < rug_fuzz_1);
        let _rug_ed_tests_llm_16_533_llm_16_533_rrrruuuugggg_tanh_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_534_llm_16_534 {
    use crate::float::Float;
    #[test]
    fn test_to_degrees() {
        let _rug_st_tests_llm_16_534_llm_16_534_rrrruuuugggg_test_to_degrees = 0;
        let rug_fuzz_0 = 0.0_f64;
        let rug_fuzz_1 = 2.0;
        let rug_fuzz_2 = 0.5;
        let rug_fuzz_3 = 1.0_f64;
        let rug_fuzz_4 = 57.29577951308232;
        let pi = std::f64::consts::PI;
        let zero_rad = rug_fuzz_0;
        let pi_rad = pi;
        let two_pi_rad = rug_fuzz_1 * pi;
        let half_pi_rad = rug_fuzz_2 * pi;
        let zero_deg = zero_rad.to_degrees();
        let pi_deg = pi_rad.to_degrees();
        let two_pi_deg = two_pi_rad.to_degrees();
        let half_pi_deg = half_pi_rad.to_degrees();
        debug_assert_eq!(zero_deg, 0.0);
        debug_assert_eq!(pi_deg, 180.0);
        debug_assert_eq!(two_pi_deg, 360.0);
        debug_assert_eq!(half_pi_deg, 90.0);
        let one_rad = rug_fuzz_3;
        let one_deg = one_rad.to_degrees();
        debug_assert!((one_deg - rug_fuzz_4).abs() < std::f64::EPSILON);
        let _rug_ed_tests_llm_16_534_llm_16_534_rrrruuuugggg_test_to_degrees = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_535_llm_16_535 {
    use crate::float::Float;
    #[test]
    fn test_to_radians() {
        let _rug_st_tests_llm_16_535_llm_16_535_rrrruuuugggg_test_to_radians = 0;
        let rug_fuzz_0 = 0.0_f64;
        let rug_fuzz_1 = 90.0_f64;
        let rug_fuzz_2 = 180.0_f64;
        let rug_fuzz_3 = 360.0_f64;
        let rug_fuzz_4 = 1e-10;
        let rug_fuzz_5 = 0.0;
        let rug_fuzz_6 = 2.0;
        let rug_fuzz_7 = 2.0;
        let pi = std::f64::consts::PI;
        let degree_0 = rug_fuzz_0;
        let degree_90 = rug_fuzz_1;
        let degree_180 = rug_fuzz_2;
        let degree_360 = rug_fuzz_3;
        let radian_0 = degree_0.to_radians();
        let radian_90 = degree_90.to_radians();
        let radian_180 = degree_180.to_radians();
        let radian_360 = degree_360.to_radians();
        let epsilon = rug_fuzz_4;
        debug_assert!(
            (radian_0 - rug_fuzz_5).abs() < epsilon,
            "0 degrees should convert to 0 radians."
        );
        debug_assert!(
            (radian_90 - pi / rug_fuzz_6).abs() < epsilon,
            "90 degrees should convert to PI/2 radians."
        );
        debug_assert!(
            (radian_180 - pi).abs() < epsilon,
            "180 degrees should convert to PI radians."
        );
        debug_assert!(
            (radian_360 - rug_fuzz_7 * pi).abs() < epsilon,
            "360 degrees should convert to 2*PI radians."
        );
        let _rug_ed_tests_llm_16_535_llm_16_535_rrrruuuugggg_test_to_radians = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_536_llm_16_536 {
    use crate::float::Float;
    #[test]
    fn test_trunc() {
        let _rug_st_tests_llm_16_536_llm_16_536_rrrruuuugggg_test_trunc = 0;
        let rug_fuzz_0 = 3.14;
        let rug_fuzz_1 = 3.14;
        let rug_fuzz_2 = 0.0;
        let rug_fuzz_3 = 0.0;
        debug_assert_eq!(< f64 as Float > ::trunc(rug_fuzz_0), 3.0);
        debug_assert_eq!(< f64 as Float > ::trunc(- rug_fuzz_1), - 3.0);
        debug_assert_eq!(< f64 as Float > ::trunc(rug_fuzz_2), 0.0);
        debug_assert_eq!(< f64 as Float > ::trunc(- rug_fuzz_3), - 0.0);
        debug_assert_eq!(< f64 as Float > ::trunc(f64::INFINITY), f64::INFINITY);
        debug_assert_eq!(< f64 as Float > ::trunc(f64::NEG_INFINITY), f64::NEG_INFINITY);
        debug_assert!(< f64 as Float > ::trunc(f64::NAN).is_nan());
        let _rug_ed_tests_llm_16_536_llm_16_536_rrrruuuugggg_test_trunc = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_537_llm_16_537 {
    use crate::float::FloatConst;
    #[test]
    fn test_f64_e() {
        let _rug_st_tests_llm_16_537_llm_16_537_rrrruuuugggg_test_f64_e = 0;
        let rug_fuzz_0 = 2.718281828459045;
        let e = f64::E();
        let known_e: f64 = rug_fuzz_0;
        debug_assert!((e - known_e).abs() < f64::EPSILON);
        let _rug_ed_tests_llm_16_537_llm_16_537_rrrruuuugggg_test_f64_e = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_538_llm_16_538 {
    use crate::float::FloatConst;
    #[test]
    fn test_frac_1_pi() {
        let _rug_st_tests_llm_16_538_llm_16_538_rrrruuuugggg_test_frac_1_pi = 0;
        let rug_fuzz_0 = 1.0;
        let result = <f64 as FloatConst>::FRAC_1_PI();
        let expected = rug_fuzz_0 / std::f64::consts::PI;
        debug_assert!((result - expected).abs() < std::f64::EPSILON);
        let _rug_ed_tests_llm_16_538_llm_16_538_rrrruuuugggg_test_frac_1_pi = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_539_llm_16_539 {
    use crate::float::FloatConst;
    #[test]
    fn frac_1_sqrt_2_test() {
        let _rug_st_tests_llm_16_539_llm_16_539_rrrruuuugggg_frac_1_sqrt_2_test = 0;
        let rug_fuzz_0 = 1f64;
        let rug_fuzz_1 = 2f64;
        let value = f64::FRAC_1_SQRT_2();
        let expected = rug_fuzz_0 / rug_fuzz_1.sqrt();
        debug_assert_eq!(value, expected);
        let _rug_ed_tests_llm_16_539_llm_16_539_rrrruuuugggg_frac_1_sqrt_2_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_544 {
    use crate::float::FloatConst;
    #[test]
    fn test_frac_pi_4() {
        let _rug_st_tests_llm_16_544_rrrruuuugggg_test_frac_pi_4 = 0;
        let frac_pi_4 = f64::FRAC_PI_4();
        let expected = std::f64::consts::FRAC_PI_4;
        debug_assert_eq!(frac_pi_4, expected);
        let _rug_ed_tests_llm_16_544_rrrruuuugggg_test_frac_pi_4 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_547_llm_16_547 {
    use crate::float::FloatConst;
    #[test]
    fn ln_10_test() {
        let _rug_st_tests_llm_16_547_llm_16_547_rrrruuuugggg_ln_10_test = 0;
        let ln_10 = f64::LN_10();
        let known_ln_10 = std::f64::consts::LN_10;
        debug_assert_eq!(ln_10, known_ln_10);
        let _rug_ed_tests_llm_16_547_llm_16_547_rrrruuuugggg_ln_10_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_548_llm_16_548 {
    use crate::float::FloatConst;
    #[test]
    fn test_ln_2() {
        let _rug_st_tests_llm_16_548_llm_16_548_rrrruuuugggg_test_ln_2 = 0;
        let ln_2 = <f64 as FloatConst>::LN_2();
        let expected = std::f64::consts::LN_2;
        debug_assert_eq!(ln_2, expected);
        let _rug_ed_tests_llm_16_548_llm_16_548_rrrruuuugggg_test_ln_2 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_552_llm_16_552 {
    use crate::FloatConst;
    #[test]
    fn log2_e_test() {
        let _rug_st_tests_llm_16_552_llm_16_552_rrrruuuugggg_log2_e_test = 0;
        const EXPECTED: f64 = std::f64::consts::LOG2_E;
        let result = <f64 as FloatConst>::LOG2_E();
        debug_assert!((result - EXPECTED).abs() < std::f64::EPSILON);
        let _rug_ed_tests_llm_16_552_llm_16_552_rrrruuuugggg_log2_e_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_556_llm_16_556 {
    use crate::float::FloatCore;
    #[test]
    fn test_abs() {
        let _rug_st_tests_llm_16_556_llm_16_556_rrrruuuugggg_test_abs = 0;
        let rug_fuzz_0 = 3.0;
        let rug_fuzz_1 = 3.0;
        let rug_fuzz_2 = 0.0;
        let rug_fuzz_3 = 0.0;
        debug_assert_eq!(< f64 as FloatCore > ::abs(- rug_fuzz_0), 3.0);
        debug_assert_eq!(< f64 as FloatCore > ::abs(rug_fuzz_1), 3.0);
        debug_assert_eq!(< f64 as FloatCore > ::abs(rug_fuzz_2), 0.0);
        debug_assert_eq!(< f64 as FloatCore > ::abs(- rug_fuzz_3), 0.0);
        debug_assert_eq!(
            < f64 as FloatCore > ::abs(std::f64::INFINITY), std::f64::INFINITY
        );
        debug_assert_eq!(
            < f64 as FloatCore > ::abs(std::f64::NEG_INFINITY), std::f64::INFINITY
        );
        debug_assert!(< f64 as FloatCore > ::abs(std::f64::NAN).is_nan());
        let _rug_ed_tests_llm_16_556_llm_16_556_rrrruuuugggg_test_abs = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_557_llm_16_557 {
    use super::*;
    use crate::*;
    #[test]
    fn ceil_test() {
        let _rug_st_tests_llm_16_557_llm_16_557_rrrruuuugggg_ceil_test = 0;
        let rug_fuzz_0 = 3.2;
        let rug_fuzz_1 = 3.0;
        let rug_fuzz_2 = 3.0;
        let rug_fuzz_3 = 3.2;
        let rug_fuzz_4 = 0.0;
        let rug_fuzz_5 = 0.0;
        debug_assert_eq!(f64::ceil(- rug_fuzz_0), - 3.0);
        debug_assert_eq!(f64::ceil(- rug_fuzz_1), - 3.0);
        debug_assert_eq!(f64::ceil(rug_fuzz_2), 3.0);
        debug_assert_eq!(f64::ceil(rug_fuzz_3), 4.0);
        debug_assert_eq!(f64::ceil(rug_fuzz_4), 0.0);
        debug_assert_eq!(f64::ceil(- rug_fuzz_5), - 0.0);
        debug_assert_eq!(f64::ceil(f64::INFINITY), f64::INFINITY);
        debug_assert_eq!(f64::ceil(f64::NEG_INFINITY), f64::NEG_INFINITY);
        debug_assert!(f64::ceil(f64::NAN).is_nan());
        let _rug_ed_tests_llm_16_557_llm_16_557_rrrruuuugggg_ceil_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_558_llm_16_558 {
    use crate::float::FloatCore;
    use std::num::FpCategory::*;
    #[test]
    fn test_classify() {
        let _rug_st_tests_llm_16_558_llm_16_558_rrrruuuugggg_test_classify = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 0.0;
        let rug_fuzz_2 = 1.0;
        let rug_fuzz_3 = 1.0;
        let rug_fuzz_4 = 2.0f64;
        let rug_fuzz_5 = 1023;
        debug_assert_eq!(f64::classify(rug_fuzz_0), Zero);
        debug_assert_eq!(f64::classify(- rug_fuzz_1), Zero);
        debug_assert_eq!(f64::classify(rug_fuzz_2), Normal);
        debug_assert_eq!(f64::classify(- rug_fuzz_3), Normal);
        debug_assert_eq!(f64::classify(f64::INFINITY), Infinite);
        debug_assert_eq!(f64::classify(f64::NEG_INFINITY), Infinite);
        debug_assert_eq!(f64::classify(f64::NAN), Nan);
        debug_assert_eq!(f64::classify(f64::MIN), Normal);
        debug_assert_eq!(f64::classify(f64::MAX), Normal);
        debug_assert_eq!(f64::classify(f64::EPSILON), Normal);
        let subnormal = f64::MIN_POSITIVE / rug_fuzz_4.powi(rug_fuzz_5);
        debug_assert_eq!(f64::classify(subnormal), Subnormal);
        let _rug_ed_tests_llm_16_558_llm_16_558_rrrruuuugggg_test_classify = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_559_llm_16_559 {
    use crate::float::FloatCore;
    #[test]
    fn epsilon_f64() {
        let _rug_st_tests_llm_16_559_llm_16_559_rrrruuuugggg_epsilon_f64 = 0;
        let eps = f64::epsilon();
        debug_assert_eq!(eps, std::f64::EPSILON);
        let _rug_ed_tests_llm_16_559_llm_16_559_rrrruuuugggg_epsilon_f64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_560_llm_16_560 {
    use crate::float::FloatCore;
    #[test]
    fn test_floor() {
        let _rug_st_tests_llm_16_560_llm_16_560_rrrruuuugggg_test_floor = 0;
        let rug_fuzz_0 = 3.3;
        let rug_fuzz_1 = 3.0;
        let rug_fuzz_2 = 3.3;
        let rug_fuzz_3 = 3.0;
        debug_assert_eq!(< f64 as FloatCore > ::floor(rug_fuzz_0), 3.0);
        debug_assert_eq!(< f64 as FloatCore > ::floor(rug_fuzz_1), 3.0);
        debug_assert_eq!(< f64 as FloatCore > ::floor(- rug_fuzz_2), - 4.0);
        debug_assert_eq!(< f64 as FloatCore > ::floor(- rug_fuzz_3), - 3.0);
        let _rug_ed_tests_llm_16_560_llm_16_560_rrrruuuugggg_test_floor = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_561_llm_16_561 {
    use crate::float::FloatCore;
    #[test]
    fn test_fract() {
        let _rug_st_tests_llm_16_561_llm_16_561_rrrruuuugggg_test_fract = 0;
        let rug_fuzz_0 = 3.5_f64;
        let rug_fuzz_1 = 3.5_f64;
        let rug_fuzz_2 = 4.0_f64;
        let num1 = rug_fuzz_0;
        let num2 = -rug_fuzz_1;
        let num3 = rug_fuzz_2;
        let fract1 = num1.fract();
        let fract2 = num2.fract();
        let fract3 = num3.fract();
        debug_assert_eq!(fract1, 0.5_f64);
        debug_assert_eq!(fract2, - 0.5_f64);
        debug_assert_eq!(fract3, 0.0_f64);
        let _rug_ed_tests_llm_16_561_llm_16_561_rrrruuuugggg_test_fract = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_562_llm_16_562 {
    use crate::float::FloatCore;
    #[test]
    fn test_infinity() {
        let _rug_st_tests_llm_16_562_llm_16_562_rrrruuuugggg_test_infinity = 0;
        let inf = <f64 as FloatCore>::infinity();
        debug_assert!(inf.is_infinite());
        debug_assert!(inf.is_sign_positive());
        let _rug_ed_tests_llm_16_562_llm_16_562_rrrruuuugggg_test_infinity = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_564_llm_16_564 {
    use crate::float::FloatCore;
    #[test]
    fn test_is_finite() {
        let _rug_st_tests_llm_16_564_llm_16_564_rrrruuuugggg_test_is_finite = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 0.0;
        let rug_fuzz_2 = 1.0;
        debug_assert_eq!(< f64 as FloatCore > ::is_finite(rug_fuzz_0), true);
        debug_assert_eq!(< f64 as FloatCore > ::is_finite(- rug_fuzz_1), true);
        debug_assert_eq!(< f64 as FloatCore > ::is_finite(rug_fuzz_2), true);
        debug_assert_eq!(< f64 as FloatCore > ::is_finite(f64::INFINITY), false);
        debug_assert_eq!(< f64 as FloatCore > ::is_finite(f64::NEG_INFINITY), false);
        debug_assert_eq!(< f64 as FloatCore > ::is_finite(f64::NAN), false);
        let _rug_ed_tests_llm_16_564_llm_16_564_rrrruuuugggg_test_is_finite = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_565_llm_16_565 {
    use crate::float::FloatCore;
    #[test]
    fn test_is_infinite() {
        let _rug_st_tests_llm_16_565_llm_16_565_rrrruuuugggg_test_is_infinite = 0;
        let rug_fuzz_0 = 0f64;
        let rug_fuzz_1 = 1f64;
        let rug_fuzz_2 = 1f64;
        debug_assert!(f64::INFINITY.is_infinite());
        debug_assert!(f64::NEG_INFINITY.is_infinite());
        debug_assert!(! f64::NAN.is_infinite());
        debug_assert!(! rug_fuzz_0.is_infinite());
        debug_assert!(! rug_fuzz_1.is_infinite());
        debug_assert!(! (- rug_fuzz_2).is_infinite());
        let _rug_ed_tests_llm_16_565_llm_16_565_rrrruuuugggg_test_is_infinite = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_566_llm_16_566 {
    use crate::float::FloatCore;
    #[test]
    fn test_is_nan() {
        let _rug_st_tests_llm_16_566_llm_16_566_rrrruuuugggg_test_is_nan = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 0.0;
        let rug_fuzz_2 = 1.0;
        let rug_fuzz_3 = 1.0;
        debug_assert_eq!(< f64 as FloatCore > ::is_nan(f64::NAN), true);
        debug_assert_eq!(< f64 as FloatCore > ::is_nan(rug_fuzz_0), false);
        debug_assert_eq!(< f64 as FloatCore > ::is_nan(- rug_fuzz_1), false);
        debug_assert_eq!(< f64 as FloatCore > ::is_nan(f64::INFINITY), false);
        debug_assert_eq!(< f64 as FloatCore > ::is_nan(f64::NEG_INFINITY), false);
        debug_assert_eq!(< f64 as FloatCore > ::is_nan(rug_fuzz_2), false);
        debug_assert_eq!(< f64 as FloatCore > ::is_nan(- rug_fuzz_3), false);
        let _rug_ed_tests_llm_16_566_llm_16_566_rrrruuuugggg_test_is_nan = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_567_llm_16_567 {
    use crate::float::FloatCore;
    #[test]
    fn test_is_normal() {
        let _rug_st_tests_llm_16_567_llm_16_567_rrrruuuugggg_test_is_normal = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 0.0;
        let rug_fuzz_2 = 1.0;
        let rug_fuzz_3 = 1.0;
        let rug_fuzz_4 = 2.0;
        debug_assert_eq!(< f64 as FloatCore > ::is_normal(rug_fuzz_0), false);
        debug_assert_eq!(< f64 as FloatCore > ::is_normal(- rug_fuzz_1), false);
        debug_assert_eq!(< f64 as FloatCore > ::is_normal(rug_fuzz_2), true);
        debug_assert_eq!(< f64 as FloatCore > ::is_normal(- rug_fuzz_3), true);
        debug_assert_eq!(< f64 as FloatCore > ::is_normal(f64::INFINITY), false);
        debug_assert_eq!(< f64 as FloatCore > ::is_normal(f64::NEG_INFINITY), false);
        debug_assert_eq!(< f64 as FloatCore > ::is_normal(f64::NAN), false);
        debug_assert_eq!(< f64 as FloatCore > ::is_normal(f64::MIN_POSITIVE), false);
        debug_assert_eq!(
            < f64 as FloatCore > ::is_normal(f64::MIN_POSITIVE * rug_fuzz_4), true
        );
        debug_assert_eq!(< f64 as FloatCore > ::is_normal(f64::MAX), true);
        debug_assert_eq!(< f64 as FloatCore > ::is_normal(f64::EPSILON), false);
        let _rug_ed_tests_llm_16_567_llm_16_567_rrrruuuugggg_test_is_normal = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_568_llm_16_568 {
    use crate::float::FloatCore;
    #[test]
    fn test_is_sign_negative() {
        let _rug_st_tests_llm_16_568_llm_16_568_rrrruuuugggg_test_is_sign_negative = 0;
        let rug_fuzz_0 = 0.0;
        let rug_fuzz_1 = 1.0;
        let rug_fuzz_2 = 0.0;
        let rug_fuzz_3 = 1.0;
        debug_assert!(< f64 as FloatCore > ::is_sign_negative(- rug_fuzz_0));
        debug_assert!(< f64 as FloatCore > ::is_sign_negative(- rug_fuzz_1));
        debug_assert!(< f64 as FloatCore > ::is_sign_negative(- std::f64::MIN));
        debug_assert!(! < f64 as FloatCore > ::is_sign_negative(rug_fuzz_2));
        debug_assert!(! < f64 as FloatCore > ::is_sign_negative(rug_fuzz_3));
        debug_assert!(! < f64 as FloatCore > ::is_sign_negative(std::f64::MAX));
        let _rug_ed_tests_llm_16_568_llm_16_568_rrrruuuugggg_test_is_sign_negative = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_569_llm_16_569 {
    use crate::float::FloatCore;
    #[test]
    fn test_is_sign_positive() {
        let _rug_st_tests_llm_16_569_llm_16_569_rrrruuuugggg_test_is_sign_positive = 0;
        let rug_fuzz_0 = 3.14;
        let rug_fuzz_1 = 0.0;
        let rug_fuzz_2 = 3.14;
        debug_assert!(< f64 as FloatCore > ::is_sign_positive(rug_fuzz_0));
        debug_assert!(< f64 as FloatCore > ::is_sign_positive(rug_fuzz_1));
        debug_assert!(! < f64 as FloatCore > ::is_sign_positive(- rug_fuzz_2));
        let _rug_ed_tests_llm_16_569_llm_16_569_rrrruuuugggg_test_is_sign_positive = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_571_llm_16_571 {
    use crate::float::FloatCore;
    #[test]
    fn test_max_value() {
        let _rug_st_tests_llm_16_571_llm_16_571_rrrruuuugggg_test_max_value = 0;
        let max_val = <f64 as FloatCore>::max_value();
        debug_assert_eq!(max_val, std::f64::MAX);
        let _rug_ed_tests_llm_16_571_llm_16_571_rrrruuuugggg_test_max_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_572_llm_16_572 {
    use crate::float::FloatCore;
    #[test]
    fn test_min() {
        let _rug_st_tests_llm_16_572_llm_16_572_rrrruuuugggg_test_min = 0;
        let rug_fuzz_0 = 1.0;
        let rug_fuzz_1 = 2.0;
        let rug_fuzz_2 = 1.0;
        let rug_fuzz_3 = 2.0;
        let rug_fuzz_4 = 0.0;
        let rug_fuzz_5 = 0.0;
        let rug_fuzz_6 = 2.0;
        let rug_fuzz_7 = 2.0;
        let rug_fuzz_8 = 2.0;
        let rug_fuzz_9 = 2.0;
        debug_assert_eq!(< f64 as FloatCore > ::min(rug_fuzz_0, rug_fuzz_1), 1.0);
        debug_assert_eq!(< f64 as FloatCore > ::min(- rug_fuzz_2, rug_fuzz_3), - 1.0);
        debug_assert_eq!(< f64 as FloatCore > ::min(rug_fuzz_4, rug_fuzz_5), 0.0);
        debug_assert_eq!(< f64 as FloatCore > ::min(f64::INFINITY, rug_fuzz_6), 2.0);
        debug_assert_eq!(
            < f64 as FloatCore > ::min(f64::NEG_INFINITY, rug_fuzz_7), f64::NEG_INFINITY
        );
        debug_assert!(< f64 as FloatCore > ::min(f64::NAN, rug_fuzz_8).is_nan());
        debug_assert_eq!(< f64 as FloatCore > ::min(rug_fuzz_9, f64::NAN), 2.0);
        let _rug_ed_tests_llm_16_572_llm_16_572_rrrruuuugggg_test_min = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_573_llm_16_573 {
    use crate::float::FloatCore;
    #[test]
    fn test_min_positive_value() {
        let _rug_st_tests_llm_16_573_llm_16_573_rrrruuuugggg_test_min_positive_value = 0;
        let min_val = <f64 as FloatCore>::min_positive_value();
        debug_assert_eq!(min_val, std::f64::MIN_POSITIVE);
        let _rug_ed_tests_llm_16_573_llm_16_573_rrrruuuugggg_test_min_positive_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_574_llm_16_574 {
    use crate::float::FloatCore;
    #[test]
    fn test_min_value() {
        let _rug_st_tests_llm_16_574_llm_16_574_rrrruuuugggg_test_min_value = 0;
        let min_val = <f64 as FloatCore>::min_value();
        debug_assert_eq!(min_val, f64::MIN);
        let _rug_ed_tests_llm_16_574_llm_16_574_rrrruuuugggg_test_min_value = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_575_llm_16_575 {
    use crate::float::FloatCore;
    #[test]
    fn nan_test() {
        let _rug_st_tests_llm_16_575_llm_16_575_rrrruuuugggg_nan_test = 0;
        let nan = <f64 as FloatCore>::nan();
        debug_assert!(nan.is_nan());
        debug_assert!(! (nan == nan));
        let _rug_ed_tests_llm_16_575_llm_16_575_rrrruuuugggg_nan_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_576_llm_16_576 {
    use crate::float::FloatCore;
    #[test]
    fn test_neg_infinity() {
        let _rug_st_tests_llm_16_576_llm_16_576_rrrruuuugggg_test_neg_infinity = 0;
        let neg_inf = <f64 as FloatCore>::neg_infinity();
        debug_assert!(neg_inf.is_infinite() && neg_inf.is_sign_negative());
        let _rug_ed_tests_llm_16_576_llm_16_576_rrrruuuugggg_test_neg_infinity = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_577_llm_16_577 {
    use crate::float::FloatCore;
    #[test]
    fn test_neg_zero() {
        let _rug_st_tests_llm_16_577_llm_16_577_rrrruuuugggg_test_neg_zero = 0;
        let neg_zero = <f64 as FloatCore>::neg_zero();
        debug_assert!(neg_zero.is_sign_negative());
        debug_assert_eq!(neg_zero, - 0.0f64);
        let _rug_ed_tests_llm_16_577_llm_16_577_rrrruuuugggg_test_neg_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_578_llm_16_578 {
    use crate::float::FloatCore;
    #[test]
    fn test_powi() {
        let _rug_st_tests_llm_16_578_llm_16_578_rrrruuuugggg_test_powi = 0;
        let rug_fuzz_0 = 2.0f64;
        let rug_fuzz_1 = 3;
        let rug_fuzz_2 = 8.0f64;
        let rug_fuzz_3 = 5.0f64;
        let rug_fuzz_4 = 2;
        let rug_fuzz_5 = 0.04f64;
        let rug_fuzz_6 = 1e-10;
        let rug_fuzz_7 = 2.0f64;
        let rug_fuzz_8 = 0;
        let rug_fuzz_9 = 1.0f64;
        let rug_fuzz_10 = 3.0f64;
        let rug_fuzz_11 = 3;
        let rug_fuzz_12 = 27.0f64;
        let rug_fuzz_13 = 2.0f64;
        let rug_fuzz_14 = 2;
        let rug_fuzz_15 = 4.0f64;
        let a = rug_fuzz_0;
        let b = rug_fuzz_1;
        let result = <f64 as FloatCore>::powi(a, b);
        let expected = rug_fuzz_2;
        debug_assert_eq!(result, expected);
        let a = rug_fuzz_3;
        let b = -rug_fuzz_4;
        let result = <f64 as FloatCore>::powi(a, b);
        let expected = rug_fuzz_5;
        debug_assert!((result - expected).abs() < rug_fuzz_6);
        let a = rug_fuzz_7;
        let b = rug_fuzz_8;
        let result = <f64 as FloatCore>::powi(a, b);
        let expected = rug_fuzz_9;
        debug_assert_eq!(result, expected);
        let a = -rug_fuzz_10;
        let b = rug_fuzz_11;
        let result = <f64 as FloatCore>::powi(a, b);
        let expected = -rug_fuzz_12;
        debug_assert_eq!(result, expected);
        let a = -rug_fuzz_13;
        let b = rug_fuzz_14;
        let result = <f64 as FloatCore>::powi(a, b);
        let expected = rug_fuzz_15;
        debug_assert_eq!(result, expected);
        let _rug_ed_tests_llm_16_578_llm_16_578_rrrruuuugggg_test_powi = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_580_llm_16_580 {
    use crate::float::FloatCore;
    #[test]
    fn test_round() {
        let _rug_st_tests_llm_16_580_llm_16_580_rrrruuuugggg_test_round = 0;
        let rug_fuzz_0 = 3.3;
        let rug_fuzz_1 = 3.5;
        let rug_fuzz_2 = 3.7;
        let rug_fuzz_3 = 3.3;
        let rug_fuzz_4 = 3.5;
        let rug_fuzz_5 = 3.7;
        let rug_fuzz_6 = 0.0;
        let rug_fuzz_7 = 0.0;
        debug_assert_eq!(< f64 as FloatCore > ::round(rug_fuzz_0), 3.0);
        debug_assert_eq!(< f64 as FloatCore > ::round(rug_fuzz_1), 4.0);
        debug_assert_eq!(< f64 as FloatCore > ::round(rug_fuzz_2), 4.0);
        debug_assert_eq!(< f64 as FloatCore > ::round(- rug_fuzz_3), - 3.0);
        debug_assert_eq!(< f64 as FloatCore > ::round(- rug_fuzz_4), - 4.0);
        debug_assert_eq!(< f64 as FloatCore > ::round(- rug_fuzz_5), - 4.0);
        debug_assert_eq!(< f64 as FloatCore > ::round(rug_fuzz_6), 0.0);
        debug_assert_eq!(< f64 as FloatCore > ::round(- rug_fuzz_7), - 0.0);
        debug_assert!(< f64 as FloatCore > ::round(f64::NAN).is_nan());
        debug_assert_eq!(< f64 as FloatCore > ::round(f64::INFINITY), f64::INFINITY);
        debug_assert_eq!(
            < f64 as FloatCore > ::round(f64::NEG_INFINITY), f64::NEG_INFINITY
        );
        let _rug_ed_tests_llm_16_580_llm_16_580_rrrruuuugggg_test_round = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_581_llm_16_581 {
    use crate::float::FloatCore;
    #[test]
    fn test_signum_positive() {
        let _rug_st_tests_llm_16_581_llm_16_581_rrrruuuugggg_test_signum_positive = 0;
        let rug_fuzz_0 = 42.0f64;
        let positive = rug_fuzz_0;
        debug_assert_eq!(positive.signum(), 1.0);
        let _rug_ed_tests_llm_16_581_llm_16_581_rrrruuuugggg_test_signum_positive = 0;
    }
    #[test]
    fn test_signum_negative() {
        let _rug_st_tests_llm_16_581_llm_16_581_rrrruuuugggg_test_signum_negative = 0;
        let rug_fuzz_0 = 42.0f64;
        let negative = -rug_fuzz_0;
        debug_assert_eq!(negative.signum(), - 1.0);
        let _rug_ed_tests_llm_16_581_llm_16_581_rrrruuuugggg_test_signum_negative = 0;
    }
    #[test]
    fn test_signum_zero() {
        let _rug_st_tests_llm_16_581_llm_16_581_rrrruuuugggg_test_signum_zero = 0;
        let rug_fuzz_0 = 0.0f64;
        let rug_fuzz_1 = 0.0f64;
        let zero = rug_fuzz_0;
        debug_assert_eq!(zero.signum(), 0.0);
        let neg_zero = -rug_fuzz_1;
        debug_assert_eq!(neg_zero.signum(), 0.0);
        let _rug_ed_tests_llm_16_581_llm_16_581_rrrruuuugggg_test_signum_zero = 0;
    }
    #[test]
    fn test_signum_nan() {
        let _rug_st_tests_llm_16_581_llm_16_581_rrrruuuugggg_test_signum_nan = 0;
        let nan = std::f64::NAN;
        debug_assert!(nan.signum().is_nan());
        let _rug_ed_tests_llm_16_581_llm_16_581_rrrruuuugggg_test_signum_nan = 0;
    }
    #[test]
    fn test_signum_infinity() {
        let _rug_st_tests_llm_16_581_llm_16_581_rrrruuuugggg_test_signum_infinity = 0;
        let infinity = std::f64::INFINITY;
        debug_assert_eq!(infinity.signum(), 1.0);
        let neg_infinity = std::f64::NEG_INFINITY;
        debug_assert_eq!(neg_infinity.signum(), - 1.0);
        let _rug_ed_tests_llm_16_581_llm_16_581_rrrruuuugggg_test_signum_infinity = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_582_llm_16_582 {
    use crate::float::FloatCore;
    #[test]
    fn test_to_degrees() {
        let _rug_st_tests_llm_16_582_llm_16_582_rrrruuuugggg_test_to_degrees = 0;
        let rug_fuzz_0 = 180.0;
        let rug_fuzz_1 = 1e-10;
        let rug_fuzz_2 = 0.0f64;
        let rug_fuzz_3 = 0.0;
        let rug_fuzz_4 = 2.0;
        let rug_fuzz_5 = 360.0;
        let rug_fuzz_6 = 2.0;
        let rug_fuzz_7 = 90.0;
        let rug_fuzz_8 = 4.0;
        let rug_fuzz_9 = 45.0;
        let radians = std::f64::consts::PI;
        let degrees = radians.to_degrees();
        let expected = rug_fuzz_0;
        let tol = rug_fuzz_1;
        debug_assert!(
            (degrees - expected).abs() < tol,
            "Radians to degrees conversion failed. Expected {}, got {}", expected,
            degrees
        );
        let radians = rug_fuzz_2;
        let degrees = radians.to_degrees();
        let expected = rug_fuzz_3;
        debug_assert!(
            (degrees - expected).abs() < tol,
            "Radians to degrees conversion failed. Expected {}, got {}", expected,
            degrees
        );
        let radians = rug_fuzz_4 * std::f64::consts::PI;
        let degrees = radians.to_degrees();
        let expected = rug_fuzz_5;
        debug_assert!(
            (degrees - expected).abs() < tol,
            "Radians to degrees conversion failed. Expected {}, got {}", expected,
            degrees
        );
        let radians = -std::f64::consts::PI / rug_fuzz_6;
        let degrees = radians.to_degrees();
        let expected = -rug_fuzz_7;
        debug_assert!(
            (degrees - expected).abs() < tol,
            "Radians to degrees conversion failed. Expected {}, got {}", expected,
            degrees
        );
        let radians = std::f64::consts::PI / rug_fuzz_8;
        let degrees = radians.to_degrees();
        let expected = rug_fuzz_9;
        debug_assert!(
            (degrees - expected).abs() < tol,
            "Radians to degrees conversion failed. Expected {}, got {}", expected,
            degrees
        );
        let _rug_ed_tests_llm_16_582_llm_16_582_rrrruuuugggg_test_to_degrees = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_583_llm_16_583 {
    use crate::float::FloatCore;
    #[test]
    fn test_to_radians() {
        let _rug_st_tests_llm_16_583_llm_16_583_rrrruuuugggg_test_to_radians = 0;
        let rug_fuzz_0 = 180.0;
        let rug_fuzz_1 = 1e-10;
        let degrees: f64 = rug_fuzz_0;
        let radians = degrees.to_radians();
        let expected = std::f64::consts::PI;
        debug_assert!((radians - expected).abs() < rug_fuzz_1);
        let _rug_ed_tests_llm_16_583_llm_16_583_rrrruuuugggg_test_to_radians = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_584_llm_16_584 {
    use crate::float::FloatCore;
    #[test]
    fn trunc_test() {
        let _rug_st_tests_llm_16_584_llm_16_584_rrrruuuugggg_trunc_test = 0;
        let rug_fuzz_0 = 3.9;
        let rug_fuzz_1 = 3.0;
        let rug_fuzz_2 = 3.9;
        let rug_fuzz_3 = 3.0;
        let rug_fuzz_4 = 0.0;
        let rug_fuzz_5 = 0.0;
        debug_assert_eq!(< f64 as FloatCore > ::trunc(rug_fuzz_0), 3.0);
        debug_assert_eq!(< f64 as FloatCore > ::trunc(rug_fuzz_1), 3.0);
        debug_assert_eq!(< f64 as FloatCore > ::trunc(- rug_fuzz_2), - 3.0);
        debug_assert_eq!(< f64 as FloatCore > ::trunc(- rug_fuzz_3), - 3.0);
        debug_assert_eq!(< f64 as FloatCore > ::trunc(rug_fuzz_4), 0.0);
        debug_assert_eq!(< f64 as FloatCore > ::trunc(- rug_fuzz_5), - 0.0);
        debug_assert!(< f64 as FloatCore > ::trunc(f64::NAN).is_nan());
        debug_assert_eq!(< f64 as FloatCore > ::trunc(f64::INFINITY), f64::INFINITY);
        debug_assert_eq!(
            < f64 as FloatCore > ::trunc(f64::NEG_INFINITY), f64::NEG_INFINITY
        );
        let _rug_ed_tests_llm_16_584_llm_16_584_rrrruuuugggg_trunc_test = 0;
    }
}
#[cfg(test)]
mod tests_rug_211 {
    use super::*;
    use crate::float::FloatCore;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_211_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 0.0;
        let inf: f32 = <f32 as FloatCore>::infinity();
        debug_assert!(inf.is_infinite());
        debug_assert!(inf > rug_fuzz_0);
        let _rug_ed_tests_rug_211_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_212 {
    use crate::float::FloatCore;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_212_rrrruuuugggg_test_rug = 0;
        debug_assert_eq!(
            < f32 as FloatCore > ::min_positive_value(), std::f32::MIN_POSITIVE
        );
        let _rug_ed_tests_rug_212_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_213 {
    use super::*;
    use crate::float::FloatCore;
    #[test]
    fn test_max_value() {
        let _rug_st_tests_rug_213_rrrruuuugggg_test_max_value = 0;
        let max_float: f32 = <f32 as FloatCore>::max_value();
        debug_assert_eq!(max_float, std::f32::MAX);
        let _rug_ed_tests_rug_213_rrrruuuugggg_test_max_value = 0;
    }
}
#[cfg(test)]
mod tests_rug_214 {
    use crate::float::FloatCore;
    #[test]
    fn test_to_radians() {
        let _rug_st_tests_rug_214_rrrruuuugggg_test_to_radians = 0;
        let rug_fuzz_0 = 180.0;
        let mut p0: f32 = rug_fuzz_0;
        let radians = <f32 as FloatCore>::to_radians(p0);
        debug_assert!((radians - std::f32::consts::PI).abs() < f32::EPSILON);
        let _rug_ed_tests_rug_214_rrrruuuugggg_test_to_radians = 0;
    }
}
#[cfg(test)]
mod tests_rug_215 {
    use super::*;
    use crate::float::FloatCore;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_215_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 3.0;
        let rug_fuzz_1 = 5.0;
        let mut p0: f64 = rug_fuzz_0;
        let mut p1: f64 = rug_fuzz_1;
        debug_assert_eq!((< f64 as FloatCore > ::max) (p0, p1), 5.0);
        let _rug_ed_tests_rug_215_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_216 {
    use super::*;
    use crate::float::FloatCore;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_216_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 2.0;
        let mut p0: f64 = rug_fuzz_0;
        debug_assert_eq!(< f64 as FloatCore > ::recip(p0), 0.5);
        let _rug_ed_tests_rug_216_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_217 {
    use super::*;
    use crate::float::Float;
    #[test]
    fn test_neg_infinity() {
        let _rug_st_tests_rug_217_rrrruuuugggg_test_neg_infinity = 0;
        let neg_inf: f64 = f64::neg_infinity();
        debug_assert!(neg_inf.is_infinite() && neg_inf.is_sign_negative());
        let _rug_ed_tests_rug_217_rrrruuuugggg_test_neg_infinity = 0;
    }
}
#[cfg(test)]
mod tests_rug_218 {
    use super::*;
    use crate::Float;
    #[test]
    fn test_log2() {
        let _rug_st_tests_rug_218_rrrruuuugggg_test_log2 = 0;
        let rug_fuzz_0 = 2.0;
        let mut p0: f64 = rug_fuzz_0;
        debug_assert_eq!(< f64 as Float > ::log2(p0), 1.0);
        let _rug_ed_tests_rug_218_rrrruuuugggg_test_log2 = 0;
    }
}
#[cfg(test)]
mod tests_rug_219 {
    use super::*;
    use crate::Float;
    #[test]
    fn test_tan() {
        let _rug_st_tests_rug_219_rrrruuuugggg_test_tan = 0;
        let rug_fuzz_0 = 0.0;
        let p0: f64 = rug_fuzz_0;
        debug_assert_eq!(< f64 as Float > ::tan(p0), 0.0f64.tan());
        let _rug_ed_tests_rug_219_rrrruuuugggg_test_tan = 0;
    }
}
#[cfg(test)]
mod tests_rug_220 {
    use super::*;
    use crate::FloatConst;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_220_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 1.0;
        let value = <f32 as FloatConst>::FRAC_1_PI();
        let expected = rug_fuzz_0 / std::f32::consts::PI;
        debug_assert!((value - expected).abs() < std::f32::EPSILON);
        let _rug_ed_tests_rug_220_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_221 {
    use crate::float::FloatConst;
    #[test]
    fn test_frac_2_pi() {
        const EXPECTED: f32 = 2.0 / std::f32::consts::PI;
        let result = <f32 as FloatConst>::FRAC_2_PI();
        assert!((result - EXPECTED).abs() < std::f32::EPSILON);
    }
}
#[cfg(test)]
mod tests_rug_223 {
    use super::*;
    use crate::FloatConst;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_223_rrrruuuugggg_test_rug = 0;
        let value = <f32 as FloatConst>::FRAC_PI_2();
        debug_assert!((value - std::f32::consts::FRAC_PI_2).abs() < f32::EPSILON);
        let _rug_ed_tests_rug_223_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_225 {
    use super::*;
    use crate::float::FloatConst;
    #[test]
    fn test_ln_10() {
        let _rug_st_tests_rug_225_rrrruuuugggg_test_ln_10 = 0;
        let rug_fuzz_0 = 2.3025851f32;
        let ln_10 = <f32 as FloatConst>::LN_10();
        let expected = rug_fuzz_0;
        debug_assert!((ln_10 - expected).abs() < f32::EPSILON);
        let _rug_ed_tests_rug_225_rrrruuuugggg_test_ln_10 = 0;
    }
}
#[cfg(test)]
mod tests_rug_226 {
    use crate::float::FloatConst;
    #[test]
    fn test_ln_2() {
        let _rug_st_tests_rug_226_rrrruuuugggg_test_ln_2 = 0;
        let value = <f32 as FloatConst>::LN_2();
        let expected = std::f32::consts::LN_2;
        debug_assert!((value - expected).abs() < std::f32::EPSILON);
        let _rug_ed_tests_rug_226_rrrruuuugggg_test_ln_2 = 0;
    }
}
#[cfg(test)]
mod tests_rug_227 {
    use crate::float::FloatConst;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_227_rrrruuuugggg_test_rug = 0;
        let log10_e: f32 = <f32 as FloatConst>::LOG10_E();
        let expected: f32 = std::f32::consts::LOG10_E;
        debug_assert!((log10_e - expected).abs() < f32::EPSILON);
        let _rug_ed_tests_rug_227_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_228 {
    use super::*;
    use crate::float::FloatConst;
    #[test]
    fn test_pi() {
        let _rug_st_tests_rug_228_rrrruuuugggg_test_pi = 0;
        let pi = <f32 as FloatConst>::PI();
        debug_assert_eq!(pi, std::f32::consts::PI);
        let _rug_ed_tests_rug_228_rrrruuuugggg_test_pi = 0;
    }
}
#[cfg(test)]
mod tests_rug_229 {
    use crate::float::FloatConst;
    #[test]
    fn test_sqrt_2() {
        let _rug_st_tests_rug_229_rrrruuuugggg_test_sqrt_2 = 0;
        debug_assert_eq!(< f32 as FloatConst > ::SQRT_2(), 1.4142135_f32);
        let _rug_ed_tests_rug_229_rrrruuuugggg_test_sqrt_2 = 0;
    }
}
#[cfg(test)]
mod tests_rug_230 {
    use super::*;
    use crate::float::FloatConst;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_230_rrrruuuugggg_test_rug = 0;
        let tau: f32 = <f32 as FloatConst>::TAU();
        debug_assert_eq!(tau, std::f32::consts::PI * 2.0);
        let _rug_ed_tests_rug_230_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_231 {
    use super::*;
    use crate::float::FloatConst;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_231_rrrruuuugggg_test_rug = 0;
        debug_assert_eq!(< f32 as FloatConst > ::LOG10_2(), 0.3010299956639812);
        let _rug_ed_tests_rug_231_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_232 {
    use super::*;
    use crate::float::FloatConst;
    #[test]
    fn test_log2_10() {
        let _rug_st_tests_rug_232_rrrruuuugggg_test_log2_10 = 0;
        let rug_fuzz_0 = 3.321928;
        let value: f32 = <f32 as FloatConst>::LOG2_10();
        let expected: f32 = rug_fuzz_0;
        debug_assert!((value - expected).abs() < f32::EPSILON);
        let _rug_ed_tests_rug_232_rrrruuuugggg_test_log2_10 = 0;
    }
}
#[cfg(test)]
mod tests_rug_233 {
    use super::*;
    use crate::float::FloatConst;
    #[test]
    fn test_frac_2_pi() {
        let _rug_st_tests_rug_233_rrrruuuugggg_test_frac_2_pi = 0;
        let frac_2_pi = <f64 as FloatConst>::FRAC_2_PI();
        debug_assert!((frac_2_pi - std::f64::consts::FRAC_2_PI).abs() < f64::EPSILON);
        let _rug_ed_tests_rug_233_rrrruuuugggg_test_frac_2_pi = 0;
    }
}
#[cfg(test)]
mod tests_rug_235 {
    use crate::float::FloatConst;
    #[test]
    fn test_frac_pi_2() {
        let _rug_st_tests_rug_235_rrrruuuugggg_test_frac_pi_2 = 0;
        let value = <f64 as FloatConst>::FRAC_PI_2();
        let expected = std::f64::consts::FRAC_PI_2;
        debug_assert_eq!(value, expected);
        let _rug_ed_tests_rug_235_rrrruuuugggg_test_frac_pi_2 = 0;
    }
}
#[cfg(test)]
mod tests_rug_236 {
    use super::*;
    use crate::float::FloatConst;
    #[test]
    fn test_frac_pi_3() {
        let _rug_st_tests_rug_236_rrrruuuugggg_test_frac_pi_3 = 0;
        let rug_fuzz_0 = 3.0;
        let frac_pi_3 = <f64 as FloatConst>::FRAC_PI_3();
        let expected = std::f64::consts::PI / rug_fuzz_0;
        debug_assert!((frac_pi_3 - expected).abs() < std::f64::EPSILON);
        let _rug_ed_tests_rug_236_rrrruuuugggg_test_frac_pi_3 = 0;
    }
}
#[cfg(test)]
mod tests_rug_237 {
    use crate::float::FloatConst;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_237_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 6.0;
        let frac_pi_6 = <f64 as FloatConst>::FRAC_PI_6();
        let expected = std::f64::consts::PI / rug_fuzz_0;
        debug_assert!((frac_pi_6 - expected).abs() < std::f64::EPSILON);
        let _rug_ed_tests_rug_237_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_238 {
    use super::*;
    use crate::float::FloatConst;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_238_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 8.0;
        let frac_pi_8 = <f64 as FloatConst>::FRAC_PI_8();
        let expected = std::f64::consts::PI / rug_fuzz_0;
        debug_assert!((frac_pi_8 - expected).abs() < std::f64::EPSILON);
        let _rug_ed_tests_rug_238_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_239 {
    use super::*;
    use crate::float::FloatConst;
    #[test]
    fn test_log10_e() {
        let _rug_st_tests_rug_239_rrrruuuugggg_test_log10_e = 0;
        let log10_e = <f64 as FloatConst>::LOG10_E();
        let expected = std::f64::consts::LOG10_E;
        debug_assert_eq!(log10_e, expected);
        let _rug_ed_tests_rug_239_rrrruuuugggg_test_log10_e = 0;
    }
}
#[cfg(test)]
mod tests_rug_240 {
    use super::*;
    use crate::float::FloatConst;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_240_rrrruuuugggg_test_rug = 0;
        let pi = <f64 as FloatConst>::PI();
        debug_assert_eq!(pi, std::f64::consts::PI);
        let _rug_ed_tests_rug_240_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_241 {
    use crate::float::FloatConst;
    #[test]
    fn test_sqrt_2() {
        let _rug_st_tests_rug_241_rrrruuuugggg_test_sqrt_2 = 0;
        let rug_fuzz_0 = 1.4142135623730951_f64;
        let sqrt_2 = <f64 as FloatConst>::SQRT_2();
        let expected = rug_fuzz_0;
        debug_assert!((sqrt_2 - expected).abs() < f64::EPSILON);
        let _rug_ed_tests_rug_241_rrrruuuugggg_test_sqrt_2 = 0;
    }
}
#[cfg(test)]
mod tests_rug_242 {
    use super::*;
    use crate::float::FloatConst;
    #[test]
    fn test_tau() {
        let _rug_st_tests_rug_242_rrrruuuugggg_test_tau = 0;
        debug_assert_eq!(< f64 as FloatConst > ::TAU(), std::f64::consts::PI * 2.0);
        let _rug_ed_tests_rug_242_rrrruuuugggg_test_tau = 0;
    }
}
#[cfg(test)]
mod tests_rug_243 {
    use crate::float::FloatConst;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_243_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 0.3010299956639812;
        let result = <f64 as FloatConst>::LOG10_2();
        let expected = rug_fuzz_0;
        debug_assert!((result - expected).abs() < f64::EPSILON);
        let _rug_ed_tests_rug_243_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_244 {
    use super::*;
    use crate::float::FloatConst;
    #[test]
    fn test_log2_10() {
        let _rug_st_tests_rug_244_rrrruuuugggg_test_log2_10 = 0;
        let rug_fuzz_0 = 3.321928094887362;
        let log2_10: f64 = <f64 as FloatConst>::LOG2_10();
        debug_assert!((log2_10 - rug_fuzz_0).abs() < f64::EPSILON);
        let _rug_ed_tests_rug_244_rrrruuuugggg_test_log2_10 = 0;
    }
}
#[cfg(test)]
mod tests_rug_245 {
    use super::*;
    use crate::real::Real;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_245_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 0.0;
        type T = f64;
        let min_val = T::min_positive_value();
        debug_assert!(min_val > rug_fuzz_0);
        let _rug_ed_tests_rug_245_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_246 {
    use super::*;
    use crate::float::Float;
    use crate::real::Real;
    #[test]
    fn test_fract() {
        let _rug_st_tests_rug_246_rrrruuuugggg_test_fract = 0;
        let rug_fuzz_0 = 3.75;
        let rug_fuzz_1 = 0.75;
        let mut p0: f32 = rug_fuzz_0;
        let result = <f32 as Real>::fract(p0);
        debug_assert!((result - rug_fuzz_1).abs() < f32::EPSILON);
        let _rug_ed_tests_rug_246_rrrruuuugggg_test_fract = 0;
    }
}
#[cfg(test)]
mod tests_rug_247 {
    use super::*;
    use crate::real::Real;
    use crate::float::Float;
    use std::marker::Sized;
    use std::ops::Neg;
    #[test]
    fn test_min() {
        let _rug_st_tests_rug_247_rrrruuuugggg_test_min = 0;
        let rug_fuzz_0 = 13.5;
        let rug_fuzz_1 = 42.7;
        let p0: f64 = rug_fuzz_0;
        let p1: f64 = rug_fuzz_1;
        debug_assert_eq!(Real::min(p0, p1), p0);
        let _rug_ed_tests_rug_247_rrrruuuugggg_test_min = 0;
    }
}
