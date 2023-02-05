#include <vector>

using namespace std;

class Solution {
public:
    vector<int> countPoints(vector<vector<int>>& points, vector<vector<int>>& queries) {
        vector<int> answer(queries.size(), 0);
        for (int i = 0; i < queries.size(); i++) {
            auto query = queries[i];
            for (auto point : points) {
                int x = point[0] - query[0];
                int y = point[1] - query[1];
                int r = query[2];
                if (x * x + y * y <= r * r) {
                    answer[i]++;
                }
            }
        }
        return answer;
    }
};