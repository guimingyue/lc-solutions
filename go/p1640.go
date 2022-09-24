package leetcode

func canFormArray(arr []int, pieces [][]int) bool {
	m := map[int]int{}
	for i, p := range pieces {
		m[p[0]] = i
	}
	i := 0
	for i < len(arr) {
		v := arr[i]
		if p, ok := m[v]; ok {
			for _, pv := range pieces[p] {
				if arr[i] != pv {
					return false
				}
				i++
			}
		} else {
			return false
		}
	}
	return true
}
