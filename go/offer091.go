package leetcode

func minCost(costs [][]int) int {
	min := func(v1 int, v2 int) int {
		if v1 > v2 {
			return v2
		} else {
			return v1
		}
	}
	dp0, dp1, dp2 := costs[0][0], costs[0][1], costs[0][2]
	for _, cost := range costs[1:] {
		_dp0 := cost[0] + min(dp1, dp2)
		_dp1 := cost[1] + min(dp0, dp2)
		_dp2 := cost[2] + min(dp0, dp1)
		dp0, dp1, dp2 = _dp0, _dp1, _dp2
	}
	return min(min(dp0, dp1), dp2)
}
