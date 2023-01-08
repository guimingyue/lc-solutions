class Solution {
public:
    int maxValue(int n, int index, int maxSum) {
        int avg = maxSum / n;
        int left = avg, right = maxSum;
        while (left < right) {
            long mid = (left + right + 1) / 2;
            long sum = 0;
            if (mid - index > 0) {
                sum += (mid - 1 + mid - index) * (index) / 2;
            } else {
                sum += (1 + mid - 1) * (mid - 1) / 2 + (index - (mid-1));
            }
            int idx = n - index;
            if (mid - idx > 0) {
                sum += (mid + mid - idx + 1) * idx / 2;
            } else {
                sum += (1 + mid) * mid / 2 + (idx - mid);
            }

            if (sum > maxSum) {
                right = mid - 1;
            } else {
                left = mid;
            }
        }
        return left;
    }
};
