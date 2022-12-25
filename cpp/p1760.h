#include <vector>

using namespace std;

class Solution {
public:
    int minimumSize(vector<int>& nums, int maxOperations) {
        int left = 1, right = *max_element(nums.begin(), nums.end());
        int res = 0;
        while (left <= right) {
            int y = (left + right) / 2;
            int x = 0;
            for (auto num : nums) {
                x += (num - 1) / y;
            }
            if (x <= maxOperations) {
                res = y;
                right = y-1;
            } else {
                left = y + 1;
            }
        }
        return res;
    }
};