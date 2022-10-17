package leetcode

func totalFruit(fruits []int) int {
	p1, p2 := -1, -1
	res, max := 0, 0
	for i, v := range fruits {
		if p1 == -1 || p2 == -1 {
			max++
		} else if v == p1 || v == p2 {
			max++
		} else {
			j := 0
			for i-j-1 >= 0 && fruits[i-j-1] == p1 {
				j++
			}
			max = j + 1
		}
		if p1 != v {
			p2 = p1
			p1 = v
		}

		if max > res {
			res = max
		}
	}
	return res
}
