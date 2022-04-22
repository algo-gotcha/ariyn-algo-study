package tree

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

func (n *BinaryNode) Left() *BinaryNode {
	if n == nil { // necessary?
		return nil
	}
	return n.Children[Left]
}

func (n *BinaryNode) Right() *BinaryNode {
	if n == nil {
		return nil
	}

	return n.Children[Right]
}

func (n *BinaryNode) BF() int {
	rcDepth := n.Right().MaxDepth()
	lcDepth := n.Left().MaxDepth()

	return rcDepth - lcDepth
}

func (n *BinaryNode) AddChild(c *BinaryNode) (added bool) {
	if n == nil {
		return false
	}

	var position int

	if n.Key < c.Key {
		position = Right
	} else {
		position = Left
	}

	if n.Children[position] == nil {
		n.Children[position] = c
		c.Parent = n
		c.LocationInParent = position

		added = true
	} else {
		added = n.Children[position].AddChild(c)
	}

	return
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
	if n == nil || n.Parent == nil {
		return 0
	}
	return n.Parent.Depth() + 1
}

func (n *BinaryNode) MaxDepth() int {
	if n == nil {
		return 0
	}

	lDepth := n.Left().MaxDepth() + 1
	rDepth := n.Right().MaxDepth() + 1

	if lDepth < rDepth {
		return rDepth
	} else {
		return lDepth
	}
}
