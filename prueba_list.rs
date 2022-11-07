use std::cell::{RefCell};
use std::rc::Rc;

struct Node {
    value: String,
    next: Link,
    prev: Link,
}

type Link = Option<Rc<RefCell<Node>>>;

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
            prev: None,
        }))
    }
}

pub struct DoublyLinkedList {
    head: Link,
    tail: Link,
    pub length: u64,
}

impl DoublyLinkedList {
    pub fn new() -> DoublyLinkedList {
        DoublyLinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn push(&mut self, value: String) {
        let new = Node::new(value);
        match self.tail.take() {
            Some(old) => {
                old.borrow_mut().next = Some(new.clone());
                new.borrow_mut().prev = Some(old);
            }
            None => self.head = Some(new.clone()),
        };
        self.length += 1;
        self.tail = Some(new);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                next.borrow_mut().prev = None;
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Something is terribly wrong")
                .into_inner()
                .value
        })
    }
   
}

fn main() {
    let mut link_list = DoublyLinkedList::new();
    println!("{}", link_list.length);

    link_list.push("A".to_string());
    link_list.push("B".to_string());
    link_list.push("C".to_string());

    println!("{}", link_list.length);
    link_list.pop();
    println!("{}", link_list.length);
}
