package leetcode

func minEatingSpeed(piles []int, h int) int {
	max := piles[0]
	sum := 0
	size := len(piles)
	for i := 0; i < size; i++ {
		sum += piles[i]
		if piles[i] > max {
			max = piles[i]
		}
	}
	min := sum / h
	if sum%h > 0 {
		min += 1
	}

	l := min
	r := max
	for l < r {
		mid := (l + r) / 2
		time := 0
		for i := 0; i < size; i++ {
			time += piles[i] / mid
			if piles[i]%mid > 0 {
				time += 1
			}
		}
		if time > h {
			l = mid + 1
		} else {
			r = mid
		}
	}

	return l
}
