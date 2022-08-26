package leetcode

func lenLongestFibSubseq(arr []int) int {
	m := make(map[int]int, len(arr))
	dp := make([][]int, len(arr))
	for i, v := range arr {
		m[v] = i
		dp[i] = make([]int, len(arr))
	}
	max := func(v1, v2 int) int {
		if v1 > v2 {
			return v1
		} else {
			return v2
		}
	}

	res := 0
	for i, x := range arr {
		for j := i - 1; j >= 0 && arr[j]*2 > x; j-- {
			if k, ok := m[x-arr[j]]; ok {
				dp[j][i] = max(dp[k][j]+1, 3)
				if dp[j][i] > res {
					res = dp[j][i]
				}
			}
		}
	}
	return res
}
