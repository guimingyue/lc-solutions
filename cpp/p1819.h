#include <vector>

using namespace std;

class Solution {
public:
    int countDifferentSubsequenceGCDs(vector<int>& nums) {
        int maxVal = *max_element(nums.begin(), nums.end());
        vector<bool> occured(maxVal + 1, false);
        for (int num : nums) {
            occured[num] = true;
        }
        int res = 0;
        for (int i = 1; i <= maxVal; i++) {
            int subGcd = 0;
            for (int j = i; j <= maxVal; j += i) {
                if (!occured[j]) {
                    continue;
                }
                if (subGcd == 0) {
                    subGcd = j;
                } else {
                    subGcd = gcd(subGcd, j);
                }
                if (subGcd == i) {
                    res++;
                    break;
                }
            }
        }
        return res;
    }
};