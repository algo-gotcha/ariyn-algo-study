use std::ptr::NonNull;

struct Node<T> {
    value: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(v:T) -> Node<T> {
        Node {value:v, next:None}
    }
}

struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    // tail: Option<Node>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList{head: None}
    }

    fn push(&mut self, v: T) {
        self.push_node(Box::new(Node::new(v)))
    }

    fn push_node(&mut self, mut node: Box<Node<T>>) {
        node.next = self.head;
        self.head = Some(Box::leak(node).into());
    }

    fn pop(&mut self) -> Option<T> {
        self.pop_node().map(|n| n.value)
    }

    fn pop_node(&mut self) -> Option<Box<Node<T>>> {
        self.head.map(|n| unsafe {
            let n = Box::from_raw(n.as_ptr());
            self.head = n.next;

            n
        })
    }
}

fn main() {
    let l: &mut LinkedList<i32> = &mut LinkedList::new();
    l.push(1);
    l.push(2);
    l.push(3);
    l.push(4);
    l.push(5);


    loop {
        let element = l.pop();
        if element.is_none() {
            break;
        }

        println!("{}", element.unwrap());
    }
}