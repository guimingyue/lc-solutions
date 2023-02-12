#include <vector>

using namespace std;

class Solution {
public:
    int fillCups(vector<int>& amount) {
        int res = 0;
        while (amount[0] > 0 || amount[1] > 0 || amount[2] > 0) {
            res++;
            if (amount[0] >= amount[1] && amount[1] >= amount[2]) {
                amount[0] -= 1;
                amount[1] -= 1;
            } else if (amount[0] >= amount[2] && amount[2] >= amount[1]) {
                amount[0] -= 1;
                amount[2] -= 1;
            } else if (amount[1] >= amount[0] && amount[0] >= amount[2]) {
                amount[1] -= 1;
                amount[0] -= 1;
            } else if (amount[1] >= amount[2] && amount[2] >= amount[0]) {
                amount[1] -= 1;
                amount[2] -= 1;
            } else if (amount[2] >= amount[1] && amount[1] >= amount[0]) {
                amount[2] -= 1;
                amount[1] -= 1;
            } else {
                amount[2] -= 1;
                amount[0] -= 1;
            }
        }
        return res;
    }
};