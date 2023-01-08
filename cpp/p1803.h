#include <vector>
#include <array>

using namespace std;

struct Trie {
    array<Trie*, 2> cld{nullptr, nullptr};
    int sum;

    Trie(): sum(0) {};
};

class Solution {

    Trie* root = nullptr;

    static constexpr int HIGH_HIT = 14;


public:
    int countPairs(vector<int>& nums, int low, int high) {
        return fnum(nums, high) - fnum(nums, low-1);
    }

    void add(int num) {
        Trie* cur = root;
        for (int k = HIGH_HIT; k >= 0; k--) {
            int bit = (num >> k) & 1;
            if (cur->cld[bit] == nullptr) {
                cur->cld[bit] = new Trie();
            }
            cur = cur->cld[bit];
            cur->sum++;
        }
    }

    int get(int num, int x) {
        Trie* cur = root;
        int sum = 0;
        for (int k = HIGH_HIT; k >= 0; k--) {
            int r = (num >> k) & 1;
            if ((x >> k) & 1) {
                if (cur->cld[r] != nullptr) {
                    sum += cur->cld[r]->sum;
                }
                if (cur->cld[r ^ 1] == nullptr) {
                    return sum;
                }
                cur = cur->cld[r ^ 1];
            } else {
                if (cur->cld[r] == nullptr) {
                    return sum;
                }
                cur = cur->cld[r];
            }
        }
        sum += cur-> sum;
        return sum;
    }

    int fnum(vector<int>& nums, int x) {
        root = new Trie();
        int f = 0;
        for (int i = 1; i < nums.size(); i++) {
            add(nums[i-1]);
            f += get(nums[i], x);
        }
        return f;
    }
};