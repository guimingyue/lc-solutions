package leetcode

import (
	"sort"
)

func canPartitionKSubsets(nums []int, k int) bool {
	m := map[int]int{}
	sum := 0
	max := nums[0]
	maxFunc := func(a, b int) int {
		if a > b {
			return a
		} else {
			return b
		}
	}

	for _, v := range nums {
		sum += v
		m[v]++
		max = maxFunc(v, max)
	}

	if sum%k != 0 {
		return false
	}
	avg := sum / k
	if avg < max {
		return false
	}

	sort.Ints(nums)

	dp := make([]bool, 1<<len(nums))
	for i := 0; i < len(dp); i++ {
		dp[i] = true
	}

	return pDfs(1<<len(nums)-1, 0, avg, nums, dp)
}

func pDfs(s, p, avg int, nums []int, dp []bool) bool {
	if s == 0 {
		return true
	}
	if !dp[s] {
		return false
	}
	dp[s] = false
	for i, v := range nums {
		if v+p > avg {
			return false
		}
		if (s>>i)&1 != 0 {
			if pDfs(s^(1<<i), (p+v)%avg, avg, nums, dp) {
				return true
			}
		}
	}
	return false
}
