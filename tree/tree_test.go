// tree
// defined acronyms
// rc, RC, R.C., r.c. = Right Child
// lc, LC, L.C., l.c. = Left Child
// r, R = Root
// t, T = Tree
// n, N = Node

// some expressions
// r.lc = root > left child
// rc.lc = right child > left child (2 depth)
// rc.lc.rc = right child > left child > right child. NOT GOOD EXPRESSIONS. find alternative ways.
package tree

import (
	"github.com/stretchr/testify/assert"
	"strconv"
	"testing"
)

func Test_Add(t *testing.T) {
	t.Run("새로 만든 tree에 값을 추가 했을때, 순서대로 tree.nodes에 저장됨", func(t *testing.T) {
		tree := NewTree()
		tree.Add("test", 1)
		tree.Add("test2", 2)

		assert.Equal(t, 2, len(tree.nodes))
		assert.Equal(t, 1, tree.nodes[0].Data)
		assert.Equal(t, 2, tree.nodes[1].Data)
	})

	t.Run("새로 만든 tree에 값을 추가 했을때, binary search tree처럼 왼쪽 오른쪽을 구분해서 저장됨", func(t *testing.T) {
		tree := NewTree()
		tree.Add("test2", 2)
		tree.Add("test3", 3)
		tree.Add("test4", 4)
		tree.Add("test1", 1)

		assert.Equal(t, 4, len(tree.nodes))
		assert.Equal(t, 2, tree.nodes[0].Data)
		assert.Equal(t, 3, tree.nodes[1].Data)
		assert.Equal(t, 4, tree.nodes[2].Data)
		assert.Equal(t, 1, tree.nodes[3].Data)

		assert.Equal(t, 3, tree.root.Children[Right].Data)
		assert.Equal(t, 4, tree.root.Children[Right].Children[Right].Data)
		assert.Equal(t, 1, tree.root.Children[Left].Data)
	})
}

func Test_Get(t *testing.T) {
	t.Run("추가된 키로 Get 했을 때, 해당 key에 저장된 노드를 반환", func(t *testing.T) {
		tree := NewTree()
		tree.Add("test2", 2)

		tree.Add("test3", 3)

		tree.Add("test4", 4)

		assert.Equal(t, 3, tree.Get("test3").Data)
	})

	t.Run("추가되지 않은 키로 Get 했을 때, nil을 반환", func(t *testing.T) {
		tree := NewTree()
		tree.Add("test1", 1)
		tree.Add("test2", 2)
		tree.Add("test3", 3)

		assert.Nil(t, tree.Get("non-exists-key"))
	})

	t.Run("root가 없는 비어있는 tree에서 Get 했을 때, nil을 반환", func(t *testing.T) {
		tree := NewTree()

		assert.Nil(t, tree.Get("test1"))
	})
}

