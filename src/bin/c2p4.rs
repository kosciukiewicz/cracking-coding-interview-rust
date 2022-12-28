use coding_interview_rust::utils::linked_list::LinkedList;
use std::fmt::Display;
use std::hash::Hash;
use std::rc::Rc;

trait PartitionList<T> {
    fn partition_list(&mut self, x: T) -> LinkedList<T>;
}

impl<T> PartitionList<T> for LinkedList<T>
where
    T: Clone + Hash + Eq + Display + PartialOrd,
{
    fn partition_list(&mut self, x: T) -> LinkedList<T> {
        let mut list_1 = LinkedList::new();
        let mut list_2 = LinkedList::new();

        for node_ref in self.ref_iter() {
            let node = node_ref.borrow();
            if node.val < x {
                list_1.push_back(node.val.clone());
            } else {
                list_2.push_back(node.val.clone());
            }
        }

        {
            let tail_1 = list_1.tail.as_ref().unwrap();
            let mut tail_1_ref = tail_1.borrow_mut();
            let head_2 = list_2.head.as_ref().unwrap();
            let mut head_2_ref = head_2.borrow_mut();

            tail_1_ref.next = list_2.head.as_ref().cloned();
            head_2_ref.prev = list_1.tail.as_ref().map(Rc::downgrade);
        }

        list_1
    }
}

#[cfg(test)]
mod tests {
    use super::PartitionList;
    use coding_interview_rust::utils::linked_list::LinkedList;
    use rstest::rstest;

    #[rstest]
    #[case(vec ! [1, 2, 1, 3], 2, vec ! [1, 1, 2, 3])]
    #[case(vec ! [1, 2, 1, 2, 3], 2, vec ! [1, 1, 2, 2, 3])]
    fn partition_list(#[case] input_vec: Vec<i32>, #[case] x: i32, #[case] expected: Vec<i32>) {
        let mut linked_list = LinkedList::from_vec(input_vec);

        assert_eq!(linked_list.partition_list(x).to_vec(), expected);
    }
}

fn main() {
    let v = vec![1, 2, 1, 3];
    let mut linked_list = LinkedList::from_vec(v);

    println!("{:?}", linked_list.partition_list(2).to_vec());
}
