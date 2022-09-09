package leetcode

func constructArray(n int, k int) []int {
	res := make([]int, 0)
	for i := 1; i < n-k; i++ {
		res = append(res, i)
	}
	i := n - k
	j := n
	for i <= j {
		res = append(res, i)
		if i != j {
			res = append(res, j)
		}
		i++
		j--
	}
	return res
}
