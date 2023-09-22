trait Stack<T> {
    fn push(&mut self, element: T);
    fn pop(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
    fn length(&self) -> usize;
}

#[derive(Debug)]
struct ArrayStack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> for ArrayStack<T> {
    fn push(&mut self, element: T) {
        self.items.push(element)
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    fn length(&self) -> usize {
        self.items.len()
    }
}

impl<T> Default for ArrayStack<T> {
    fn default() -> Self {
        ArrayStack { items: vec![] }
    }
}

struct StackQueue<T> {
    stack: ArrayStack<T>,
    reversed_stack: ArrayStack<T>,
}

impl<T> Default for StackQueue<T> {
    fn default() -> Self {
        StackQueue {
            stack: ArrayStack::default(),
            reversed_stack: ArrayStack::default(),
        }
    }
}

impl<T> StackQueue<T> {
    fn push(&mut self, element: T) {
        self.stack.push(element);
    }

    fn pop(&mut self) -> Option<T> {
        if self.len() == 0 {
            return None;
        }
        self.transfer_elements_if_needed();
        self.reversed_stack.pop()
    }

    fn peek(&mut self) -> Option<&T> {
        if self.len() == 0 {
            return None;
        }
        self.transfer_elements_if_needed();
        self.reversed_stack.peek()
    }

    fn len(&mut self) -> usize {
        self.stack.length() + self.reversed_stack.length()
    }

    fn transfer_elements_if_needed(&mut self) {
        if self.reversed_stack.length() == 0 {
            let stack_length = self.stack.length();
            for _ in 0..stack_length {
                self.reversed_stack.push(self.stack.pop().unwrap());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::StackQueue;
    use rstest::{fixture, rstest};

    #[fixture]
    fn queue() -> StackQueue<i32> {
        let mut queue = StackQueue::default();
        queue.push(1);
        queue.push(2);
        queue.push(3);
        queue.push(4);
        queue
    }

    #[rstest]
    fn test_push(queue: StackQueue<i32>) {
        let mut mut_queue = queue;
        mut_queue.push(5);

        assert_eq!(mut_queue.len(), 5);
        assert_eq!(*mut_queue.peek().unwrap(), 1);
    }

    #[rstest]
    fn test_pop(queue: StackQueue<i32>) {
        let mut mut_queue = queue;

        assert_eq!(mut_queue.pop().unwrap(), 1);
    }
}

fn main() {
    let mut queue: StackQueue<i32> = StackQueue::default();
    queue.push(5);
    queue.push(4);
    queue.push(3);
    println!("{:?}", queue.pop());
    println!("{:?}", queue.peek());
}
