#include <vector>
#include <unordered_map>

using namespace std;

class Solution {
public:
    int arithmeticTriplets(vector<int>& nums, int diff) {
        unordered_map<int, int> map;
        for (int i = 0; i < nums.size(); i++) {
            map.insert({nums[i], i});
        }
        int res = 0;
        for (int i = 0; i < nums.size(); i++) {
            if (map.count(nums[i] + diff) > 0 && map.count(nums[i] + diff + diff) > 0) {
                res++;
            }
        }
        return res;
    }
};