#include <vector>

using namespace std;

class Solution {
     static constexpr int inf = 0x3f3f3f3f;
public:
    int minSideJumps(vector<int>& obstacles) {
        int n = obstacles.size() - 1;
        vector<vector<int>> dp(n+1, vector<int>(3));
        dp[0][0] = 1, dp[0][1] = 0, dp[0][2] = 1;
        for (int i = 1; i <= n; i++) {
            int minCost = inf;
            for (int j = 0; j < 3; j++) {
                if (obstacles[i] - 1 == j) {
                    dp[i][j] = inf;
                } else {
                    minCost = min(dp[i-1][j], minCost);
                }
            }
            for (int j = 0; j < 3; j++) {
                if (obstacles[i] - 1 == j) {
                    continue;
                }
                dp[i][j] = min(dp[i-1][j], minCost + 1);
            }
        }

        return min(min(dp[n][0], dp[n][1]),dp[n][2]);
    }
};