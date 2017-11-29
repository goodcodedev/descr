extern crate nom;
extern crate elapsed;
#[macro_use]
extern crate descr_common;
extern crate descr_lang;
pub mod lang_data;
pub mod process;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
