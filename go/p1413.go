package leetcode

func minStartValue(nums []int) int {
	sum := 0
	min := 100
	for _, v := range nums {
		sum += v
		if sum < min {
			min = sum
		}
	}
	if min >= 1 {
		return 1
	} else {
		return 1 - min
	}
}
