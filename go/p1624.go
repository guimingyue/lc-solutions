package leetcode

func maxLengthBetweenEqualCharacters(s string) int {
	m := make(map[rune]int)
	res := -1
	max := func(a, b int) int {
		if a > b {
			return a
		} else {
			return b
		}
	}
	for i, ch := range s {
		if idx, ok := m[ch]; ok {
			res = max(res, i-idx-1)
		} else {
			m[ch] = i
		}
	}
	return res
}
