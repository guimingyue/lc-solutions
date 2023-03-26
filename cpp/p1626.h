#include <vector>

using namespace std;

class Solution {
public:
    int bestTeamScore(vector<int>& scores, vector<int>& ages) {
        vector<pair<int, int>> people;
        for (int i = 0; i < scores.size(); i++) {
            people.push_back({scores[i], ages[i]});
        }
        sort(people.begin(), people.end());
        vector<int> dp(scores.size(), 0);
        int res = 0;
        for (int i = 0; i < people.size(); i++) {
            for (int j = i-1; j >= 0; j--) {
                if (people[i].second >= people[j].second) {
                    dp[i] = max(dp[i], dp[j]);
                }
            }
            dp[i] += people[i].first;
            res = max(res, dp[i]);
        }
        return res;
    }
};