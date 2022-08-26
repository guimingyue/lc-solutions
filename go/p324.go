package leetcode

import "sort"

func wiggleSort(nums []int) {
	sort.Slice(nums, func(i, j int) bool {
		return nums[i]-nums[j] > 0
	})
	arr := make([]int, len(nums))
	for i := 0; i < len(nums); i++ {
		arr[i] = nums[i]
	}
	i := 0
	for ; 2*i+1 < len(arr); i++ {
		nums[2*i+1] = arr[i]
	}
	base := i
	for ; i < len(arr); i++ {
		nums[2*(i-base)] = arr[i]
	}
}
