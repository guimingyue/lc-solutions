#include <vector>
#include <queue>

using namespace std;

class Solution {
public:
    struct Ratio {
        int pass, total;
        bool operator < (const Ratio &oth) const {
            return (long long) (oth.total + 1) * oth.total * (total - pass) < (long long) (total + 1) * total * (oth.total - oth.pass);
        }
    };
    double maxAverageRatio(vector<vector<int>>& classes, int extraStudents) {
        priority_queue<Ratio> queue;
        for (auto &a : classes) {
            queue.push({a[0], a[1]});
        }

        while (extraStudents-- > 0) {
            auto [pass, total] = queue.top();
            queue.pop();
            queue.push({pass + 1, total+1});
        }
        double res = 0;
        while (!queue.empty()) {
            auto [p, t] = queue.top();
            queue.pop();
            res += 1.0 * p / t;
        }
        return res / classes.size();
    }
};