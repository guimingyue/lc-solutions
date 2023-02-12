#include <string>
#include <vector>
#include <unordered_set>

using namespace std;

struct Cmp {
    bool operator ()(const string &a, string &b){
        return a.length() < b.length();
    }
};

class Solution {
public:
    vector<string> removeSubfolders(vector<string>& folder) {
        vector<string> res;
        sort(folder.begin(), folder.end(), Cmp());
        unordered_set<string> set(folder.begin(), folder.end());
        for (int i = folder.size() - 1; i >= 0; i--) {
            string s = folder[i];
            set.erase(s);
            int len = s.length();
            bool hasSub = false;
            for (int j = len; j > 1 ; j--) {
                if (s[j-1] != '/') {
                    continue;
                }
                if (set.count(s.substr(0, j-1)) > 0) {
                    hasSub = true;
                    break;
                }
            }
            if (!hasSub) {
                res.push_back(s);
            }
        }
        return res;
    }
};