/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
 func findFrequentTreeSum(root *TreeNode) []int {
	m := make(map[int]int)
	findSumCnt(root, m)
	res := make([]int, 0)
	maxCnt := 0
	for v, cnt := range m {
		if cnt > maxCnt {
			res = nil
			maxCnt = cnt
		}
		if cnt == maxCnt {
			res = append(res, v)
		}
	}
	return res
}

func findSumCnt(root *TreeNode, m map[int]int) int {
	if root == nil {
		return 0
	}
	sum := root.Val
	if root.Left != nil {
		sum += findSumCnt(root.Left, m)
	}
	if root.Right != nil {
		sum += findSumCnt(root.Right, m)
	}
	cnt, ok := m[sum]
	if ok {
		m[sum] = cnt + 1
	} else {
		m[sum] = 1
	}
	return sum
}