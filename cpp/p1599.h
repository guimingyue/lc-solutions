#include <vector>

using namespace std;

class Solution {
public:
    int minOperationsMaxProfit(vector<int>& customers, int boardingCost, int runningCost) {
        int total = 0, maxProfit = 0, res = -1, profit = 0;
        int turn = 0;
        while (turn < customers.size() || total > 0) {
            turn++;
            if (turn <= customers.size()) {
                total += customers[turn-1];
            }
            if (total >= 4) {
                profit +=  4 * boardingCost - runningCost;
                total -= 4;
            } else {
                profit += total * boardingCost - runningCost;
                total = 0;
            }
            if (profit > maxProfit) {
                maxProfit = profit;
                res = turn;
            }
        }
        return res;
    }
};