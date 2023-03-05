#include <vector>

using namespace std;

class Solution {
public:
    vector<vector<int>> largestLocal(vector<vector<int>>& grid) {
        int n = grid.size();
        vector<vector<int>> res(n-2, vector<int>(n-2));
        for (int i = 0; i < n-2; i++) {
            for (int j = 0; j < n-2; j++) {
                int val = grid[i][j];
                for (int k = 0; k < 3; k++) {
                    for (int p = 0; p < 3; p++) {
                        val = max(val, grid[i+k][j+p]);
                    }
                }
                res[i][j] = val;
            }
        }
        return res;
    }
};