/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
 func addOneRow(root *TreeNode, val int, depth int) *TreeNode {
    if depth == 1 {
		return &TreeNode{Val: val, Left: root}
	}
	queue := make([]*TreeNode, 1)
	queue[0] = root
	for depth > 2 {
		depth--
		k := len(queue)
		for i := 0; i < k; i++ {
			node := queue[i]
			if node.Left != nil {
				queue = append(queue, node.Left)
			}
			if node.Right != nil {
				queue = append(queue, node.Right)
			}
		}
		queue = queue[k:]
	}

	k := len(queue)
	for i := 0; i < k; i++ {
		node := queue[i]
		node.Left = &TreeNode{Val: val, Left: node.Left}
		node.Right = &TreeNode{Val: val, Right: node.Right}
	}
	return root
}