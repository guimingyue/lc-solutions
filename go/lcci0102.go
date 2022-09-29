package leetcode

func CheckPermutation(s1 string, s2 string) bool {
	cnt := make([]int, 26)
	for _, ch := range s1 {
		idx := rune(ch) - 'a'
		cnt[int(idx)]++
	}
	for _, ch := range s2 {
		idx := rune(ch) - 'a'
		cnt[int(idx)]--
	}

	for _, v := range cnt {
		if v != 0 {
			return false
		}
	}
	return true
}
