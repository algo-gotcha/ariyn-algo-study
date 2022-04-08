use std::borrow::{Borrow, BorrowMut};
use std::cell::{Ref, RefCell};
use std::cmp::Ordering;
use std::collections::LinkedList;
use std::convert::Infallible;
use std::ops::Index;
use std::rc::Rc;

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use crate::node::IterationType::{Postorder, Preorder};
    use crate::node::{Node, Tree};

    #[test]
    /// new 함수를 호출할 경우 초기화된 node를 생성
    fn create_node() {
        const EXPECT: i32 = 1;
        let mut node = Node::new(EXPECT);

        assert_eq!(0, node.size());
        assert_eq!(EXPECT, node.value);
        assert_eq!(true, node.parent.is_none());
    }

    #[test]
    /// add_child함수를 호출할 경우, parent와 children의 연결이 성립되고 size를 1 증가
    fn add_child() {
        const PARENT_VALUE:i32 = 1;
        const CHILD_VALUE:i32 = 2;
        let mut node = Node::new(PARENT_VALUE);
        let child = Node::new(CHILD_VALUE);

        node.add_child(child);

        let child = node.get_child(0).unwrap();
        assert_eq!(CHILD_VALUE, (*child).borrow_mut().value);

        let value = (*child).borrow_mut().parent.as_ref().unwrap().borrow_mut().value;
        assert_eq!(node.value, value);
        assert_eq!(1, node.size());
    }

    #[test]
    /// get_child 함수를 여러번 호출해도 노드를 삭제하지 않음
    fn get_child_multiple_times() {
        let mut node = Node::new(1);
        node.add_child(Node::new(2));

        assert_eq!(true, node.get_child(0).is_some());
        assert_eq!(true, node.get_child(0).is_some());
    }

    #[test]
    /// node의 children에 있는 인덱스로 삭제할 경우, Option<Node>를 반환하며 size가 1 줄어듬
    fn delete_child() {
        const EXPECT_VALUE:i32 = 2;
        let mut node = Node::new(1);
        node.add_child(Node::new(EXPECT_VALUE));

        assert_eq!(1, node.size());

        let actual = node.delete_child(0);

        assert_eq!(EXPECT_VALUE, (*actual.unwrap()).borrow_mut().value);
        assert_eq!(0, node.size());
    }

    #[test]
    /// node의 size를 벗어나는 index로 삭제할 경우, None을 반환하며 size를 변경하지 않음
    fn delete_child_index_bigger_then_size() {
        let mut node = Node::new(1);
        node.add_child(Node::new(2));

        assert_eq!(1, node.size());

        let actual = node.delete_child(1);
        assert_eq!(true, actual.is_none());
        assert_eq!(1, node.size);
    }

    #[test]
    /// iter 함수를 Preorder 파라미터(=IterationType::Preorder)로 호출할 경우, preorder (NLR) 알맞은 순서로 정렬된 iter 타입을 반환
    /// 이경우 순서는 1,2,3,4,5,6,7,10,8,9 가 된다.
    fn iter_with_dfs() {
        let mut node = init_basic_tree();
        let mut actual:Vec<i32> = Vec::new();

        let mut iterator = node.iter(Preorder);
        while let Some(next) = iterator.next() {
            actual.push(next.borrow_mut().value);
        }

        let expect: &[i32] = &[1,2,3,4,5,6,7,10,8,9];
        assert_eq!(Vec::from(expect), actual);
    }

    #[test]
    /// iter 함수를 Postorder 파라미터(=IterationType::Postorder)로 호출할 경우, postorder (LRN) 알맞은 순서로 정렬된 iter 타입을 반환
    /// 이경우 순서는 3,4,5,2,10,7,8,9,6,1 가 된다.
    fn iter_with_bfs() {
        let mut node = init_basic_tree();
        let mut actual:Vec<i32> = Vec::new();

        let mut iterator = node.iter(Postorder);
        while let Some(next) = iterator.next() {
            actual.push(next.borrow_mut().value);
        }

        let expect: &[i32] = &[3, 4, 5, 2, 10, 7, 8, 9, 6, 1];
        assert_eq!(Vec::from(expect), actual);
    }

    #[test]
    fn add_node_during_iter() {
        let mut node = init_basic_tree();

        let mut iterator = node.iter(Preorder);
        while let Some(next) = iterator.next() {
            if next.borrow_mut().value == 3 {
                next.borrow_mut().add_child(Node::new(15));
            }
        }

        let mut actual:Vec<i32> = Vec::new();

        let mut iterator = node.iter(Preorder);
        while let Some(next) = iterator.next() {
            actual.push(next.borrow_mut().value);
        }

        let expect: &[i32] = &[1,2,3,15,4,5,6,7,10,8,9];
        assert_eq!(Vec::from(expect), actual);

    }

    /// tree struct
    ///      1
    ///    /   \
    ///   2     6
    /// / |\   /|\
    /// 3 4 5 7 8 9
    ///       |
    ///      10
    ///
    /// preorder(NLR) : 1, 2, 3, 4, 5, 6, 7, 10, 8, 9
    /// postorder(LRN): 3, 4, 5, 2, 10, 7, 8, 9, 6, 1
    /// inorder (LNR) : undefined. there is 3 childre for some nodes.
    fn init_basic_tree() -> Node {
        let mut node = Node::new(1);

        let mut c1 = Node::new(2);
        c1.add_child(Node::new(3));
        c1.add_child(Node::new(4));
        c1.add_child(Node::new(5));

        node.add_child(c1);

        let mut c2 = Node::new(6);

        let mut c3 = Node::new(7);
        c3.add_child(Node::new(10));

        c2.add_child(c3);

        c2.add_child(Node::new(8));
        c2.add_child(Node::new(9));

        node.add_child(c2);

        node
    }

    #[test]
    fn add_to_tree() {
        let mut tree = Tree::new();

        tree.add(1);

        assert_eq!(true, tree.root.is_some());
        assert_eq!(1, (*tree.root.unwrap().clone()).borrow().value);
    }

    #[test]
    ///      1
    ///    /  \
    ///   2    3
    ///  / \  / \
    /// 4  5 6   7
    fn add_to_tree_with_complete_tree_style() {
        let mut tree = Tree::new();

        tree.add(1);
        tree.add(2);
        tree.add(3);
        tree.add(4);
        tree.add(5);
        tree.add(6);
        tree.add(7);

        let mut actual:Vec<i32> = Vec::new();

        let mut iterator = (*tree.root.unwrap().clone()).borrow_mut().iter(Preorder);

        while let Some(next) = iterator.next() {
            actual.push(next.borrow_mut().value);
        }

        let expect: &[i32] = &[1,2,4,5,3,6,7];
        assert_eq!(Vec::from(expect), actual);
    }

    #[test]
    /// delete 2
    ///      1      ->         1
    ///    /  \              /  \
    ///   2    3            7    3
    ///  / \  / \          / \  /
    /// 4  5 6   7        4  5 6
    fn delete_from_tree_with_complete_tree_style() {
        let mut tree = Tree::new();

        tree.add(1);
        tree.add(2);
        tree.add(3);
        tree.add(4);
        tree.add(5);
        tree.add(6);
        tree.add(7);

        let is_deleted = tree.delete(2);

        assert_eq!(true, is_deleted);

        let mut actual:Vec<i32> = Vec::new();

        let mut iterator = (*tree.root.unwrap().clone()).borrow_mut().iter(Preorder);
        while let Some(next) = iterator.next() {
            actual.push(next.borrow_mut().value);
        }

        let expect: &[i32] = &[1,7,4,5,3,6];
        assert_eq!(Vec::from(expect), actual);
    }
}

