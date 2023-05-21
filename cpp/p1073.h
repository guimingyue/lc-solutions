#include <vector>

using namespace std;


class Solution {
public:
    vector<int> addNegabinary(vector<int>& arr1, vector<int>& arr2) {
         int i1 = arr1.size() - 1, i2 = arr2.size() - 1, flag = 1, carry = 0;
        vector<int> arr;
        while (i1 >= 0 && i2 >= 0) {
            int s = arr1[i1] + arr2[i2] - carry;
            if (s == -1) {
                arr.insert(arr.begin(), 1);
                carry = -1;
            } else {
                arr.insert(arr.begin(), s % 2);
                carry = s / 2;
            }
            i1--;
            i2--;
        }
        while (i1 >= 0) {
            int s = arr1[i1] - carry;
            if (s == -1) {
                arr.insert(arr.begin(), 1);
                carry = -1;
            } else {
                arr.insert(arr.begin(), s % 2);
                carry = s / 2;
            }
            i1--;
        }
        while (i2 >= 0) {
            int s = arr2[i2] - carry;
            if (s == -1) {
                arr.insert(arr.begin(), 1);
                carry = -1;
            } else {
                arr.insert(arr.begin(), s % 2);
                carry = s / 2;
            }
            i2--;
        }

        if (carry != 0) {
            arr.insert(arr.begin(), 1);
            arr.insert(arr.begin(), 1);
        }

        int i = 0;
        while (i < arr.size() - 1 && arr[i] == 0) {
            i++;
        }
        vector<int> res;
        while (i < arr.size()) {
            res.push_back(arr[i]);
            i++;
        }
        return res;
    }
};