use std::{cell::RefCell, rc::Rc};

struct ListNode<T> {
    item: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> ListNode<T> {
    fn new(item: T) -> Self {
        Self {
            item,
            next: None,
            prev: None,
        }
    }
}

type Link<T> = Option<Rc<RefCell<ListNode<T>>>>;

#[derive(Default)]
pub struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    size: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn clear(&mut self)-> Option<T> {
        while self.size!=0 {
            self.head.take().map(|prev_head| {
                self.size -= 1;
                match prev_head.borrow_mut().next.take() {
                    Some(node) => {
                        node.borrow_mut().prev = None;
                        self.head = Some(node);
                        println!("Eliminando...");
                    }
                    None => {
                        self.tail.take();
                        println!("Eliminando...");
                    }
                }
                Rc::try_unwrap(prev_head).ok().unwrap().into_inner().item
            });   
        }
            ;None
    }

}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        while let Some(node) = self.head.take() {
            let _ = node.borrow_mut().prev.take();
            self.head = node.borrow_mut().next.take();
            println!("DESTROYED\n");
        }
        self.tail.take();
    }
}


fn main(){
    //let mut list = DoublyLinkedList::new();
    let mut list = DoublyLinkedList::new();
    for i in 0..3 {
        list.push_back(i);
    }
    //Borrar todos
    list.clear();
    println!("{:?}",list.len());

}