use std::mem;

#[derive(Default)]
pub struct LinkedStack {
    head: Link,
}

#[derive(Default)]
enum Link {
    #[default]
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl LinkedStack {
    pub fn new() -> Self {
        LinkedStack { head: Link::Empty }
    }

    /// Inserts an element at the beginning of the list.
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    /// Removes and returns the first element in the list.
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

// Manually implement Drop traint, because derived drop causes recursion and
// can not be optimized for tal-recursion
impl Drop for LinkedStack {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node goes out of scope and gets
            // its `next` field has been set to an empty link
            // no unbound recursion occurs
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedStack;

    #[test]
    fn module_loaded() {}

    #[test]
    fn push_pop() {
        let mut list = LinkedStack::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
