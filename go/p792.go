package leetcode

func numMatchingSubseq(s string, words []string) int {
	isSubSeq := func(a map[rune][]int, b string) bool {
		idx := make([]int, 26)
		for k := 0; k < len(idx); k++ {
			idx[k] = -1
		}
		sIdx := -1
		for _, ch := range b {
			if list, ok := a[ch]; ok {
				chIdx := ch - 'a'
				idx[chIdx] += 1
				if len(list) <= idx[chIdx] {
					return false
				}
				if list[idx[chIdx]] < sIdx {
					j := 0
					for j < len(list) {
						if list[j] >= sIdx {
							break
						}
						j++
					}
					if j == len(list) {
						return false
					}
					idx[chIdx] = j
					sIdx = list[j]
				} else {
					sIdx = list[idx[chIdx]]
				}
			} else {
				return false
			}
		}
		return true
	}
	m := make(map[rune][]int)
	for i, ch := range s {
		m[ch] = append(m[ch], i)
	}
	res := 0
	for _, word := range words {
		if isSubSeq(m, word) {
			res++
		}
	}
	return res
}
