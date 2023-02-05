#include <string> 
#include <unordered_map>

using namespace std;

class Solution {
public:
    string decodeMessage(string key, string message) {
        unordered_map<char, char> map;
        char c = 'a';
        for (char ch : key) {
            if (ch == ' ' || map.count(ch) > 0) {
                continue;
            }
            map.insert({ch, c++});
        }
        string res;
        for (char ch : message) {
            if (ch != ' ') {
                ch = map[ch];
            }
            res.push_back(ch);
        }
        return res;
    }
};