package leetcode

func sumSubarrayMins(arr []int) int {
	const MOD = 1_000_000_007
	var stack []int
	n := len(arr)
	left := make([]int, n)
	for i, v := range arr {
		for len(stack) > 0 && arr[stack[len(stack)-1]] >= v {
			stack = stack[0 : len(stack)-1]
		}
		if len(stack) == 0 {
			left[i] = i + 1
		} else {
			left[i] = i - stack[len(stack)-1]
		}
		stack = append(stack, i)
	}

	stack = stack[0:0]
	right := make([]int, n)
	for i := n - 1; i >= 0; i-- {
		for len(stack) > 0 && arr[stack[len(stack)-1]] > arr[i] {
			stack = stack[0 : len(stack)-1]
		}
		if len(stack) == 0 {
			right[i] = n - i
		} else {
			right[i] = stack[len(stack)-1] - i
		}
		stack = append(stack, i)
	}

	res := 0
	for i, v := range arr {
		res = (res + left[i]*right[i]*v) % MOD
	}
	return res
}
