use coding_interview_rust::utils::linked_list::{Node, SingleLinkedList};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type NodeRef = Rc<RefCell<Node<i32>>>;

fn get_loop_node(list: SingleLinkedList<i32>) -> Option<NodeRef> {
    let mut loop_node: Option<NodeRef> = None;

    if let Some(head) = list.head {
        let mut slow_iter: Option<NodeRef> = head.borrow().next.clone();
        let mut fast_iter: Option<NodeRef> = if let Some(next) = head.borrow().next.clone() {
            next.borrow().next.clone()
        } else {
            None
        };

        while let (Some(slot_iter_ref), Some(fast_iter_ref)) =
            (slow_iter.as_ref(), fast_iter.as_ref())
        {
            if Rc::ptr_eq(slot_iter_ref, fast_iter_ref) {
                break;
            }

            slow_iter = slow_iter.unwrap().borrow().next.clone();
            fast_iter = if let Some(next) = fast_iter.unwrap().borrow().next.clone() {
                next.borrow().next.clone()
            } else {
                None
            };
        }

        let mut start_iter: Option<NodeRef> = Some(head);
        while slow_iter.as_ref().is_some() && start_iter.as_ref().is_some() {
            if let (Some(p3_ref), Some(p1_ref)) = (start_iter.as_ref(), slow_iter.as_ref()) {
                if Rc::ptr_eq(p3_ref, p1_ref) {
                    loop_node = Some(p3_ref.clone());
                    break;
                }
            }
            slow_iter = slow_iter.unwrap().borrow().next.clone();
            start_iter = start_iter.unwrap().borrow().next.clone();
        }

        return loop_node;
    }

    loop_node
}

fn prepare_looped_list(list: Vec<i32>) -> SingleLinkedList<i32> {
    let mut ref_map: HashMap<i32, NodeRef> = HashMap::new();
    let mut linked_list: SingleLinkedList<i32> = SingleLinkedList::new();
    for item in list.iter() {
        let node_ref = if ref_map.contains_key(item) {
            ref_map.get(item).unwrap().clone()
        } else {
            let new_node_ref = Rc::new(RefCell::new(Node {
                val: *item,
                next: None,
                prev: None,
            }));
            ref_map.insert(*item, new_node_ref.clone());
            new_node_ref
        };
        linked_list.push_node(node_ref)
    }

    linked_list
}

#[cfg(test)]
mod tests {
    use crate::{get_loop_node, prepare_looped_list};
    use rstest::rstest;
    use std::rc::Rc;

    #[rstest]
    #[case(vec ! [2, 4, 5, 6, 7], None)]
    #[case(vec ! [0, 1, 5, 6, 7, 1], Some(1))]
    #[case(vec ! [0, 1, 5, 6, 1], Some(1))]
    #[case(vec ! [0, 1, 5, 6, 7, 5], Some(5))]
    #[case(vec ! [0, 1, 0], Some(0))]
    fn test_get_intersection_node(#[case] list: Vec<i32>, #[case] expected_loop_node: Option<i32>) {
        let linked_list = prepare_looped_list(list);
        let expected_ref = if let Some(node_val) = expected_loop_node {
            let mut expected_ref_val = None;
            for node_ref in linked_list.ref_iter() {
                if node_ref.borrow().val == node_val {
                    expected_ref_val = Some(node_ref);
                    break;
                }
            }
            expected_ref_val
        } else {
            None
        };

        let actual_ref = get_loop_node(linked_list);
        let result = match (actual_ref.as_ref(), expected_ref.as_ref()) {
            (Some(actual_ref), Some(expected_ref)) => Rc::ptr_eq(actual_ref, expected_ref),
            (Some(_), None) => false,
            (None, Some(_)) => false,
            (None, None) => true,
        };

        assert!(result)
    }
}

fn main() {
    let list = prepare_looped_list(vec![1, 2, 4, 5, 6, 4]);
    println!("{:?}", get_loop_node(list).unwrap().borrow().val);
}
