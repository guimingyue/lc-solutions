package leetcode

func minOperations(logs []string) int {
	stack := make([]string, 0)
	for _, str := range logs {
		switch str {
		case "../":
			if len(stack) > 0 {
				stack = stack[0 : len(stack)-1]
			}
		case "./":
			continue
		default:
			stack = append(stack, str)
		}
	}
	return len(stack)
}
