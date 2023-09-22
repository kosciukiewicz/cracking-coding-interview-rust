trait Stack<T> {
    fn push(&mut self, element: T);
    fn pop(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
    fn length(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.length() == 0
    }
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

fn sort_stack<T: std::cmp::PartialOrd>(stack: &mut ArrayStack<T>) {
    if stack.is_empty() {
        return;
    }

    let stack_length = stack.length();
    let mut sorted_stack = ArrayStack::<T>::default();

    for _ in 0..stack_length {
        let element_to_push = stack.pop().unwrap();
        if sorted_stack.is_empty() || *sorted_stack.peek().unwrap() <= element_to_push {
            sorted_stack.push(element_to_push)
        } else {
            let mut elements_to_transfer: usize = 0;
            while !sorted_stack.is_empty() && *sorted_stack.peek().unwrap() > element_to_push {
                stack.push(sorted_stack.pop().unwrap());
                elements_to_transfer += 1;
            }
            sorted_stack.push(element_to_push);
            for _ in 0..elements_to_transfer {
                sorted_stack.push(stack.pop().unwrap())
            }
        }
    }

    while !sorted_stack.is_empty() {
        stack.push(sorted_stack.pop().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::{sort_stack, ArrayStack, Stack};
    use rstest::rstest;

    #[rstest]
    #[case(vec ! [1, 4, 2, 1, 5, 8], vec ! [8, 5, 4, 2, 1, 1 ])]
    #[case(vec ! [7, 2, 1, 4], vec ! [7, 4, 2, 1])]
    #[case(vec ! [1, 1, 1, 1], vec ! [1, 1, 1, 1])]
    #[case(vec ! [], vec ! [])]
    fn test_sort_stack(#[case] stack_vec: Vec<i32>, #[case] expected_vec: Vec<i32>) {
        let mut stack = ArrayStack::default();
        for element in stack_vec {
            stack.push(element)
        }

        sort_stack(&mut stack);

        let mut actual_vec: Vec<i32> = Vec::new();
        while !stack.is_empty() {
            actual_vec.insert(0, stack.pop().unwrap())
        }

        assert_eq!(expected_vec, actual_vec);
    }
}

fn main() {
    let mut stack: ArrayStack<i32> = ArrayStack::default();
    stack.push(1);
    stack.push(5);
    stack.push(4);
    stack.push(3);
    stack.push(8);
    println!("{:?}", stack);
    sort_stack(&mut stack);
    println!("{:?}", stack);
}
