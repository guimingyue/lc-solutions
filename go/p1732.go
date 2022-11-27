package leetcode

func largestAltitude(gain []int) int {
	cur := 0
	max := 0
	for _, v := range gain {
		cur += v
		if cur > max {
			max = cur
		}
	}
	return max
}
