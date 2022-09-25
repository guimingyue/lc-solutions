package leetcode

func rotatedDigits(n int) int {
	m := map[int]int{0: 0, 1: 1, 2: 5, 3: -1, 4: -1, 5: 2, 6: 9, 7: -1, 8: 8, 9: 6}
	rotate := func(v int) (bool, int) {
		num := 0
		exp := 1
		for v > 0 {
			vr, _ := m[v%10]
			if vr == -1 {
				return false, -1
			}
			num += exp * vr
			v = v / 10
			exp *= 10
		}
		return true, num
	}
	res := 0
	for i := 1; i <= n; i++ {
		good, rv := rotate(i)
		if good && rv != i {
			res++
		}
	}
	return res
}
