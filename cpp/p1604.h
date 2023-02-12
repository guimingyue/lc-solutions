#include <vector>
#include <string>
#include <unordered_map>
#include <set>

using namespace std;

class Solution {
public:
    vector<string> alertNames(vector<string>& keyName, vector<string>& keyTime) {
        unordered_map<string, set<string>> map;
        for (int i = 0; i < keyName.size(); i++) {
            string name = keyName[i];
            string time = keyTime[i];


            if (map.count(name) <= 0) {
                set<string> s;
                s.insert(time);
                map.insert({name, s});
            } else {
                map[name].insert(time);
            }
        }
        vector<string> res;
        for (auto [key, value] : map) {
            if (value.size() < 3) {
                continue;
            }

            auto it = value.begin();
            string pre0 = *it;
            it++;
            string pre1 = *it;
            it++;
            while (it != value.end()) {
                auto v = *it;
                if (isSameHour(pre0, v) && isSameHour(pre1, v)) {
                    res.push_back(key);
                    break;
                }
                pre0 = pre1;
                pre1 = v;
                it++;
            }
        }
        sort(res.begin(), res.end());
        return res;
    }

    bool isSameHour(string a, string b) {
        auto ah = std::atoi(a.substr(0, 2).c_str());
        auto am = std::atoi(a.substr(3, 2).c_str());
        auto bh = std::atoi(b.substr(0, 2).c_str());
        auto bm = std::atoi(b.substr(3, 2).c_str());
        if (bh < ah || (bh == ah && bm < am)) {
            return false;
        }
        if ((bh - ah == 0 && bm - am > 0) || (bh - ah >= 0 && bh - ah <= 1 && bm - am <= 0)) {
            return true;
        }
        return false;
    }
};