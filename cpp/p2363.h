#include <vector>
#include <map>

using namespace std;

class Solution {
public:
    vector<vector<int>> mergeSimilarItems(vector<vector<int>>& items1, vector<vector<int>>& items2) {
        map<int, int> map;
        for (auto item : items1) {
            map.insert({item[0], item[1]});
        }
        for(auto item: items2) {
            map[item[0]] += item[1];
        }
        vector<vector<int>> res;
        for (auto [k, v] : map) {
            res.push_back({k, v});
        }
        return res;
    }
};