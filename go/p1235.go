package leetcode

import "sort"

func jobScheduling(startTime []int, endTime []int, profit []int) int {
	idx := make([]int, len(endTime))
	for i := 0; i < len(endTime); i++ {
		idx[i] = i
	}
	sort.Slice(idx, func(i, j int) bool {
		return endTime[idx[i]] < endTime[idx[j]]
	})
	dp := make([]int, len(startTime)+1)
	dp[0] = 0
	max := func(a, b int) int {
		if a >= b {
			return a
		} else {
			return b
		}
	}
	for i := 1; i <= len(idx); i++ {
		k := sort.Search(i, func(j int) bool {
			return endTime[idx[j]] > startTime[idx[i-1]]
		})
		dp[i] = max(dp[i-1], dp[k]+profit[idx[i-1]])
	}

	return dp[len(startTime)]
}

/*
[1,3,3,3,5,5,13,5]
[17,14,8,11,14,7,17,9]
[9,3,7,18,2,17,4,6]


[24,24,7,2,1,13,6,14,18,24]
[27,27,20,7,14,22,20,24,19,27]
[6,1,4,2,3,6,5,6,9,8]
*/
