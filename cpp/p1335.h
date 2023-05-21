#include <vector>

using namespace std;

class Solution {
public:
    int minDifficulty(vector<int>& jobDifficulty, int d) {
        int n = jobDifficulty.size();
        if (jobDifficulty.size() < d) {
            return -1;
        }
        vector<vector<int>> dp(d, vector(n, 300000));
        int maxv = 0;
        for (int i = 0; i < n; i++) {
            maxv = max(maxv, jobDifficulty[i]);
            dp[0][i] = maxv;
        }
        for (int i = 1; i < d; i++) {
            for (int j = i; j < n; j++) {
                int m = 0;
                for (int k = j; k >= i; k--) {
                    m = max(m, jobDifficulty[k]);
                    dp[i][j] = min(dp[i-1][k-1] + m, dp[i][j]);
                }
            }
        }
        return dp[d-1][n-1];
    }
};