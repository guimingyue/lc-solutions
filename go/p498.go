func findDiagonalOrder(mat [][]int) []int {
	dir := [][]int{{-1, 1}, {1, -1}}
	res := make([]int, 0)
	m, n := len(mat)-1, len(mat[0])-1

	for k := 0; k <= n; k++ {
		i := 0
		j := k
		min := min_num(m-i, k)
		if k%2 == 0 {
			i += min
			j -= min
		}
		d := dir[k%2]
		for i >= 0 && i <= m && j >= 0 && j <= n {
			res = append(res, mat[i][j])
			i += d[0]
			j += d[1]
		}
	}

	for k := 1; k <= m; k++ {
		i := k
		j := n
		min := min_num(m-i, n)
		d := dir[(n+k)%2]
		if (n+k)%2 == 0 {
			i += min
			j -= min
		}
		for i >= 0 && i <= m && j >= 0 && j <= n {
			res = append(res, mat[i][j])
			i += d[0]
			j += d[1]
		}
	}

	return res
}

func min_num(a int, b int) int {
	if a < b {
		return a
	} else {
		return b
	}
}