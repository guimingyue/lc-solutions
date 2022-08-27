package leetcode

import "fmt"

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func printTree(root *TreeNode) [][]string {
	height := -1
	queue := make([]*TreeNode, 0)
	queue = append(queue, root)
	for len(queue) > 0 {
		height++
		size := len(queue)
		for _, node := range queue[0:size] {
			if node.Left != nil {
				queue = append(queue, node.Left)
			}
			if node.Right != nil {
				queue = append(queue, node.Right)
			}
		}
		queue = queue[size:]
	}
	type Elem struct {
		node *TreeNode
		c    int
	}
	res := make([][]string, height+1)
	level := 0
	n := 1<<(height+1) - 1
	q := make([]*Elem, 0)
	q = append(q, &Elem{node: root, c: (n - 1) / 2})
	for i := 0; i < height+1; i++ {
		l := make([]string, n)
		/*for i := 0; i < n; i++ {
			l = append(l, "")
		}*/
		size := len(q)
		for _, elem := range q[0:size] {
			l[elem.c] = fmt.Sprint(elem.node.Val)
			if elem.node.Left != nil {
				q = append(q, &Elem{node: elem.node.Left, c: elem.c - 1<<(height-level-1)})
			}
			if elem.node.Right != nil {
				q = append(q, &Elem{node: elem.node.Right, c: elem.c + 1<<(height-level-1)})
			}
		}
		res[level] = l
		q = q[size:]
		level++
	}
	return res
}
