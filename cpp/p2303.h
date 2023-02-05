#include <vector>

using namespace std;

class Solution {
public:
    double calculateTax(vector<vector<int>>& brackets, int income) {
        double res = 0;
        int before = 0;
        for (auto bracket : brackets) {
            if (income > bracket[0]) {
                res += (bracket[0] - before) * (bracket[1] / 100.0);
                before = bracket[0];
            } else {
                res += (income - before) * (bracket[1] / 100.0);
                return res;
            }
        }
        return res;
    }
};