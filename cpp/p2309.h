#include <string>

using namespace std;

class Solution {
public:
    string greatestLetter(string s) {
        bool little[26] = {false}, upper[26] = {false};
        for (auto ch : s) {
            if (ch >= 'a' && ch <= 'z') {
                little[ch-'a'] = true;
            } else {
                upper[ch -'A'] = true;
            }
        }
        string res;
        for (int i = 25; i >= 0; i--) {
            if (little[i] && upper[i]) {
                res.push_back('A' + i);
                break;
            }
        }
        return res;
    }
};