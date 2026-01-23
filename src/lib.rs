//! Synapse DB - A database implementation in Rust
//!
//! This crate provides the core database functionality.

pub mod query;
pub mod storage;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
