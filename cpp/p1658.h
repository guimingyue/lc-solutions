#include <vector>
#include <unordered_map>

using namespace std;

class Solution {
public:
    int minOperations(vector<int>& nums, int x) {
        unordered_map<int, int> map;
        int sum = 0;
        for (int i = -1; i < (int)nums.size(); i++) {
            if (i >= 0) {
                sum += nums[i];
            }
            map.insert({sum, i});
        }
        if (sum < x) {
            return -1;
        }
        int rsum = 0;
        int res = nums.size() + 1;
        for (int k = nums.size(); k >= 0; k--) {
            if (k < nums.size()) {
                rsum += nums[k];
            }
            if (rsum > x) {
                break;
            }
            int v = x - rsum;
            if (map.count(v) > 0) {
                res = min(res, map[v] + 1 + (int)nums.size() - k);
            }
        }
        return res > nums.size() ? -1 : res;
    }
};