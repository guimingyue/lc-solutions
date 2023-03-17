#include <vector>

using namespace std;

class Solution {
public:
    int maximalNetworkRank(int n, vector<vector<int>>& roads) {
        vector<vector<int>> connect(n, vector<int>(n, false));
        vector<int> degree(n, 0);
        for (auto road : roads) {
            connect[road[0]][road[1]] = true;
            connect[road[1]][road[0]] = true;
            degree[road[0]]++;
            degree[road[1]]++;
        }
        int res = 0;
        for(int i = 0; i < n; i++) {
            for (int j = i+1; j < n; j++) {
                int rank = degree[i] + degree[j] + (connect[i][j] ? -1 : 0);
                res = max(res, rank);
            }
        }
        return res;
    }
};