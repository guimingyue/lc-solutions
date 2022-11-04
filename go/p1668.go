package leetcode

import "strings"

func maxRepeating(sequence string, word string) int {
	i := 0
	for ; i*len(word) <= len(sequence); i++ {
		sub := strings.Repeat(word, i+1)
		if strings.Index(sequence, sub) < 0 {
			return i
		}
	}
	return i
}
