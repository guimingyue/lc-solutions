package leetcode

func busyStudent(startTime []int, endTime []int, queryTime int) int {
	res := 0
	for i := 0; i < len(startTime); i++ {
		s := startTime[i]
		e := endTime[i]
		if e >= queryTime && s <= queryTime {
			res++
		}
	}
	return res
}
