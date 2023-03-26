#include <string>

using namespace std;

class Solution {
public:
    bool checkPalindromeFormation(string a, string b) {
        return checkConcatenation(a, b) || checkConcatenation(b, a);
    }

    bool checkConcatenation(string a, string b) {
        int left = 0, right = a.size() - 1;
        while (left < a.size() && right >= 0 && a[left] == b[right]) {
            left++;
            right--;
        }
        if (left >= right) {
            return true;
        }
        return checkSelfPalindrome(a, left, right) || checkSelfPalindrome(b, left, right);
    }

    bool checkSelfPalindrome(string s, int start, int end) {
        while (start < end) {
            if (s[start] != s[end]) {
                return false;
            }
            start++;
            end--;
        }
        return true;
    }
};