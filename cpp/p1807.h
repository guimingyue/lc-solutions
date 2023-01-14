#include <string>
#include <vector>
#include <unordered_map>

using namespace std;

class Solution {
public:
    string evaluate(string s, vector<vector<string>>& knowledge) {
        unordered_map<string, string> map;
        for (auto v : knowledge) {
            map.insert({v[0], v[1]});
        }
        string result;
        bool begin = false;
        string key;
        for (auto ch : s) {
            if(ch == '(') {
                begin = true;
            } else if (ch == ')') {
                if (map.count(key) > 0) {
                    result.append(map[key]);
                } else {
                    result.push_back('?');
                }
                key.clear();
                begin = false;
            } else if (begin) {
                key.push_back(ch);
            } else {
                result.push_back(ch);
            }
        }
        return result;
    }
};