package leetcode

import "sort"

func frequencySort(nums []int) []int {
	type Pair struct {
		val int
		num int
	}

	pairs := make(map[int]Pair)
	for _, v := range nums {
		p, ok := pairs[v]
		if ok {

			pairs[v] = Pair{val: v, num: p.num + 1}
		} else {
			pairs[v] = Pair{val: v, num: 1}
		}
	}

	vals := make([]int, len(pairs))
	i := 0
	for v, _ := range pairs {
		vals[i] = v
		i++
	}
	sort.Slice(vals, func(i, j int) bool {
		p1, _ := pairs[vals[i]]
		p2, _ := pairs[vals[j]]
		if p1.num != p2.num {
			return p1.num < p2.num
		} else {
			return p1.val > p2.val
		}
	})
	res := make([]int, 0)
	for _, v := range vals {
		p, _ := pairs[v]
		for i := 0; i < p.num; i++ {
			res = append(res, v)
		}
	}

	return res
}
