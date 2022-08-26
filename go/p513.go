package leetcode

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func findBottomLeftValue(root *TreeNode) int {
	queue := make([]*TreeNode, 0)
	queue = append(queue, root)
	res := 0
	for len(queue) > 0 {
		size := len(queue)
		for i := 0; i < size; i++ {
			elem := queue[0]
			queue = queue[1:]
			if i == 0 {
				res = elem.Val
			}
			if elem.Left != nil {
				queue = append(queue, elem.Left)
			}
			if elem.Right != nil {
				queue = append(queue, elem.Right)
			}
		}
	}
	return res
}
