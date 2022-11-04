package leetcode

func partitionDisjoint(nums []int) int {
	min := func(a, b int) int {
		if a < b {
			return a
		} else {
			return b
		}
	}

	n := len(nums)
	minRight := make([]int, n)
	minRight[n-1] = nums[n-1]
	for i := n - 2; i >= 0; i-- {
		minRight[i] = min(nums[i], minRight[i+1])
	}

	max := func(a, b int) int {
		if a > b {
			return a
		} else {
			return b
		}
	}

	maxLeft := nums[0]
	for i := 0; i < n-1; i++ {
		maxLeft = max(nums[i], maxLeft)
		if maxLeft <= minRight[i+1] {
			return i + 1
		}
	}

	return n - 1
}

/*
[3,5,1,4,6,8,5]
[3,4,2,8,5]
[3,0,2,8,3]
[3,0,3,8,3]
[3,0,3,8,6]
[5,0,3,8,6]
[1,1,1,0,6,12]
*/
