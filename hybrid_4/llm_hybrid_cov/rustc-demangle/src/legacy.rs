use core::char;
use core::fmt;
/// Representation of a demangled symbol name.
pub struct Demangle<'a> {
    inner: &'a str,
    /// The number of ::-separated elements in the original name.
    elements: usize,
}
/// De-mangles a Rust symbol into a more readable version
///
/// All Rust symbols by default are mangled as they contain characters that
/// cannot be represented in all object files. The mangling mechanism is similar
/// to C++'s, but Rust has a few specifics to handle items like lifetimes in
/// symbols.
///
/// This function will take a **mangled** symbol and return a value. When printed,
/// the de-mangled version will be written. If the symbol does not look like
/// a mangled symbol, the original value will be written instead.
///
/// # Examples
///
/// ```
/// use rustc_demangle::demangle;
///
/// assert_eq!(demangle("_ZN4testE").to_string(), "test");
/// assert_eq!(demangle("_ZN3foo3barE").to_string(), "foo::bar");
/// assert_eq!(demangle("foo").to_string(), "foo");
/// ```
pub fn demangle(s: &str) -> Result<(Demangle, &str), ()> {
    let inner = if s.starts_with("_ZN") {
        &s[3..]
    } else if s.starts_with("ZN") {
        &s[2..]
    } else if s.starts_with("__ZN") {
        &s[4..]
    } else {
        return Err(());
    };
    if inner.bytes().any(|c| c & 0x80 != 0) {
        return Err(());
    }
    let mut elements = 0;
    let mut chars = inner.chars();
    let mut c = chars.next().ok_or(())?;
    while c != 'E' {
        if !c.is_digit(10) {
            return Err(());
        }
        let mut len = 0usize;
        while let Some(d) = c.to_digit(10) {
            len = len
                .checked_mul(10)
                .and_then(|len| len.checked_add(d as usize))
                .ok_or(())?;
            c = chars.next().ok_or(())?;
        }
        for _ in 0..len {
            c = chars.next().ok_or(())?;
        }
        elements += 1;
    }
    Ok((Demangle { inner, elements }, chars.as_str()))
}
fn is_rust_hash(s: &str) -> bool {
    s.starts_with('h') && s[1..].chars().all(|c| c.is_digit(16))
}
impl<'a> fmt::Display for Demangle<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut inner = self.inner;
        for element in 0..self.elements {
            let mut rest = inner;
            while rest.chars().next().unwrap().is_digit(10) {
                rest = &rest[1..];
            }
            let i: usize = inner[..(inner.len() - rest.len())].parse().unwrap();
            inner = &rest[i..];
            rest = &rest[..i];
            if f.alternate() && element + 1 == self.elements && is_rust_hash(&rest) {
                break;
            }
            if element != 0 {
                f.write_str("::")?;
            }
            if rest.starts_with("_$") {
                rest = &rest[1..];
            }
            loop {
                if rest.starts_with('.') {
                    if let Some('.') = rest[1..].chars().next() {
                        f.write_str("::")?;
                        rest = &rest[2..];
                    } else {
                        f.write_str(".")?;
                        rest = &rest[1..];
                    }
                } else if rest.starts_with('$') {
                    let (escape, after_escape) = if let Some(end) = rest[1..].find('$') {
                        (&rest[1..=end], &rest[end + 2..])
                    } else {
                        break;
                    };
                    let unescaped = match escape {
                        "SP" => "@",
                        "BP" => "*",
                        "RF" => "&",
                        "LT" => "<",
                        "GT" => ">",
                        "LP" => "(",
                        "RP" => ")",
                        "C" => ",",
                        _ => {
                            if escape.starts_with('u') {
                                let digits = &escape[1..];
                                let all_lower_hex = digits
                                    .chars()
                                    .all(|c| match c {
                                        '0'..='9' | 'a'..='f' => true,
                                        _ => false,
                                    });
                                let c = u32::from_str_radix(digits, 16)
                                    .ok()
                                    .and_then(char::from_u32);
                                if let (true, Some(c)) = (all_lower_hex, c) {
                                    if !c.is_control() {
                                        c.fmt(f)?;
                                        rest = after_escape;
                                        continue;
                                    }
                                }
                            }
                            break;
                        }
                    };
                    f.write_str(unescaped)?;
                    rest = after_escape;
                } else if let Some(i) = rest.find(|c| c == '$' || c == '.') {
                    f.write_str(&rest[..i])?;
                    rest = &rest[i..];
                } else {
                    break;
                }
            }
            f.write_str(rest)?;
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use std::prelude::v1::*;
    macro_rules! t {
        ($a:expr, $b:expr) => {
            assert!(ok($a, $b))
        };
    }
    macro_rules! t_err {
        ($a:expr) => {
            assert!(ok_err($a))
        };
    }
    macro_rules! t_nohash {
        ($a:expr, $b:expr) => {
            { assert_eq!(format!("{:#}", ::demangle($a)), $b); }
        };
    }
    fn ok(sym: &str, expected: &str) -> bool {
        match ::try_demangle(sym) {
            Ok(s) => {
                if s.to_string() == expected {
                    true
                } else {
                    println!("\n{}\n!=\n{}\n", s, expected);
                    false
                }
            }
            Err(_) => {
                println!("error demangling");
                false
            }
        }
    }
    fn ok_err(sym: &str) -> bool {
        match ::try_demangle(sym) {
            Ok(_) => {
                println!("succeeded in demangling");
                false
            }
            Err(_) => ::demangle(sym).to_string() == sym,
        }
    }
    #[test]
    fn demangle() {
        t_err!("test");
        t!("_ZN4testE", "test");
        t_err!("_ZN4test");
        t!("_ZN4test1a2bcE", "test::a::bc");
    }
    #[test]
    fn demangle_dollars() {
        t!("_ZN4$RP$E", ")");
        t!("_ZN8$RF$testE", "&test");
        t!("_ZN8$BP$test4foobE", "*test::foob");
        t!("_ZN9$u20$test4foobE", " test::foob");
        t!("_ZN35Bar$LT$$u5b$u32$u3b$$u20$4$u5d$$GT$E", "Bar<[u32; 4]>");
    }
    #[test]
    fn demangle_many_dollars() {
        t!("_ZN13test$u20$test4foobE", "test test::foob");
        t!("_ZN12test$BP$test4foobE", "test*test::foob");
    }
    #[test]
    fn demangle_osx() {
        t!(
            "__ZN5alloc9allocator6Layout9for_value17h02a996811f781011E",
            "alloc::allocator::Layout::for_value::h02a996811f781011"
        );
        t!(
            "__ZN38_$LT$core..option..Option$LT$T$GT$$GT$6unwrap18_MSG_FILE_LINE_COL17haf7cb8d5824ee659E",
            "<core::option::Option<T>>::unwrap::_MSG_FILE_LINE_COL::haf7cb8d5824ee659"
        );
        t!(
            "__ZN4core5slice89_$LT$impl$u20$core..iter..traits..IntoIterator$u20$for$u20$$RF$$u27$a$u20$$u5b$T$u5d$$GT$9into_iter17h450e234d27262170E",
            "core::slice::<impl core::iter::traits::IntoIterator for &'a [T]>::into_iter::h450e234d27262170"
        );
    }
    #[test]
    fn demangle_windows() {
        t!("ZN4testE", "test");
        t!("ZN13test$u20$test4foobE", "test test::foob");
        t!("ZN12test$RF$test4foobE", "test&test::foob");
    }
    #[test]
    fn demangle_elements_beginning_with_underscore() {
        t!("_ZN13_$LT$test$GT$E", "<test>");
        t!("_ZN28_$u7b$$u7b$closure$u7d$$u7d$E", "{{closure}}");
        t!("_ZN15__STATIC_FMTSTRE", "__STATIC_FMTSTR");
    }
    #[test]
    fn demangle_trait_impls() {
        t!(
            "_ZN71_$LT$Test$u20$$u2b$$u20$$u27$static$u20$as$u20$foo..Bar$LT$Test$GT$$GT$3barE",
            "<Test + 'static as foo::Bar<Test>>::bar"
        );
    }
    #[test]
    fn demangle_without_hash() {
        let s = "_ZN3foo17h05af221e174051e9E";
        t!(s, "foo::h05af221e174051e9");
        t_nohash!(s, "foo");
    }
    #[test]
    fn demangle_without_hash_edgecases() {
        t_nohash!("_ZN3fooE", "foo");
        t_nohash!("_ZN3foo3barE", "foo::bar");
        t_nohash!("_ZN3foo20h05af221e174051e9abcE", "foo");
        t_nohash!("_ZN3foo5h05afE", "foo");
        t_nohash!("_ZN17h05af221e174051e93fooE", "h05af221e174051e9::foo");
        t_nohash!("_ZN3foo16ffaf221e174051e9E", "foo::ffaf221e174051e9");
        t_nohash!("_ZN3foo17hg5af221e174051e9E", "foo::hg5af221e174051e9");
    }
    #[test]
    fn demangle_thinlto() {
        t!("_ZN3fooE.llvm.9D1C9369", "foo");
        t!("_ZN3fooE.llvm.9D1C9369@@16", "foo");
        t_nohash!(
            "_ZN9backtrace3foo17hbb467fcdaea5d79bE.llvm.A5310EB9", "backtrace::foo"
        );
    }
    #[test]
    fn demangle_llvm_ir_branch_labels() {
        t!(
            "_ZN4core5slice77_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$9index_mut17haf9727c2edfbc47bE.exit.i.i",
            "core::slice::<impl core::ops::index::IndexMut<I> for [T]>::index_mut::haf9727c2edfbc47b.exit.i.i"
        );
        t_nohash!(
            "_ZN4core5slice77_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$9index_mut17haf9727c2edfbc47bE.exit.i.i",
            "core::slice::<impl core::ops::index::IndexMut<I> for [T]>::index_mut.exit.i.i"
        );
    }
    #[test]
    fn demangle_ignores_suffix_that_doesnt_look_like_a_symbol() {
        t_err!("_ZN3fooE.llvm moocow");
    }
    #[test]
    fn dont_panic() {
        ::demangle("_ZN2222222222222222222222EE").to_string();
        ::demangle("_ZN5*70527e27.ll34csaғE").to_string();
        ::demangle("_ZN5*70527a54.ll34_$b.1E").to_string();
        ::demangle(
                "\
             _ZN5~saäb4e\n\
             2734cOsbE\n\
             5usage20h)3\0\0\0\0\0\0\07e2734cOsbE\
             ",
            )
            .to_string();
    }
    #[test]
    fn invalid_no_chop() {
        t_err!("_ZNfooE");
    }
    #[test]
    fn handle_assoc_types() {
        t!(
            "_ZN151_$LT$alloc..boxed..Box$LT$alloc..boxed..FnBox$LT$A$C$$u20$Output$u3d$R$GT$$u20$$u2b$$u20$$u27$a$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$9call_once17h69e8f44b3723e1caE",
            "<alloc::boxed::Box<alloc::boxed::FnBox<A, Output=R> + 'a> as core::ops::function::FnOnce<A>>::call_once::h69e8f44b3723e1ca"
        );
    }
    #[test]
    fn handle_bang() {
        t!(
            "_ZN88_$LT$core..result..Result$LT$$u21$$C$$u20$E$GT$$u20$as$u20$std..process..Termination$GT$6report17hfc41d0da4a40b3e8E",
            "<core::result::Result<!, E> as std::process::Termination>::report::hfc41d0da4a40b3e8"
        );
    }
    #[test]
    fn demangle_utf8_idents() {
        t_nohash!(
            "_ZN11utf8_idents157_$u10e1$$u10d0$$u10ed$$u10db$$u10d4$$u10da$$u10d0$$u10d3$_$u10d2$$u10d4$$u10db$$u10e0$$u10d8$$u10d4$$u10da$$u10d8$_$u10e1$$u10d0$$u10d3$$u10d8$$u10da$$u10d8$17h21634fd5714000aaE",
            "utf8_idents::საჭმელად_გემრიელი_სადილი"
        );
    }
    #[test]
    fn demangle_issue_60925() {
        t_nohash!(
            "_ZN11issue_609253foo37Foo$LT$issue_60925..llv$u6d$..Foo$GT$3foo17h059a991a004536adE",
            "issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo"
        );
    }
}
#[cfg(test)]
mod tests_llm_16_7_llm_16_7 {
    use crate::legacy::demangle;
    use crate::Demangle;
    #[test]
    fn test_demangle_valid_symbols() {
        let _rug_st_tests_llm_16_7_llm_16_7_rrrruuuugggg_test_demangle_valid_symbols = 0;
        let rug_fuzz_0 = "_ZN4testE";
        let rug_fuzz_1 = "test";
        let rug_fuzz_2 = "demangle failed";
        let samples = vec![
            (rug_fuzz_0, rug_fuzz_1), ("_ZN3foo3barE", "foo::bar"),
            ("_ZN12a_really_long7exampleE", "a_really_long::example"), ("__ZN3foo3barE",
            "foo::bar"), ("ZN3foo3barE", "foo::bar")
        ];
        for (mangled, expected) in samples {
            let (demangled, rest) = demangle(mangled).expect(rug_fuzz_2);
            debug_assert_eq!(demangled.to_string(), expected);
            debug_assert!(rest.is_empty());
        }
        let _rug_ed_tests_llm_16_7_llm_16_7_rrrruuuugggg_test_demangle_valid_symbols = 0;
    }
    #[test]
    fn test_demangle_invalid_symbols() {
        let _rug_st_tests_llm_16_7_llm_16_7_rrrruuuugggg_test_demangle_invalid_symbols = 0;
        let rug_fuzz_0 = "foo";
        let samples = vec![
            rug_fuzz_0, "_ZN4test", "_ZN12a_really_long7", "__ZN", "3foo", "_ZN4tästE"
        ];
        for mangled in samples {
            debug_assert!(demangle(mangled).is_err());
        }
        let _rug_ed_tests_llm_16_7_llm_16_7_rrrruuuugggg_test_demangle_invalid_symbols = 0;
    }
    #[test]
    fn test_demangle_symbols_with_rest() {
        let _rug_st_tests_llm_16_7_llm_16_7_rrrruuuugggg_test_demangle_symbols_with_rest = 0;
        let rug_fuzz_0 = "_ZN4testErest_of_string";
        let rug_fuzz_1 = "test";
        let rug_fuzz_2 = "rest_of_string";
        let rug_fuzz_3 = "demangle failed";
        let samples = vec![
            (rug_fuzz_0, rug_fuzz_1, rug_fuzz_2), ("_ZN3foo3barEtail", "foo::bar",
            "tail")
        ];
        for (mangled, expected, expected_rest) in samples {
            let (demangled, rest) = demangle(mangled).expect(rug_fuzz_3);
            debug_assert_eq!(demangled.to_string(), expected);
            debug_assert_eq!(rest, expected_rest);
        }
        let _rug_ed_tests_llm_16_7_llm_16_7_rrrruuuugggg_test_demangle_symbols_with_rest = 0;
    }
    #[test]
    fn test_demangle_empty_string() {
        let _rug_st_tests_llm_16_7_llm_16_7_rrrruuuugggg_test_demangle_empty_string = 0;
        let rug_fuzz_0 = "";
        debug_assert!(demangle(rug_fuzz_0).is_err());
        let _rug_ed_tests_llm_16_7_llm_16_7_rrrruuuugggg_test_demangle_empty_string = 0;
    }
}
#[cfg(test)]
mod tests_rug_1 {
    use super::*;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_1_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "h1234abcd";
        let p0: &str = rug_fuzz_0;
        debug_assert!(crate ::legacy::is_rust_hash(& p0));
        let _rug_ed_tests_rug_1_rrrruuuugggg_test_rug = 0;
    }
}
