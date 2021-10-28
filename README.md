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
|66. Plus One|[p66](./src/lc/p66/Solution.rs)|`Vec`,`while`|
|67. Add Binary|[p67](./src/lc/p67/Solution.rs)|`String`,`Rev`,`Iterator`,`Option`|
|74. Search a 2D Matrix|[p74](./src/lc/p74.rs)|`Vec`|
|82. Remove Duplicates from Sorted List II|[p82](./src/lc/p82/Solution.rs)|`Vec`|
|83. Remove Duplicates from Sorted List|[p83](./src/lc/p83/Solution.rs)|`mut`, `Reference`,`Option`|
|125. Valid Palindrome|[p125](./src/lc/p125/Solution.rs)|`String`,`Rev`,`Iterator`,`Option`|
|166. Fraction to Recurring Decimal|[p166](./src/lc/p166/Solution.rs)|`String`,`Vec`,`char`|
|169. Majority Element|[p169](./src/lc/p169.rs)|`Vec`,`Iterator`|
|187. Repeated DNA Sequences|[p187](./src/lc/p187/Solution.rs)|`String`,`Vec`,`HashMap`|
|206. Reverse Linked List|[p206](./src/lc/p206/Solution.rs)|`Option`,`Box`|
|211. Design Add and Search Words Data Structure|[p211](./src/lc/p211.rs)|`Default`,`HashMap`|
|229. Majority Element II|[p229](./src/lc/p229.rs)|`Vec`,`Iterator`|
|230. Kth Smallest Element in a BST|[p230](./src/lc/p230.rs)|`Rc`,`RefCell`,`Vec`|
|240. Search a 2D Matrix II|[p240](./src/lc/p240.rs)|`Vec`|
|273. Integer to English Words|[p273](./src/lc/p273/Solution.rs)|`String`|
|282. Expression Add Operators|[p282](./src/lc/p282.rs)|`Vec`,`String`|
|301. Remove Invalid Parentheses|[p301](./src/lc/p301.rs)|`Tuple`,`String`,`HashSet`|
|352. Data Stream as Disjoint Intervals|[p352](./src/lc/p352/Solution.rs)|`struct`,`Vec`,`method`,`impl`|
|405. Convert a Number to Hexadecimal|[p405](./src/lc/p405/Solution.rs)|`String`,`Range`,`char`|
|412. Fizz Buzz|[p412](./src/lc/p412.rs)|`String`, `formate!`|
|414. Third Maximum Number|[p414](./src/lc/p414/Solution.rs)|`Vec`, `if-let`|
|434. Number of Segments in a String|[p434](./src/lc/p434/Solution.rs)|`String`|
|441. Arranging Coins|[p441](./src/lc/p441/Solution.rs)|`as`|
|453. Minimum Moves to Equal Array Elements|[p453](./src/lc/p453.rs)|`Vec`|
|476. Number Complement|[p476](./src/lc/p476.rs)|`>>`,`<<`|
|482. License Key Formatting|[p482](./src/lc/p482/Solution.rs)|`String`,`slice`|
|492. Construct the Rectangle|[p492](./src/lc/p492.rs)|`sqrt`,`Vec`|
|496. Next Greater Element I|[p496](./src/lc/p496.rs)|`HashMap`,`Vec`|
|638. Shopping Offers|[p638](./src/lc/p638.rs)|`Vec`,`Reference`,`std::cmp::min`|
|852. Peak Index in a Mountain Array|[p852](./src/lc/p852.rs)|`Vec`|
|869. Reordered Power of 2|[p869](./src/lc/p869.rs)|`HashSet`, `BinaryHeap`, `Reverse`|
|1009. Complement of Base 10 Integer|[p1009](./src/lc/p1009.rs)|`>>`,`<<`|
|1436. Destination City|[p1436](./src/lc/p1436/Solution.rs)|`String`,`Vec`,`HashMap`|
