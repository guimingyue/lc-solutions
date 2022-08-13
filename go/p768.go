func maxChunksToSorted(arr []int) int {
    maxf := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}
	stack := make([]int, 0)
	for i := 0; i < len(arr); i++ {
		if len(stack) > 0 {
			if stack[len(stack)-1] <= arr[i] {
				stack = append(stack, arr[i])
			} else {
				max := arr[i]
				for len(stack) > 0 && stack[len(stack)-1] > arr[i] {
					pop := stack[len(stack)-1]
					stack = stack[:len(stack)-1]
					max = maxf(pop, max)
				}
				stack = append(stack, max)
			}
		} else {
			stack = append(stack, arr[i])
		}
	}
	return len(stack)
}