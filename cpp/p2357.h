#include <vector>

using namespace std;

class Solution {
public:
    int minimumOperations(vector<int>& nums) {
        sort(nums.begin(), nums.end());
        int sum = 0, res = 0, i = 0;
        for (; i < nums.size(); i++) {
            while (i < nums.size() && nums[i] == 0) i++;
            if (i >= nums.size()) {
                return res;
            }
            if (nums[i] <= sum) {
                continue;
            }
            sum += nums[i] - sum;
            res++;
            if (i == nums.size() || sum >= nums[nums.size() - 1]) {
                return res;
            }
        }
        return res;
    }
};