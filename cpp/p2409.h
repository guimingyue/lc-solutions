#include <string>

using namespace std;

class Solution {
public:
    int countDaysTogether(string arriveAlice, string leaveAlice, string arriveBob, string leaveBob) {
        int daym[12] = {31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31};
        int days[13];
        int sum = 0;
        for (int i = 1; i < 13; i++) {
            days[i] = sum;
            sum += daym[i-1];
        }

        if (arriveAlice > leaveBob || arriveBob > leaveAlice) {
            return 0;
        }
       
        string arr = max(arriveAlice, arriveBob);
        string leav = min(leaveAlice, leaveBob);
        int sm = std::atoi(arr.substr(0, 2).c_str());
        int sd = std::atoi(arr.substr(3, 2).c_str());
        int em = std::atoi(leav.substr(0, 2).c_str());
        int ed = std::atoi(leav.substr(3, 2).c_str());
        
        return days[em] + ed - days[sm] - sd + 1;
    }

    /*
    "08-06"
"12-08"
"02-04"
"09-01"
"09-01"
"10-19"
"06-19"
"10-20"
"10-20"
"12-22"
"06-21"
"07-05"
"08-15"
"08-18"
"08-16"
"08-19"
"10-01"
"10-31"
"11-01"
"12-31"
    */
};