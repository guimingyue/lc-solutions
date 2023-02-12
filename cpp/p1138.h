#include <string>

using namespace std;

class Solution {
public:
    string alphabetBoardPath(string target) {
        string res;
        char cur = 'a';
        for (char ch : target) {
            if (ch != cur) {
                int r = (ch-'a') / 5 - (cur-'a') / 5;
                int c = (ch-'a') % 5 - (cur-'a') % 5;
                char push;
                if (r < 0) {
                    push = 'U';
                } else {
                    push = 'D';
                }
                if (ch == 'z' && c != 0) {
                    push_back_by_num(res, push, abs(r) - 1);
                } else {
                    push_back_by_num(res, push, abs(r));
                }

                if (c < 0) {
                    push = 'L';
                } else {
                    push = 'R';
                }
                push_back_by_num(res, push, abs(c));
                if (ch == 'z' && c != 0) {
                    push_back_by_num(res, 'D', 1);
                }
                cur = ch;
            }
            res.push_back('!');
        }
        return res;
    }

    void push_back_by_num(string &res, char ch, int num) {
        while (num-- > 0) {
            res.push_back(ch);
        }
    }
};