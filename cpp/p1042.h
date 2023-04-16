#include <vector>

using namespace std;

class Solution {
public:
    vector<int> gardenNoAdj(int n, vector<vector<int>>& paths) {
        vector<vector<int>> adj(n);
        for (vector<int> path : paths) {
            adj[path[0]-1].emplace_back(path[1]-1);
            adj[path[1]-1].emplace_back(path[0]-1);
        }
        vector<int> res(n);
        for (int i = 1; i <= n; i++) {
            vector<bool> colored(5, false);
            for (auto aj : adj[i-1]) {
                colored[res[aj]] = true;
            }
            for (int j = 1; j <= 4; j++) {
                if (!colored[j]) {
                    res[i-1] = j;
                    break;
                }
            }
        }
        return res;
    }
};