package leetcode

func checkPowersOfThree(n int) bool {
	arr := []int{1, 3, 9, 27, 81, 243, 729, 2187, 6561, 19683, 59049, 177147, 531441, 1594323, 4782969}
	idx := len(arr) - 1
	for n > 0 && idx >= 0 {
		for n < arr[idx] {
			idx--
		}
		n = n - arr[idx]
		idx--
	}
	return n == 0
}
