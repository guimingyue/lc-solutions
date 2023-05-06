#include <vector>

using namespace std;

class Solution {
public:
    int hardestWorker(int n, vector<vector<int>>& logs) {
        int maxi = logs[0][0];
        int maxl = logs[0][1];
        for (int i = 1; i < logs.size(); i++) {
            int time = logs[i][1] - logs[i-1][1];
            if (time > maxl) {
                maxi = logs[i][0];
                maxl = time;
            } else if (time == maxl) {
                maxi = min(maxi, logs[i][0]);
            }
        }
        return maxi;
    }
};