func constructMaximumBinaryTree(nums []int) *TreeNode {
	if len(nums) == 0 {
		return nil
	}
	maxIdx := 0
	max := nums[0]
	for i, v := range nums {
		if v > max {
			maxIdx = i
			max = v
		}
	}
	return &TreeNode{Val: max, Left: constructMaximumBinaryTree(nums[:maxIdx]), Right: constructMaximumBinaryTree(nums[maxIdx+1:])}
}
