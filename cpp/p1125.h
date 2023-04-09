#include <vector>
#include <unordered_map>
#include <string>

using namespace std;

class Solution {
public:
    vector<int> smallestSufficientTeam(vector<string>& req_skills, vector<vector<string>>& people) {
        int n = req_skills.size(), m = people.size();
        unordered_map<string, int> skills;
        for (int i = 0; i < n; i++) {
            skills[req_skills[i]] = i;
        }
        vector<vector<int>> dp(1 << n);
        for (int i = 0; i < m; i++) {
            int cur_skill = 0;
            for (auto sk : people[i]) {
                cur_skill = cur_skill | 1 << skills[sk];
            }
            for (int prev = 0; prev < dp.size(); prev++) {
                if (prev > 0 && dp[prev].empty()) {
                    continue;
                }
                int comb = prev | cur_skill;
                if (comb == prev) {
                    continue;
                }
                if (dp[comb].empty() || dp[prev].size() + 1 < dp[comb].size()) {
                    dp[comb] = dp[prev];
                    dp[comb].push_back(i);
                }
            }
        }
        return dp[(1 << n) - 1];
    }
};