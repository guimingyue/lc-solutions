func oddCells(m int, n int, indices [][]int) int {
	matrix := make([][]int, m)
	for i := 0; i < m; i++ {
		matrix[i] = make([]int, n)
	}
	for _, v := range indices {
		for i := 0; i < n; i++ {
			matrix[v[0]][i] += 1
		}
		for i := 0; i < m; i++ {
			matrix[i][v[1]] += 1
		}
	}
	res := 0
	for _, vx := range matrix {
		for _, v := range vx {
			if v%2 == 1 {
				res++
			}
		}
	}
	return res
}
