package leetcode

func minCostToMoveChips(position []int) int {
	abs := func(a int, b int) int {
		d := a - b
		if d < 0 {
			return -d
		}
		return d
	}
	move := func(positions []int, pos int) int {
		res := 0
		for i := 0; i < len(positions); i++ {
			if abs(position[i], pos)%2 != 0 {
				res += 1
			}
		}
		return res
	}
	m1 := move(position, 0)
	m2 := move(position, 1)
	if m1 < m2 {
		return m1
	} else {
		return m2
	}
}
