#include <vector>
#include <unordered_map>
#include <unordered_set>

using namespace std;

class Solution {
public:
    int numPairsDivisibleBy60(vector<int>& time) {
        unordered_map<int, long> map;
        unordered_set<int> visited;
        for (auto t : time) {
            map[t]++;
        }
        long res = 0;
        for (auto it = map.begin(); it != map.end(); it++) {
            visited.insert(it->first);
            for (int v = 60; v <= 1000; v += 60) {
                if (it->first > v) {
                    continue;
                }
                int diff = v - it->first;
                if (map.count(diff) <= 0) {
                    continue;
                }
                if (diff == it->first && it->second > 1) {
                    res += it->second * (it->second - 1) / 2;
                }  else if (diff != it->first && visited.count(diff) <= 0) {
                    res += it->second * map[diff];
                }
            }
        }
        return res;
    }
};