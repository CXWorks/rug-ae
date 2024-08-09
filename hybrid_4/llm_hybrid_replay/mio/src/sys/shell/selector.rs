use std::io;
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, RawFd};
use std::time::Duration;
pub type Event = usize;
pub type Events = Vec<Event>;
#[derive(Debug)]
pub struct Selector {}
impl Selector {
    pub fn try_clone(&self) -> io::Result<Selector> {
        os_required!();
    }
    pub fn select(&self, _: &mut Events, _: Option<Duration>) -> io::Result<()> {
        os_required!();
    }
    #[cfg(all(debug_assertions, not(target_os = "wasi")))]
    pub fn register_waker(&self) -> bool {
        os_required!();
    }
}
#[cfg(unix)]
cfg_any_os_ext! {
    use crate :: { Interest, Token }; impl Selector { pub fn register(& self, _ : RawFd,
    _ : Token, _ : Interest) -> io::Result < () > { os_required!(); } pub fn reregister(&
    self, _ : RawFd, _ : Token, _ : Interest) -> io::Result < () > { os_required!(); }
    pub fn deregister(& self, _ : RawFd) -> io::Result < () > { os_required!(); } }
}
#[cfg(target_os = "wasi")]
cfg_any_os_ext! {
    use crate :: { Interest, Token }; impl Selector { pub fn register(& self, _ :
    wasi::Fd, _ : Token, _ : Interest) -> io::Result < () > { os_required!(); } pub fn
    reregister(& self, _ : wasi::Fd, _ : Token, _ : Interest) -> io::Result < () > {
    os_required!(); } pub fn deregister(& self, _ : wasi::Fd) -> io::Result < () > {
    os_required!(); } }
}
cfg_io_source! {
    #[cfg(debug_assertions)] impl Selector { pub fn id(& self) -> usize { os_required!();
    } }
}
#[cfg(unix)]
impl AsRawFd for Selector {
    fn as_raw_fd(&self) -> RawFd {
        os_required!()
    }
}
#[allow(clippy::trivially_copy_pass_by_ref)]
pub mod event {
    use crate::sys::Event;
    use crate::Token;
    use std::fmt;
    pub fn token(_: &Event) -> Token {
        os_required!();
    }
    pub fn is_readable(_: &Event) -> bool {
        os_required!();
    }
    pub fn is_writable(_: &Event) -> bool {
        os_required!();
    }
    pub fn is_error(_: &Event) -> bool {
        os_required!();
    }
    pub fn is_read_closed(_: &Event) -> bool {
        os_required!();
    }
    pub fn is_write_closed(_: &Event) -> bool {
        os_required!();
    }
    pub fn is_priority(_: &Event) -> bool {
        os_required!();
    }
    pub fn is_aio(_: &Event) -> bool {
        os_required!();
    }
    pub fn is_lio(_: &Event) -> bool {
        os_required!();
    }
    pub fn debug_details(_: &mut fmt::Formatter<'_>, _: &Event) -> fmt::Result {
        os_required!();
    }
}
#[cfg(test)]
mod tests_llm_16_12 {
    use super::*;
    use crate::*;
    use std::os::unix::io::AsRawFd;
    #[test]
    fn test_as_raw_fd() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let selector = Selector {};
        let raw_fd = selector.as_raw_fd();
        debug_assert!(raw_fd >= rug_fuzz_0, "RawFd should be non-negative");
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_44 {
    use super::*;
    use crate::*;
    use std::os::unix::io::AsRawFd;
    use std::fmt::Debug;
    use std::io;
    use std::time::Duration;
    #[test]
    #[cfg(all(debug_assertions, not(target_os = "wasi")))]
    fn test_register_waker() {
        let _rug_st_tests_llm_16_44_rrrruuuugggg_test_register_waker = 0;
        let selector = Selector {};
        let is_registered = selector.register_waker();
        let _rug_ed_tests_llm_16_44_rrrruuuugggg_test_register_waker = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_45_llm_16_45 {
    use crate::sys::shell::selector::Selector;
    use crate::sys::Events;
    use std::io;
    use std::os::unix::io::AsRawFd;
    use std::time::Duration;
    #[test]
    fn select_with_no_timeout() -> io::Result<()> {
        let selector = Selector {};
        let mut events = Events::with_capacity(1024);
        selector.select(&mut events, None)
    }
    #[test]
    fn select_with_timeout() -> io::Result<()> {
        let selector = Selector {};
        let mut events = Events::with_capacity(1024);
        let timeout = Duration::from_millis(100);
        selector.select(&mut events, Some(timeout))
    }
    #[test]
    fn try_clone_selector() -> io::Result<()> {
        let selector = Selector {};
        let cloned_selector = selector.try_clone()?;
        assert_eq!(selector.as_raw_fd(), cloned_selector.as_raw_fd());
        Ok(())
    }
    #[cfg(all(debug_assertions, not(target_os = "wasi")))]
    #[test]
    fn register_waker_returns_expected() {
        let selector = Selector {};
        assert_eq!(selector.register_waker(), true);
    }
}
#[cfg(test)]
mod tests_llm_16_46 {
    use super::*;
    use crate::*;
    use std::io;
    use std::os::unix::io::AsRawFd;
    use std::time::Duration;
    use std::io::Error;
    #[test]
    fn test_try_clone() {
        let _rug_st_tests_llm_16_46_rrrruuuugggg_test_try_clone = 0;
        let selector = Selector {};
        let clone_result = selector.try_clone();
        match clone_result {
            Ok(clone) => {
                debug_assert_eq!(selector.as_raw_fd(), clone.as_raw_fd());
            }
            Err(e) => panic!("try_clone failed with error: {}", e),
        }
        let _rug_ed_tests_llm_16_46_rrrruuuugggg_test_try_clone = 0;
    }
}
#[cfg(test)]
mod tests_rug_2 {
    use super::*;
    #[test]
    fn test_is_readable() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: usize = rug_fuzz_0;
        debug_assert!(crate ::sys::shell::selector::event::is_readable(& p0));
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_3 {
    use super::*;
    #[test]
    fn test_is_writable() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: usize = rug_fuzz_0;
        debug_assert_eq!(crate ::sys::shell::selector::event::is_writable(& p0), false);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_5 {
    use super::*;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: usize = rug_fuzz_0;
        let _result = crate::sys::shell::selector::event::is_read_closed(&p0);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_6 {
    use super::*;
    #[test]
    fn test_is_write_closed() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: Event = unsafe { std::mem::transmute(rug_fuzz_0) };
        let result = crate::sys::shell::selector::event::is_write_closed(&p0);
        debug_assert_eq!(result, false);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_7 {
    use super::*;
    #[test]
    fn test_is_priority() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: usize = rug_fuzz_0;
        crate::sys::shell::selector::event::is_priority(&p0);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_8 {
    use super::*;
    #[test]
    fn test_is_aio() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: usize = rug_fuzz_0;
        debug_assert_eq!(
            crate ::sys::shell::selector::event::is_aio(& p0), os_required!()
        );
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_9 {
    use super::*;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: usize = rug_fuzz_0;
        let _result = crate::sys::shell::selector::event::is_lio(&p0);
             }
}
}
}    }
}
