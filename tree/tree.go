package tree

import "math"

type Tree struct {
	root  *BinaryNode
	nodes []*BinaryNode
}

func NewTree() (t Tree) {
	t = Tree{
		root:  nil,
		nodes: make([]*BinaryNode, 0),
	}
	return t
}

func (t *Tree) Add(key string, data interface{}) {
	node := NewNode(key, data)

	if t.root == nil {
		t.root = node
	} else {
		t.root.AddChild(node)
	}
	t.nodes = append(t.nodes, node)
}

func (t *Tree) Get(key string) *BinaryNode {
	if t.root == nil {
		return nil
	}

	return t.root.Find(key)
}

func (t *Tree) Pop(key string) (b *BinaryNode) {
	if t.root == nil {
		return nil
	}

	b = t.root.Find(key)
	if b == nil {
		return
	}

	targetNode := t.getRightmost(b.Children[Left])
	if targetNode == nil {
		targetNode = t.getLeftmost(b.Children[Right])
	}

	if b.Parent != nil {
		b.Parent.Children[b.LocationInParent] = targetNode
	} else {
		t.root = targetNode
	}

	if targetNode != nil {
		targetNode.Children[Left] = b.Children[Left]
		targetNode.Children[Right] = b.Children[Right]
	}
	b.Children[Left] = nil
	b.Children[Right] = nil

	return
}

func (t *Tree) getRightmost(node *BinaryNode) (b *BinaryNode) {
	if node == nil {
		return nil
	}

	if node.Children[Right] != nil {
		return t.getRightmost(node.Children[Right])
	}

	return node
}

func (t *Tree) getLeftmost(node *BinaryNode) (b *BinaryNode) {
	if node == nil {
		return nil
	}

	if node.Children[Left] != nil {
		return t.getLeftmost(node.Children[Left])
	}

	return node
}

func (t *Tree) getDeepest(node *BinaryNode) (b *BinaryNode, depth int) {
	var leftNode, rightNode *BinaryNode
	var leftDepth, rightDepth int

	if node.Children[Left] != nil {
		leftNode, leftDepth = t.getDeepest(node.Children[Left])
	} else {
		leftNode = node
		leftDepth = node.Depth()
	}

	if node.Children[Right] != nil {
		rightNode, rightDepth = t.getDeepest(node.Children[Right])
	} else {
		rightNode = node
		rightDepth = node.Depth()
	}

	if rightDepth < leftDepth {
		return leftNode, leftDepth
	} else {
		return rightNode, rightDepth
	}
}

func (t *Tree) shouldReBalance() (should bool) {
	if t.root == nil {
		return false
	}

	var leftDepth, rightDepth int

	if t.root.Children[Left] != nil {
		_, leftDepth = t.getDeepest(t.root.Children[Left])
	}

	if t.root.Children[Right] != nil {
		_, rightDepth = t.getDeepest(t.root.Children[Right])
	}

	return 2 <= math.Abs(float64(rightDepth-leftDepth))
}