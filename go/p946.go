package leetcode

func validateStackSequences(pushed []int, popped []int) bool {
	stack := make([]int, 0)
	i := 0
	j := 0
	for j < len(popped) {
		for i < len(pushed) && pushed[i] != popped[j] {
			stack = append(stack, pushed[i])
			i++
		}
		for i < len(pushed) && pushed[i] == popped[j] {
			i++
			j++
		}
		for len(stack) > 0 && j < len(popped) && stack[len(stack)-1] == popped[j] {
			stack = stack[:len(stack)-1]
			j++
		}
		if i >= len(pushed) && j < len(popped) && len(stack) > 0 {
			return false
		}

	}
	return true
}
