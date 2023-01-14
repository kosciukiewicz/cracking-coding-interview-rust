use coding_interview_rust::utils::linked_list::{LinkedList, Node};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type NodeRef = Rc<RefCell<Node<i32>>>;

fn is_palindrome(list: LinkedList<i32>) -> bool {
    let mut buf_stack: VecDeque<i32> = VecDeque::new();

    if list.head.is_some() {
        let mut head_1 = list.head.as_ref().cloned();
        let mut head_2 = head_1.clone();

        while let Some(_head_1) = head_1.as_ref().cloned() {
            let _head_2 = head_2.unwrap().borrow().clone();
            head_1 = _head_1.borrow().next.as_ref().cloned();

            if let Some(_head_1) = head_1.as_ref().cloned() {
                buf_stack.push_back(_head_2.val);

                head_1 = _head_1.borrow().next.as_ref().cloned();
            }

            head_2 = _head_2.next;
        }

        while let Some(_head_2) = head_2.as_ref().cloned() {
            let val = buf_stack.pop_back().unwrap();
            if val != _head_2.borrow().val {
                return false;
            }
            head_2 = _head_2.borrow().next.as_ref().cloned();
        }

        true
    } else {
        true
    }
}

fn _recursive_helper(node: NodeRef, len: usize) -> (Option<NodeRef>, bool) {
    return match len {
        0 => (Some(node), true),
        1 => {
            return (Some(node.borrow().clone().next.unwrap()), true);
        }
        _ => {
            let (node_2, result) = _recursive_helper(node.borrow().next.clone().unwrap(), len - 2);
            if let Some(_node_2) = node_2.as_ref().cloned() {
                let _node2 = node_2.unwrap().borrow().clone();
                let _result = result && node.borrow().val == _node2.val;
                (_node2.next.as_ref().cloned(), _result)
            } else {
                (None, result)
            }
        }
    };
}

fn is_palindrome_recursive(list: LinkedList<i32>) -> bool {
    let len = list.length;
    if len == 0 {
        return true;
    }

    let (_, result) = _recursive_helper(list.head.as_ref().unwrap().clone(), len);
    result
}

#[cfg(test)]
mod tests {
    use super::is_palindrome;
    use crate::is_palindrome_recursive;
    use coding_interview_rust::utils::linked_list::LinkedList;
    use rstest::rstest;

    #[rstest]
    #[case(vec ! [1, 4, 1], true)]
    #[case(vec ! [1, 4, 1, 2], false)]
    #[case(vec ! [1, 4, 4, 1], true)]
    #[case(vec ! [1, 4, 5, 4, 1], true)]
    #[case(vec ! [1, 4, 5, 1, 4], false)]
    fn test_is_palindrome(#[case] list: Vec<i32>, #[case] expected: bool) {
        assert_eq!(is_palindrome(LinkedList::from_vec(list)), expected);
    }

    #[rstest]
    #[case(vec ! [1, 4, 1], true)]
    #[case(vec ! [1, 4, 1, 2], false)]
    #[case(vec ! [1, 4, 4, 1], true)]
    #[case(vec ! [1, 4, 5, 4, 1], true)]
    #[case(vec ! [1, 4, 5, 1, 4], false)]
    fn test_is_palindrome_recursive(#[case] list: Vec<i32>, #[case] expected: bool) {
        assert_eq!(
            is_palindrome_recursive(LinkedList::from_vec(list)),
            expected
        );
    }
}

fn main() {
    println!(
        "{}",
        is_palindrome(LinkedList::from_vec(vec![1, 2, 1, 1, 2, 1]))
    );
    println!(
        "{}",
        is_palindrome_recursive(LinkedList::from_vec(vec![1, 2, 1, 1, 2, 1]))
    );
}
