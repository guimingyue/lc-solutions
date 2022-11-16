package leetcode

func isIdealPermutation(nums []int) bool {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}
	n := len(nums)
	minSuf := nums[n-1]
	for i := n - 2; i > 0; i-- {
		if nums[i-1] > minSuf {
			return false
		}
		minSuf = min(minSuf, nums[i])
	}
	return true
}
