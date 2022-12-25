use coding_interview_rust::utils::linked_list::{LinkedList, Node};
use std::cell::{Ref, RefCell};
use std::fmt::Display;
use std::hash::Hash;
use std::rc::Rc;

trait KthFromLast<T> {
    fn kth_from_last_1(&mut self, k: usize) -> Option<T>;
    fn kth_from_last_2(&mut self, k: usize) -> Option<T>;
    fn kth_from_last_3(&mut self, k: usize) -> Option<T>;

    fn _kth_from_last_recursive(&self, node: Ref<Node<T>>, k: usize) -> (usize, T)
    where
        T: Clone,
    {
        if let Some(node_ref) = node.next.as_ref().cloned() {
            let (index, val) = self._kth_from_last_recursive(node_ref.borrow(), k);
            if index >= k {
                (index + 1, val)
            } else {
                (index + 1, node.val.clone())
            }
        } else {
            (0, node.val.clone())
        }
    }
}

impl<T> KthFromLast<T> for LinkedList<T>
where
    T: Clone + Hash + Eq + Display,
{
    fn kth_from_last_1(&mut self, k: usize) -> Option<T> {
        for (i, node_ref) in self.ref_iter().enumerate() {
            if self.length - i - 1 == k {
                return Some(node_ref.borrow().val.clone());
            }
        }

        None
    }

    fn kth_from_last_2(&mut self, k: usize) -> Option<T> {
        if let Some(head) = self.head.as_ref().cloned() {
            let (_, val) = self._kth_from_last_recursive(head.borrow(), k);
            Some(val)
        } else {
            None
        }
    }

    fn kth_from_last_3(&mut self, k: usize) -> Option<T> {
        if let Some(head) = self.head.as_ref() {
            let mut p1: Rc<RefCell<Node<T>>> = head.clone();
            let mut p2: Rc<RefCell<Node<T>>> = head.clone();

            for _ in 0..k {
                let node_ref = p1.borrow().next.as_ref().cloned();
                if let Some(node_ref) = node_ref {
                    p1 = node_ref;
                } else {
                    break;
                }
            }

            let mut node_ref_op = p1.borrow().next.as_ref().cloned();
            while let Some(node_ref) = node_ref_op {
                p1 = node_ref;
                let next = p2.borrow().next.as_ref().cloned().unwrap();
                p2 = next;
                node_ref_op = p1.borrow().next.as_ref().cloned();
            }

            let val = p2.borrow().val.clone();
            Some(val)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::KthFromLast;
    use coding_interview_rust::utils::linked_list::LinkedList;
    use rstest::rstest;

    #[rstest]
    #[case(vec ! [1, 1, 2, 3], 2, Some(1))]
    #[case(vec ! [1, 1, 2, 3, 44, 44, 55, 55], 4, Some(3))]
    #[case(vec ! [1, 2, 3, 4, 5], 2, Some(3))]
    fn test_kth_from_last_1(
        #[case] input_vec: Vec<i32>,
        #[case] k: usize,
        #[case] expected: Option<i32>,
    ) {
        let mut linked_list = LinkedList::from_vec(input_vec);
        let actual = linked_list.kth_from_last_1(k);

        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case(vec ! [1, 1, 2, 3], 2, Some(1))]
    #[case(vec ! [1, 1, 2, 3, 44, 44, 55, 55], 4, Some(3))]
    #[case(vec ! [1, 2, 3, 4, 5], 2, Some(3))]
    fn test_kth_from_last_2(
        #[case] input_vec: Vec<i32>,
        #[case] k: usize,
        #[case] expected: Option<i32>,
    ) {
        let mut linked_list = LinkedList::from_vec(input_vec);
        let actual = linked_list.kth_from_last_2(k);

        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case(vec ! [1, 1, 2, 3], 2, Some(1))]
    #[case(vec ! [1, 1, 2, 3, 44, 44, 55, 55], 4, Some(3))]
    #[case(vec ! [1, 2, 3, 4, 5], 2, Some(3))]
    fn test_kth_from_last_3(
        #[case] input_vec: Vec<i32>,
        #[case] k: usize,
        #[case] expected: Option<i32>,
    ) {
        let mut linked_list = LinkedList::from_vec(input_vec);
        let actual = linked_list.kth_from_last_3(k);

        assert_eq!(actual, expected);
    }
}

fn main() {
    let v = vec![1, 1, 2, 3];
    let mut linked_list = LinkedList::from_vec(v);

    println!("{:?}", linked_list.kth_from_last_1(2));
    println!("{:?}", linked_list.kth_from_last_2(2));
    println!("{:?}", linked_list.kth_from_last_3(2));
}
