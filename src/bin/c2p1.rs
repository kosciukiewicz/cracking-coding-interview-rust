use coding_interview_rust::utils::linked_list::LinkedList;
use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hash;

trait RemoveDuplicates {
    fn remove_duplicates(&mut self);
}

impl<T> RemoveDuplicates for LinkedList<T>
where
    T: Clone + Hash + Eq + Display,
{
    fn remove_duplicates(&mut self) {
        let mut buf: HashSet<T> = HashSet::new();
        if self.length == 0 {
            return;
        }

        for node_ref in self.ref_iter() {
            let current_node = node_ref.borrow();
            let val = current_node.val.clone();

            if buf.contains(&val) {
                if let Some(prev) = current_node.prev.as_ref() {
                    if let Some(next) = current_node.next.as_ref() {
                        prev.upgrade().unwrap().borrow_mut().next = Some(next.clone());
                        next.borrow_mut().prev = Some(prev.clone());
                    } else {
                        prev.upgrade().unwrap().borrow_mut().next = None;
                    }
                }
            } else {
                buf.insert(val);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::RemoveDuplicates;
    use coding_interview_rust::utils::linked_list::LinkedList;
    use rstest::rstest;

    #[rstest]
    #[case(vec ! [1, 1, 2, 3], vec ! [1, 2, 3])]
    #[case(vec ! [1, 1, 2, 3, 44, 44, 55, 55], vec ! [1, 2, 3, 44, 55])]
    #[case(vec ! [1, 1, 1, 1], vec ! [1])]
    #[case(vec ! [], vec ! [])]
    fn test_remove_duplicates(#[case] input_vec: Vec<i32>, #[case] expected: Vec<i32>) {
        let mut linked_list = LinkedList::from_vec(input_vec);
        linked_list.remove_duplicates();

        assert_eq!(linked_list.to_vec(), expected);
    }
}

fn main() {
    let v = vec![1, 1, 2, 3];
    let mut linked_list = LinkedList::from_vec(v);
    linked_list.remove_duplicates();

    println!("{:?}", linked_list.to_vec());
}
