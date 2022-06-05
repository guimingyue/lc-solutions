func findKthPositive(arr []int, k int) int {
	var s = make([]int, len(arr))
	for i, v := range arr {
		s[i] = v - i - 1
	}
	var i, j = 0, len(s) - 1
	// 有相等的，就找最最侧的，否则就找到应该插入的位置
	// 1,1,1,3,3,5
	for i < j {
		mid := (i + j) / 2
		if  s[mid] < k {
			i = mid + 1
		} else {
			j = mid
		}
	}
	if s[i] >= k {
		return arr[i] - (s[i] - k + 1)
	} else {
		return arr[i] - (s[i] - k)
	}
}
