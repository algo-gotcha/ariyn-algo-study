#[cfg(test)]
mod tests {
    use crate::linked_list::{LinkedList, Node};
    use std::ptr::NonNull;

    #[test]
    /// i32값을 넣었을때, 입력된 값이 저장되는가?
    fn push_i32() {
        let mut ll: LinkedList<i32> = LinkedList::new();

        ll.push(i32::MAX);

        unsafe{ assert_eq!(i32::MAX, (*ll.head.unwrap().as_ptr()).value) }

        let mut ll: LinkedList<i32> = LinkedList::new();

        ll.push(i32::MIN);

        unsafe{ assert_eq!(i32::MIN, (*ll.head.unwrap().as_ptr()).value)}
    }

    #[test]
    /// i32값이 저장되어 있을때, 저장된 값이 pop 되는가?
    fn pop_i32() {
        let expect = i32::MAX;
        let mut ll: LinkedList<i32> = LinkedList::new();
        ll.head = Some(NonNull::from(Box::leak(Box::from(Node::new(expect)))));

        let actual = ll.pop();

        assert_eq!(expect, actual.unwrap());

        let expect = i32::MIN;
        let mut ll: LinkedList<i32> = LinkedList::new();
        ll.head = Some(NonNull::from(Box::leak(Box::from(Node::new(expect)))));

        let actual = ll.pop();

        assert_eq!(expect, actual.unwrap());
    }

    #[test]
    /// heap에 저장되는 대표적인 String값을 넣었을 때, 해당 값이 저장되는가?
    fn push_string() {
        let expect = "Hello, World!!";
        let mut ll:LinkedList<String> = LinkedList::new();

        ll.push(String::from(expect));

        unsafe{assert_eq!(expect, (*ll.head.unwrap().as_ptr()).value)}
    }

    #[test]
    /// heap에 저장되는 대표적인 String값이 들어 있을때, 해당 값이 pop되는가?
    fn pop_string() {
        let expect = "Hello, World!!";
        let mut ll:LinkedList<String> = LinkedList::new();
        ll.head = Some(NonNull::from(Box::leak(Box::from(Node::new(String::from(expect))))));


        let actual = ll.pop();

        assert_eq!(expect, actual.unwrap())
    }

    #[test]
    /// LinkedList에 아무것도 들어있지 않을때, pop을 하면 None이 리턴되는가?
    fn pop_when_empty() {
        let mut ll: LinkedList<i32> = LinkedList::new();

        let actual = ll.pop();

        assert_eq!(None, actual);

        assert_eq!(None, ll.head);
        assert_eq!(None, ll.tail);
    }

    #[test]
    /// 비어있는 LinkedList에 push를 하면, head와 tail이 해당 node를 가리키는가?
    fn push_when_empty_head_and_tail_points_well() {
        let mut ll: LinkedList<i32> = LinkedList::new();

        ll.push(1);

        assert_eq!(ll.head, ll.tail);
    }

    #[test]
    /// LinkedList에 push를 두번 하면, 새로운 노드가 head가 되고 기존 head가 tail이 되는가?
    fn push_when_list_pushed_twice_head_and_tail_points_well() {
        let first_pushed_value = 1;
        let later_pushed_value = 2;

        let mut ll: LinkedList<i32> = LinkedList::new();

        ll.push(first_pushed_value);
        ll.push(later_pushed_value);

        assert_ne!(ll.head, ll.tail);
        unsafe {
            assert_eq!(ll.tail, (*ll.head.unwrap().as_ptr()).next);

            assert_eq!(later_pushed_value, (*ll.head.unwrap().as_ptr()).value);
            assert_eq!(first_pushed_value, (*ll.tail.unwrap().as_ptr()).value);
        }
    }

    #[test]
    /// LinkedList에 push를 세 번 하면, 새로운 노드가 head가 되고 tail이 유지되는가?
    fn push_when_list_pushed_three_times_head_and_tail_points_well() {
        let first_pushed_value = 1;
        let second_pushed_value = 2;
        let third_pushed_value = 3;

        let mut ll: LinkedList<i32> = LinkedList::new();

        ll.push(first_pushed_value);
        ll.push(second_pushed_value);
        ll.push(third_pushed_value);

        assert_ne!(ll.head, ll.tail);
        unsafe {
            assert_eq!(ll.tail, (*(*ll.head.unwrap().as_ptr()).next.unwrap().as_ptr()).next);

            assert_eq!(third_pushed_value, (*ll.head.unwrap().as_ptr()).value);
            assert_eq!(second_pushed_value, (*(*ll.head.unwrap().as_ptr()).next.unwrap().as_ptr()).value);
            assert_eq!(first_pushed_value, (*ll.tail.unwrap().as_ptr()).value);
        }
    }

