package leetcode

import "strings"

func defangIPaddr(address string) string {
	str := strings.Builder{}
	for _, ch := range address {
		if ch == '.' {
			str.WriteString("[.]")
		} else {
			str.WriteByte(byte(ch))
		}
	}
	return str.String()
}
