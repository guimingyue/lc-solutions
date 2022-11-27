package leetcode

func numSubarrayBoundedMax(nums []int, left int, right int) int {
	last1, last2 := -1, -1
	res := 0
	for i, x := range nums {
		if x >= left && x <= right {
			last1 = i
		} else if x > right {
			last2 = i
			last1 = -1
		}
		if last1 != -1 {
			res += last1 - last2
		}
	}
	return res
}
