package leetcode

func maxProduct(nums []int) int {
	var v1, v2 int
	if nums[0] >= nums[1] {
		v1, v2 = nums[0], nums[1]
	} else {
		v1, v2 = nums[1], nums[0]
	}

	for _, v := range nums[2:] {
		if v > v1 {
			v2 = v1
			v1 = v
		} else if v > v2 {
			v2 = v
		}
	}
	return (v1 - 1) * (v2 - 1)
}
