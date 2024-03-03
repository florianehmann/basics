pub mod collections;

#[cfg(test)]
mod tests {
    use crate::collections::SimpleVector;

    #[test]
    fn load_simple_vector() {
        let _ = SimpleVector::<i32>::new();
    }
}
