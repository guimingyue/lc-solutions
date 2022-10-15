package leetcode

func areAlmostEqual(s1 string, s2 string) bool {
	a1, b1, a2, b2 := ' ', ' ', ' ', ' '
	count := 0
	for i := 0; i < len(s1); i++ {
		if s1[i] != s2[i] {
			count++
			if count > 2 {
				return false
			}
			if a1 == ' ' {
				a1 = int32(s1[i])
				b1 = int32(s2[i])
			} else {
				if a1 != int32(s2[i]) || b1 != int32(s1[i]) {
					return false
				} else {
					a2 = int32(s1[i])
					b2 = int32(s2[i])
				}
			}

		}
	}
	if a1 != b2 || a2 != b1 {
		return false
	}
	return true
}
