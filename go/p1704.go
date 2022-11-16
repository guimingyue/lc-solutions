package leetcode

func halvesAreAlike(s string) bool {
	l := len(s) / 2
	const BASE = 'a' - 'A'
	num := 0
	for i, ch := range s {
		if ch >= 'a' && ch <= 'z' {
			ch = ch - BASE
		}
		if ch != 'A' && ch != 'E' && ch != 'I' && ch != 'O' && ch != 'U' {
			continue
		}
		if i < l {
			num++
		} else {
			num--
		}
	}
	return num == 0
}
