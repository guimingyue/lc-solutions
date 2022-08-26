package leetcode

func findAndReplacePattern(words []string, pattern string) []string {
	res := make([]string, 0)
	for i := 0; i < len(words); i++ {
		if len(pattern) != len(words[i]) {
			continue
		}
		m1 := make(map[uint8]uint8)
		m2 := make(map[uint8]uint8)
		add := true
		for k := 0; k < len(pattern); k++ {
			ch, ok := m1[pattern[k]]
			if !ok {
				_, ok2 := m2[words[i][k]]
				if ok2 {
					add = false
					break
				}
				m1[pattern[k]] = words[i][k]
				m2[words[i][k]] = pattern[k]
			} else {
				if ch != words[i][k] {
					add = false
					break
				}
			}
		}
		if add {
			res = append(res, words[i])
		}
	}
	return res
}
