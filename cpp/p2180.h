class Solution {
public:
    int countEven(int num) {
        int res = 0;
        for (int i = 2; i <= num; i++) {
            if (isSumEven(i)) {
                res++;
            }
        }
        return res;
    }

    bool isSumEven(int val) {
        int sum = 0;
        while (val > 0) {
            sum += val % 10;
            val = val / 10;
        }
        return sum % 2 == 0;
    }
};