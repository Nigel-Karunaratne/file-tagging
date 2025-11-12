// Files in library
mod workspace;
mod tagfile;
mod errors;
mod tag;

// Public interface for library
pub use workspace::Workspace;
pub use errors::TagAddError;

//*******************************************************************/
// - keep test binding fn for now
pub fn three_mult(first: u64, second: u64, third: u64) -> u64 {
    first * second * third
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = three_mult(2, 7, 8);
        assert_eq!(result, 52);
    }
}
