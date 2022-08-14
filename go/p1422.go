func maxScore(s string) int {
    zeroNum := 0
	oneNum := 0
	for _, c := range s {
		if c == '1' {
			oneNum++
		} else {
			zeroNum++
		}
	}
	left := 0
	right := oneNum
	if s[0] == '0' {
		left += 1
	} else {
		right -= 1
	}
	max := left + right
	for i := 1; i < len(s)-1; i++ {
		if s[i] == '1' {
			right--
		} else {
			left++
		}
		if left+right > max {
			max = left + right
		}
	}
	return max
}