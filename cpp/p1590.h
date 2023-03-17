#include <vector>
#include <unordered_map>

using namespace std;


class Solution {
public:
    int minSubarray(vector<int>& nums, int p) {
        int x = 0;
        for (auto v : nums) {
            x = (x + v) % p;
        }
        if (x == 0) {
            return 0;
        }
        int res = nums.size();
        int y = 0;
        unordered_map<int, int> map;
        for (int i = 0; i < nums.size(); i++) {
            map[y] = i;
            y = (y + nums[i]) % p;
            if (map.count((y - x + p) % p) > 0) {
                res = min(res, i - map[(y - x + p) % p] + 1);
            }
        }
        return res == nums.size() ? -1 : res;
    }
};