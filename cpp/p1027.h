#include <vector>

using namespace std;

class Solution {
public:
    int longestArithSeqLength(vector<int>& nums) {
        auto [minit, maxit] = minmax_element(nums.begin(), nums.end());
        int diff = *maxit - *minit;
        int res = 1;
        for (int d = -diff; d <= diff; d++) {
            vector<int> dp(*maxit + 1, -1);
            for (int num : nums) {
                int pre = num - d;
                if (pre >= *minit && pre <= *maxit && dp[pre] > -1) {
                    dp[num] = max(dp[num], dp[pre] + 1);
                    res = max(res, dp[num]);
                }
                dp[num] = max(dp[num], 1);
            }
        }
        return res;
    }
};