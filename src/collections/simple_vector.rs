#[derive(Debug)]
pub struct SimpleVector<T> {
    size: usize,
    array: Box<[Option<T>]>,
}

const INITIAL_CAPACITY: usize = 1;
const GROWTH_FACTOR: usize = 2;
impl<T: std::marker::Copy> SimpleVector<T> {
    /// Creates an empty vector.
    pub fn new() -> Self {
        let array = [None; INITIAL_CAPACITY];
        Self {
            size: 0,
            array: Box::new(array),
        }
    }

    /// Inserts an element at the end of the vector.
    pub fn push(&mut self, value: T) {
        if self.size == self.array.len() {
            self.grow();
        }

        self.size += 1;
        self.array[self.size - 1] = Some(value);
    }

    /// Returns the element at position `index`.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    pub fn get(&self, index: usize) -> T {
        self.check_index_within_bounds(index);

        self.array[index].expect("populated index is Some")
    }

    /// Removes and returns the element at position `index`, shifting all elements after it to the left.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    pub fn remove(&mut self, index: usize) -> T {
        self.check_index_within_bounds(index);

        let value = self.array[index].take().expect("populated index is Some");
        self.array[index..self.size].copy_within(1.., 0);
        value
    }

    /// Checks if `index` is within the populated bounds of the vector by creating
    /// a panic if the index is out of bounds.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    fn check_index_within_bounds(&self, index: usize) {
        if index > self.size - 1 {
            panic!("index out of bounds");
        }
    }

    fn grow(&mut self) {
        let mut new_array: Box<[Option<T>]> =
            std::iter::repeat_n(None, self.array.len() * GROWTH_FACTOR).collect();

        new_array[..self.size].copy_from_slice(&self.array[..self.size]);

        self.array = new_array;
    }
}

impl<T: std::marker::Copy> Default for SimpleVector<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        let _: SimpleVector<i32> = SimpleVector::new();
    }

    #[test]
    fn test_push() {
        let mut v = SimpleVector::<i32>::new();

        const TEST_VALUE: i32 = 1;
        v.push(TEST_VALUE);
        v.push(TEST_VALUE);

        assert_eq!(Some(TEST_VALUE), v.array[0]);
        assert_eq!(Some(TEST_VALUE), v.array[1]);
    }

    #[test]
    fn test_remove() {
        let mut v = SimpleVector::<i32>::new();

        for i in 1..6 {
            v.push(i);
        }

        v.remove(4);
        v.remove(1);

        assert_eq!(v.array[0..3], [Some(1), Some(3), Some(4)]);
    }

    #[test]
    fn test_grow() {
        let mut v = SimpleVector::<i32>::new();

        for i in 1..(INITIAL_CAPACITY + 2) {
            v.push(i as i32);
        }

        assert_eq!(
            v.array[INITIAL_CAPACITY].unwrap() - 1,
            INITIAL_CAPACITY as i32
        );
    }
}
