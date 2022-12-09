package leetcode

func squareIsWhite(coordinates string) bool {
	square := [][]bool{
		{false, true, false, true, false, true, false, true},
		{true, false, true, false, true, false, true, false},
		{false, true, false, true, false, true, false, true},
		{true, false, true, false, true, false, true, false},
		{false, true, false, true, false, true, false, true},
		{true, false, true, false, true, false, true, false},
		{false, true, false, true, false, true, false, true},
		{true, false, true, false, true, false, true, false},
	}
	return square[int(coordinates[0])-int('a')][int(coordinates[1])-int('1')]
}
