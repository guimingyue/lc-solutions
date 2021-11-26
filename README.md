# lc-rust

LeetCode problem solutions using Rust programming language.

## Common Util

* [ListNode](./src/lc/common/ListNode.rs)
  * `new_from(Vec<i32>)` -> Option<Box<ListNode>>: create a linked list from a `Vec<i32>` object. 
  * `to_vec(Option<Box<ListNode>>) -> Vec<i32>`: serialize a linked list to a `Vec<i32>` object


## Solutions
The following table is the link for problem solutions with realted Rust topics or keywords.

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
|187. Repeated DNA Sequences|[p187](./src/lc/p187/Solution.rs)|`String`,`Vec`,`HashMap`|
|190. Reverse Bits|[p190](./src/lc/p190.rs)|`>>`, `<<`|
|191. Number of 1 Bits|[p191](./src/lc/p191.rs)|`u32`,`i32`|
|206. Reverse Linked List|[p206](./src/lc/p206/Solution.rs)|`Option`,`Box`|
|211. Design Add and Search Words Data Structure|[p211](./src/lc/p211.rs)|`Default`,`HashMap`|
|229. Majority Element II|[p229](./src/lc/p229.rs)|`Vec`,`Iterator`|
|230. Kth Smallest Element in a BST|[p230](./src/lc/p230.rs)|`Rc`,`RefCell`,`Vec`|
|231. Power of Two|[p231](./src/lc/p231.rs)|`&`|
|240. Search a 2D Matrix II|[p240](./src/lc/p240.rs)|`Vec`|
|260. Single Number III|[p260](./src/lc/p260.rs)|`Vec`, `^`, `>>`,`<<`|
|268. Missing Number|[p268](./src/lc/p268.rs)|`Vec`|
|273. Integer to English Words|[p273](./src/lc/p273/Solution.rs)|`String`|
|282. Expression Add Operators|[p282](./src/lc/p282.rs)|`Vec`,`String`|
|299. Bulls and Cows|[p299](./src/lc/p299.rs)|`HashMap`,`BTreeSet`,`format!`|
|300. Longest Increasing Subsequence|[p300](./src/lc/p300.rs)|`Vec`,`max`|
|301. Remove Invalid Parentheses|[p301](./src/lc/p301.rs)|`Tuple`,`String`,`HashSet`|
|318. Maximum Product of Word Lengths|[p318](./src/lc/p318.rs)|`Vec`,`String`|
|319. Bulb Switcher|[p319](./src/lc/p319.rs)|`sqrt`|
|335. Self Crossing|[p352](./src/lc/p335.rs)|`Vec`|
|352. Data Stream as Disjoint Intervals|[p352](./src/lc/p352/Solution.rs)|`struct`,`Vec`,`method`,`impl`|
|367. Valid Perfect Square|[p367](./src/lc/p367.rs)|`Vec`|
|374. Guess Number Higher or Lower|[p374](./src/lc/p374.rs)|`i32`|
|375. Guess Number Higher or Lower II|[p375](./src/lc/p375.rs)|`Vec`,`min`,`max`|
|384. Shuffle an Array|[p384](./src/lc/p384.rs)|`Vec`,`clone`,`rand`|
|397. Integer Replacement|[p397](./src/lc/p397.rs)|`min`|
|391. Perfect Rectangle|[p391](./src/lc/p391.rs)|`HashMap`,`unwrap_or`,`struct`|
|405. Convert a Number to Hexadecimal|[p405](./src/lc/p405/Solution.rs)|`String`,`Range`,`char`|
|407. Trapping Rain Water II|[p407](./src/lc/p407.rs)|`Vec`,`BinaryHeap`,`std::cmp::Reverse`,`std::cmp::max`|
|412. Fizz Buzz|[p412](./src/lc/p412.rs)|`String`, `formate!`|
|414. Third Maximum Number|[p414](./src/lc/p414/Solution.rs)|`Vec`, `if-let`|
|423. Reconstruct Original Digits from English|[p423](./src/lc/p423.rs)|`HashMap`,`Array`|
|434. Number of Segments in a String|[p434](./src/lc/p434/Solution.rs)|`String`|
|441. Arranging Coins|[p441](./src/lc/p441/Solution.rs)|`as`|
|453. Minimum Moves to Equal Array Elements|[p453](./src/lc/p453.rs)|`Vec`|
|458. Poor Pigs|[p458](./src/lc/p458.rs)|`log2`,`ceil`|
|476. Number Complement|[p476](./src/lc/p476.rs)|`>>`,`<<`|
|482. License Key Formatting|[p482](./src/lc/p482/Solution.rs)|`String`,`slice`|
|488. Zuma Game|[p488](./src/lc/p488.rs)|`HashSet`|
|492. Construct the Rectangle|[p492](./src/lc/p492.rs)|`sqrt`,`Vec`|
|495. Teemo Attacking|[p495](./src/lc/p495.rs)|`Vec`|
|496. Next Greater Element I|[p496](./src/lc/p496.rs)|`HashMap`,`Vec`|
|500. Keyboard Row|[p500](./src/lc/p500.rs)|`Vec`,`String`,`const`|
|520. Detect Capital|[p520](./src/lc/p520.rs)|`String`|
|563. Binary Tree Tilt|[p563](./src/lc/p563.rs)|`Option`,`Rc`,`RefCell`,`abs`|
|575. Distribute Candies|[p575](./src/lc/p575.rs)|`HashMap`,`std::cmp::min`|
|594. Longest Harmonious Subsequence|[p594](./src/lc/p594.rs)|`BTreeMap`,`max`|
|598. Range Addition II|[p598](./src/lc/p598.rs)|`Vec`,`min`|
|629. K Inverse Pairs Array|[p629](./src/lc/p629.rs)|`Vec`|
|638. Shopping Offers|[p638](./src/lc/p638.rs)|`Vec`,`Reference`,`std::cmp::min`|
|677. Map Sum Pairs|[p677](./src/lc/p677.rs)|`HashMap`,`PartialEq`,`Eq`,`Default`|
|700. Search in a Binary Search Tree|[p700](./src/lc/p700.rs)|`Rc`,`RefCell`,`Option`|
|852. Peak Index in a Mountain Array|[p852](./src/lc/p852.rs)|`Vec`|
|859. Buddy Strings|[p859](./src/lc/p859.rs)|`String`|
|869. Reordered Power of 2|[p869](./src/lc/p869.rs)|`HashSet`,`BinaryHeap`,`Reverse`|
|935. Knight Dialer|[p935](./src/lc/p935.rs)|`Vec`|
|1009. Complement of Base 10 Integer|[p1009](./src/lc/p1009.rs)|`>>`,`<<`|
|1218. Longest Arithmetic Subsequence of Given Difference|[p1218](./src/lc/p1218.rs)|`Vec`,`max`,`HashMap`|
|1436. Destination City|[p1436](./src/lc/p1436/Solution.rs)|`String`,`Vec`,`HashMap`|
|1631. Path With Minimum Effort|[p1631](./src/lc/p1631.rs)|`Vec`,`BinaryHeap`,`min`,`max`,`Reverse`,`Tuple`|
