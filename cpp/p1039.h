#include <vector>
#include <unordered_map>

using namespace std;

class Solution {
private:
    vector<int> values;
    unordered_map<int, int> memo;

public:
    int minScoreTriangulation(vector<int>& values) {
        this->values = values;
        return dp(0, values.size()-1);
    }

    int dp(int i, int j) {
        if (i + 2 > j) {
            return 0;
        }
        if (i + 2 == j) {
            return values[i] * values[i+1] * values[j];
        }
        int key = i * values.size() + j;
        if (!memo.count(key)) {
            int minScore = INT_MAX;
            for (int k = i+1; k < j; k++) {
                minScore = min(minScore, values[i] * values[k] * values[j] + dp(i, k) + dp(k, j));
            }
            memo[key] = minScore;
        }
        return memo[key];
    }

};