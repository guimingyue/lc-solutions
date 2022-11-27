package leetcode

func champagneTower(poured int, query_row int, query_glass int) float64 {
	rows := []float64{float64(poured)}
	for i := 1; i <= query_row; i++ {
		nextRow := make([]float64, i+1)
		for j, k := range rows {
			if k > 1 {
				nextRow[j] += (k - 1) / 2
				nextRow[j+1] += (k - 1) / 2
			}
		}
		rows = nextRow
	}
	if rows[query_glass] < 1 {
		return rows[query_glass]
	}
	return 1
}
