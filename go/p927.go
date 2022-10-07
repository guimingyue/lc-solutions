package leetcode

func threeEqualParts(arr []int) []int {
	sum := 0
	for _, v := range arr {
		sum += v
	}
	if sum%3 != 0 {
		return []int{-1, -1}
	}
	if sum == 0 {
		return []int{0, 2}
	}
	partial := sum / 3
	var first, second, third, cur int
	for i, v := range arr {
		if v == 1 {
			if cur == 0 {
				first = i
			} else if cur == partial {
				second = i
			} else if cur == 2*partial {
				third = i
			}
			cur++
		}
	}
	length := len(arr) - third
	if first+length <= second && second+length <= third {
		for i := 0; third+i < len(arr); i++ {
			if arr[first+i] != arr[second+i] || arr[first+i] != arr[third+i] {
				return []int{-1, -1}
			}
		}
		return []int{first + length - 1, second + length}
	}
	return []int{-1, -1}
}
