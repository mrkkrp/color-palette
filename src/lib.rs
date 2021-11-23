mod color;

pub use crate::color::{mix, Component, Ryb, BLACK, WHITE};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
