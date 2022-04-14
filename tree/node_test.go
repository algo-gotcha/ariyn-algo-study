package tree

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestBinaryNode_Depth(t *testing.T) {
	t.Run("parent가 없는 경우, depth = 1", func(t *testing.T) {
		n := NewNode("test", nil)
		assert.Equal(t, 0, n.Depth())
	})

	t.Run("parent가 두개 있는 경우, depth = 2", func(t *testing.T) {
		n3 := NewNode("test3", nil)
		n2 := NewNode("test2", nil)
		n1 := NewNode("test1", nil)

		n1.AddChild(n2)
		n2.AddChild(n3)

		assert.Equal(t, 2, n3.Depth())
	})
}
