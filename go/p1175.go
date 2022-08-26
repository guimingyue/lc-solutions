package leetcode

func numPrimeArrangements(n int) int {
	arr := []int{2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97}
	i := len(arr) - 1
	for ; i >= 0; i-- {
		if arr[i] <= n {
			break
		}
	}

	MOD := 1000000007
	a := func(v int) int {
		res := 1
		for v > 0 {
			res = (res * v) % MOD
			v--
		}
		return res
	}
	return a(i+1) * a(n-i-1) % MOD
}
