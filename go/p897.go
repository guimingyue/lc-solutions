package leetcode

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func increasingBST(root *TreeNode) *TreeNode {
	res, _ := inOrder(root)
	return res
}

func inOrder(root *TreeNode) (*TreeNode, *TreeNode) {
	if root.Left != nil {
		res, last := inOrder(root.Left)
		last.Right = &TreeNode{Val: root.Val}
		last = last.Right
		if root.Right != nil {
			var newLast *TreeNode
			last.Right, newLast = inOrder(root.Right)
			last = newLast
		}
		return res, last
	}
	res := &TreeNode{Val: root.Val}
	if root.Right != nil {
		var last *TreeNode
		res.Right, last = inOrder(root.Right)
		return res, last
	}
	return res, res
}
