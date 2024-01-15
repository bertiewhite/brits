use thiserror::Error;

// tmp allow dead_code
#[allow(dead_code)]
#[derive(Default)]
pub struct Queue<T: Clone> {
    head: Option<Node<T>>,
    length: i32,
}
// tmp allow dead_code
//
#[allow(dead_code)]
struct Node<T: Clone> {
    data: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Error, Debug)]
pub enum QueueErr {
    #[error("Queue is empty")]
    EmptyQueueError,
}

// tmp allow dead_code
#[allow(dead_code)]
impl<T: Clone> Queue<T> {
    pub fn add(mut self, elem: T) -> Self {
        let mut node = new(elem);
        match self.head {
            Option::None => self.head = Option::Some(node),
            Option::Some(head) => {
                node = node.set_next(head);
                self.head = Option::Some(node);
            }
        }
        self.length += 1;
        return self;
    }

    pub fn pop(&mut self) -> Result<T, QueueErr> {
        let head = self.head.take().ok_or_else(|| QueueErr::EmptyQueueError)?;
        match head.next {
            Option::None => {}
            Option::Some(next) => self.head = Option::Some(*next),
        }
        self.length -= 1;
        return Ok(head.data);
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

fn new<T: Clone>(elem: T) -> Node<T> {
    Node {
        data: elem,
        next: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_adds() {
        let mut q: Queue<i32> = Queue::default();
        q = q.add(3);
        q = q.add(2);

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
