use std::borrow::{BorrowMut};

#[cfg(test)]
mod tests {
    use crate::tree::IterationType::{Postorder, Preorder};
    use crate::tree::Node;

    #[test]
    /// new 함수를 호출할 경우 초기화된 node를 생성
    fn create_node() {
        const EXPECT: i32 = 1;
        let mut node = Node::new(EXPECT);

        assert_eq!(0, node.size());
        assert_eq!(EXPECT, node.value);
        assert_eq!(None, node.parent);
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
        assert_eq!(CHILD_VALUE, child.value);

        unsafe {
            let value = (*child.parent().unwrap()).value;
            assert_eq!(node.value, value);
        }
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

        assert_eq!(EXPECT_VALUE, actual.unwrap().value);
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
            actual.push(next.value);
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
            actual.push(next.value);
        }

        let expect: &[i32] = &[3, 4, 5, 2, 10, 7, 8, 9, 6, 1];
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
}

struct Node {
    value: i32,
    size: i32,
    parent: Option<*mut Node>,
    children: Vec<Node>,
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

    fn parent(&mut self) -> Option<*mut Node> {
        self.parent.clone()
    }

    fn add_child(&mut self, mut child: Node) {
        child.parent = Some(self);
        self.children.push(child);
        self.size = self.size + 1;
    }

    fn get_child(&mut self, index: i32) -> Option<&mut Node> {
        self.children.get_mut(index as usize)
    }

    fn delete_child(&mut self, index: i32) -> Option<Node> {
        if self.size <= index {
            return None;
        }

        let deleted = self.children.remove(index as usize);
        self.size = self.size - 1;
        Some(deleted)
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
    root: Box<Node>,
    // current: *mut Node,
    // index: i32,
    routes: Vec<Box<Node>>,
    iteration_type:IterationType,
}

impl Iterator for NodeIterator {
    type Item = Box<Node>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.routes.pop();
        node
    }
}

impl NodeIterator {
    fn new(root: Node, iteration_type: IterationType) -> NodeIterator {

        let mut iterator = NodeIterator{
            root: Box::new(root),
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

    fn find_route(&mut self, curr: Box<Node>, iteration_type: &IterationType) {
        match iteration_type {
            IterationType::Preorder => {
                self.routes.push(curr);

                let curr = self.routes.last().unwrap().clone();

                for n in curr.children.clone() {
                    self.find_route(Box::new(n), iteration_type);
                }
            },
            IterationType::Postorder => {
                for n in curr.children.clone() {
                    self.find_route(Box::new(n), iteration_type);
                }

                self.routes.push(curr);
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn print_routes(&mut self) {
        for n in self.routes.clone() {
            println!("{} - ", n.value);
        }
    }
}