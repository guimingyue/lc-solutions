package leetcode

import "github.com/emirpasic/gods/sets/hashset"

func arrayNesting(nums []int) int {
	set := hashset.New()
	max := 0
	for i, v := range nums {
		if !set.Contains(v) {
			k := i
			cur := 0
			for !set.Contains(nums[k]) {
				set.Add(nums[k])
				cur += 1
				k = nums[k]
			}
			if cur > max {
				max = cur
			}
		}
	}
	return max
}
