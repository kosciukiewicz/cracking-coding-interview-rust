use coding_interview_rust::utils::linked_list::{LinkedList, Node};
use std::cell::RefCell;
use std::rc::Rc;

fn sum_digits_reverse_internal(
    linked_list: &mut LinkedList<i32>,
    d1_val: i32,
    d2_val: i32,
    carry: i32,
) -> i32 {
    let mut new_value = d1_val + d2_val + carry;
    if new_value >= 10 {
        new_value -= 10;
        linked_list.push_back(new_value);
        1
    } else {
        linked_list.push_back(new_value);
        0
    }
}

fn sum_digits_reverse(digit_1: LinkedList<i32>, digit_2: LinkedList<i32>) -> LinkedList<i32> {
    let mut result = LinkedList::new();

    let mut d1_head = digit_1.head.as_ref().cloned();
    let mut d2_head = digit_2.head.as_ref().cloned();
    let mut carry = 0;
    let mut d1_val: i32;
    let mut d2_val: i32;

    while d1_head.is_some() || d2_head.is_some() {
        if let Some(node) = d1_head.as_ref().cloned() {
            d1_head = node.borrow().clone().next;
            d1_val = node.borrow().val;
        } else {
            d1_head = None;
            d1_val = 0;
        };
        if let Some(node) = d2_head.as_ref().cloned() {
            d2_head = node.borrow().clone().next;
            d2_val = node.borrow().val;
        } else {
            d2_head = None;
            d2_val = 0;
        };

        carry = sum_digits_reverse_internal(&mut result, d1_val, d2_val, carry);
    }
    result
}

fn pad_list(linked_list: &mut LinkedList<i32>, padding: usize) {
    for _ in 0..padding {
        linked_list.push_start(0)
    }
}

fn sum_digits_forward_internal(
    d1_ref: Option<Rc<RefCell<Node<i32>>>>,
    d2_ref: Option<Rc<RefCell<Node<i32>>>>,
) -> (LinkedList<i32>, i32) {
    match (d1_ref.as_ref(), d2_ref.as_ref()) {
        (Some(_d1_ref), Some(_d2_ref)) => {
            let (mut result, carry) = sum_digits_forward_internal(
                _d1_ref.borrow().next.as_ref().cloned(),
                _d2_ref.borrow().next.as_ref().cloned(),
            );

            let mut new_value = _d1_ref.borrow().val + _d2_ref.borrow().val + carry;

            let carry = if new_value >= 10 {
                new_value -= 10;
                1
            } else {
                0
            };

            result.push_start(new_value);
            (result, carry)
        }
        (None, None) => (LinkedList::new(), 0),
        _ => panic!("Unexpected behaviour"),
    }
}

fn sum_digits_forward(digit_1: LinkedList<i32>, digit_2: LinkedList<i32>) -> LinkedList<i32> {
    let mut _digit_1 = LinkedList::from_vec(digit_1.to_vec());
    let mut _digit_2 = LinkedList::from_vec(digit_2.to_vec());
    let d1_length = digit_1.length;
    let d2_length = digit_2.length;

    if d1_length != d2_length {
        if d1_length > d2_length {
            pad_list(&mut _digit_2, d1_length - d2_length)
        } else {
            pad_list(&mut _digit_1, d2_length - d1_length)
        }
    }

    println!("{:?}", _digit_1.to_vec());
    println!("{:?}", _digit_2.to_vec());
    let (mut result, carry) = sum_digits_forward_internal(
        _digit_1.head.as_ref().cloned(),
        _digit_2.head.as_ref().cloned(),
    );

    if carry > 0 {
        result.push_start(carry)
    }

    result
}

#[cfg(test)]
mod tests {
    use super::sum_digits_reverse;
    use crate::sum_digits_forward;
    use coding_interview_rust::utils::linked_list::LinkedList;
    use rstest::rstest;

    #[rstest]
    #[case(vec ! [1, 2], vec ! [1, 1], vec ! [2, 3])]
    #[case(vec ! [9, 2], vec ! [4, 1], vec ! [3, 4])]
    #[case(vec ! [9, 2], vec ! [1, 4, 1], vec ! [0, 7, 1])]
    fn test_sum_digits_reverse(
        #[case] digit_1_vec: Vec<i32>,
        #[case] digit_2_vec: Vec<i32>,
        #[case] expected: Vec<i32>,
    ) {
        assert_eq!(
            sum_digits_reverse(
                LinkedList::from_vec(digit_1_vec),
                LinkedList::from_vec(digit_2_vec),
            )
            .to_vec(),
            expected
        );
    }

    #[rstest]
    #[case(vec ! [2, 1], vec ! [1, 1], vec ! [3, 2])]
    #[case(vec ! [2, 9], vec ! [1, 4], vec ! [4, 3])]
    #[case(vec ! [2, 9], vec ! [1, 4, 1], vec ! [1, 7, 0])]
    fn test_sum_digits_forward(
        #[case] digit_1_vec: Vec<i32>,
        #[case] digit_2_vec: Vec<i32>,
        #[case] expected: Vec<i32>,
    ) {
        assert_eq!(
            sum_digits_forward(
                LinkedList::from_vec(digit_1_vec),
                LinkedList::from_vec(digit_2_vec),
            )
            .to_vec(),
            expected
        );
    }
}

fn main() {
    println!(
        "{:?}",
        sum_digits_reverse(
            LinkedList::from_vec(vec![1, 2]),
            LinkedList::from_vec(vec![3, 2]),
        )
        .to_vec()
    );

    println!(
        "{:?}",
        sum_digits_forward(
            LinkedList::from_vec(vec![1, 2]),
            LinkedList::from_vec(vec![3, 2]),
        )
        .to_vec()
    );
}
