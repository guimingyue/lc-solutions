package leetcode

func buildArray(target []int, n int) []string {
	var res []string
	i, j := 1, 0
	for i <= n && j < len(target) {
		if target[j] == i {
			res = append(res, "Push")
			j++
		} else {
			res = append(res, "Push")
			res = append(res, "Pop")
		}
		i++
	}
	return res
}
