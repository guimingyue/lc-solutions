package leetcode

import (
	"github.com/emirpasic/gods/sets/hashset"
	"strings"
)

func replaceWords(dictionary []string, sentence string) string {
	set := hashset.New()
	for _, s := range dictionary {
		set.Add(s)
	}
	strs := strings.Split(sentence, " ")
	arr := make([]string, len(strs))
	for idx, str := range strs {
		i := 1
		for ; i < len(str); i++ {
			if set.Contains(str[0:i]) {
				arr[idx] = str[0:i]
				break
			}
		}
		if i >= len(str) {
			arr[idx] = str
		}
	}
	str := ""
	for _, s := range arr {
		if len(str) > 0 {
			str = str + " " + s
		} else {
			str = s
		}
	}
	return str
}
