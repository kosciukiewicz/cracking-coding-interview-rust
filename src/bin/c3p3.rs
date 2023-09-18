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

impl<T> ArrayStack<T> {
    fn remove_bottom(&mut self) -> T {
        self.items.remove(0)
    }
}

impl<T> Default for ArrayStack<T> {
    fn default() -> Self {
        ArrayStack { items: Vec::new() }
    }
}

#[derive(Debug)]
struct SetOfStacks<T> {
    stacks: Vec<ArrayStack<T>>,
    capacity: usize,
}

impl<T> Stack<T> for SetOfStacks<T> {
    fn push(&mut self, element: T) {
        let n_stacks = self.stacks.len();
        if n_stacks == 0 || self.stacks.last().unwrap().length() >= self.capacity {
            let mut stack: ArrayStack<T> = ArrayStack::default();
            stack.push(element);
            self.stacks.push(stack);
        } else {
            let stack = self.stacks.get_mut(n_stacks - 1).unwrap();
            stack.push(element);
        }
    }

    fn pop(&mut self) -> Option<T> {
        let n_stacks = self.stacks.len();
        if n_stacks == 0 {
            None
        } else {
            let stack = self.stacks.get_mut(n_stacks - 1).unwrap();
            let element = stack.pop();
            if stack.length() == 0 {
                self.stacks.pop();
            }
            element
        }
    }

    fn peek(&self) -> Option<&T> {
        if let Some(stack) = self.stacks.last() {
            stack.peek()
        } else {
            None
        }
    }

    fn length(&self) -> usize {
        self.stacks.iter().map(|stack| stack.length()).sum()
    }
}

impl<T> SetOfStacks<T> {
    fn new(capacity: usize) -> Self {
        SetOfStacks {
            stacks: Vec::new(),
            capacity,
        }
    }

    fn peek_at_n(&self, n: usize) -> Option<&T> {
        if let Some(stack) = self.stacks.get(n) {
            stack.peek()
        } else {
            None
        }
    }

    fn pop_at_n(&mut self, n: usize) -> Option<T> {
        self.shift(n, true)
    }

    fn shift(&mut self, n: usize, from_top: bool) -> Option<T> {
        if self.stacks.len() <= n {
            None
        } else {
            let stack = self.stacks.get_mut(n).unwrap();
            let removed_item = if from_top {
                stack.pop()
            } else {
                Some(stack.remove_bottom())
            };
            let stack_len = stack.length();
            if stack_len == 0 {
                self.stacks.remove(n);
            } else if stack_len > n {
                let element = self.shift(n + 1, false).unwrap();
                let new_stack = self.stacks.get_mut(n).unwrap();
                new_stack.push(element);
            }

            removed_item
        }
    }
}

impl<T> Default for SetOfStacks<T> {
    fn default() -> Self {
        SetOfStacks {
            stacks: Vec::new(),
            capacity: 5,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{SetOfStacks, Stack};
    use rstest::{fixture, rstest};

    #[fixture]
    fn stack() -> SetOfStacks<i32> {
        let mut stack = SetOfStacks::new(3);
        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.push(4);
        stack.push(5);
        stack.push(6);
        stack.push(7);
        stack
    }

    #[rstest]
    #[case(1, 3)]
    #[case(2, 3)]
    #[case(3, 4)]
    fn test_push(
        #[case] n_elements_to_push: i32,
        #[case] expected_stacks: usize,
        stack: SetOfStacks<i32>,
    ) {
        let mut mut_stack = stack;
        for _ in 0..n_elements_to_push {
            mut_stack.push(1)
        }

        assert_eq!(mut_stack.stacks.len(), expected_stacks)
    }

    #[rstest]
    #[case(0, 3)]
    #[case(1, 2)]
    #[case(2, 2)]
    #[case(4, 1)]
    fn test_pop(
        #[case] n_elements_to_pop: i32,
        #[case] expected_stacks: usize,
        stack: SetOfStacks<i32>,
    ) {
        let mut mut_stack = stack;
        for _ in 0..n_elements_to_pop {
            mut_stack.pop();
        }

        assert_eq!(mut_stack.stacks.len(), expected_stacks)
    }

    #[rstest]
    #[case(0, 7)]
    #[case(1, 6)]
    #[case(2, 5)]
    #[case(4, 3)]
    fn test_peek(
        #[case] n_elements_to_pop: i32,
        #[case] expected_element: i32,
        stack: SetOfStacks<i32>,
    ) {
        let mut mut_stack = stack;
        for _ in 0..n_elements_to_pop {
            mut_stack.pop();
        }

        assert_eq!(*mut_stack.peek().unwrap(), expected_element)
    }

    #[rstest]
    #[case(0, Some(3))]
    #[case(1, Some(6))]
    #[case(2, Some(7))]
    #[case(3, None)]
    fn test_peek_at_n(
        #[case] n: usize,
        #[case] expected_element: Option<i32>,
        stack: SetOfStacks<i32>,
    ) {
        if let Some(element) = stack.peek_at_n(n) {
            assert_eq!(Some(element), expected_element.as_ref())
        } else {
            assert_eq!(None, expected_element)
        }
    }

    #[rstest]
    #[case(0, Some(3))]
    #[case(1, Some(6))]
    #[case(2, Some(7))]
    #[case(3, None)]
    fn test_pop_at_n(
        #[case] n: usize,
        #[case] expected_element: Option<i32>,
        stack: SetOfStacks<i32>,
    ) {
        let mut mut_stack = stack;
        let n_stacks = mut_stack.stacks.len();
        let element = mut_stack.pop_at_n(n);

        println!("{:?}", mut_stack);

        if let Some(element) = element {
            assert_eq!(Some(element), expected_element);
            assert_eq!(mut_stack.stacks.len(), n_stacks - 1);
        } else {
            assert_eq!(None, expected_element)
        }
    }
}

fn main() {
    let mut triple_stack = SetOfStacks::new(123);
    triple_stack.push(5);
    triple_stack.push(2);
    triple_stack.peek_at_n(2);
    triple_stack.pop_at_n(2);
}
