trait MinStack<T> {
    fn push(&mut self, element: T);
    fn pop(&mut self) -> T;
    fn peek(&self) -> T;
    fn length(&self) -> usize;
    fn get_min(&self) -> T;
}

struct AdditionalStackMinStack<T> {
    length: usize,
    items: Vec<T>,
    min_values: Vec<T>,
}

impl<T> Default for AdditionalStackMinStack<T> {
    fn default() -> Self {
        AdditionalStackMinStack {
            length: 0,
            items: Vec::new(),
            min_values: Vec::new(),
        }
    }
}

impl MinStack<i32> for AdditionalStackMinStack<i32> {
    fn push(&mut self, element: i32) {
        self.items.push(element);
        self.length += 1;
        if self.min_values.is_empty() {
            self.min_values.push(element);
        } else if *self.min_values.last().unwrap() >= element {
            self.min_values.push(element)
        }
    }

    fn pop(&mut self) -> i32 {
        let element = self.items.pop().unwrap();
        self.length -= 1;
        if element == *self.min_values.last().unwrap() {
            self.min_values.pop();
        }

        element
    }

    fn peek(&self) -> i32 {
        return *self.items.last().unwrap();
    }

    fn length(&self) -> usize {
        self.length
    }

    fn get_min(&self) -> i32 {
        *self.min_values.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{AdditionalStackMinStack, MinStack};
    use rstest::{fixture, rstest};

    #[fixture]
    fn stack() -> AdditionalStackMinStack<i32> {
        let mut stack = AdditionalStackMinStack::default();
        stack.push(2);
        stack.push(3);
        stack.push(1);
        stack.push(1);
        stack.push(2);
        stack.push(2);
        stack
    }

    #[rstest]
    fn test_min(stack: AdditionalStackMinStack<i32>) {
        assert_eq!(stack.get_min(), 1)
    }

    #[rstest]
    #[case(0, 0)]
    #[case(5, 1)]
    #[case(1, 1)]
    fn test_push(
        #[case] element_to_push: i32,
        #[case] expected_min: i32,
        stack: AdditionalStackMinStack<i32>,
    ) {
        let mut mut_stack = stack;
        mut_stack.push(element_to_push);
        assert_eq!(mut_stack.get_min(), expected_min)
    }

    #[rstest]
    #[case(1, 1)]
    #[case(2, 1)]
    #[case(4, 2)]
    fn test_pop(
        #[case] times_pop: usize,
        #[case] expected_min: i32,
        stack: AdditionalStackMinStack<i32>,
    ) {
        let mut mut_stack = stack;
        for _ in 0..times_pop {
            mut_stack.pop();
        }
        assert_eq!(mut_stack.get_min(), expected_min)
    }
}

fn main() {
    let mut triple_stack = AdditionalStackMinStack::default();
    triple_stack.push(5);
    triple_stack.push(2);
}
