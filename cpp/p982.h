#include <vector>

using namespace std;

class Solution {
public:
    int countTriplets(vector<int>& nums) {
        vector<int> vec(1 << 16);
        for (auto x : nums) {
            for (auto y : nums) {
                vec[x&y]++;
            }
        }
        int res = 0;
        for (auto k : nums) {
            for (int mask = 0; mask < (1 << 16); mask++) {
                if ((mask & k) == 0) {
                    res += vec[mask];
                }
            }
        }
        return res;
    }
};