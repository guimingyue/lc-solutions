#include <vector>
#include <unordered_map>
#include <unordered_set>

using namespace std;

class Solution {
public:
    vector<int> findingUsersActiveMinutes(vector<vector<int>>& logs, int k) {
        unordered_map<int, unordered_set<int>> map;
        for (auto log : logs) {
            if (map.count(log[0]) <= 0) {
                unordered_set<int> set;
                map[log[0]] = set;
            }
            map[log[0]].insert(log[1]);
        }
        vector<int> res(k, 0);
        for (auto entry : map) {
            res[entry.second.size()-1] += 1;
        }
        return res;
    }
};