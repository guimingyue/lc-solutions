#include <string>

using namespace std;

class Solution {
public:
    string printBin(double num) {
        int val = num * 1000000;
        while (val % 10 == 0) {
            val /= 10;
        }
        string res;
        res.push_back('0');
        res.push_back('.');
        while (num > 0) {
            num = num * 2;
            val = val * 2;
            if (val % 10 != 0) {
                return "ERROR";
            } else {
                val /= 10;
            }
            if (num >= 1) {
                res.push_back('1');
                num -= 1;
            } else {
                res.push_back('0');
            }
        }
        return res;
    }
};