/// Some basic collections implemented from scratch in Rust.
///
/// Linked Lists are based on this [book](https://rust-unofficial.github.io/too-many-lists/)
pub mod collections;

#[cfg(test)]
mod tests {
    use crate::collections::SimpleVector;

    #[test]
    fn load_simple_vector() {
        let _ = SimpleVector::<i32>::new();
    }
}
