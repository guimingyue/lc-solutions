#include <string>

using namespace std;

class Solution {
public:
    int countTime(string time) {
        int v1 = 0, v2 = 0;
        if (time[0] == '?' && time[1] == '?') {
            v1 = 24;
        } else if (time[0] == '?') {
            if (time[1] >= '0' && time[1] <= '3') {
                v1 = 3;
            } else {
                v1 = 2;
            }
        } else if (time[1] == '?') {
            if (time[0] == '2') {
                v1 = 4;
            } else {
                v1 = 10;
            }
        } else {
            v1 = 1;
        }
        if (time[3] == '?' && time[4] == '?') {
            v2 = 60;
        } else if (time[3] == '?') {
            v2 = 6;
        } else if (time[4] == '?') {
            v2 = 10;
        } else {
            v2 = 1;
        }
        return v1 * v2;
    }
};