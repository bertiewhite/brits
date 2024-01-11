use std::fmt;

// tmp allow dead_code
#[allow(dead_code)]
pub struct Queue<T: Clone> {
    head: Option<Node<T>>,
    length: i32,
}

// tmp allow dead_code
#[allow(dead_code)]
struct Node<T: Clone> {
    data: T,
    next: Option<Box<Node<T>>>,
}

// tmp allow dead_code
#[allow(dead_code)]
pub fn new<T: Clone>() -> Queue<T> {
    return Queue {
        head: None,
        length: 0,
    };
}

pub struct EmptyQueueError;
impl fmt::Display for EmptyQueueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Queue is empty")
    }
}

// tmp allow dead_code
#[allow(dead_code)]
impl<'a, T: Clone> Queue<T> {
    pub fn add(&mut self, elem: T) {
        let mut node = Node {
            data: elem,
            next: None,
        };
        match self.head {
            Option::None => self.head = Option::Some(node),
            Option::Some(_) => {
                let head = self.head.take().expect("won't happen");
                node = node.set_next(head);
                self.head = Option::Some(node);
            }
        }
        self.length += 1
    }

    pub fn pop(&mut self) -> Result<T, EmptyQueueError> {
        match self.head {
            Option::None => return Err(EmptyQueueError),
            Option::Some(_) => {
                let head = self.head.take().expect("won't happen");
                match head.next {
                    Option::None => {}
                    Option::Some(next) => self.head = Option::Some(*next),
                }
                self.length -= 1;
                return Ok(head.data);
            }
        };
    }
}

// tmp allow dead_code
#[allow(dead_code)]
impl<'a, T: Clone> Node<T> {
    fn set_next(mut self, node: Node<T>) -> Self {
        self.next = Option::Some(Box::new(node));
        return self;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_adds() {
        let mut q: Queue<i32> = new();
        q.add(3);
        q.add(2);

        let mut res = q.pop();
        match res {
            Ok(val) => assert_eq!(val, 2),
            Err(_) => assert!(false),
        }

        res = q.pop();
        match res {
            Ok(val) => assert_eq!(val, 3),
            Err(_) => assert!(false),
        }

        res = q.pop();
        match res {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }
}
