#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    bool checkDistances(string s, vector<int>& distance) {
        vector<int> d(26, -1);
        for (int i = 0; i < s.size(); i++) {
            char ch = s[i];
            int idx = ch - 'a';
            if (d[idx] == -1) {
                d[idx] = i;
            } else {
                if (distance[idx] != i - d[idx] - 1) {
                    return false;
                }
            }
        }
        return true;
    }
};