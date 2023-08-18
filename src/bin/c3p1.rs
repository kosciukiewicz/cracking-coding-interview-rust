trait MultiStack<T> {
    fn push(&mut self, stack_index: usize, element: T);
    fn pop(&mut self, stack_index: usize) -> T;
    fn peek(&self, stack_index: usize) -> T;
    fn length(&self, stack_index: usize) -> usize;
}

struct FixedLengthMultiStack<T> {
    max_size: usize,
    items: Vec<Option<T>>,
    sizes: Vec<usize>,
}

impl<T: Clone> FixedLengthMultiStack<T> {
    fn new(stacks: usize, max_size: usize) -> Self {
        FixedLengthMultiStack {
            max_size,
            items: vec![None; stacks * max_size],
            sizes: vec![0; stacks],
        }
    }
}

impl MultiStack<i32> for FixedLengthMultiStack<i32> {
    fn push(&mut self, stack_index: usize, element: i32) {
        let stack_size = self.sizes.get(stack_index).unwrap();
        self.items[stack_index * self.max_size + stack_size] = Some(element);
        self.sizes[stack_index] += 1
    }

    fn pop(&mut self, stack_index: usize) -> i32 {
        let stack_size = *self.sizes.get(stack_index).unwrap();
        self.sizes[stack_index] -= 1;
        let element = self.items[stack_index * self.max_size + stack_size - 1].unwrap();
        self.items[stack_index * self.max_size + stack_size - 1] = None;
        element
    }

    fn peek(&self, stack_index: usize) -> i32 {
        let stack_size = *self.sizes.get(stack_index).unwrap();
        self.items[stack_index * self.max_size + stack_size - 1].unwrap()
    }

    fn length(&self, stack_index: usize) -> usize {
        self.sizes[stack_index]
    }
}

#[cfg(test)]
mod tests {
    use crate::{FixedLengthMultiStack, MultiStack};
    use rstest::{fixture, rstest};

    #[fixture]
    fn triple_stack() -> FixedLengthMultiStack<i32> {
        let mut triple_stack = FixedLengthMultiStack::new(3, 5);
        triple_stack.push(0, 3);
        triple_stack.push(0, 2);
        triple_stack.push(0, 1);
        triple_stack.push(1, 3);
        triple_stack.push(1, 4);
        triple_stack
    }

    #[rstest]
    #[case(0, 3)]
    #[case(1, 2)]
    #[case(2, 0)]
    fn test_length(
        #[case] stack_index: usize,
        #[case] expected_stack_size: usize,
        triple_stack: FixedLengthMultiStack<i32>,
    ) {
        assert_eq!(triple_stack.length(stack_index), expected_stack_size)
    }

    #[rstest]
    #[case(0, 0, 4)]
    #[case(1, 3, 3)]
    #[case(2, 1, 1)]
    fn test_push(
        #[case] stack_index: usize,
        #[case] element: i32,
        #[case] expected_stack_size: usize,
        triple_stack: FixedLengthMultiStack<i32>,
    ) {
        let mut stack = triple_stack;
        stack.push(stack_index, element);
        assert_eq!(stack.length(stack_index), expected_stack_size)
    }

    #[rstest]
    #[case(0, 1)]
    #[case(1, 4)]
    fn test_peek(
        #[case] stack_index: usize,
        #[case] expected_element: i32,
        triple_stack: FixedLengthMultiStack<i32>,
    ) {
        assert_eq!(triple_stack.peek(stack_index), expected_element)
    }

    #[rstest]
    #[case(0, 1)]
    #[case(1, 4)]
    fn test_pop(
        #[case] stack_index: usize,
        #[case] expected_element: i32,
        triple_stack: FixedLengthMultiStack<i32>,
    ) {
        let mut stack = triple_stack;
        let expected_stack_length = stack.length(stack_index) - 1;
        let element = stack.pop(stack_index);
        assert_eq!(element, expected_element);
        assert_eq!(stack.length(stack_index), expected_stack_length);
    }
}

fn main() {
    let mut triple_stack = FixedLengthMultiStack::new(3, 5);
    triple_stack.push(0, 3);
    triple_stack.push(1, 3);
}
