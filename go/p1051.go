func heightChecker(heights []int) int {
	n := len(heights)
	m := make([]int, n)
	for i := 0; i < n; i++ {
		m[i] = heights[i]
	}
	sort.Ints(heights)
	count := 0
	for i := 0; i < len(heights); i++ {
		if heights[i] != m[i] {
			count++
		}
	}
	return count
}