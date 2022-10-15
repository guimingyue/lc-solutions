package leetcode

func scoreOfParentheses(s string) int {
	max := func(a, b int) int {
		if a > b {
			return a
		} else {
			return b
		}
	}
	stack := []int{0}
	for _, ch := range s {
		if ch == '(' {
			stack = append(stack, 0)
		} else {
			l := len(stack)
			v := stack[l-1]
			w := stack[l-2]
			stack[l-2] = w + max(2*v, 1)
			stack = stack[0 : l-1]
		}
	}
	return stack[0]
}
