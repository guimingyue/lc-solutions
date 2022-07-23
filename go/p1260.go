func shiftGrid(grid [][]int, k int) [][]int {
    if k == 0 {
		return grid
	}
	m, n := len(grid), len(grid[0])
	res := make([][]int, m)
	for i := 0; i < len(res); i++ {
		res[i] = make([]int, n)
	}

	k = k % (m * n)
	if k == 0 {
		return grid
	}
	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			x := (i + (j+k)/n) % m
			y := (j + k) % n
			res[x][y] = grid[i][j]
		}
	}
	return res
}