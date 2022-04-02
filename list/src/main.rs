use std::ptr::NonNull;

struct Node<T> {
    value: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(v:T) -> Node<T> {
        Node {value:v, next:None, prev:None}
    }
}

struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList{head: None, tail: None}
    }

    fn push(&mut self, v: T) {
        self.push_node(Box::new(Node::new(v)))
    }

    fn push_node(&mut self, mut node: Box<Node<T>>) {
        node.next = self.head;
        self.head = Some(Box::leak(node).into());

        match self.head {
          Some(n) => unsafe{
            (*n.as_ptr()).prev = self.head;
          }
          _ => {}
        };


        if self.tail == None {
            self.tail = self.head;
        }
    }

    fn pop(&mut self) -> Option<T> {
        self.pop_node().map(|n| n.value)
    }

    fn pop_node(&mut self) -> Option<Box<Node<T>>> {
        self.head.map(|n| unsafe {
            let n = Box::from_raw(n.as_ptr());
            self.head = n.next;

            match self.head {
                Some(n) => unsafe {
                    (*n.as_ptr()).prev = None;
                }
                _ => {}
            }

            if n.next == None {
                self.tail = None;
            }

            n
        })
    }

    fn node_at(&mut self, index:i32) -> Option<NonNull<Node<T>>> {
        let mut x = self.head.clone();

        let mut current_index = 0;
        loop {
            unsafe{
                if (*x.unwrap().as_ptr()).next.is_none() {
                    break;
                }
            }

            x = x.map(|n| unsafe{n.as_ref().next}).unwrap();
            current_index = current_index+1;

            if current_index == index {
                break
            }
        }

        x
    }
}

fn main() {
    let l: &mut LinkedList<i32> = &mut LinkedList::new();
    l.push(1);
    l.push(2);
    l.push(3);
    l.push(4);
    l.push(5);

    let node = l.node_at(3);
    unsafe{ println!("popped index {} is {}", 3, (*node.as_ref().unwrap().as_ptr()).value); }

    loop {
        let element = l.pop();
        if element.is_none() {
            break;
        }

        println!("{}", element.unwrap());
    }
}