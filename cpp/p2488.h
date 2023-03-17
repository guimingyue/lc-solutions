#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
public:
    int countSubarrays(vector<int>& nums, int k) {
        unordered_map<int, int> map;
        map[0] = 1;
        int sum = 0;
        int res = 0;
        bool passed = false;
        for (int i = 0; i < nums.size(); i++) {
            sum += nums[i] == k ? 0 : nums[i] > k ? 1 : -1;
            if (nums[i] == k) {
                passed = true;
            }
            if (!passed) {
                map[sum]++;
            } else {
                res += map[sum] + map[sum-1];
            }
        }
        return res;
    }
};