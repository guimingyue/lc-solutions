import "sort"

func arrayRankTransform(arr []int) []int {
	var cpy = make([]int, len(arr))
	copy(cpy, arr)
	sort.Ints(cpy)
	m := make(map[int]int, 0)
	for _, v := range cpy {
		if _, ok := m[v]; !ok {
			m[v] = len(m) + 1
		}
	}
	res := make([]int, len(arr))
	for i, v := range arr {
		res[i] = m[v]
	}
	return res
}
