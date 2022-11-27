package leetcode

func nthMagicalNumber(n int, a int, b int) int {
	min := func(a, b int) int {
		if a < b {
			return a
		}
		return b
	}
	var gcd func(a, b int) int
	gcd = func(a, b int) int {
		if b != 0 {
			return gcd(b, a%b)
		}
		return a
	}
	l := min(a, b)
	r := n * min(a, b)
	c := a / gcd(a, b) * b

	for l < r {
		mid := (l + r) / 2
		cnt := mid/a + mid/b - mid/c
		if cnt < n {
			l = mid + 1
		} else {
			r = mid
		}
	}
	return r % 1_000_000_007

}
