package leetcode

func countBalls(lowLimit int, highLimit int) int {
	m := make(map[int]int)
	max := 0
	for i := lowLimit; i <= highLimit; i++ {
		k := i
		s := 0
		for k > 0 {
			s += k % 10
			k = k / 10
		}
		m[s]++
		v, _ := m[s]
		if v > max {
			max = v
		}
	}
	return max
}
