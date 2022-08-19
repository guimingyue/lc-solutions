/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
 func deepestLeavesSum(root *TreeNode) int {
    queue := make([]*TreeNode, 0)
	queue = append(queue, root)
	levelSum := 0
	for len(queue) > 0 {
		sum := 0
		size := len(queue)
		for _, node := range queue[:size] {
			if node.Left == nil && node.Right == nil {
				sum += node.Val
			} else {
				if node.Left != nil {
					queue = append(queue, node.Left)
				}
				if node.Right != nil {
					queue = append(queue, node.Right)
				}
			}
		}
		queue = queue[size:]
		
			levelSum = sum
	
	}
	return levelSum
}