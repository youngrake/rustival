mod fb2parser;
mod reversedtree;
mod threegramindex;

pub use fb2parser::Fb2Parser;
pub use reversedtree::ReversedTree;
pub use threegramindex::Threegramindex;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