func Test_Pop(t *testing.T) {
	t.Run("pop을 했을 때, 해당 자리에 left의 가장 오른쪽 node로 치환", func(t *testing.T) {
		wanted := 200
		tree := NewTree()

		tree.Add("180", 180)
		tree.Add("200", wanted)
		tree.Add("190", 190)
		tree.Add("191", 191)
		tree.Add("189", 189)
		tree.Add("210", 210)
		tree.Add("209", 209)
		tree.Add("211", 211)

		n := tree.Pop("200")
		assert.Equal(t, wanted, n.Data)

		assert.Equal(t, 191, tree.root.Children[Right].Data)
	})

	t.Run("root를 pop을 했을 때, 해당 자리에 left의 가장 오른쪽 node로 치환", func(t *testing.T) {
		wanted := 180
		tree := NewTree()

		//               r
		//         1             8
		//      5    2        9      12
		//     7 6  4  3   11  10  13  14
		// ->
		//               3
		//         1             8
		//      5    2        9      12
		//     7 6  4       11  10  13  14

		tree.Add("180", 180) // r
		tree.Add("160", 160) // 1
		tree.Add("170", 170) // 2
		tree.Add("171", 171) // 3 => New Root
		tree.Add("169", 169) // 4
		tree.Add("150", 150) // 5
		tree.Add("151", 151) // 6
		tree.Add("149", 149) // 7
		tree.Add("200", 200) // 8
		tree.Add("190", 190) // 9
		tree.Add("191", 191) // 10
		tree.Add("189", 189) // 11
		tree.Add("210", 210) // 12
		tree.Add("209", 209) // 13
		tree.Add("211", 211) // 14

		n := tree.Pop("180")
		assert.Equal(t, wanted, n.Data)

		assert.Equal(t, 171, tree.root.Data)
	})

	t.Run("left가 없는 노드를 pop을 했을 때, 해당 자리에 right의 가장 왼쪽 node로 치환", func(t *testing.T) {
		wanted := 180
		tree := NewTree()

		tree.Add("180", 180) // r
		tree.Add("200", 200) // 8
		tree.Add("190", 190) // 9
		tree.Add("191", 191) // 10
		tree.Add("189", 189) // 11
		tree.Add("210", 210) // 12
		tree.Add("209", 209) // 13
		tree.Add("211", 211) // 14

		n := tree.Pop("180")
		assert.Equal(t, wanted, n.Data)

		assert.Equal(t, 189, tree.root.Data)
	})

	t.Run("children이 없는 노드를 pop을 했을 때, 아무 동작도 안하고 pop만 수행", func(t *testing.T) {
		wanted := 200
		tree := NewTree()

		tree.Add("180", 180)
		tree.Add("200", 200)

		n := tree.Pop("200")
		assert.Equal(t, wanted, n.Data)

		assert.Equal(t, 180, tree.root.Data)
		assert.Nil(t, tree.root.Children[Right])
	})

	t.Run("children이 없는 root 노드를 pop을 했을 때, children이 없는 pop과 동일한 동작 수행 후 root 삭제", func(t *testing.T) {
		wanted := 180
		tree := NewTree()

		tree.Add("180", 180)

		n := tree.Pop("180")
		assert.Equal(t, wanted, n.Data)

		assert.Nil(t, tree.root)
	})
}

func Test_Rightmost(t *testing.T) {
	t.Run("자식의 depth가 1인 node가 있을 때, 해당 노드의 Rightmost(left child) = left child이다", func(t *testing.T) {
		tree := NewTree()
		tree.Add("180", 180)
		tree.Add("160", 160)
		tree.Add("200", 200)

		rightMost := tree.getRightmost(tree.root.Children[Left])

		assert.NotNil(t, rightMost)
		assert.Equal(t, tree.nodes[1].Key, rightMost.Key)
		assert.Equal(t, tree.nodes[1].Data, rightMost.Data)
	})

	t.Run("깊이가 얼마나 깊던지, rightmost node를 반환한다", func(t *testing.T) {
		tree := NewTree()
		tree.Add("180", 180)
		tree.Add("160", 160)

		for i := 161; i < 180; i++ {
			tree.Add(strconv.Itoa(i), i)
		}

		rightMost := tree.getRightmost(tree.root.Children[Left])

		assert.NotNil(t, rightMost)
		assert.Equal(t, 179, rightMost.Data)
	})
}

func Test_ShouldReBalance(t *testing.T) {
	t.Run("루트의 왼쪽과 오른쪽의 깊이가 2 이상 차이날 때, rebalance 필요", func(t *testing.T) {
		tree := NewTree()
		tree.Add("180", 180)
		tree.Add("181", 181)
		tree.Add("182", 182)

		assert.True(t, tree.shouldReBalance())
	})

	t.Run("루트의 왼쪽과 오른쪽의 깊이가 2 미만 차이날 때, rebalance 필요 없음", func(t *testing.T) {
		tree := NewTree()
		tree.Add("180", 180)
		tree.Add("181", 181)

		assert.False(t, tree.shouldReBalance())

		tree = NewTree()
		tree.Add("180", 180)
		tree.Add("181", 181)
		tree.Add("179", 79)

		assert.False(t, tree.shouldReBalance())
	})

	t.Run("루트의 자식이 없을때, rebalance 필요 없음", func(t *testing.T) {
		tree := NewTree()
		tree.Add("180", 180)

		assert.False(t, tree.shouldReBalance())
	})

	t.Run("루트가 없을때, rebalance 필요 없음", func(t *testing.T) {
		tree := NewTree()
		assert.False(t, tree.shouldReBalance())
	})
}

