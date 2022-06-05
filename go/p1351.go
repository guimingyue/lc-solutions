func countNegatives(grid [][]int) int {
    var m, n = len(grid) - 1, len(grid[0]) - 1
	res := 0
	for i := 0; i <= m; i++ {
		var l, r = 0, n
		for l < r {
			mid := (l + r) / 2
			if grid[i][mid] >= 0 {
				l = mid + 1
			} else {
				r = mid
			}
		}
		if grid[i][l] < 0 {
			res += n - l + 1
		}
	}
	return res
}