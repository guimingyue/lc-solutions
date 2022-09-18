package p827

var d = []int{0, -1, 0, 1, 0}

func largestIsland(grid [][]int) int {
	max := func(a, b int) int {
		if a > b {
			return a
		} else {
			return b
		}
	}
	res := 0
	n := len(grid)
	tag := make([][]int, n)
	for i := 0; i < n; i++ {
		tag[i] = make([]int, n)
	}
	area := make(map[int]int)
	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			if grid[i][j] == 0 || tag[i][j] != 0 {
				continue
			}
			t := i*n + j + 1
			islandArea := dfs(grid, i, j, tag, t)
			area[t] = islandArea
			res = max(res, islandArea)
		}
	}

	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			if grid[i][j] == 0 {
				z := 1
				connected := make(map[int]struct{}, 0)
				for k := 0; k < 4; k++ {
					x := i + d[k]
					y := j + d[k+1]
					if !valid(x, y, len(grid)) || tag[x][y] == 0 {
						continue
					}
					t := tag[x][y]
					_, ok := connected[t]
					if ok {
						continue
					}
					z += area[t]
					connected[t] = struct{}{}
				}
				res = max(res, z)
			}
		}
	}
	return res
}

func dfs(grid [][]int, i, j int, tag [][]int, t int) int {
	tag[i][j] = t
	area := 1
	for k := 0; k < 4; k++ {
		x := i + d[k]
		y := j + d[k+1]
		if valid(x, y, len(grid)) && grid[x][y] == 1 && tag[x][y] == 0 {
			area += dfs(grid, x, y, tag, t)
		}
	}
	return area
}

func valid(x, y, n int) bool {
	return x >= 0 && x < n && y >= 0 && y < n
}
