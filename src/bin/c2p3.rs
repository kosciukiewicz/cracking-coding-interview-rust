use coding_interview_rust::utils::linked_list::LinkedList;
use std::fmt::Display;
use std::hash::Hash;

trait DeleteNode<T> {
    fn delete_node_1(&mut self, index: usize);
    fn delete_node_2(&mut self, index: usize);
}

impl<T> DeleteNode<T> for LinkedList<T>
where
    T: Clone + Hash + Eq + Display,
{
    fn delete_node_1(&mut self, index: usize) {
        for (i, node_ref) in self.ref_iter().enumerate() {
            let current_node = node_ref.borrow();
            if i == index {
                if let Some(prev) = current_node.prev.as_ref() {
                    if let Some(next) = current_node.next.as_ref() {
                        prev.upgrade().unwrap().borrow_mut().next = Some(next.clone());
                        next.borrow_mut().prev = Some(prev.clone());
                    } else {
                        prev.upgrade().unwrap().borrow_mut().next = None;
                    }
                }
            }
        }
    }

    fn delete_node_2(&mut self, index: usize) {
        for (i, node_ref) in self.ref_iter().enumerate() {
            let mut current_node = node_ref.borrow_mut();
            if i == index {
                if let Some(next_node) = current_node.next.as_ref().cloned() {
                    let next_node_ref = next_node.borrow();
                    current_node.val = next_node_ref.val.clone();
                    current_node.next = next_node_ref.next.clone();
                    if let Some(new_next_ref) = next_node_ref.next.as_ref() {
                        new_next_ref.borrow_mut().prev =
                            Some(next_node_ref.prev.as_ref().unwrap().clone());
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::DeleteNode;
    use coding_interview_rust::utils::linked_list::LinkedList;
    use rstest::rstest;

    #[rstest]
    #[case(vec ! [1, 1, 2, 3], 2, vec ! [1, 1, 3])]
    #[case(vec ! [1, 1, 2, 3], 1, vec ! [1, 2, 3])]
    fn test_delete_node_1(
        #[case] input_vec: Vec<i32>,
        #[case] index: usize,
        #[case] expected: Vec<i32>,
    ) {
        let mut linked_list = LinkedList::from_vec(input_vec);
        linked_list.delete_node_1(index);

        assert_eq!(linked_list.to_vec(), expected);
    }

    #[rstest]
    #[case(vec ! [1, 1, 2, 3], 2, vec ! [1, 1, 3])]
    #[case(vec ! [1, 1, 2, 3], 1, vec ! [1, 2, 3])]
    fn test_delete_node_2(
        #[case] input_vec: Vec<i32>,
        #[case] index: usize,
        #[case] expected: Vec<i32>,
    ) {
        let mut linked_list = LinkedList::from_vec(input_vec);
        linked_list.delete_node_2(index);

        assert_eq!(linked_list.to_vec(), expected);
    }
}

fn main() {
    let v = vec![1, 1, 2, 3];
    let mut linked_list = LinkedList::from_vec(v);

    println!("{:?}", linked_list.delete_node_1(2));
    println!("{:?}", linked_list.delete_node_2(2));
}
