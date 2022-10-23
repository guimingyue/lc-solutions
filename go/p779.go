package leetcode

func kthGrammar(n int, k int) int {
	var recFn func(k, n int) int

	recFn = func(k, n int) int {
		if n == 1 {
			return 0
		}
		v := recFn(k/2, n-1)
		if v == 0 {
			if k%2 == 0 {
				return 0
			} else {
				return 1
			}
		} else {
			if k%2 == 0 {
				return 1
			} else {
				return 0
			}
		}
	}
	return recFn(k-1, n)
}
