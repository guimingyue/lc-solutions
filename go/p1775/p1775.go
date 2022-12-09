package p1775

import "sort"

func minOperations(nums1 []int, nums2 []int) int {
	sum := func(nums []int) (min, max, sum int) {
		min, max, sum = len(nums), 6*len(nums), 0
		for _, v := range nums {
			sum += v
		}
		return
	}
	min1, max1, sum1 := sum(nums1)
	min2, max2, sum2 := sum(nums2)
	if sum1 == sum2 {
		return 0
	}
	if min1 > max2 || min2 > max1 {
		return -1
	}
	if sum1 < sum2 {
		sum1, sum2 = sum2, sum1
		nums1, nums2 = nums2, nums1
	}
	sort.Ints(nums1)
	sort.Ints(nums2)
	i1, i2 := len(nums1)-1, 0
	diff := sum1 - sum2
	res := 0
	for i1 >= 0 || i2 < len(nums2) {
		res++
		var d1, d2 int
		if i1 >= 0 && i2 < len(nums2) {
			d1 = nums1[i1] - 1
			d2 = 6 - nums2[i2]
		} else if i1 < 0 {
			d1 = 0
			d2 = 6 - nums2[i2]
		} else {
			d1 = nums1[i1] - 1
			d2 = 0
		}

		if d1 >= diff || d2 >= diff {
			return res
		} else {
			if d1 > d2 {
				diff -= d1
				i1--
			} else {
				diff -= d2
				i2++
			}
		}
	}
	return -1
}
