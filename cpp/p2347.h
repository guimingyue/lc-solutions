#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    string bestHand(vector<int>& ranks, vector<char>& suits) {
        int as[4] = {0};
        for (auto ch : suits) {
            as[ch - 'a']++;
        }
        if (as[0] == 5 || as[1] == 5 || as[2] == 5 || as[3] == 5) {
            return "Flush";
        }
        vector<int> a(14, 0);
        a[ranks[0]]++;
        a[ranks[1]]++;
        a[ranks[2]]++;
        a[ranks[3]]++;
        a[ranks[4]]++;
        sort(a.begin(), a.end(), greater<int>());
        for (int i = 0; i < 14; i++) {
            if (a[i] >= 3) {
                return "Three of a Kind";
            } else if (a[i] == 2) {
                return "Pair";
            }
        }
        return "High Card";

    }
};