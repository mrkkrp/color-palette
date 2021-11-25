mod color;

pub use crate::color::{mix, Component, Ryb};
pub use crate::color::{BLACK, BLUE, CYAN, GREEN, PURPLE, RED, WHITE, YELLOW};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