func Test_TreeReBalance(t *testing.T) {
	// when left is deeper then right
	//   1. set left child to root
	//   2. set left child's right child to previous root's left child
	//   2. make previous root to new root's right child
	//           6            ->     4
	//         /   \               /   \
	//        4     8            3      6
	//      /   \               /     /  \
	//     3     5            1      5     8
	//    /
	//   1
	t.Run("왼쪽이 2 이상 긴 경우, lc를 r로 변경하고 이전 r를 새로운 r의 rc로 변경 (일반 상태)", func(t *testing.T) {
		tree := NewTree()

		tree.Add("6", nil)
		tree.Add("4", nil)
		tree.Add("8", nil)
		tree.Add("3", nil)
		tree.Add("5", nil)
		tree.Add("1", nil)

		tree.reBalance()

		assert.Equal(t, "4", tree.root.Key)
		assert.Equal(t, "3", tree.root.Children[Left].Key)
		assert.Equal(t, "1", tree.root.Children[Left].Children[Left].Key)
		assert.Equal(t, "6", tree.root.Children[Right].Key)
		assert.Equal(t, "5", tree.root.Children[Right].Children[Left].Key)
		assert.Equal(t, "8", tree.root.Children[Right].Children[Right].Key)
	})

	// when rc is deeper is mirrored to lc is deeper
	t.Run("왼쪽이 2 이상 길지만 lc.rc가 없는 경우, 일반 상태와 동일하지만 기존 r의 lc가 비어있는 상태로 변환", func(t *testing.T) {
		tree := NewTree()

		tree.Add("6", nil)
		tree.Add("4", nil)
		tree.Add("8", nil)
		tree.Add("3", nil)
		tree.Add("1", nil)

		tree.reBalance()

		assert.Equal(t, "4", tree.root.Key)
		assert.Equal(t, "3", tree.root.Children[Left].Key)
		assert.Equal(t, "1", tree.root.Children[Left].Children[Left].Key)
		assert.Equal(t, "6", tree.root.Children[Right].Key)
		assert.Equal(t, "8", tree.root.Children[Right].Children[Right].Key)
	})

	t.Run("오른쪽이 2 이상 긴 경우, rc를 r로 변경하고 이전 r를 새로운 r의 lc로 변경 (일반 상태)", func(t *testing.T) {
		tree := NewTree()

		tree.Add("6", nil)
		tree.Add("4", nil)
		tree.Add("8", nil)
		tree.Add("7", nil)
		tree.Add("9", nil)
		tree.Add("99", nil)

		tree.reBalance()

		assert.Equal(t, "8", tree.root.Key)
		assert.Equal(t, "6", tree.root.Children[Left].Key)
		assert.Equal(t, "4", tree.root.Children[Left].Children[Left].Key)
		assert.Equal(t, "7", tree.root.Children[Left].Children[Right].Key)
		assert.Equal(t, "9", tree.root.Children[Right].Key)
		assert.Equal(t, "99", tree.root.Children[Right].Children[Right].Key)
	})

	t.Run("오른쪽이 2 이상 길지만 rc.lc가 없는 경우, 일반 상태와 동일하지만 기존 r의 rc가 비어있는 상태로 변환", func(t *testing.T) {
		tree := NewTree()

		tree.Add("6", nil)
		tree.Add("4", nil)
		tree.Add("8", nil)
		tree.Add("9", nil)
		tree.Add("99", nil)

		tree.reBalance()

		assert.Equal(t, "8", tree.root.Key)
		assert.Equal(t, "6", tree.root.Children[Left].Key)
		assert.Equal(t, "4", tree.root.Children[Left].Children[Left].Key)
		assert.Equal(t, "9", tree.root.Children[Right].Key)
		assert.Equal(t, "99", tree.root.Children[Right].Children[Right].Key)
	})
}
