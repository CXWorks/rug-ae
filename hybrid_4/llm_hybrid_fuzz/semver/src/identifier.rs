use crate::alloc::alloc::{alloc, dealloc, handle_alloc_error, Layout};
use core::isize;
use core::mem;
use core::num::{NonZeroU64, NonZeroUsize};
use core::ptr::{self, NonNull};
use core::slice;
use core::str;
use core::usize;
const PTR_BYTES: usize = mem::size_of::<NonNull<u8>>();
const TAIL_BYTES: usize = 8 * (PTR_BYTES < 8) as usize
    - PTR_BYTES * (PTR_BYTES < 8) as usize;
#[repr(C, align(8))]
pub(crate) struct Identifier {
    head: NonNull<u8>,
    tail: [u8; TAIL_BYTES],
}
impl Identifier {
    pub(crate) const fn empty() -> Self {
        const HEAD: NonNull<u8> = unsafe { NonNull::new_unchecked(!0 as *mut u8) };
        Identifier {
            head: HEAD,
            tail: [!0; TAIL_BYTES],
        }
    }
    pub(crate) unsafe fn new_unchecked(string: &str) -> Self {
        let len = string.len();
        debug_assert!(len <= isize::MAX as usize);
        match len as u64 {
            0 => Self::empty(),
            1..=8 => {
                let mut bytes = [0u8; mem::size_of::<Identifier>()];
                unsafe {
                    ptr::copy_nonoverlapping(string.as_ptr(), bytes.as_mut_ptr(), len)
                };
                unsafe {
                    mem::transmute::<
                        [u8; mem::size_of::<Identifier>()],
                        Identifier,
                    >(bytes)
                }
            }
            9..=0xff_ffff_ffff_ffff => {
                let size = bytes_for_varint(unsafe { NonZeroUsize::new_unchecked(len) })
                    + len;
                let align = 2;
                if mem::size_of::<usize>() < 8 {
                    let max_alloc = usize::MAX / 2 - align;
                    assert!(size <= max_alloc);
                }
                let layout = unsafe { Layout::from_size_align_unchecked(size, align) };
                let ptr = unsafe { alloc(layout) };
                if ptr.is_null() {
                    handle_alloc_error(layout);
                }
                let mut write = ptr;
                let mut varint_remaining = len;
                while varint_remaining > 0 {
                    unsafe { ptr::write(write, varint_remaining as u8 | 0x80) };
                    varint_remaining >>= 7;
                    write = unsafe { write.add(1) };
                }
                unsafe { ptr::copy_nonoverlapping(string.as_ptr(), write, len) };
                Identifier {
                    head: ptr_to_repr(ptr),
                    tail: [0; TAIL_BYTES],
                }
            }
            0x100_0000_0000_0000..=0xffff_ffff_ffff_ffff => {
                unreachable!(
                    "please refrain from storing >64 petabytes of text in semver version"
                );
            }
            #[cfg(no_exhaustive_int_match)]
            _ => unreachable!(),
        }
    }
    pub(crate) fn is_empty(&self) -> bool {
        let empty = Self::empty();
        let is_empty = self.head == empty.head && self.tail == empty.tail;
        mem::forget(empty);
        is_empty
    }
    fn is_inline(&self) -> bool {
        self.head.as_ptr() as usize >> (PTR_BYTES * 8 - 1) == 0
    }
    fn is_empty_or_inline(&self) -> bool {
        self.is_empty() || self.is_inline()
    }
    pub(crate) fn as_str(&self) -> &str {
        if self.is_empty() {
            ""
        } else if self.is_inline() {
            unsafe { inline_as_str(self) }
        } else {
            unsafe { ptr_as_str(&self.head) }
        }
    }
}
impl Clone for Identifier {
    fn clone(&self) -> Self {
        if self.is_empty_or_inline() {
            Identifier {
                head: self.head,
                tail: self.tail,
            }
        } else {
            let ptr = repr_to_ptr(self.head);
            let len = unsafe { decode_len(ptr) };
            let size = bytes_for_varint(len) + len.get();
            let align = 2;
            let layout = unsafe { Layout::from_size_align_unchecked(size, align) };
            let clone = unsafe { alloc(layout) };
            if clone.is_null() {
                handle_alloc_error(layout);
            }
            unsafe { ptr::copy_nonoverlapping(ptr, clone, size) }
            Identifier {
                head: ptr_to_repr(clone),
                tail: [0; TAIL_BYTES],
            }
        }
    }
}
impl Drop for Identifier {
    fn drop(&mut self) {
        if self.is_empty_or_inline() {
            return;
        }
        let ptr = repr_to_ptr_mut(self.head);
        let len = unsafe { decode_len(ptr) };
        let size = bytes_for_varint(len) + len.get();
        let align = 2;
        let layout = unsafe { Layout::from_size_align_unchecked(size, align) };
        unsafe { dealloc(ptr, layout) }
    }
}
impl PartialEq for Identifier {
    fn eq(&self, rhs: &Self) -> bool {
        if self.is_empty_or_inline() {
            self.head == rhs.head && self.tail == rhs.tail
        } else if rhs.is_empty_or_inline() {
            false
        } else {
            unsafe { ptr_as_str(&self.head) == ptr_as_str(&rhs.head) }
        }
    }
}
unsafe impl Send for Identifier {}
unsafe impl Sync for Identifier {}
fn ptr_to_repr(original: *mut u8) -> NonNull<u8> {
    let modified = (original as usize | 1).rotate_right(1);
    let diff = modified.wrapping_sub(original as usize);
    let modified = original.wrapping_add(diff);
    unsafe { NonNull::new_unchecked(modified) }
}
fn repr_to_ptr(modified: NonNull<u8>) -> *const u8 {
    let modified = modified.as_ptr();
    let original = (modified as usize) << 1;
    let diff = original.wrapping_sub(modified as usize);
    modified.wrapping_add(diff)
}
fn repr_to_ptr_mut(repr: NonNull<u8>) -> *mut u8 {
    repr_to_ptr(repr) as *mut u8
}
unsafe fn inline_len(repr: &Identifier) -> NonZeroUsize {
    let repr = unsafe { ptr::read(repr as *const Identifier as *const NonZeroU64) };
    #[cfg(no_nonzero_bitscan)]
    let repr = repr.get();
    #[cfg(target_endian = "little")]
    let zero_bits_on_string_end = repr.leading_zeros();
    #[cfg(target_endian = "big")]
    let zero_bits_on_string_end = repr.trailing_zeros();
    let nonzero_bytes = 8 - zero_bits_on_string_end as usize / 8;
    unsafe { NonZeroUsize::new_unchecked(nonzero_bytes) }
}
unsafe fn inline_as_str(repr: &Identifier) -> &str {
    let ptr = repr as *const Identifier as *const u8;
    let len = unsafe { inline_len(repr) }.get();
    let slice = unsafe { slice::from_raw_parts(ptr, len) };
    unsafe { str::from_utf8_unchecked(slice) }
}
unsafe fn decode_len(ptr: *const u8) -> NonZeroUsize {
    let [first, second] = unsafe { ptr::read(ptr as *const [u8; 2]) };
    if second < 0x80 {
        unsafe { NonZeroUsize::new_unchecked((first & 0x7f) as usize) }
    } else {
        return unsafe { decode_len_cold(ptr) };
        #[cold]
        #[inline(never)]
        unsafe fn decode_len_cold(mut ptr: *const u8) -> NonZeroUsize {
            let mut len = 0;
            let mut shift = 0;
            loop {
                let byte = unsafe { *ptr };
                if byte < 0x80 {
                    return unsafe { NonZeroUsize::new_unchecked(len) };
                }
                ptr = unsafe { ptr.add(1) };
                len += ((byte & 0x7f) as usize) << shift;
                shift += 7;
            }
        }
    }
}
unsafe fn ptr_as_str(repr: &NonNull<u8>) -> &str {
    let ptr = repr_to_ptr(*repr);
    let len = unsafe { decode_len(ptr) };
    let header = bytes_for_varint(len);
    let slice = unsafe { slice::from_raw_parts(ptr.add(header), len.get()) };
    unsafe { str::from_utf8_unchecked(slice) }
}
fn bytes_for_varint(len: NonZeroUsize) -> usize {
    #[cfg(no_nonzero_bitscan)]
    let len = len.get();
    let usize_bits = mem::size_of::<usize>() * 8;
    let len_bits = usize_bits - len.leading_zeros() as usize;
    (len_bits + 6) / 7
}
#[cfg(test)]
mod tests_llm_16_2_llm_16_2 {
    use crate::Identifier;
    use std::mem;
    use std::ptr::NonNull;
    const TAIL_BYTES: usize = mem::size_of::<usize>() - 1;
    unsafe fn heap_allocated_identifier(content: &str) -> Identifier {
        Identifier::new_unchecked(content)
    }
    #[test]
    fn test_clone_empty_identifier() {
        let id = Identifier::empty();
        let cloned = id.clone();
        assert!(id.eq(& cloned));
    }
    #[test]
    fn test_clone_inline_identifier() {
        let id = unsafe { Identifier::new_unchecked("inline") };
        let cloned = id.clone();
        assert!(id.eq(& cloned));
    }
    #[test]
    fn test_clone_heap_allocated_identifier() {
        let id = unsafe { heap_allocated_identifier("heap_allocated") };
        let cloned = id.clone();
        assert!(id.eq(& cloned));
    }
}
#[cfg(test)]
mod tests_llm_16_3 {
    use crate::Identifier;
    #[test]
    fn test_eq_empty_identifiers() {
        let _rug_st_tests_llm_16_3_rrrruuuugggg_test_eq_empty_identifiers = 0;
        let id1 = Identifier::empty();
        let id2 = Identifier::empty();
        debug_assert!(id1.eq(& id2));
        let _rug_ed_tests_llm_16_3_rrrruuuugggg_test_eq_empty_identifiers = 0;
    }
    #[test]
    fn test_eq_inline_identifiers() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        unsafe {
            let id1 = Identifier::new_unchecked(rug_fuzz_0);
            let id2 = Identifier::new_unchecked(rug_fuzz_1);
            debug_assert!(id1.eq(& id2));
        }
             }
});    }
    #[test]
    fn test_eq_mixed_identifiers() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        unsafe {
            let id1 = Identifier::new_unchecked(rug_fuzz_0);
            let id2 = Identifier::empty();
            debug_assert!(! id1.eq(& id2));
        }
             }
});    }
    #[test]
    fn test_eq_heap_identifiers() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        unsafe {
            let id1 = Identifier::new_unchecked(rug_fuzz_0);
            let id2 = Identifier::new_unchecked(rug_fuzz_1);
            debug_assert!(id1.eq(& id2));
        }
             }
});    }
    #[test]
    fn test_eq_different_identifiers() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        unsafe {
            let id1 = Identifier::new_unchecked(rug_fuzz_0);
            let id2 = Identifier::new_unchecked(rug_fuzz_1);
            debug_assert!(! id1.eq(& id2));
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_4 {
    use super::*;
    use crate::*;
    #[test]
    fn test_drop_empty_identifier() {
        let _rug_st_tests_llm_16_4_rrrruuuugggg_test_drop_empty_identifier = 0;
        let mut id = Identifier::empty();
        std::mem::drop(&mut id);
        debug_assert!(id.is_empty(), "Identifier should be empty after dropping");
        let _rug_ed_tests_llm_16_4_rrrruuuugggg_test_drop_empty_identifier = 0;
    }
    #[test]
    fn test_drop_heap_allocated_identifier() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let long_str = rug_fuzz_0;
        let mut id = unsafe { Identifier::new_unchecked(long_str) };
        let ptr = id.head;
        std::mem::drop(&mut id);
             }
});    }
    #[test]
    fn test_drop_inline_identifier() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let inline_str = rug_fuzz_0;
        let mut id = unsafe { Identifier::new_unchecked(inline_str) };
        std::mem::drop(&mut id);
        debug_assert!(id.is_inline(), "Identifier should be inline after dropping");
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_28_llm_16_28 {
    use crate::Identifier;
    use std::ptr::NonNull;
    use std::mem;
    use std::hash::{Hash, Hasher};
    #[test]
    fn as_str_empty_identifier() {
        let _rug_st_tests_llm_16_28_llm_16_28_rrrruuuugggg_as_str_empty_identifier = 0;
        let id = Identifier::empty();
        debug_assert_eq!(id.as_str(), "");
        let _rug_ed_tests_llm_16_28_llm_16_28_rrrruuuugggg_as_str_empty_identifier = 0;
    }
    #[test]
    fn as_str_inline_identifier() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(u8, u8, u8, u8, u8, u8, u8, u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut bytes = [rug_fuzz_0; mem::size_of::<Identifier>()];
        let id_bytes = [
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
            rug_fuzz_5,
            rug_fuzz_6,
            rug_fuzz_7,
            rug_fuzz_8,
        ];
        bytes[..id_bytes.len()].copy_from_slice(&id_bytes);
        let inline_id: Identifier = unsafe { mem::transmute(bytes) };
        debug_assert_eq!(id_bytes, inline_id.as_str().as_bytes());
             }
});    }
    #[test]
    fn as_str_heap_identifier() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let heap_str = rug_fuzz_0;
        let heap_id = unsafe { Identifier::new_unchecked(heap_str) };
        debug_assert_eq!(heap_str, heap_id.as_str());
             }
});    }
    #[test]
    fn as_str_equals_own_hash() {
        let _rug_st_tests_llm_16_28_llm_16_28_rrrruuuugggg_as_str_equals_own_hash = 0;
        let id = Identifier::empty();
        let id_str = id.as_str();
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        id_str.hash(&mut hasher);
        let str_hash = hasher.finish();
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        id.hash(&mut hasher);
        let id_hash = hasher.finish();
        debug_assert_eq!(str_hash, id_hash);
        let _rug_ed_tests_llm_16_28_llm_16_28_rrrruuuugggg_as_str_equals_own_hash = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_29_llm_16_29 {
    use crate::Identifier;
    use std::mem;
    use std::ptr::NonNull;
    #[test]
    fn test_identifier_empty() {
        let _rug_st_tests_llm_16_29_llm_16_29_rrrruuuugggg_test_identifier_empty = 0;
        let empty_identifier = Identifier::empty();
        debug_assert!(empty_identifier.is_empty());
        debug_assert_eq!(empty_identifier.as_str(), "");
        debug_assert_eq!(
            mem::size_of_val(& empty_identifier.head), mem::size_of:: < NonNull < u8 > >
            ()
        );
        debug_assert_eq!(
            empty_identifier.tail.len(), mem::size_of_val(& empty_identifier.tail)
        );
        let _rug_ed_tests_llm_16_29_llm_16_29_rrrruuuugggg_test_identifier_empty = 0;
    }
    #[test]
    fn test_identifier_default_equals_empty() {
        let _rug_st_tests_llm_16_29_llm_16_29_rrrruuuugggg_test_identifier_default_equals_empty = 0;
        let default_identifier = Identifier::default();
        let empty_identifier = Identifier::empty();
        debug_assert!(default_identifier == empty_identifier);
        let _rug_ed_tests_llm_16_29_llm_16_29_rrrruuuugggg_test_identifier_default_equals_empty = 0;
    }
    #[test]
    fn test_identifier_empty_is_cloneable() {
        let _rug_st_tests_llm_16_29_llm_16_29_rrrruuuugggg_test_identifier_empty_is_cloneable = 0;
        let empty_identifier = Identifier::empty();
        let empty_identifier_clone = empty_identifier.clone();
        debug_assert!(empty_identifier == empty_identifier_clone);
        let _rug_ed_tests_llm_16_29_llm_16_29_rrrruuuugggg_test_identifier_empty_is_cloneable = 0;
    }
    #[test]
    fn test_identifier_empty_is_default() {
        let _rug_st_tests_llm_16_29_llm_16_29_rrrruuuugggg_test_identifier_empty_is_default = 0;
        let empty_identifier = Identifier::empty();
        let default_identifier = Identifier::default();
        debug_assert!(empty_identifier == default_identifier);
        let _rug_ed_tests_llm_16_29_llm_16_29_rrrruuuugggg_test_identifier_empty_is_default = 0;
    }
    #[test]
    fn test_identifier_empty_is_not_inline() {
        let _rug_st_tests_llm_16_29_llm_16_29_rrrruuuugggg_test_identifier_empty_is_not_inline = 0;
        let empty_identifier = Identifier::empty();
        debug_assert!(! empty_identifier.is_inline());
        let _rug_ed_tests_llm_16_29_llm_16_29_rrrruuuugggg_test_identifier_empty_is_not_inline = 0;
    }
    #[test]
    fn test_identifier_empty_hash() {
        let _rug_st_tests_llm_16_29_llm_16_29_rrrruuuugggg_test_identifier_empty_hash = 0;
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let empty_identifier = Identifier::empty();
        let mut hasher = DefaultHasher::new();
        empty_identifier.hash(&mut hasher);
        let empty_identifier_hash = hasher.finish();
        let another_empty_identifier = Identifier::empty();
        let mut hasher = DefaultHasher::new();
        another_empty_identifier.hash(&mut hasher);
        let another_empty_identifier_hash = hasher.finish();
        debug_assert_eq!(empty_identifier_hash, another_empty_identifier_hash);
        let _rug_ed_tests_llm_16_29_llm_16_29_rrrruuuugggg_test_identifier_empty_hash = 0;
    }
    #[test]
    fn test_identifier_empty_drop() {
        let _rug_st_tests_llm_16_29_llm_16_29_rrrruuuugggg_test_identifier_empty_drop = 0;
        let empty_identifier = Identifier::empty();
        mem::drop(empty_identifier);
        let _rug_ed_tests_llm_16_29_llm_16_29_rrrruuuugggg_test_identifier_empty_drop = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_30_llm_16_30 {
    use crate::identifier::Identifier;
    use std::mem;
    #[test]
    fn identifier_is_empty_on_default() {
        let _rug_st_tests_llm_16_30_llm_16_30_rrrruuuugggg_identifier_is_empty_on_default = 0;
        let identifier = Identifier::default();
        debug_assert!(identifier.is_empty());
        let _rug_ed_tests_llm_16_30_llm_16_30_rrrruuuugggg_identifier_is_empty_on_default = 0;
    }
    #[test]
    fn identifier_is_not_empty_after_assigning() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let identifier = unsafe { Identifier::new_unchecked(rug_fuzz_0) };
        debug_assert!(! identifier.is_empty());
             }
});    }
    #[test]
    fn identifier_is_empty_after_empty_call() {
        let _rug_st_tests_llm_16_30_llm_16_30_rrrruuuugggg_identifier_is_empty_after_empty_call = 0;
        let identifier = Identifier::empty();
        debug_assert!(identifier.is_empty());
        let _rug_ed_tests_llm_16_30_llm_16_30_rrrruuuugggg_identifier_is_empty_after_empty_call = 0;
    }
    #[test]
    fn identifier_is_empty_after_drop() {
        let _rug_st_tests_llm_16_30_llm_16_30_rrrruuuugggg_identifier_is_empty_after_drop = 0;
        let identifier = Identifier::default();
        mem::drop(identifier);
        let _rug_ed_tests_llm_16_30_llm_16_30_rrrruuuugggg_identifier_is_empty_after_drop = 0;
    }
    #[test]
    fn identifier_clone_retains_empty_state() {
        let _rug_st_tests_llm_16_30_llm_16_30_rrrruuuugggg_identifier_clone_retains_empty_state = 0;
        let identifier = Identifier::empty();
        let cloned_identifier = identifier.clone();
        debug_assert!(cloned_identifier.is_empty());
        let _rug_ed_tests_llm_16_30_llm_16_30_rrrruuuugggg_identifier_clone_retains_empty_state = 0;
    }
    #[test]
    fn identifier_clone_retains_non_empty_state() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let identifier = unsafe { Identifier::new_unchecked(rug_fuzz_0) };
        let cloned_identifier = identifier.clone();
        debug_assert!(! cloned_identifier.is_empty());
             }
});    }
    #[test]
    fn identifier_is_empty_after_mem_forget() {
        let _rug_st_tests_llm_16_30_llm_16_30_rrrruuuugggg_identifier_is_empty_after_mem_forget = 0;
        let identifier = Identifier::empty();
        mem::forget(identifier);
        let _rug_ed_tests_llm_16_30_llm_16_30_rrrruuuugggg_identifier_is_empty_after_mem_forget = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_31 {
    use crate::Identifier;
    #[test]
    fn is_empty_or_inline_empty() {
        let _rug_st_tests_llm_16_31_rrrruuuugggg_is_empty_or_inline_empty = 0;
        let identifier = Identifier::empty();
        debug_assert!(identifier.is_empty_or_inline());
        let _rug_ed_tests_llm_16_31_rrrruuuugggg_is_empty_or_inline_empty = 0;
    }
    #[test]
    fn is_empty_or_inline_inline() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let string = rug_fuzz_0;
        let identifier = unsafe { Identifier::new_unchecked(string) };
        debug_assert!(identifier.is_empty_or_inline());
             }
});    }
    #[test]
    fn is_empty_or_inline_not_inline_nor_empty() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let string = rug_fuzz_0;
        let identifier = unsafe { Identifier::new_unchecked(string) };
        debug_assert!(! identifier.is_empty_or_inline());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_32 {
    use crate::Identifier;
    use std::ptr::NonNull;
    #[test]
    fn test_identifier_is_inline_for_empty() {
        let _rug_st_tests_llm_16_32_rrrruuuugggg_test_identifier_is_inline_for_empty = 0;
        let empty_identifier = Identifier::empty();
        debug_assert!(empty_identifier.is_inline());
        let _rug_ed_tests_llm_16_32_rrrruuuugggg_test_identifier_is_inline_for_empty = 0;
    }
    #[test]
    fn test_identifier_is_inline_for_inline() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let inline_identifier = unsafe { Identifier::new_unchecked(rug_fuzz_0) };
        debug_assert!(inline_identifier.is_inline());
             }
});    }
    #[test]
    fn test_identifier_is_inline_for_not_inline() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let not_inline_identifier = unsafe { Identifier::new_unchecked(rug_fuzz_0) };
        debug_assert!(! not_inline_identifier.is_inline());
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_33_llm_16_33 {
    use super::*;
    use crate::*;
    #[test]
    fn new_unchecked_empty() {
        let identifier = unsafe { Identifier::new_unchecked("") };
        assert!(identifier.is_empty());
    }
    #[test]
    #[should_panic(
        expected = "please refrain from storing >64 petabytes of text in semver version"
    )]
    fn new_unchecked_too_large() {
        let large_str = "a".repeat(0x100_0000_0000_0000);
        let _identifier = unsafe { Identifier::new_unchecked(&large_str) };
    }
    #[test]
    fn new_unchecked_inline() {
        let s = "abcd";
        assert!(s.len() <= 8);
        let identifier = unsafe { Identifier::new_unchecked(s) };
        assert_eq!(s, identifier.as_str());
    }
    #[test]
    fn new_unchecked_heap() {
        let s = "abcdefghi";
        assert!(s.len() > 8);
        let identifier = unsafe { Identifier::new_unchecked(s) };
        assert_eq!(s, identifier.as_str());
    }
    #[test]
    fn new_unchecked_zero() {
        let s = "0";
        let identifier = unsafe { Identifier::new_unchecked(s) };
        assert_eq!(s, identifier.as_str());
    }
    #[test]
    fn new_unchecked_bounds() {
        let s = "bounds";
        let identifier = unsafe { Identifier::new_unchecked(s) };
        assert_eq!(s, identifier.as_str());
    }
    #[test]
    #[should_panic]
    fn new_unchecked_unsafe() {
        unsafe {
            let s = std::str::from_utf8_unchecked(&[0xFF, 0xFF, 0xFF, 0xFF]);
            let _identifier = Identifier::new_unchecked(s);
        }
    }
}
#[cfg(test)]
mod tests_llm_16_34 {
    use super::*;
    use crate::*;
    use std::num::NonZeroUsize;
    #[test]
    fn test_bytes_for_small_varint() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let len = NonZeroUsize::new(rug_fuzz_0).unwrap();
        debug_assert_eq!(bytes_for_varint(len), 1);
             }
});    }
    #[test]
    fn test_bytes_for_large_varint() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let len = NonZeroUsize::new(std::usize::MAX).unwrap();
        let max_varint_bytes = (std::mem::size_of::<usize>() * rug_fuzz_0 + rug_fuzz_1)
            / rug_fuzz_2;
        debug_assert_eq!(bytes_for_varint(len), max_varint_bytes);
             }
});    }
    #[test]
    fn test_bytes_for_varint_with_specific_values() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let test_values = vec![
            (NonZeroUsize::new(rug_fuzz_0).unwrap(), rug_fuzz_1), (NonZeroUsize::new(127)
            .unwrap(), 1), (NonZeroUsize::new(128).unwrap(), 2),
            (NonZeroUsize::new(16383).unwrap(), 2), (NonZeroUsize::new(16384).unwrap(),
            3)
        ];
        for (len, expected_bytes) in test_values {
            debug_assert_eq!(bytes_for_varint(len), expected_bytes);
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_35 {
    use super::*;
    use crate::*;
    use std::ptr;
    use std::num::NonZeroUsize;
    #[test]
    fn test_decode_len_single_byte() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u8, u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bytes = [rug_fuzz_0, rug_fuzz_1];
        let len = unsafe { identifier::decode_len(bytes.as_ptr()) };
        debug_assert_eq!(NonZeroUsize::new(rug_fuzz_2).unwrap(), len);
             }
});    }
    #[test]
    fn test_decode_len_two_bytes() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u8, u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bytes = [rug_fuzz_0, rug_fuzz_1];
        let len = unsafe { identifier::decode_len(bytes.as_ptr()) };
        debug_assert_eq!(NonZeroUsize::new(rug_fuzz_2).unwrap(), len);
             }
});    }
    #[test]
    fn test_decode_len_long() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(u8, u8, u8, u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bytes = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let len = unsafe { identifier::decode_len(bytes.as_ptr()) };
        debug_assert_eq!(NonZeroUsize::new(rug_fuzz_4).unwrap(), len);
             }
});    }
    #[test]
    #[should_panic]
    fn test_decode_len_with_zero() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bytes = [rug_fuzz_0, rug_fuzz_1];
        let _ = unsafe { identifier::decode_len(bytes.as_ptr()) };
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_37_llm_16_37 {
    use crate::identifier::{Identifier, inline_as_str};
    #[test]
    fn test_inline_as_str_empty() {
        let _rug_st_tests_llm_16_37_llm_16_37_rrrruuuugggg_test_inline_as_str_empty = 0;
        let empty = Identifier::empty();
        unsafe {
            let result = inline_as_str(&empty);
            debug_assert!(result.is_empty());
        }
        let _rug_ed_tests_llm_16_37_llm_16_37_rrrruuuugggg_test_inline_as_str_empty = 0;
    }
    #[test]
    fn test_inline_as_str_single_char() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let single_char = unsafe { Identifier::new_unchecked(rug_fuzz_0) };
        unsafe {
            let result = inline_as_str(&single_char);
            debug_assert_eq!(result, "a");
        }
             }
});    }
    #[test]
    fn test_inline_as_str_multiple_chars() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let multi_char = unsafe { Identifier::new_unchecked(rug_fuzz_0) };
        unsafe {
            let result = inline_as_str(&multi_char);
            debug_assert_eq!(result, "rust");
        }
             }
});    }
    #[test]
    #[should_panic(expected = "attempted to index into character boundary")]
    fn test_inline_as_str_non_ascii() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let non_ascii = unsafe { Identifier::new_unchecked(rug_fuzz_0) };
        unsafe {
            let _result = inline_as_str(&non_ascii);
        }
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_38 {
    use std::num::NonZeroUsize;
    use std::ptr;
    use crate::identifier::Identifier;
    use crate::identifier::inline_len;
    #[test]
    fn test_inline_len_empty() {
        let _rug_st_tests_llm_16_38_rrrruuuugggg_test_inline_len_empty = 0;
        let empty = Identifier::empty();
        let length = unsafe { inline_len(&empty) };
        debug_assert_eq!(length, NonZeroUsize::new(8).unwrap());
        let _rug_ed_tests_llm_16_38_rrrruuuugggg_test_inline_len_empty = 0;
    }
    #[test]
    fn test_inline_len_one_byte() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let one_byte = unsafe { Identifier::new_unchecked(rug_fuzz_0) };
        let length = unsafe { inline_len(&one_byte) };
        debug_assert_eq!(length, NonZeroUsize::new(1).unwrap());
             }
});    }
    #[test]
    fn test_inline_len_multiple_bytes() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let multiple_bytes = unsafe { Identifier::new_unchecked(rug_fuzz_0) };
        let length = unsafe { inline_len(&multiple_bytes) };
        debug_assert_eq!(length, NonZeroUsize::new(7).unwrap());
             }
});    }
    #[test]
    fn test_inline_len_max_bytes() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let max_bytes = unsafe { Identifier::new_unchecked(rug_fuzz_0) };
        let length = unsafe { inline_len(&max_bytes) };
        debug_assert_eq!(length, NonZeroUsize::new(8).unwrap());
             }
});    }
    #[test]
    #[should_panic]
    fn test_inline_len_panic_on_null_bytes() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let null_bytes = unsafe { Identifier::new_unchecked(rug_fuzz_0) };
        let _ = unsafe { inline_len(&null_bytes) };
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_39 {
    use super::*;
    use crate::*;
    use std::ptr::NonNull;
    #[test]
    fn test_ptr_as_str() {
        unsafe {
            let s = "example";
            let ptr = s.as_ptr();
            let non_null_ptr = NonNull::new(ptr as *mut u8).unwrap();
            let result = ptr_as_str(&non_null_ptr);
            assert_eq!(result, "example");
        }
    }
    unsafe fn repr_to_ptr(repr: NonNull<u8>) -> *const u8 {
        repr.as_ptr()
    }
    unsafe fn decode_len(_ptr: *const u8) -> usize {
        7
    }
}
#[cfg(test)]
mod tests_llm_16_40 {
    use super::*;
    use crate::*;
    use std::ptr::NonNull;
    #[test]
    fn test_ptr_to_repr_non_null() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(u8, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut data = rug_fuzz_0;
        let ptr = &mut data as *mut u8;
        let non_null_repr = ptr_to_repr(ptr);
        debug_assert!(std::ptr::NonNull::new(ptr).is_some());
        debug_assert_eq!(non_null_repr.as_ptr() as usize & rug_fuzz_1, 1);
             }
});    }
    #[test]
    #[should_panic(expected = "is not zero")]
    fn test_ptr_to_repr_null() {
        let _rug_st_tests_llm_16_40_rrrruuuugggg_test_ptr_to_repr_null = 0;
        let ptr = std::ptr::null_mut::<u8>();
        let _non_null_repr = ptr_to_repr(ptr);
        let _rug_ed_tests_llm_16_40_rrrruuuugggg_test_ptr_to_repr_null = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_41 {
    use super::*;
    use crate::*;
    use std::ptr::NonNull;
    #[test]
    fn test_repr_to_ptr() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut value = rug_fuzz_0;
        let modified_ref: NonNull<u8> = NonNull::from(&mut value);
        let result_ptr = repr_to_ptr(modified_ref);
        let expected_ptr: *const u8 = &value as *const u8;
        debug_assert_eq!(
            expected_ptr, result_ptr,
            "The transformed pointer should be equal to the original pointer"
        );
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_42 {
    use super::*;
    use crate::*;
    use std::ptr::NonNull;
    #[test]
    fn test_repr_to_ptr_mut() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(u8, &str, u8) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut value: u8 = rug_fuzz_0;
        let nonnull_ptr = NonNull::new(&mut value as *mut u8).expect(rug_fuzz_1);
        let ptr_mut = repr_to_ptr_mut(nonnull_ptr);
        unsafe {
            debug_assert_eq!(* ptr_mut, 42);
            *ptr_mut = rug_fuzz_2;
            debug_assert_eq!(value, 24);
        }
             }
});    }
}
