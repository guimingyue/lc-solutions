func asteroidCollision(asteroids []int) []int {
    res := make([]int, 0)
	for _, aster := range asteroids {
		alive := true
		for alive && aster < 0 && len(res) > 0 && res[len(res)-1] > 0 {
			alive = res[len(res)-1] < -aster
			if res[len(res)-1] <= -aster {
				res = res[0 : len(res)-1]
			}
		}
		if alive {
			res = append(res, aster)
		}
	}
	return res
}