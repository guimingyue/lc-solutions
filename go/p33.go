package leetcode

func search(nums []int, target int) int {
	return searchDfs(nums, target, 0, len(nums)-1)
}

func searchDfs(nums []int, target, left, right int) int {
	if left > right {
		return -1
	}
	if right == left {
		if nums[left] == target {
			return left
		} else {
			return -1
		}
	}
	mid := (left + right) / 2
	if nums[mid] == target {
		return mid
	} else {
		if target >= nums[left] && target <= nums[mid] && nums[left] < nums[mid] {
			return searchDfs(nums, target, left, mid)
		} else {
			idx := searchDfs(nums, target, left, mid)
			if idx >= 0 {
				return idx
			}
			return searchDfs(nums, target, mid+1, right)
		}
	}
}
