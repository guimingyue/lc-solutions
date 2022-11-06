package leetcode

func searchII(nums []int, target int) bool {
	return searchIIDfs(nums, target, 0, len(nums)-1)
}

func searchIIDfs(nums []int, target, left, right int) bool {
	if left > right {
		return false
	}
	if left == right {
		return nums[left] == target
	}
	mid := (left + right) / 2
	if nums[mid] == target {
		return true
	} else {
		if nums[left] < nums[mid] && target >= nums[left] && target < nums[mid] {
			return searchIIDfs(nums, target, left, mid)
		} else {
			if searchIIDfs(nums, target, left, mid-1) {
				return true
			}
			return searchIIDfs(nums, target, mid+1, right)
		}
	}
}
