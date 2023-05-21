#include <vector>
#include <unordered_map>

using namespace std;

class Solution {
public:
    int maxEqualRowsAfterFlips(vector<vector<int>>& matrix) {
        int m = matrix.size(), n = matrix[0].size();
        unordered_map<string, int> mp;
        for (int i = 0; i < m; i++) {
            string s;
            for (int j = 0; j < n; j++) {
                char ch = '0' + (matrix[i][0] ^ matrix[i][j]);
                s.push_back(ch);
            }
            mp[s]++;
        }
        int res = 0;
        for (auto &[k, v] : mp) {
            res = max(res, v);
        }
        return res;
    }
};