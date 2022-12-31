use std::cell::RefCell;
use std::fmt::Display;
use std::rc::{Rc, Weak};

#[derive(Clone)]
pub struct Node<T> {
    pub val: T,
    pub next: Option<Rc<RefCell<Self>>>,
    pub prev: Option<Weak<RefCell<Self>>>,
}

pub struct LinkedList<T> {
    pub head: Option<Rc<RefCell<Node<T>>>>,
    pub tail: Option<Rc<RefCell<Node<T>>>>,
    pub length: usize,
}

impl<T> Default for LinkedList<T>
where
    T: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn push_start(&mut self, v: T) {
        let node = Node {
            val: v,
            next: self.head.as_ref().cloned(),
            prev: None,
        };
        let node_ref = Rc::new(RefCell::new(node));
        if let Some(head) = self.head.as_mut() {
            head.as_ref().borrow_mut().prev = Some(Rc::downgrade(&node_ref));
        } else {
            self.tail = Some(node_ref.clone());
        }

        self.head = Some(node_ref);
        self.length += 1
    }

    pub fn push_back(&mut self, v: T) {
        let node = Node {
            val: v,
            next: None,
            prev: self.tail.as_ref().map(Rc::downgrade),
        };
        let node_ref = Rc::new(RefCell::new(node));
        if let Some(tail) = self.tail.as_mut() {
            tail.as_ref().borrow_mut().next = Some(node_ref.clone());
        } else {
            self.head = Some(node_ref.clone());
        }

        self.tail = Some(node_ref);
        self.length += 1
    }

    pub fn from_vec(v: Vec<T>) -> Self {
        let mut list = LinkedList::new();
        for val in v.iter() {
            list.push_back(val.clone())
        }

        list
    }

    pub fn to_vec(&self) -> Vec<T> {
        let mut result: Vec<T> = Vec::new();
        for node in self.ref_iter() {
            result.push(node.borrow().val.clone());
        }
        result
    }

    pub fn ref_iter(&self) -> RefIter<T> {
        RefIter {
            next: self.head.clone(),
        }
    }
}

pub struct RefIter<T> {
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Iterator for RefIter<T> {
    type Item = Rc<RefCell<Node<T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur) = self.next.as_ref().cloned() {
            self.next = (*cur).borrow().next.clone();
            return Some(cur);
        }
        None
    }
}

impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, w: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(w, "[")?;
        let mut node = self.head.clone();
        while let Some(n) = node {
            write!(w, "{}", n.borrow().val).unwrap();
            node = n.borrow().next.clone();
            if node.is_some() {
                write!(w, ", ").unwrap();
            }
        }
        write!(w, "]")
    }
}
