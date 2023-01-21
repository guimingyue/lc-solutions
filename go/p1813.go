package leetcode

import "strings"

func areSentencesSimilar(sentence1 string, sentence2 string) bool {
	if len(sentence1) > len(sentence2) {
		sentence1, sentence2 = sentence2, sentence1
	} else if len(sentence1) == len(sentence2) {
		return strings.Compare(sentence1, sentence2) == 0
	}
	str1 := strings.Split(sentence1, " ")
	str2 := strings.Split(sentence2, " ")
	i := 0
	for ; i < len(str1); i++ {
		if str1[i] != str2[i] {
			break
		}
	}
	j := len(str1) - 1
	j2 := len(str2) - 1
	for ; j >= i; j-- {
		if str1[j] != str2[j2] {
			break
		}
		j2--
	}
	return j < i
}
