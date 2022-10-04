package leetcode

func minAddToMakeValid(s string) int {
	var stack []rune
	for _, ch := range s {
		if ch == '(' {
			stack = append(stack, '(')
		} else {
			l := len(stack)
			if l > 0 && stack[l-1] == '(' {
				stack = stack[0 : l-1]
			} else {
				stack = append(stack, ')')
			}
		}
	}
	return len(stack)
}
