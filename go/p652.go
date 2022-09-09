package leetcode

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func findDuplicateSubtrees(root *TreeNode) []*TreeNode {
	type Pair struct {
		root  *TreeNode
		value int
	}
	repeat := make(map[*TreeNode]struct{})
	seen := make(map[[3]int]Pair)
	idx := 0
	var dfs func(*TreeNode) int
	dfs = func(node *TreeNode) int {
		if node == nil {
			return 0
		}
		tri := [3]int{node.Val, dfs(node.Left), dfs(node.Right)}
		if pair, ok := seen[tri]; ok {
			repeat[pair.root] = struct{}{}
			return pair.value
		} else {
			idx++
			seen[tri] = Pair{node, idx}
			return idx
		}
	}
	dfs(root)
	res := make([]*TreeNode, 0)
	for node, _ := range repeat {
		res = append(res, node)
	}
	return res
}
