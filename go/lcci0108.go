package leetcode

func setZeroes(matrix [][]int) {
	lx := len(matrix)
	ly := len(matrix[0])
	mem := make([][]int, 0)
	for i, x := range matrix {
		for j, y := range x {
			if y == 0 {
				mem = append(mem, []int{i, j})
			}
		}
	}

	for _, m := range mem {
		for k := 0; k < ly; k++ {
			matrix[m[0]][k] = 0
		}
		for p := 0; p < lx; p++ {
			matrix[p][m[1]] = 0
		}
	}
}
