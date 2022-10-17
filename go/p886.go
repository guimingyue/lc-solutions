package leetcode

func possibleBipartition(n int, dislikes [][]int) bool {
	g := make([][]int, n)
	for _, d := range dislikes {
		x, y := d[0]-1, d[1]-1
		g[x] = append(g[x], y)
		g[y] = append(g[y], x)
	}
	color := make([]int, n)
	var dfs func(i, c int) bool
	dfs = func(i, c int) bool {
		color[i] = c
		for _, v := range g[i] {
			if color[v] == c || color[v] == 0 && !dfs(v, 3^c) {
				return false
			}
		}
		return true
	}
	for i := 0; i < n; i++ {
		if color[i] == 0 && !dfs(i, 1) {
			return false
		}
	}
	return true
}
