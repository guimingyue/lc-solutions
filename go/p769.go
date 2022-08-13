func maxChunksToSorted(arr []int) int {
    res := 0
	max := 0
	maxf := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	for i, v := range arr {
		max = maxf(v, max)
		if i == max {
			res++
		}
	}
	return res
}