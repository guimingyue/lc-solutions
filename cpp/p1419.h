#include <string>
#include <unordered_map>

using namespace std;

class Solution {
public:
    int minNumberOfFrogs(string croakOfFrogs) {
        unordered_map<string, int> map;
        int res = 0, num = 0;
        for (char ch : croakOfFrogs) {
            if (ch == 'c') {
                map["c"]++;
                num++;
                res = max(res, num);
            } else if (ch == 'r') {
                if (map["c"] == 0) {
                    return -1;
                }
                map["c"]--;
                map["cr"]++;
            } else if (ch == 'o') {
                if (map["cr"] == 0) {
                    return -1;
                }
                map["cr"]--;
                map["cro"]++;
            } else if (ch == 'a') {
                if (map["cro"] == 0) {
                    return -1;
                }
                map["cro"]--;
                map["croa"]++;
            } else {
                if (map["croa"] == 0) {
                    return -1;
                }
                map["croa"]--;
                num--;
            }
        }
        if (map["c"] != 0 || map["cr"] != 0 || map["cro"] != 0 || map["croa"] != 0) {
            return -1;
        }
        return res;
    }
};