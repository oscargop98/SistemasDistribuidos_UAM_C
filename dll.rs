
use std::{cell::RefCell, rc::Rc, fmt::Debug};
// use std::ops::Deref;

#[derive(Debug)]
#[derive(PartialEq)]
struct ListNode<T> {
    item: T,
    next: Link<T>,
    prev: Link<T>,
}


impl<T> ListNode<T> where T: Debug + Ord {
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

impl<T> DoublyLinkedList<T> where T: Debug + Ord {
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

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|prev_tail| {
            self.size -= 1;
            match prev_tail.borrow_mut().prev.take() {
                Some(node) => {
                    node.borrow_mut().next = None;
                    self.tail = Some(node);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(prev_tail).ok().unwrap().into_inner().item
        })
    }

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
    // Sin terminar
    pub fn insertar(&mut self,item: T, _pos: usize) {
        //Si no hay ni un nodo
        if self.size==0{
            let node = Rc::new(RefCell::new(ListNode::new(item)));
            self.head = Some(Rc::clone(&node));
            self.tail = Some(node);
            self.size = 1;
        }
        //Si ya hay un nodo en la lista
        else {
            //Posicion invalida
            if _pos < 1{
                println!("La posicion debe de ser >=1");
            }
            //Push front
            else if _pos==1{
                self.push_front(item);
                println!("Imsertar - push front")
            }
            //push back
            else if _pos==self.size{
                self.push_back(item);
                println!("Imsertar - push back")
            }
            //Inserta en otra posicion
            else {
                let mut _node = Rc::new(RefCell::new(ListNode::new(item)));
                println!("Nodo creado en un pos distinta de head y tail");
                // let nulo:Option<Rc<RefCell<ListNode<T>>>>;
                let tope= _pos - 1;
                println!("El tope  de la pos es : {:?}",tope);
                // Esto let Some(prev_posicion) = self.head.take() debe de estar fuera del if
                // let temporal  = Some(prev_posicion) = self.head.take();
                //Node * temp = head;
                if let Some(prev_posicion) = self.head.take() {

                    // println!("El temporal es: {:?}",prev_posicion.borrow_mut().item);

                    println!("valor de prev_posicion: {:?}",prev_posicion.borrow_mut().item);
                    // println!("El apuntado es nulo: {:?}", prev_posicion.as_ptr().is_null());

                    //======recorrer=====
                    for _i in 1..tope {
                        //Recorrer el apuntador
                        // println!("valor de prev_posicion for: {:?}",prev_posicion.borrow_mut().item);
                        // println!("Entro al for, iteracion: {:?}",_i);
                        //  if(temp != NULL)
                        if prev_posicion.as_ptr().is_null() == false {
                            
                            // TODO: implementar : temp = temp->next, Hace el recorrido;
                            // prev_posicion = Rc::clone(&_node);

                            // prev_posicion = prev_posicion.borrow_mut().next;
                            
                            // prev_posicion = prev_posicion.node.borrow_mut().prev 
                            

                            // prev_posicion = prev_posicion.borrow_mut().next;
                            // prev_posicion= Rc::clone(&node);
                            // println!("valor de prev_posicion...: {:?}",prev_posicion.borrow_mut().next);

                            
                            // prev_posicion = prev_posicion.;
                            // println!("El puntero sigue siendo No nulo");
                            // println!("Prev_posicion next es: {:?}",prev_posicion.borrow_mut().next);
                        }
                    }

                    // if prev_posicion.as_ptr().is_null() != nulo.is_none() {
                    //     _node.borrow_mut().next = prev_posicion.borrow_mut().next;
                    //     _node.borrow_mut().prev = Some(prev_posicion);
                    //     prev_posicion.borrow_mut().next = Some(_node);

                    //     if _node.borrow_mut().next != nulo {
                    //         _node.borrow_mut().next.prev = _node;
                    //     }

                    //     else {
                    //         //Cuando el nodo previo es null
                    //         println!("El nodo previo es null");
                    //     }
                    // }
                }
                // Si el nodo anterior no es nulo, ajuste los enlaces





            }
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
    for i in 1..8 {
        list.push_back(i);
    }
    // //pop back
    // list.pop_back();
    println!("tamanio de la lista: {:?}",list.len());
    list.insertar(10, 1);
    // println!("=========== insercciion 2 =======");
    // list.insertar(18, 6);
    //Borrar todos
    list.clear();
    println!("Tamanio de la lista: {:?}",list.len());

    // drop(list);
}