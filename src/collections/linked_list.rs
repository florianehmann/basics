#[derive(Debug)]
pub struct LinkedList<T> {
    head: Node<T>,
}

#[derive(Debug)]
enum Node<T> {
    Empty,
    Filled { data: T, link: Box<Node<T>> },
}

impl<T> Node<T> {
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: Node::Empty }
    }

    pub fn push(&mut self) {
        let _tail = self.find_tail();

        // create new tail node
        // link previous tail and new tail
        todo!();
    }

    fn find_tail(&self) -> &Node<T> {
        let mut tail_node = &self.head;
        while tail_node.is_empty() {
            if let Node::Filled { link, .. } = tail_node {
                if let Node::Empty = link.as_ref() {
                    break;
                };
                tail_node = link;
            };
        }

        tail_node
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{LinkedList, Node};

    #[test]
    fn teat_create() {
        let _ = LinkedList::<i32>::new();
    }

    #[test]
    fn test_find_tail() {
        let node3 = Node::Filled {
            data: 3,
            link: Box::new(Node::Empty),
        };
        let node2 = Node::Filled {
            data: 2,
            link: Box::new(node3),
        };
        let node1 = Node::Filled {
            data: 1,
            link: Box::new(node2),
        };
        let list = LinkedList { head: node1 };

        let final_data = match list.find_tail() {
            Node::Filled { data, .. } => *data,
            _ => panic!("no data found"),
        };
        assert_eq!(final_data, 3);
    }
}
