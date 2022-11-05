//Modificaciones uriel
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

    pub fn find(item: T) -> usize {
        let mut cont = 0; //counter for the interation
        let encontrado = false;
        while cont < size_of(){
            if encontrado == true{
                return cont;
            }
            if()

        }

        return 
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

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|prev_head| {
            self.size -= 1;
            match prev_head.borrow_mut().next.take() {
                Some(node) => {
                    node.borrow_mut().prev = None;
                    self.head = Some(node);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(prev_head).ok().unwrap().into_inner().item
        })
    }

        // erase: elimina el nodo de la posición indicada por el
        // usuario, regresa un iterador nodo que sigue después del
        // nodo eliminado.
    
        // find: encuentra un objeto en la lista, regresa un iterador
        // al elemento buscado si el elemento se encuentra se
        // encuentra, de lo contrario se regresa None.
    
        pub fn push_back(&mut self, item: T) {
            let node = Rc::new(RefCell::new(ListNode::new(item)));
            if let Some(prev_tail) = self.tail.take() {
                prev_tail.borrow_mut().next = Some(Rc::clone(&node));
                node.borrow_mut().prev = Some(prev_tail);
                self.tail = Some(node);
                self.size += 1;
            } else {
                self.head = Some(Rc::clone(&node));
                self.tail = Some(node);
                self.size = 1;
            }
        }
    // insert: inserta un nodo en la posición indicada por el
    // usuario, regresa un iterador al nodo insertado.

        pub fn insert(&mut self, item: T, pos: u32){
            let node = Rc::new(RefCell::new(ListNode::new(item)));

            if self.size == 0 {
                self.head = Some(Rc::clone(&node));
                self.tail = Some(node);
                self.size = 1;
                println!("No Existia aun ningun node --Se creo uno\n");
            }else if self.size == 1{
                self.push_front(item);
            }else if self.size == 2{
                

            }

            
        
        }


        pub fn push_front(&mut self, item: T) {
            let node = Rc::new(RefCell::new(ListNode::new(item)));
            if let Some(prev_head) = self.head.take() {
                prev_head.borrow_mut().prev = Some(Rc::clone(&node));
                node.borrow_mut().next = Some(prev_head);
                self.head = Some(node);
                self.size += 1;
            } else {
                self.head = Some(Rc::clone(&node));
                self.tail = Some(node);
                self.size = 1;
            }
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
    //pop back
    list.pop_back();

    //Borrar todos
    list.clear();
    println!("{:?}",list.len());

    // drop(list);
}