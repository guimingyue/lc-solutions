package leetcode

func numTilings(n int) int {
	const MOD = 1_000_000_007
	dp := make([][]int, n+1)
	for i := 0; i <= n; i++ {
		dp[i] = make([]int, 4)
	}
	dp[0][0], dp[0][1], dp[0][2], dp[0][3] = 0, 0, 0, 1
	for i := 1; i <= n; i++ {
		dp[i][0] = dp[i-1][3]
		dp[i][1] = (dp[i-1][0] + dp[i-1][2]) % MOD
		dp[i][2] = (dp[i-1][0] + dp[i-1][1]) % MOD
		dp[i][3] = (((dp[i-1][0]+dp[i-1][1])%MOD+dp[i-1][2])%MOD + dp[i-1][3]) % MOD
	}
	return dp[n][3]
}
