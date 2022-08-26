package leetcode

func findNumberIn2DArray(matrix [][]int, target int) bool {
	search := func(arr []int, target int) bool {
		l, r := 0, len(arr)-1
		for l <= r {
			mid := (l + r) / 2
			if arr[mid] == target {
				return true
			} else if arr[mid] > target {
				r = mid - 1
			} else {
				l = mid + 1
			}
		}
		return false
	}
	for _, v := range matrix {
		if search(v, target) {
			return true
		}
	}
	return false
}
