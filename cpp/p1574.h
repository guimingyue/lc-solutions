#include <vector>

using namespace std;

class Solution {
public:
    int findLengthOfShortestSubarray(vector<int>& arr) {
        int n = arr.size(), j = n - 1;
        for (; j > 0; j--) {
            if (arr[j-1] > arr[j]) {
                break;
            }
        }
        if (j == 0) {
            return 0;
        }
        int i = 0, res = j;
        for (; i < n; i++) {
            while (j < n && arr[i] > arr[j]) {
                j++;
            }
            res = min(res, j - i - 1);
            if (i + 1 < n && arr[i] > arr[i+1]) {
                break;
            }
        }
        return res;
    }
};