pub struct Node {
    pub value: i32,
    pub size: i32,
    pub parent: Option<Rc<RefCell<Node>>>,
    pub children: Vec<Rc<RefCell<Node>>>,
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Node {
            value: self.value,
            size: self.size,
            parent: self.parent.clone(),
            children: self.children.clone(),
        }
    }

    fn clone_from(&mut self, _: &Self) {
        todo!()
    }
}

impl Node {
    fn new(value: i32) -> Node {
        Node {
            value,
            size:0,
            parent: None,
            children: Vec::new(),
        }
    }

    fn size(&mut self) -> i32 {
        self.size
    }

    fn add_child(&mut self, mut child: Node) {
        child.parent = Some(Rc::new(RefCell::new(self.clone())));
        self.children.push(Rc::new(RefCell::new(child)));
        self.size = self.size + 1;
    }

    fn get_child(&mut self, index: i32) -> Option<&Rc<RefCell<Node>>> {
        self.children.get(index as usize)
    }

    fn delete_child(&mut self, index: i32) -> Option<Rc<RefCell<Node>>> {
        if self.size <= index {
            return None;
        }

        let deleted = self.children.remove(index as usize);
        self.size = self.size - 1;
        Some(deleted)
    }

    fn delete_child_by_value(&mut self, target_value: i32) -> Option<Rc<RefCell<Node>>> {
        let mut index:Option<usize> = None;
        let mut iterator = self.children.iter();

        while let node = iterator.next() {
            if node.is_none() {
                break
            }

            if index.is_none() {
                index = Some(0);
            } else {
                index = Some(index.unwrap()+1);
            }

            if (*node.unwrap().clone()).borrow().value == target_value {
                break
            }
        }

        if index.is_some() {
            let deleted = self.children.remove(index.unwrap());
            self.size = self.size - 1;
            return Some(deleted);
        }

        None
    }

