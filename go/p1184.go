func distanceBetweenBusStops(distance []int, start int, destination int) int {
    d1 := 0
	s := start
	for s != destination {
		d1 += distance[s]
		s++
		if s == len(distance) {
			s = 0
		}
	}
	d2 := 0
	s = start
	for s != destination {
		s--
		if s == -1 {
			s = len(distance) - 1
		}
		d2 += distance[s]
	}
	if d1 > d2 {
		return d2
	} else {
		return d1
	}
}