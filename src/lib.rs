mod biwordindex;
mod fb2parser;
mod permutationindex;
mod reversedtree;
mod threegramindex;

pub use biwordindex::BiwordIndex;
pub use fb2parser::Fb2Parser;
pub use permutationindex::PermutationIndex;
pub use reversedtree::ReversedTree;
pub use threegramindex::Threegramindex;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