    fn replace_children(&mut self, old: Rc<RefCell<Node>>, new:Rc<RefCell<Node>>) {
        let mut iterator = self.children.iter();

        let index = self.children.iter().position(|r| {
            (**r).borrow().value == (*old).borrow().value
        });

        if let Some(index) = index {
            self.children.push(new.clone());

            let last_index = self.children.len()-1;
            self.children.swap(index, last_index);
            self.children.remove(last_index);
        }
    }

    fn iter(&mut self, iteration_type:IterationType) -> NodeIterator {
        NodeIterator::new(self.clone(), iteration_type)
    }
}

enum IterationType {
    Preorder,
    Postorder
}

impl Clone for IterationType {
    fn clone(&self) -> Self {
        match self {
            IterationType::Preorder => {
                IterationType::Preorder
            }
            IterationType:: Postorder => {
                IterationType:: Postorder
            }
        }
    }

    fn clone_from(&mut self, _: &Self) {
        todo!()
    }
}

struct NodeIterator {
    root: Rc<RefCell<Node>>,
    routes: Vec<Rc<RefCell<Node>>>,
    iteration_type:IterationType,
}

impl Iterator for NodeIterator {
    type Item = Rc<RefCell<Node>>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.routes.pop();
        node
    }
}

impl Clone for NodeIterator {
    fn clone(&self) -> Self {
        NodeIterator {
            root: self.root.clone(),
            routes: self.routes.clone(),
            iteration_type: self.iteration_type.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        todo!()
    }
}

impl NodeIterator {
    fn new(root: Node, iteration_type: IterationType) -> NodeIterator {

        let mut iterator = NodeIterator{
            root: Rc::new(RefCell::new(root)),
            // index:0,
            routes: Vec::new(),
            iteration_type,
        };

        {
            let iterator = iterator.borrow_mut();
            let root = iterator.root.clone();
            let iteration_type = iterator.iteration_type.clone();

            iterator.find_route(root, &iteration_type);
            iterator.routes.reverse();
        }

        iterator
    }

    fn len(&self) -> usize {
        self.routes.len()
    }

    fn first(&self) -> Option<Rc<RefCell<Node>>> {
        if self.routes.len() < 1 {
            return None;
        }

        Some(self.routes[0].clone())
    }

    fn find_route(&mut self, curr: Rc<RefCell<Node>>, iteration_type: &IterationType) {// TODO: 이렇게 clone을 함부로 써도 괜찮은가? 확인할 것.
        match iteration_type {
            IterationType::Preorder => {
                self.routes.push(curr);

                let curr = self.routes.last().unwrap().clone();

                for n in (*curr).borrow_mut().children.clone() {
                    self.find_route(n, iteration_type);
                }
            },
            IterationType::Postorder => {
                for n in (*curr).borrow_mut().children.clone() {
                    self.find_route(n, iteration_type);
                }

                self.routes.push(curr);
            }
        }
    }

