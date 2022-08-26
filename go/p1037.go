package leetcode

func isBoomerang(points [][]int) bool {
	if equals(points[0], points[1]) || equals(points[1], points[2]) || equals(points[0], points[2]) {
		return false
	}
	y21 := points[2][1] - points[1][1]
	x21 := points[2][0] - points[1][0]

	y10 := points[1][1] - points[0][1]
	x10 := points[1][0] - points[0][0]

	if x10 == 0 || x21 == 0 {
		return !(x10 == x21)
	} else {
		return !(float32(y21)/float32(x21) == float32(y10)/float32(x10))
	}
}

func equals(p1 []int, p2 []int) bool {
	return p1[0] == p2[0] && p1[1] == p2[1]
}
