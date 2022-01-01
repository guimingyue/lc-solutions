# lc-rust

LeetCode problem solutions using Rust programming language.

## Common Util

* [ListNode](./src/lc/common/ListNode.rs)
  * `new_from(Vec<i32>)` -> Option<Box<ListNode>>: create a linked list from a `Vec<i32>` object. 
  * `to_vec(Option<Box<ListNode>>) -> Vec<i32>`: serialize a linked list to a `Vec<i32>` object


## Solutions
The following table is the link for problem solutions with realted Rust topics or keywords.

### Problems from leetcode
|Problem|Solution|Related Rust keywords|
|-------|---------|---------|
|9. Palindrome Number|[p9](./src/lc/p9/Solution.rs)|`i32`,`if`,`while`|
|13. Roman to Integer|[p13](./src/lc/p13/Solution.rs)|`Pattern`,`for`,`Option`,`String`,`Reference`|
|19. Remove Nth Node From End of List|[p19](./src/lc/p19/Solution.rs)|`Option`,`Reference`|
|22. Generate Parentheses|[p22](./src/lc/p22.rs)|`Vec`,`String`|
|29. Divide Two Integers|[p29](./src/lc/p29.rs)|`i32`|
|38. Count and Say|[p38](./src/lc/p38.rs)|`String, char`|
|42. Trapping Rain Water|[p42](./src/lc/p42.rs)|`Vec`,`BinaryHeap`,`Reverse`,`max`|
|62. Unique Paths|[p62](./src/lc/p62.rs)|`Vec`|
|63. Unique Paths II|[p63](./src/lc/p63.rs)|`Vec`|
|64. Minimum Path Sum|[p63](./src/lc/p64.rs)|`Vec`|
|66. Plus One|[p66](./src/lc/p66/Solution.rs)|`Vec`,`while`|
|67. Add Binary|[p67](./src/lc/p67/Solution.rs)|`String`,`Rev`,`Iterator`,`Option`|
|72. Edit Distance|[p72](./src/lc/p72.rs)|`Vec`,`String`,`min`|
|74. Search a 2D Matrix|[p74](./src/lc/p74.rs)|`Vec`|
|75. Sort Colors|[p75](./src/lc/p75.rs)|`Vec`|
|82. Remove Duplicates from Sorted List II|[p82](./src/lc/p82/Solution.rs)|`Vec`|
|83. Remove Duplicates from Sorted List|[p83](./src/lc/p83/Solution.rs)|`mut`, `Reference`,`Option`|
|88. Merge Sorted Array|[p88](./src/lc/p88.rs)|`Vec`|
|93. Restore IP Addresses|[p93](./src/lc/p93.rs)|`Vec`,`parse`|
|97. Interleaving String|[p97](./src/lc/p97.rs)|`Vec`,`String`|
|118. Pascal's Triangle|[p118](./src/lc/p118.rs)|`Vec`|
|119. Pascal's Triangle II|[p119](./src/lc/p119.rs)|`Vec`|
|125. Valid Palindrome|[p125](./src/lc/p125/Solution.rs)|`String`,`Rev`,`Iterator`,`Option`|
|136. Single Number|[p136](./src/lc/p136.rs)|`Vec`,`^`|
|137. Single Number II|[p137](./src/lc/p137.rs)|`Vec`,`>>`,`<<`|
|166. Fraction to Recurring Decimal|[p166](./src/lc/p166/Solution.rs)|`String`,`Vec`,`char`|
|169. Majority Element|[p169](./src/lc/p169.rs)|`Vec`,`Iterator`|
|172. Factorial Trailing Zeroes|[p172](./src/lc/p172.rs)||
|187. Repeated DNA Sequences|[p187](./src/lc/p187/Solution.rs)|`String`,`Vec`,`HashMap`|
|190. Reverse Bits|[p190](./src/lc/p190.rs)|`>>`, `<<`|
|191. Number of 1 Bits|[p191](./src/lc/p191.rs)|`u32`,`i32`|
|200. Number of Islands|[p200](./src/lc/p200.rs)|`Vec`|
|201. Bitwise AND of Numbers Range|[p201](./src/lc/p201.rs)|`>>`,`<<`,`!`|
|206. Reverse Linked List|[p206](./src/lc/p206/Solution.rs)|`Option`,`Box`|
|211. Design Add and Search Words Data Structure|[p211](./src/lc/p211.rs)|`Default`,`HashMap`|
|228. Summary Ranges|[p228](./src/lc/p228.rs)|`Vec`,`String`|
|229. Majority Element II|[p229](./src/lc/p229.rs)|`Vec`,`Iterator`|
|230. Kth Smallest Element in a BST|[p230](./src/lc/p230.rs)|`Rc`,`RefCell`,`Vec`|
|231. Power of Two|[p231](./src/lc/p231.rs)|`&`|
|232. Implement Queue using Stacks|[p232](./src/lc/p232.rs)|`Vec`|
|238. Product of Array Except Self|[p238](./src/lc/p238.rs)|`Vec`|
|240. Search a 2D Matrix II|[p240](./src/lc/p240.rs)|`Vec`|
|242. Valid Anagram|[p242](./src/lc/p242.rs)|`String`|
|258. Add Digits|[p258](./src/lc/p258.rs)||
|260. Single Number III|[p260](./src/lc/p260.rs)|`Vec`, `^`, `>>`,`<<`|
|268. Missing Number|[p268](./src/lc/p268.rs)|`Vec`|
|273. Integer to English Words|[p273](./src/lc/p273/Solution.rs)|`String`|
|282. Expression Add Operators|[p282](./src/lc/p282.rs)|`Vec`,`String`|
|299. Bulls and Cows|[p299](./src/lc/p299.rs)|`HashMap`,`BTreeSet`,`format!`|
|300. Longest Increasing Subsequence|[p300](./src/lc/p300.rs)|`Vec`,`max`|
|301. Remove Invalid Parentheses|[p301](./src/lc/p301.rs)|`Tuple`,`String`,`HashSet`|
|318. Maximum Product of Word Lengths|[p318](./src/lc/p318.rs)|`Vec`,`String`|
|319. Bulb Switcher|[p319](./src/lc/p319.rs)|`sqrt`|
|322. Coin Change|[p322](./src/lc/p322.rs)|`Vec`|
|334. Increasing Triplet Subsequence|[p334](./src/lc/p334.rs)|`Vec`|
|335. Self Crossing|[p335](./src/lc/p335.rs)|`Vec`|
|352. Data Stream as Disjoint Intervals|[p352](./src/lc/p352/Solution.rs)|`struct`,`Vec`,`method`,`impl`|
|357. Count Numbers with Unique Digits|[p357](./src/lc/p357.rs)||
|367. Valid Perfect Square|[p367](./src/lc/p367.rs)|`Vec`|
|372. Super Pow|[p372](./src/lc/p372.rs)|`Vec`|
|374. Guess Number Higher or Lower|[p374](./src/lc/p374.rs)|`i32`|
|375. Guess Number Higher or Lower II|[p375](./src/lc/p375.rs)|`Vec`,`min`,`max`|
|383. Ransom Note|[p383](./src/lc/p383.rs)|`Vec`|
|384. Shuffle an Array|[p384](./src/lc/p384.rs)|`Vec`,`clone`,`rand`|
|397. Integer Replacement|[p397](./src/lc/p397.rs)|`min`|
|391. Perfect Rectangle|[p391](./src/lc/p391.rs)|`HashMap`,`unwrap_or`,`struct`|
|400. Nth Digit|[p400](./src/lc/p400.rs)|`pow`|
|405. Convert a Number to Hexadecimal|[p405](./src/lc/p405/Solution.rs)|`String`,`Range`,`char`|
|407. Trapping Rain Water II|[p407](./src/lc/p407.rs)|`Vec`,`BinaryHeap`,`std::cmp::Reverse`,`std::cmp::max`|
|412. Fizz Buzz|[p412](./src/lc/p412.rs)|`String`, `formate!`|
|414. Third Maximum Number|[p414](./src/lc/p414/Solution.rs)|`Vec`, `if-let`|
|419. Battleships in a Board|[p419](./src/lc/p419.rs)|`Vec`|
|423. Reconstruct Original Digits from English|[p423](./src/lc/p423.rs)|`HashMap`,`Array`|
|434. Number of Segments in a String|[p434](./src/lc/p434/Solution.rs)|`String`|
|438. Find All Anagrams in a String|[p438](./src/lc/p438.rs)|`HashMap`,`Vec`|
|441. Arranging Coins|[p441](./src/lc/p441/Solution.rs)|`as`|
|453. Minimum Moves to Equal Array Elements|[p453](./src/lc/p453.rs)|`Vec`|
|458. Poor Pigs|[p458](./src/lc/p458.rs)|`log2`,`ceil`|
|461. Hamming Distance|[p461](./src/lc/p461.rs)|`count_ones1,`^`|
|475. Heaters|[p475](./src/lc/p475.rs)|`Vec`|
|476. Number Complement|[p476](./src/lc/p476.rs)|`>>`,`<<`|
|482. License Key Formatting|[p482](./src/lc/p482/Solution.rs)|`String`,`slice`|
|488. Zuma Game|[p488](./src/lc/p488.rs)|`HashSet`|
|492. Construct the Rectangle|[p492](./src/lc/p492.rs)|`sqrt`,`Vec`|
|495. Teemo Attacking|[p495](./src/lc/p495.rs)|`Vec`|
|496. Next Greater Element I|[p496](./src/lc/p496.rs)|`HashMap`,`Vec`|
|500. Keyboard Row|[p500](./src/lc/p500.rs)|`Vec`,`String`,`const`|
|506. Relative Ranks|[p506](./src/lc/p506.rs)|`Vec`,`binary_search_by`,`sort_by`,`HashMap`|
|507. Perfect Number|[p507](./src/lc/p507.rs)|`sqrt`|
|518. Coin Change 2|[p518](./src/lc/p518.rs)|`Vec`|
|519. Random Flip Matrix|[p519](./src/lc/p519.rs)|`Vec`,`rand`|
|520. Detect Capital|[p520](./src/lc/p520.rs)|`String`|
|563. Binary Tree Tilt|[p563](./src/lc/p563.rs)|`Option`,`Rc`,`RefCell`,`abs`|
|575. Distribute Candies|[p575](./src/lc/p575.rs)|`HashMap`,`std::cmp::min`|
|594. Longest Harmonious Subsequence|[p594](./src/lc/p594.rs)|`BTreeMap`,`max`|
|598. Range Addition II|[p598](./src/lc/p598.rs)|`Vec`,`min`|
|629. K Inverse Pairs Array|[p629](./src/lc/p629.rs)|`Vec`|
|630. Course Schedule III|[p630](./src/lc/p630.rs)|`Vec`,`sort_by`,`BinaryHeap`|
|638. Shopping Offers|[p638](./src/lc/p638.rs)|`Vec`,`Reference`,`std::cmp::min`|
|677. Map Sum Pairs|[p677](./src/lc/p677.rs)|`HashMap`,`PartialEq`,`Eq`,`Default`|
|686. Repeated String Match|[p686](./src/lc/p686.rs)|`String`,`contains`|
|689. Maximum Sum of 3 Non-Overlapping Subarrays|[p689](./src/lc/p689.rs)|`Vec`|
|700. Search in a Binary Search Tree|[p700](./src/lc/p700.rs)|`Rc`,`RefCell`,`Option`|
|709. To Lower Case|[p709](./src/lc/p709.rs)|`match`|
|748. Shortest Completing Word|[p748](./src/lc/p748.rs)|`Vec`,`match`|
|786. K-th Smallest Prime Fraction|[p786](./src/lc/p786.rs)|`sort_by`,`PartialOrd`,`Ord`,`PartialEq`,`BinaryHeap`,`Reverse`|
|794. Valid Tic-Tac-Toe State|[p794](./src/lc/p794.rs)|`Vec`|
|807. Max Increase to Keep City Skyline|[p807](./src/lc/p807.rs)|`Vec`|
|825. Friends Of Appropriate Ages|[p825](./src/lc/p825.rs)|`Vec`|
|846. Hand of Straights|[p846](./src/lc/p846.rs)|`BTreeMap`,`VecDeque`,`FromIterator`|
|851. Loud and Rich|[p851](./src/lc/p851.rs)|`Vec`|
|852. Peak Index in a Mountain Array|[p852](./src/lc/p852.rs)|`Vec`|
|859. Buddy Strings|[p859](./src/lc/p859.rs)|`String`|
|869. Reordered Power of 2|[p869](./src/lc/p869.rs)|`HashSet`,`BinaryHeap`,`Reverse`|
|911. Online Election|[p911](./src/lc/p911.rs)|`Vec`,`HashMap`,`binary_search`,`match`|
|935. Knight Dialer|[p935](./src/lc/p935.rs)|`Vec`|
|997. Find the Town Judge|[p997](./src/lc/p997.rs)|`Vec`|
|1005. Maximize Sum Of Array After K Negations|[p1005](./src/lc/p1005.rs)|`Vec`,`sort`,`binary_search`,`min`|
|1009. Complement of Base 10 Integer|[p1009](./src/lc/p1009.rs)|`>>`,`<<`|
|1034. Coloring A Border|[p1034](./src/lc/p1034.rs)|`Vec`|
|1078. Occurrences After Bigram|[p1078](./src/lc/p1078.rs)|`Vec`,`String`,`match-if`|
|1154. Day of the Year|[p1154](./src/lc/p1154.rs)|`match, match-if, const`|
|1218. Longest Arithmetic Subsequence of Given Difference|[p1218](./src/lc/p1218.rs)|`Vec`,`max`,`HashMap`|
|1436. Destination City|[p1436](./src/lc/p1436/Solution.rs)|`String`,`Vec`,`HashMap`|
|1446. Consecutive Characters|[p1446](./src/lc/p1446.rs)|`String`,`max`|
|1518. Water Bottles|[p1518](./src/lc/p1518.rs)||
|1609. Even Odd Tree|[p1609](./src/lc/p1609.rs)|`Rc`,`RefCell`,`Vec`|
|1610. Maximum Number of Visible Points|[p1610](./src/lc/p1610.rs)|`Vec`,`sort_by`,`std::f64::consts::PI`,`partial_cmp`,`atan2`|
|1631. Path With Minimum Effort|[p1631](./src/lc/p1631.rs)|`Vec`,`BinaryHeap`,`min`,`max`,`Reverse`,`Tuple`|
|1705. Maximum Number of Eaten Apples|[p1705](./src/lc/p1705.rs)|`Vec`,`Reverse`|
|1780. Check if Number is a Sum of Powers of Three|[p1780](./src/lc/p1780.rs)|`pow`|
|1816. Truncate Sentence|[p1816](./src/lc/p1816.rs)|`String`|
|1995. Count Special Quadruplets|[p1995](./src/lc/p1995.rs)|`String`|
|2022. Convert 1D Array Into 2D Array|[p2022](./src/lc/p2022.rs)|`Vec`|

### 《剑指 Offer》题目
|Problem|Solution|Related Rust keywords|
|-------|---------|---------|
|剑指 Offer II 001. 整数除法|[p001](./src/offer/p001.rs)|`abs`|
