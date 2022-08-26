package p919

import "leetcode"

type CBTInserter struct {
	arr  []*leetcode.TreeNode
	root *leetcode.TreeNode
}

func Constructor(root *leetcode.TreeNode) CBTInserter {
	arr := make([]*leetcode.TreeNode, 0)
	q := make([]*leetcode.TreeNode, 0)
	q = append(q, root)
	for len(q) > 0 {
		node := q[0]
		arr = append(arr, node)
		q = q[1:]
		if node.Left != nil {
			q = append(q, node.Left)
		}
		if node.Right != nil {
			q = append(q, node.Right)
		}
	}

	a := arr
	newRoot := a[0]
	queue := make([]*leetcode.TreeNode, 0)
	queue = append(queue, newRoot)
	i := 1
	for i < len(a) {
		node := queue[0]
		queue = queue[1:]
		if i < len(a) {
			node.Left = a[i]
			queue = append(queue, a[i])
			i++
		}
		if i < len(a) {
			node.Right = a[i]
			queue = append(queue, a[i])
			i++
		}
	}
	return CBTInserter{arr, newRoot}
}

func (this *CBTInserter) Insert(val int) int {
	node := &leetcode.TreeNode{Val: val}
	this.arr = append(this.arr, node)
	idx := len(this.arr)
	parent := this.arr[idx/2-1]
	if idx%2 == 0 {
		parent.Left = node
	} else {
		parent.Right = node
	}
	return parent.Val
}

func (this *CBTInserter) Get_root() *leetcode.TreeNode {
	return this.root
}
