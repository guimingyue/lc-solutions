package p1769

func minOperations(boxes string) []int {
	abs := func(a int) int {
		if a < 0 {
			return -a
		}
		return a
	}
	n := len(boxes)
	res := make([]int, n)
	for i := 0; i < n; i++ {
		s := 0
		for j := 0; j < n; j++ {
			if boxes[j] == '1' {
				s += abs(i - j)
			}
		}
		res[i] = s
	}
	return res
}
