package leetcode

func numSpecial(mat [][]int) int {
	res := 0
	for i := 0; i < len(mat); i++ {
		for j := 0; j < len(mat[0]); j++ {
			if mat[i][j] == 0 {
				continue
			}
			allZero := true
			for k := 0; k < len(mat[0]); k++ {
				if k == j {
					continue
				}
				if mat[i][k] == 1 {
					allZero = false
				}
			}

			for k := 0; k < len(mat); k++ {
				if k == i {
					continue
				}
				if mat[k][j] == 1 {
					allZero = false
				}
			}
			if allZero {
				res++
			}
		}
	}
	return res
}
