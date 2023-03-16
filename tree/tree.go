package tree

import (
	"fmt"
	"math"
	"strings"
)

type Tree struct {
	root        *BinaryNode
	nodes       []*BinaryNode
	autoBalance bool
}

func NewTree() (t Tree) {
	t = Tree{
		root:  nil,
		nodes: make([]*BinaryNode, 0),
	}
	return t
}

func (t *Tree) SetAutoBalance(v bool) {
	t.autoBalance = v
}

func (t *Tree) Add(key string, data interface{}) {
	node := NewNode(key, data)

	if t.root == nil {
		t.root = node
	} else {
		added := t.root.AddChild(node)

		if added && t.autoBalance {
			if target := t.checkReBalance(node); target != nil {
				t.reBalance(target)
			}
		}
	}
	t.nodes = append(t.nodes, node)
}

func (t *Tree) checkReBalance(node *BinaryNode) *BinaryNode {
	if node.Parent == nil {
		return nil
	}

	bf := node.Parent.BF()
	if bf <= -2 || 2 <= bf {
		return node.Parent
	}

	return t.checkReBalance(node.Parent)
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
		if targetNode != nil && targetNode.Parent != nil {
			targetNode.Parent.Children[Left] = targetNode.Children[Right]
		}
	} else {
		if targetNode.Parent != nil {
			targetNode.Parent.Children[Right] = targetNode.Children[Left]
		}
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

// TODO: if node is nil, panic
// redundant with node.BF()?
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

// redundant with node.BF()?
func (t *Tree) shouldReBalance(node *BinaryNode) (should bool) {
	if node == nil {
		return false
	}
	var leftDepth, rightDepth int

	if node.Children[Left] != nil {
		_, leftDepth = t.getDeepest(node.Children[Left])
	}

	if node.Children[Right] != nil {
		_, rightDepth = t.getDeepest(node.Children[Right])
	}

	return 2 <= math.Abs(float64(rightDepth-leftDepth))
}

func (t *Tree) reBalance(node *BinaryNode) {
	var leftDepth, rightDepth int

	if node.Children[Left] != nil {
		_, leftDepth = t.getDeepest(node.Children[Left])
	}
	if node.Children[Right] != nil {
		_, rightDepth = t.getDeepest(node.Children[Right])
	}

	if leftDepth < rightDepth { // Right Heavy
		var leftDepth2, rightDepth2 int

		if node.Children[Right].Children[Left] != nil {
			_, leftDepth2 = t.getDeepest(node.Children[Right].Children[Left])
		}
		if node.Children[Right].Children[Right] != nil {
			_, rightDepth2 = t.getDeepest(node.Children[Right].Children[Right])
		}

		if leftDepth2 < rightDepth2 { // right-right
			t.rotate(node.Children[Right], Left)
		} else {
			// right-left rotate
			t.rotateRightLeft(node.Children[Right])
		}
	} else if rightDepth < leftDepth { // left heavy
		var leftDepth2, rightDepth2 int

		if node.Children[Left].Children[Left] != nil {
			_, leftDepth2 = t.getDeepest(node.Children[Left].Children[Left])
		}
		if node.Children[Left].Children[Right] != nil {
			_, rightDepth2 = t.getDeepest(node.Children[Left].Children[Right])
		}

		if rightDepth2 < leftDepth2 { // left-left
			t.rotate(node.Children[Left], Right)
		} else {
			// left-right rotate
			t.rotateLeftRight(node.Children[Left])
		}
	}
}

func (t *Tree) rotate(target *BinaryNode, direction int) {
	var opposite int
	if direction == Left {
		opposite = Right
	} else {
		opposite = Left
	}

	if target == nil || target.Parent == nil || target.Parent.Children[opposite] != target {
		return
	}

	parent := target.Parent
	target.Parent, parent.Parent = parent.Parent, target
	if target.Parent != nil {
		target.Parent.Children[parent.LocationInParent] = target
	}
	target.LocationInParent = parent.LocationInParent

	target.Children[direction], parent.Children[opposite] = parent, target.Children[direction]
	if parent.Children[opposite] != nil {
		parent.Children[opposite].Parent = parent
		parent.Children[opposite].LocationInParent = opposite
	}
	parent.LocationInParent = direction

	if t.root == parent {
		t.root = target
	}
}

func (t *Tree) rotateRightLeft(target *BinaryNode) {
	t.rotate(target.Children[Left], Right)
	t.rotate(target.Parent, Left)
}

func (t *Tree) rotateLeftRight(target *BinaryNode) {
	t.rotate(target.Children[Right], Left)
	t.rotate(target.Parent, Right)
}

func (t *Tree) Viz() string {
	edges := t.viz(t.root)
	return fmt.Sprintf("digraph {\n%s\n}", strings.Join(edges, "\n"))
}

func (t *Tree) viz(n *BinaryNode) (edges []string) {
	if n.Left() != nil {
		edges = append(edges, fmt.Sprintf("%s -> %s", n.Key, n.Left().Key))
		edges = append(edges, t.viz(n.Left())...)
	}

	if n.Right() != nil {
		edges = append(edges, fmt.Sprintf("%s -> %s", n.Key, n.Right().Key))
		edges = append(edges, t.viz(n.Right())...)
	}

	return
}
