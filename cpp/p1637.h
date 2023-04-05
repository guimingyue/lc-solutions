#include <vector>

using namespace std;

struct Cmp {
    bool operator ()(const vector<int> &a, vector<int> &b){
        return a[0] < b[0];
    }
};

class Solution {
public:
    int maxWidthOfVerticalArea(vector<vector<int>>& points) {
        sort(points.begin(), points.end(), Cmp());
        int d = 0;
        for (int i = 1; i < points.size(); i++) {
            d = max(d, points[i][0] - points[i-1][0]);
        }
        return d;
    }
};