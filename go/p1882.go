package leetcode

func arraySign(nums []int) int {
	n, z := 0, 0
	for _, v := range nums {
		if v < 0 {
			n++
		} else if v == 0 {
			z = 1
		}
	}
	if z > 0 {
		return 0
	} else if n%2 == 1 {
		return -1
	} else {
		return 1
	}
}
