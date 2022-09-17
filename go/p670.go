package leetcode

import "sort"

func maximumSwap(num int) int {
	vec := make([]int, 0)
	n := num
	for n > 0 {
		vec = append(vec, n%10)
		n = n / 10
	}
	cpy := make([]int, len(vec))
	copy(cpy, vec)
	sort.Slice(vec, func(i, j int) bool {
		return vec[i] < vec[j]
	})
	i := len(vec) - 1
	v1, v2 := -1, -1
	for ; i >= 0; i-- {
		if vec[i] != cpy[i] {
			v1 = cpy[i]
			v2 = vec[i]
			cpy[i] = vec[i]
			break
		}
	}
	for j := 0; j < i; j++ {
		if cpy[j] == v2 {
			cpy[j] = v1
			break
		}
	}
	res := 0
	for i = len(cpy) - 1; i >= 0; i-- {
		res = res*10 + cpy[i]
	}
	return res
}
