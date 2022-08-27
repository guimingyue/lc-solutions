package leetcode

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func widthOfBinaryTree(root *TreeNode) int {
	type Pair struct {
		node *TreeNode
		num  int
	}
	queue := make([]*Pair, 0)
	queue = append(queue, &Pair{node: root, num: 1})
	res := 1
	for len(queue) > 0 {
		size := len(queue)
		tmp := queue[size-1].num - queue[0].num + 1
		if tmp > res {
			res = tmp
		}
		for _, p := range queue {
			if p.node.Left != nil {
				queue = append(queue, &Pair{p.node.Left, 2 * p.num})
			}
			if p.node.Right != nil {
				queue = append(queue, &Pair{p.node.Right, 2*p.num + 1})
			}
		}
		queue = queue[size:]
	}
	return res
}
