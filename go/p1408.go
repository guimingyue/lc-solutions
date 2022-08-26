package leetcode

import (
	"sort"
	"strings"
)

func stringMatching(words []string) []string {
	sort.Slice(words, func(i, j int) bool {
		return len(words[i]) < len(words[j])
	})
	res := make([]string, 0)
	for i, word := range words {
		for j := i + 1; j < len(words); j++ {
			if strings.Contains(words[j], word) {
				res = append(res, word)
				break
			}
		}
	}
	return res
}
