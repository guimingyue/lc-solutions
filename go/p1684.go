package leetcode

func countConsistentStrings(allowed string, words []string) int {
	a := make([]bool, 26)
	for _, ch := range allowed {
		a[ch-'a'] = true
	}
	allow := func(arr []bool, word string) bool {
		for _, ch := range word {
			if !arr[ch-'a'] {
				return false
			}
		}
		return true
	}
	res := 0
	for _, word := range words {
		if allow(a, word) {
			res++
		}
	}
	return res
}
