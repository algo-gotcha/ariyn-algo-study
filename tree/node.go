package tree

import "log"

const (
	Left  = 1
	Right = 2
)

type BinaryNode struct {
	Parent           *BinaryNode
	LocationInParent int
	Children         [3]*BinaryNode
	Key              string
	Data             interface{}
}

func NewNode(key string, data interface{}) *BinaryNode {
	return &BinaryNode{
		Parent:   nil,
		Children: [3]*BinaryNode{},
		Key:      key,
		Data:     data,
	}
}

func (n *BinaryNode) AddChild(c *BinaryNode) (added bool) {
	parent := n.FindParent(c.Key)
	if parent == nil {
		return false
	}

	c.Parent = parent
	if parent.Key < c.Key {
		parent.Children[Right] = c
		c.LocationInParent = Right
	} else {
		parent.Children[Left] = c
		c.LocationInParent = Left
	}

	return true
}

func (n *BinaryNode) FindParent(key string) (p *BinaryNode) {
	p = n

	if p.Key < key { // c goes to right
		if p.Children[Right] == nil {
			return
		} else {
			return p.Children[Right].FindParent(key)
		}
	} else if p.Key == key { // key duplicated. impossible to insert
		return nil
	} else { // c goes to left when less
		if p.Children[Left] == nil {
			return
		} else {
			return p.Children[Left].FindParent(key)
		}
	}
}

func (n *BinaryNode) Find(key string) (p *BinaryNode) {
	p = n

	if p.Key < key {
		if p.Children[Right] == nil {
			return nil
		} else {
			return p.Children[Right].Find(key)
		}
	} else if p.Key == key {
		log.Println("returned", p)
		return
	} else {
		if p.Children[Left] == nil {
			return nil
		} else {
			return p.Children[Left].Find(key)
		}
	}
}

func (n *BinaryNode) Depth() (d int) {
	d = 1

	if n.Parent == nil {
		return 0
	}

	return d + n.Parent.Depth()
}
