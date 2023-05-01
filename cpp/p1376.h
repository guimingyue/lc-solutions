#include <vector>

using namespace std;

class Solution {
public:
    int numOfMinutes(int n, int headID, vector<int>& manager, vector<int>& informTime) {
        int res = 0;
        vector<bool> visited(n, false);
        for (int i = 0; i < n; i++) {
            if (visited[i]) {
                continue;
            }
            visited[i] = true;
            int time = 0;
            int m = manager[i];
            while (m != -1) {
                time += informTime[m];
                visited[m] = true;
                m = manager[m];
            }
            res = max(res, time);
        }
        return res;
    }
};