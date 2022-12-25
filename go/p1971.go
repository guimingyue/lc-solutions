package leetcode

func validPath(n int, edges [][]int, source int, destination int) bool {
	if source == destination {
		return true
	}
	m := make(map[int][]int)
	for _, edge := range edges {
		m[edge[0]] = append(m[edge[0]], edge[1])
		m[edge[1]] = append(m[edge[1]], edge[0])
	}

	var dfs func(m map[int][]int, src, dest int, vis map[int]struct{}) bool
	dfs = func(m map[int][]int, src, dest int, vis map[int]struct{}) bool {
		vis[src] = struct{}{}
		queue := m[src]
		for _, v := range queue {
			if _, ok := vis[v]; ok {
				continue
			}
			if v == destination {
				return true
			} else {
				if dfs(m, v, dest, vis) {
					return true
				}
			}
		}
		//delete(vis, src)
		return false
	}
	vis := make(map[int]struct{})
	return dfs(m, source, destination, vis)
}
