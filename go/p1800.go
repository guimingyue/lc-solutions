package leetcode

func maxAscendingSum(nums []int) int {
	res := 0
	i := 0
	for i < len(nums) {
		j := i + 1
		sum := nums[i]
		for j < len(nums) && nums[j-1] < nums[j] {
			sum += nums[j]
			j++
		}
		i = j
		if sum > res {
			res = sum
		}

	}
	return res
}
