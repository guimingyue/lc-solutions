/** 
 * Forward declaration of isBadVersion API.
 * @param   version   your guess about first bad version
 * @return 	 	      true if current version is bad 
 *			          false if current version is good
 * func isBadVersion(version int) bool;
 */

 func firstBadVersion(n int) int {
	var i, j = 1, n
	for i < j {
		mid := i + (j - i) / 2
		if isBadVersion(mid) {
			j = mid
		} else {
			i = mid + 1
		}
	}
	return i
}

func isBadVersion(int version)