package leetcode

func checkOnesSegment(s string) bool {
	i := len(s) - 1
	for i >= 0 {
		if s[i] == '1' {
			break
		}
		i--
	}
	for i >= 0 {
		if s[i] == '0' {
			return false
		}
		i--
	}
	return true
}
