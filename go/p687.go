package leetcode

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func longestUnivaluePath(root *TreeNode) int {
	res := 0
	dfs(root, &res)
	return res
}

func dfs(root *TreeNode, res *int) int {
	if root == nil {
		return 0
	}
	left1 := 0
	right1 := 0
	left := dfs(root.Left, res)
	right := dfs(root.Right, res)
	if root.Left != nil && root.Val == root.Left.Val {
		left1 = left + 1
	}
	if root.Right != nil && root.Val == root.Right.Val {
		right1 = right + 1
	}
	*res = max(*res, left1+right1)
	return max(left1, right1)
}

func max(a, b int) int {
	if a > b {
		return a
	} else {
		return b
	}
}
