#[macro_use]
extern crate descr_common;
extern crate descr_lang;
extern crate elapsed;
extern crate nom;
pub mod lang_data;
pub mod process;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