    #[test]
    /// LinkedList에 pop을 두번 할 경우, 노드의 순서가 stack의 동작과 같은가?
    fn pop_twice_outputs_order() {
        let mut ll: LinkedList<i32> = LinkedList::new();
        ll.push(1);
        ll.push(2);

        assert_eq!(2, ll.pop().unwrap());
        assert_eq!(1, ll.pop().unwrap());
    }

    #[test]
    /// node_at을 호출했을때, 데이터가 소유권에 따라 삭제되지 않고 남아 있는가?
    fn nodes_remains_after_node_at() {
        let mut ll: LinkedList<i32> = LinkedList::new();
        ll.push(1); // index 0 : 1
        ll.push(2); // index 0 : 2, 1 : 1
        ll.push(3); // index 0 : 3, 1 : 2, 2 : 1
        ll.push(4); // index 0 : 4, 1 : 3, 2 : 2, 3 : 1

        let index_3_node = ll.node_at(3);

        assert_eq!(1, index_3_node.unwrap().value);
        assert_eq!(4, ll.pop().unwrap());
        assert_eq!(3, ll.pop().unwrap());
        assert_eq!(2, ll.pop().unwrap());
        assert_eq!(1, ll.pop().unwrap());
    }

    #[test]
    /// 두개 이상의 값이 들어 있을때, push_at을 사용했을때 새로운 노드는 두 노드 사이에 추가되는가?
    fn push_at_when_two_nodes_exists() {
        let mut ll: LinkedList<i32> = LinkedList::new();
        ll.push(1);
        ll.push(3);

        unsafe { ll.push_at(1, 2); } // 0번째 인덱스 뒤에 2를 가진 노드를 추가한다.

        assert_eq!(3, ll.pop().unwrap());
        assert_eq!(2, ll.pop().unwrap());
        assert_eq!(1, ll.pop().unwrap());
    }
}

use std::ptr::NonNull;

struct Node<T: Clone> {
    value: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T: Clone> Node<T> {
    fn new(v:T) -> Node<T> {
        Node {value:v, next:None, prev:None}
    }
}

impl<T: Clone> Clone for Node<T> {
    #[warn(unused_variables)]
    fn clone(&self) -> Self {
        Node {
            value: self.value.clone(),
            next: self.next,
            prev: self.prev,
        }
    }

    #[warn(unused_variables)]
    fn clone_from(&mut self, _source: &Self) {
        unimplemented!()
    }
}

pub struct LinkedList<T: Clone> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
}

impl<T: Clone> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList{head: None, tail: None}
    }

    pub fn push(&mut self, v: T) {
        self.push_node(Box::new(Node::new(v)))
    }

    fn push_node(&mut self, mut node: Box<Node<T>>) {
        node.next = self.head;

        if self.head.is_some() {
            unsafe{
                (*self.head.unwrap().as_ptr()).prev = Some(Box::leak(node).into());
                self.head = (*self.head.unwrap().as_ptr()).prev;
            }
        } else {
            self.head = Some(Box::leak(node).into());
        }

        if self.tail == None {
            self.tail = self.head;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.pop_node().map(|n| n.value)
    }

    fn pop_node(&mut self) -> Option<Box<Node<T>>> {
        self.head.map(|n| unsafe {
            let n = Box::from_raw(n.as_ptr());
            self.head = n.next;

            match self.head {
                Some(n) => {
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

    fn node_at (&mut self, index:i32) -> Option<Node<T>> { // TODO: raise error when index is bigger then linked list size
        let mut x = self.head.clone();

        let mut current_index = 0;
        loop {
            if current_index == index {
                break
            }

            unsafe{
                if (*x.unwrap().as_ptr()).next.is_none() {
                    break;
                }
            }

            x = x.map(|n| unsafe{n.as_ref().next}).unwrap();
            current_index = current_index+1;
        }

        unsafe {
            Some((*x.unwrap().as_ptr()).clone())
        }
    }

    /// push_at will works like push. which means it will store data in front of nth node, just like push puts data into front.
    unsafe fn push_at(&mut self, index:i32, value:T){
        let target = self.node_at(index);
        let node = Node::new(value);

        match target {
            Some(mut target) => {
                let prev = target.prev;

                if prev.is_some() {
                    (*prev.unwrap().as_ptr()).next = Some(Box::leak(Box::new(node)).into());

                    let next = (*prev.unwrap().as_ptr()).next;
                    target.prev = next;
                    (*next.unwrap().as_ptr()).next = Some(Box::leak(Box::new(target)).into());
                } else { // target is head
                    target.prev = Some(Box::leak(Box::new(node)).into());

                    let prev = target.prev;

                    (*prev.unwrap().as_ptr()).next = Some(Box::leak(Box::new(target)).into());
                    self.head = prev;
                }
            }
            None => {unimplemented!()}
        }
    }
}
