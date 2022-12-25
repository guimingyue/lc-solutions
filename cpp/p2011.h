#include <vector>
#include <string>
using namespace std;

class Solution {
public:
    int finalValueAfterOperations(vector<string>& operations) {
        int res = 0;
        for (auto opt : operations) {
            if (opt == "++X" || opt == "X++") {
                res++;
            } else if (opt == "--X" || opt == "X--") {
                res--;
            }
        }
        return res;
    }
};