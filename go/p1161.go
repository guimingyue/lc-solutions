/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
 import "math"

func maxLevelSum(root *TreeNode) int {
	var queue []*TreeNode
	queue = append(queue, root)
	line := 0
	maxSum := math.MinInt32
	res := 0
	for size := len(queue); size > 0; size = len(queue) {
		sum := 0
		line++
		for i := 0; i < size; i++ {
			sum += queue[i].Val
			if queue[i].Left != nil {
				queue = append(queue, queue[i].Left)
			}
			if queue[i].Right != nil {
				queue = append(queue, queue[i].Right)
			}
		}
		queue = queue[size:]
		if sum > maxSum {
			res = line
			maxSum = sum
		}
	}
	return res
}
