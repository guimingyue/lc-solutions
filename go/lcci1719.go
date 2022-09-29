package leetcode

func missingTwo(nums []int) []int {
	n := len(nums) + 2
	xor := 0
	i := 0
	for ; i < n; i++ {
		xor ^= i + 1
		if i < len(nums) {
			xor ^= nums[i]
		}
	}

	v := xor
	idx := 0
	for v&(1<<idx) == 0 {
		idx++
	}
	xor1 := 0
	xor2 := 0
	for i = 0; i < n; i++ {
		if (i+1)&(1<<idx) == 0 {
			xor1 ^= i + 1
		} else {
			xor2 ^= i + 1
		}
		if i < len(nums) {
			if nums[i]&(1<<idx) == 0 {
				xor1 ^= nums[i]
			} else {
				xor2 ^= nums[i]
			}
		}
	}
	return []int{xor1, xor2}
}
