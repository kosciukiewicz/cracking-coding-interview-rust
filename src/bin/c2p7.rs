use coding_interview_rust::utils::linked_list::{Node, SingleLinkedList};
use std::cell::RefCell;
use std::rc::Rc;

type NodeRef = Rc<RefCell<Node<i32>>>;

fn _get_last_node_and_len(linked_list: &SingleLinkedList<i32>) -> (Option<NodeRef>, usize) {
    if linked_list.head.is_none() {
        return (None, 0);
    }
    let mut len = 1;
    let mut iter = linked_list.ref_iter();
    let mut last_node: NodeRef = iter.next().unwrap();
    for node in iter {
        len += 1;
        last_node = node.clone();
    }

    (Some(last_node), len)
}

fn get_intersection_node(l1: SingleLinkedList<i32>, l2: SingleLinkedList<i32>) -> Option<NodeRef> {
    let (l1_last_node, l1_len) = _get_last_node_and_len(&l1);
    let (l2_last_node, l2_len) = _get_last_node_and_len(&l2);

    if l1_len == 0 || l2_len == 0 {
        return None;
    }

    if Rc::ptr_eq(
        l1_last_node.as_ref().unwrap(),
        l2_last_node.as_ref().unwrap(),
    ) {
        let (longer, shorter) = if l1_len >= l2_len { (l1, l2) } else { (l2, l1) };

        let diff = longer.length - shorter.length;
        let mut longer_iter = longer.ref_iter().skip(diff);
        let mut shorter_iter = shorter.ref_iter();

        let mut longer_head = longer_iter.next();
        let mut shorter_head = shorter_iter.next();

        while let (Some(longer_head_node_ref), Some(shorter_head_node_ref)) =
            (longer_head.as_ref(), shorter_head.as_ref())
        {
            if Rc::ptr_eq(longer_head_node_ref, shorter_head_node_ref) {
                return longer_head;
            }

            longer_head = longer_iter.next();
            shorter_head = shorter_iter.next();
        }

        None
    } else {
        None
    }
}

fn prepare_linked_list(
    list1: Vec<i32>,
    list2: Vec<i32>,
    intersection_node_1: Option<usize>,
    intersection_node_2: Option<usize>,
) -> (
    SingleLinkedList<i32>,
    SingleLinkedList<i32>,
    Option<NodeRef>,
) {
    if intersection_node_1.is_none() || intersection_node_2.is_none() {
        return (
            SingleLinkedList::from_vec(list1),
            SingleLinkedList::from_vec(list2),
            None,
        );
    }

    let mut l1 = SingleLinkedList::new();
    let mut l2 = SingleLinkedList::new();

    for item in list1
        .iter()
        .take(intersection_node_1.unwrap_or(list1.len()))
    {
        l1.push(*item);
    }

    for item in list2
        .iter()
        .take(intersection_node_2.unwrap_or(list2.len()))
    {
        l2.push(*item);
    }

    let mut intersection_node_ref: Option<NodeRef> = None;

    for item in list1.iter().skip(intersection_node_1.unwrap_or_default()) {
        let node = Node {
            val: *item,
            next: None,
            prev: None,
        };
        let node_ref = Rc::new(RefCell::new(node));
        if intersection_node_ref.is_none() {
            intersection_node_ref = Some(node_ref.clone());
        }
        l1.push_node(node_ref.clone());
        l2.push_node(node_ref);
    }

    (l1, l2, intersection_node_ref)
}

#[cfg(test)]
mod tests {
    use super::get_intersection_node;
    use crate::prepare_linked_list;
    use rstest::rstest;
    use std::rc::Rc;

    #[rstest]
    #[case(vec ! [2, 4, 5, 6, 7], vec ! [1, 4, 5, 6, 7], Some(1), Some(1))]
    #[case(vec ! [0, 1, 5, 6, 7], vec ! [0, 1, 2, 3, 4, 5, 6, 7], Some(2), Some(5))]
    #[case(vec ! [5, 1, 5, 6, 7], vec ! [5, 5, 2, 3, 4, 5, 6, 7], Some(2), Some(5))]
    #[case(vec ! [1, 2, 3], vec ! [1, 2, 3], None, None)]
    #[case(vec ! [1, 2, 3], vec ! [1, 2, 3, 4], None, Some(1))]
    fn test_prepare_linked_list(
        #[case] list1: Vec<i32>,
        #[case] list2: Vec<i32>,
        #[case] intersection_node_1: Option<usize>,
        #[case] intersection_node_2: Option<usize>,
    ) {
        let l1_len = list1.len();
        let l2_len = list2.len();
        let (l1, l2, _) =
            prepare_linked_list(list1, list2, intersection_node_1, intersection_node_2);
        for item_1 in l1.ref_iter().take(intersection_node_1.unwrap_or(l1_len)) {
            for item_2 in l2.ref_iter().take(intersection_node_2.unwrap_or(l2_len)) {
                assert!(!Rc::ptr_eq(&item_1, &item_2));
            }
        }

        if let (Some(intersection_node_1_value), Some(intersection_node_2_value)) =
            (intersection_node_1, intersection_node_2)
        {
            let mut l1_iter = l1.ref_iter().skip(intersection_node_1_value);
            let mut l2_iter = l2.ref_iter().skip(intersection_node_2_value);

            let mut l1_next = l1_iter.next();
            let mut l2_next = l2_iter.next();
            while let (Some(l1_node), Some(l2_node)) = (l1_next.as_ref(), l2_next.as_ref()) {
                assert!(Rc::ptr_eq(l1_node, l2_node));
                l1_next = l1_iter.next();
                l2_next = l2_iter.next();
            }

            assert!(l1_next.is_none());
            assert!(l2_next.is_none());
        }
    }

    #[rstest]
    #[case(vec ! [2, 4, 5, 6, 7], vec ! [1, 4, 5, 6, 7], Some(1), Some(1))]
    #[case(vec ! [0, 1, 5, 6, 7], vec ! [0, 1, 2, 3, 4, 5, 6, 7], Some(2), Some(5))]
    #[case(vec ! [5, 1, 5, 6, 7], vec ! [5, 5, 2, 3, 4, 5, 6, 7], Some(2), Some(5))]
    #[case(vec ! [1, 2, 3], vec ! [1, 2, 3], None, None)]
    #[case(vec ! [1, 2, 3], vec ! [1, 2, 3, 4], None, Some(1))]
    fn test_get_intersection_node(
        #[case] list1: Vec<i32>,
        #[case] list2: Vec<i32>,
        #[case] intersection_node_1: Option<usize>,
        #[case] intersection_node_2: Option<usize>,
    ) {
        let (l1, l2, expected) =
            prepare_linked_list(list1, list2, intersection_node_1, intersection_node_2);

        let actual = get_intersection_node(l1, l2);
        let result = match (actual.as_ref(), expected.as_ref()) {
            (Some(actual_ref), Some(expected_ref)) => Rc::ptr_eq(actual_ref, expected_ref),
            (Some(_), None) => false,
            (None, Some(_)) => false,
            (None, None) => true,
        };

        assert!(result)
    }
}

fn main() {
    let (l1, l2, _) = prepare_linked_list(vec![1, 2, 4], vec![5, 6, 4], Some(2), Some(2));
    println!("{:?}", get_intersection_node(l1, l2).unwrap().borrow().val);
}
