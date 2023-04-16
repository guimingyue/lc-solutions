#include <string>
#include <vector>

using namespace std;

class Solution {
public:
    vector<bool> camelMatch(vector<string>& queries, string pattern) {
        vector<bool> res(queries.size(), false);
        for (int i = 0; i < queries.size(); i++) {
            string s = queries[i];
            int x = 0, y = 0;
            while (x < s.size() || y < pattern.size()) {
                if (x < s.size() && y < pattern.size()) {
                    if (s[x] == pattern[y]) {
                        x++;
                        y++;
                    } else if (islower(s[x])) {
                        x++;
                    } else {
                        break;
                    }
                } else if (x < s.size() && y == pattern.size()) {
                    if (islower(s[x])) {
                        x++;
                    } else {
                        break;
                    }
                } else if (x == s.size() && y < pattern.size()) {
                    break;
                }
            }
            if (x == s.size() && y == pattern.size()) {
                res[i] = true;
            }
        }
        return res;
    }
};