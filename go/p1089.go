package leetcode

func duplicateZeros(arr []int) {
	cnt := 0
	lastIdx := 0
	for idx, v := range arr {
		if v == 0 {
			cnt += 1
		}
		if idx+cnt >= len(arr)-1 {
			lastIdx = idx
			break
		}
	}
	if cnt == 0 {
		return
	}

	p1, p2 := len(arr)-1, lastIdx
	if lastIdx+cnt >= len(arr) {
		arr[p1] = arr[p2]
		p2--
		p1--
	}
	for p1 >= 0 {
		arr[p1] = arr[p2]
		if arr[p2] == 0 {
			p1--
			arr[p1] = arr[p2]
		}
		p1--
		p2--
	}
}
