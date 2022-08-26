package leetcode

import "sort"

func specialArray(nums []int) int {
	sort.Ints(nums)
	for i := 0; i <= len(nums); i++ {
		idx := binarySearch(nums, i)
		num := len(nums) - idx - 1
		if nums[idx] >= i {
			num++
		}
		if num == i {
			return i
		}
	}
	return -1
}

func binarySearch(nums []int, v int) int {
	var i, j = 0, len(nums) - 1
	for i < j {
		mid := i + (j-i)/2
		if nums[mid] < v {
			i = mid + 1
		} else {
			j = mid
		}
	}
	return i
}
