package leetcode

import "sort"

func findLongestChain(pairs [][]int) int {
	sort.Slice(pairs, func(i, j int) bool {
		if pairs[i][0] < pairs[j][0] {
			return true
		} else if pairs[i][0] == pairs[j][0] {
			return pairs[i][1] < pairs[j][1]
		}
		return false
	})
	dp := make([]int, len(pairs))
	dp[0] = 1

	max := func(a, b int) int {
		if a > b {
			return a
		} else {
			return b
		}
	}

	for i := 0; i < len(pairs); i++ {
		for j := i + 1; j < len(pairs); j++ {
			if pairs[i][1] < pairs[j][0] {
				dp[j] = dp[i] + 1
			} else {
				dp[j] = max(dp[i], dp[j])
			}
		}
	}
	return dp[len(pairs)-1]
}
