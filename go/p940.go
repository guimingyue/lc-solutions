package leetcode

func distinctSubseqII(s string) int {
	const MOD = 1000_000_007
	dp := make([]int, len(s))
	last := make([]int, 26)
	for i := 0; i < len(last); i++ {
		last[i] = -1
	}

	for i := 0; i < len(s); i++ {
		dp[i] = 1
		for j := 0; j < len(last); j++ {
			if last[j] != -1 {
				dp[i] = (dp[i] + dp[last[j]]) % MOD
			}
		}
		last[s[i]-'a'] = i
	}
	res := 0
	for j := 0; j < len(last); j++ {
		if last[j] != -1 {
			res = (res + dp[last[j]]) % MOD
		}
	}
	return res
}
