package leetcode

import "strings"

func mergeAlternately(word1 string, word2 string) string {
	res := strings.Builder{}
	i, j := 0, 0
	for i < len(word1) && j < len(word2) {
		res.WriteByte(word1[i])
		res.WriteByte(word2[j])
		i++
		j++
	}
	if i < len(word1) {
		res.WriteString(word1[i:])
	}
	if j < len(word2) {
		res.WriteString(word2[j:])
	}
	return res.String()
}
