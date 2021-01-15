use std::mem;

pub struct List<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, None),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        let result;
        match mem::replace(&mut self.head, None) {
            None => result = None,
            Some(node) => {
                result = Some(node.elem);
                self.head = node.next;
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_push() {
        let mut s1 = List::<i32>::new();
        s1.push(8);
        s1.push(7);

        assert_eq!(s1.pop().unwrap(), 7);
        assert_eq!(s1.pop().unwrap(), 8);
    }
}
