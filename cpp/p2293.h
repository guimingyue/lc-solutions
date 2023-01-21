#include <vector>

using namespace std;

class Solution {
public:
    int minMaxGame(vector<int>& nums) {
        while (nums.size() > 1) {
            int newLen = nums.size() / 2;
            vector<int> newNums(newLen, 0);
            for (int i = 0; i < newLen; i++) {
                if (i % 2 == 0) {
                    newNums[i] = min(nums[2 * i], nums[2 * i + 1]);
                } else {
                    newNums[i] = max(nums[2 * i], nums[2 * i + 1]);
                }
            }
            nums = (vector<int>&) newNums;

        }   
        return nums[0]; 
    }


};