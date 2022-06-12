func minFlipsMonoIncr(s string) int {
	var dp0, dp1 = 0, 0
	n := len(s)
	for i := 0; i < n; i++ {
		dp0New := dp0
		dp1New := min(dp0, dp1)
		if s[i] == '1' {
			dp0New++
		} else {
			dp1New++
		}
		dp0, dp1 = dp0New, dp1New
	}

	return min(dp0, dp1)

}

func min(v1 int, v2 int) int {
	if v1 < v2 {
		return v1
	} else {
		return v2
	}
}