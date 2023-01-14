#include <vector>

using namespace std;

class Solution {
public:
    int reinitializePermutation(int n) {
        vector<int> perm;
        for (int i = 0; i < n; i++) {
            perm.push_back(i);
        }
        int res = 0;
        do {
            res++;
            vector<int> arr;
            for (int i = 0; i < n; i++) {
                if (i % 2 == 0) {
                    arr.push_back(perm[i/2]);
                } else {
                    arr.push_back(perm[n/2 + (i-1)/2]);
                }
            }
            perm = arr;
        } while (!isRecovered(perm));
        return res;
    }

private:
    bool isRecovered(vector<int>& perm) {
        for (int i = 0; i < perm.size(); i++) {
            if (perm[i] != i) {
                return false;
            }
        }
        return true;
    }
};