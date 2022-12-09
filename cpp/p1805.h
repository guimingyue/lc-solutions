class Solution {
public:
    int numDifferentIntegers(std::string word) {
        std::unordered_set<std::string> nums;
        auto start = 0;
        bool inNum = false;
        for (auto i = 0; i < word.length(); i++) {
            auto ch = word[i];
            if (ch >= '0' && ch <= '9') {
                if (!inNum) {
                    start = i;
                    inNum = true;
                }
            } else {
                if (inNum) {
                    nums.insert(parseNum(start, i, word));
                }
                inNum = false;
            }
        }
        if (inNum) {
            nums.insert(parseNum(start, word.length(), word));
        }
        
        return nums.size();
    }

    std::string parseNum(int start, int end, std::string word) {
        int j = start;
        while (j < end) {
            if (word[j] != '0') {
                break;
            }
            j++;
        }
        if (j == end) {
            return "0";
        } else {
            return word.substr(j, end - j);
        }
    }
};