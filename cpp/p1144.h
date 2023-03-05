#include <vector>

using namespace std;

class Solution {
public:
    int movesToMakeZigzag(vector<int>& nums) {
        return min(zigzagNums(nums, 0), zigzagNums(nums, 1));
    }

    int zigzagNums(vector<int>& nums, int start) {
        int res = 0;
        int val = start == 1 ? nums[0] : 0;
        for (int i = start; i < nums.size(); i += 2) {
            if (i - 1 >= 0) {
                if (nums[i] <= val) {
                    res += val - nums[i]+ 1;
                }
            }
            if (i +1 < nums.size()) {
                if (nums[i] <= nums[i+1]) {
                    res += nums[i+1] - nums[i] + 1;
                    val = nums[i] - 1;
                } else {
                    val = nums[i+1];
                }
            }
        }
        return res;
    }
};