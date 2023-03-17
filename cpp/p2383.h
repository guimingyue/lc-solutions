#include <vector>

using namespace std;

class Solution {
public:
    int minNumberOfHours(int initialEnergy, int initialExperience, vector<int>& energy, vector<int>& experience) {
        int ten = 0;
        for (auto en : energy) {
            ten += en;
        }
        int res = 0;
        if (ten >= initialEnergy) {
            res += ten - initialEnergy + 1;
        }
        int exp = initialExperience;
        for (auto ex : experience) {
            if (exp > ex) {
                exp += ex;
            } else {
                res += ex - exp + 1;
                exp += 1 + ex + ex - exp;
            }
        }
        return res;
    }
};