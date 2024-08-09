use crate::sys::Selector;
use crate::Token;
use std::io;
#[derive(Debug)]
pub struct Waker {}
impl Waker {
    pub fn new(_: &Selector, _: Token) -> io::Result<Waker> {
        os_required!();
    }
    pub fn wake(&self) -> io::Result<()> {
        os_required!();
    }
}
#[cfg(test)]
mod tests_llm_16_57 {
    use super::*;
    use crate::*;
    use std::io;
    use crate::sys::shell::waker::Waker;
    use crate::sys::shell::selector::Selector;
    use crate::token::Token;
    #[test]
    fn test_waker_new() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let selector = Selector {};
        let token = Token(rug_fuzz_0);
        let result = Waker::new(&selector, token);
        debug_assert!(result.is_err(), "Waker::new should return an error");
             }
}
}
}    }
}
#[cfg(test)]
mod tests_rug_21 {
    use super::*;
    #[test]
    fn test_rug() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Waker::new(&Selector {}, Token(rug_fuzz_0)).unwrap();
        p0.wake().unwrap();
             }
}
}
}    }
}
