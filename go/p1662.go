package leetcode

func arrayStringsAreEqual(word1 []string, word2 []string) bool {
	i, i1, j, j1 := 0, 0, 0, 0
	for i < len(word1) && j < len(word2) {
		for i1 < len(word1[i]) && j1 < len(word2[j]) {
			if word1[i][i1] != word2[j][j1] {
				return false
			}
			i1++
			j1++
		}
		if i1 == len(word1[i]) {
			i++
			i1 = 0
		}
		if j1 == len(word2[j]) {
			j++
			j1 = 0
		}
	}
	if i < len(word1) || j < len(word2) {
		return false
	}
	return true
}
