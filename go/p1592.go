package leetcode

import (
	"strings"
)

func reorderSpaces(text string) string {
	i := 0
	spaceCnt := 0
	words := make([]string, 0)
	wordStart := false
	wordStartIdx := 0
	for i < len(text) {
		if text[i] == ' ' {
			spaceCnt++
			if wordStart {
				words = append(words, text[wordStartIdx:i])
			}
			wordStart = false
		} else {
			if !wordStart {
				wordStart = true
				wordStartIdx = i
			}
		}
		i++
	}
	if text[len(text)-1] != ' ' {
		words = append(words, text[wordStartIdx:len(text)])
	}
	var perSpaceCnt int
	var lastSpaceCnt int
	if len(words) == 1 {
		perSpaceCnt = 0
		lastSpaceCnt = spaceCnt
	} else {
		perSpaceCnt = spaceCnt / (len(words) - 1)
		lastSpaceCnt = spaceCnt % (len(words) - 1)
	}
	builder := strings.Builder{}
	for idx, word := range words {
		builder.WriteString(word)
		scnt := perSpaceCnt
		if idx == len(words)-1 {
			scnt = lastSpaceCnt
		}
		for k := 0; k < scnt; k++ {
			builder.WriteByte(' ')
		}
	}
	return builder.String()
}
