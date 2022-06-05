type Solution struct {
	radius, x, y float64
}

func Constructor(radius float64, x_center float64, y_center float64) Solution {
	return Solution{radius, x_center, y_center}
}

func (this *Solution) RandPoint() []float64 {
	for {
		x := rand.Float64()*2 - 1
		y := rand.Float64()*2 - 1
		if x*x+y*y <= 1 {
			return []float64{x*this.radius + this.x, y*this.radius + this.y}
		}
	}
}