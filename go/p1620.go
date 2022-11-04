package leetcode

import "math"

func bestCoordinate(towers [][]int, radius int) []int {
	xmin, ymin, xmax, ymax := math.MaxInt32, math.MaxInt32, 0, 0
	for _, tower := range towers {
		if xmin > tower[0] {
			xmin = tower[0]
		}
		if ymin > tower[1] {
			ymin = tower[1]
		}
		if xmax < tower[0] {
			xmax = tower[0]
		}
		if ymax < tower[1] {
			ymax = tower[1]
		}
	}
	max := 0
	res := []int{0, 0}

	x := xmin - radius
	if x < 0 {
		x = 0
	}
	for ; x <= xmax+radius; x++ {
		y := ymin - radius
		if y < 0 {
			y = 0
		}
		for ; y <= ymax+radius; y++ {
			v := 0
			for _, tower := range towers {
				p := (tower[0]-x)*(tower[0]-x) + (tower[1]-y)*(tower[1]-y)
				d := math.Sqrt(float64(p))
				if d > float64(radius) {
					continue
				}
				v += int(float64(tower[2]) / (1 + d))
			}
			if v > max {
				max = v
				res = []int{x, y}
			}
		}
	}
	return res
}