    fn print_routes(&mut self) {
        for n in self.routes.clone() {
            print!("{} ", (*n).borrow_mut().value);
        }
        println!();
    }
}

struct Tree {
    root:Option<Rc<RefCell<Node>>>,
    children:LinkedList<Rc<RefCell<Node>>>,
}

impl Tree {
    fn new() -> Tree {
        Tree {
            root: None,
            children: LinkedList::new(),
        }
    }

    fn add(&mut self, value: i32) {
        match self.root.clone() {
            Some(root) => {
                match self.find_non_complete_child_node(root, 0) {
                    Some(mut parent) => {
                        let new_node = Rc::new(RefCell::new(Node::new(value)));

                        let children = &mut (*parent).borrow_mut().children;
                        children.push(new_node.clone());

                        (*new_node).borrow_mut().parent = Some(parent.clone());
                    }
                    _ => {
                        println!("WRONG!!!");
                    }
                }
            }
            None => {
                self.root = Some(Rc::new(RefCell::new(Node::new(value))));
            }
        }
    }

    /// delete 함수는 삭제되었다면 true를, 없는 값이라면 false를 리턴한다
    fn delete(&mut self, value:i32) -> bool {
        match self.root.clone() {
            Some(mut root) => {
                let mut iterator = (*root).borrow_mut().iter(IterationType::Preorder);

                let last_node = iterator.clone().first();
                let target_index = iterator.clone().position(|r| {
                    (*r).borrow().value == value
                });

                if last_node.is_none() || target_index.is_none() {
                    return false;
                }

                let last_node = last_node.unwrap();

                let mut original_parent = (*last_node).borrow_mut().clone().parent;
                // TODO: algorithm order is wrong. should be swap -> remove.
                if original_parent.is_some() {
                    // TODO: original_parent.delete(last_node);
                    let mut original_parent = original_parent.unwrap();
                    let mut original_parent = (*original_parent).borrow_mut();

                    let last_node_value = (*last_node).borrow().value;
                    original_parent.borrow_mut().delete_child_by_value(last_node_value);
                }

                let target_index = iterator.len() - target_index.unwrap() - 1;
                let mut target_node = iterator.borrow_mut().routes[target_index].clone();

                let mut new_parent = (*target_node).borrow().clone().parent;
                (*last_node).borrow_mut().parent = new_parent.clone();

                new_parent.map(|parent| {
                    (*parent).borrow_mut().delete_child((*target_node).borrow().value);
                    (*parent).borrow_mut().replace_children(target_node.clone(), last_node.clone());
                });

                (*last_node).borrow_mut().children = (*target_node).borrow().children.clone();

                // TODO: target_node.children.set_parent(last_node);
                let ln = (*target_node).borrow();
                let mut children_iter = ln.children.iter();
                while let Some(c) = children_iter.next() {
                    (**c).borrow_mut().parent = Some(last_node.clone());
                }

                true
            }
            _ => {
                false
            }
        }
    }

    fn find_non_complete_child_node(&mut self, node: Rc<RefCell<Node>>, depth:i32) -> Option<Rc<RefCell<Node>>> {
        if (*node).borrow().children.len() < 2 {
            return Some(node);
        }

        for n in (*node).borrow().children.clone() {
            if (*n).borrow().children.len() < 2 {
                return Some(n);
            }
        }

        for n in (*node).borrow().children.clone() {
            match self.find_non_complete_child_node(n, depth+1) {
                Some(n) => {return Some(n);}
                _ => {}
            }
        }

        None
    }

    fn print_root(&mut self) {
        self.print_children(self.root.clone().unwrap(), 0);
    }

    fn print_children(&mut self, node: Rc<RefCell<Node>>, depth:i32) {
        let node = (*node).borrow();

        println!("{}  |-{}", " ".repeat(depth as usize), (*node).value);

        for n in node.children.clone() {
            self.print_children(n, depth+1);
        }
    }
}
