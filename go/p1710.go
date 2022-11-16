package leetcode

import "sort"

func maximumUnits(boxTypes [][]int, truckSize int) int {
	sort.Slice(boxTypes, func(i, j int) bool {
		return boxTypes[i][1] >= boxTypes[j][1]
	})
	res := 0
	num := 0
	for _, box := range boxTypes {
		if num+box[0] < truckSize {
			res += box[0] * box[1]
			num += box[0]
		} else {
			res += (truckSize - num) * box[1]
			break
		}
	}
	return res
}
