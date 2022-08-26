package leetcode

func groupThePeople(groupSizes []int) [][]int {
	m := make(map[int][]int)
	for i, v := range groupSizes {
		list, _ := m[v]
		list = append(list, i)
		m[v] = list
	}
	res := make([][]int, 0)
	for k, v := range m {
		for len(v) > 0 {
			l := make([]int, 0)
			for i := 0; i < k; i++ {
				l = append(l, v[0])
				v = v[1:]
			}
			res = append(res, l)
		}
	}
	return res
}
