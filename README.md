# lc-rust

LeetCode problem solutions using programming language that I am learning. Now there are Rust, Go and C++.

## Common Util

* [ListNode](./rust/src/lc/common/ListNode.rs)
  * `new_from(Vec<i32>)` -> Option<Box<ListNode>>: create a linked list from a `Vec<i32>` object. 
  * `to_vec(Option<Box<ListNode>>) -> Vec<i32>`: serialize a linked list to a `Vec<i32>` object


## Solutions
The following table is the link for problem solutions with realted Rust topics or keywords.

### Problems from leetcode
 
| Problem                                                           | Solution                            | Related Rust keywords                            |
|:------------------------------------------------------------------|-------------------------------------|--------------------------------------------------|
| 6. ZigZag Conversion                                              | [p6](./rust/src/lc/p6.rs)                | `String`                                         |
| 9. Palindrome Number                                              | [p9](./rust/src/lc/p9/Solution.rs)       | `i32`,`if`,`while`                               |
| 13. Roman to Integer                                              | [p13](./rust/src/lc/p13/Solution.rs)     | `Pattern`,`for`,`Option`,`String`,`Reference`    |
| 19. Remove Nth Node From End of List                              | [p19](./rust/src/lc/p19/Solution.rs)     | `Option`,`Reference`                             |
| 22. Generate Parentheses                                          | [p22](./rust/src/lc/p22.rs)              | `Vec`,`String`                                   |
| 29. Divide Two Integers                                           | [p29](./rust/src/lc/p29.rs)              | `i32`                                            |
| 30. Substring with Concatenation of All Words       | [p30](./rust/src/lc/p30.rs)              | `HashMap`                                            |
| 33. Search in Rotated Sorted Array| [p33](./go/p33.go)              | |
| 38. Count and Say                                                 | [p38](./rust/src/lc/p38.rs)              | `String, char`                                   |
| 42. Trapping Rain Water                                           | [p42](./rust/src/lc/p42.rs)              | `Vec`,`BinaryHeap`,`Reverse`,`max`               |
| 62. Unique Paths                                                  | [p62](./rust/src/lc/p62.rs)              | `Vec`                                            |
| 63. Unique Paths II                                               | [p63](./rust/src/lc/p63.rs)              | `Vec`                                            |
| 64. Minimum Path Sum                                              | [p63](./rust/src/lc/p64.rs)              | `Vec`                                            |
| 66. Plus One                                                      | [p66](./rust/src/lc/p66/Solution.rs)     | `Vec`,`while`                                    |
| 67. Add Binary                                                    | [p67](./rust/src/lc/p67/Solution.rs)     | `String`,`Rev`,`Iterator`,`Option`               |
| 71. Simplify Path                                                 | [p71](./rust/src/lc/p71.rs)              | `Vec`,`String`                                   |
| 72. Edit Distance                                                 | [p72](./rust/src/lc/p72.rs)              | `Vec`,`String`,`min`                             |
| 74. Search a 2D Matrix                                            | [p74](./rust/src/lc/p74.rs)              | `Vec`                                            |
| 75. Sort Colors                                                   | [p75](./rust/src/lc/p75.rs)              | `Vec`                                            |
| 81. Search in Rotated Sorted Array II| [p81](./go/p81.go)     |  |
| 82. Remove Duplicates from Sorted List II                         | [p82](./rust/src/lc/p82/Solution.rs)     | `Vec`                                            |
| 83. Remove Duplicates from Sorted List                            | [p83](./rust/src/lc/p83/Solution.rs)     | `mut`, `Reference`,`Option`                      |
| 88. Merge Sorted Array                                            | [p88](./rust/src/lc/p88.rs)              | `Vec`                                            |
| 89. Gray Code                                                     | [p89](./rust/src/lc/p89.rs)              | `Vec`                                            |
| 93. Restore IP Addresses                                          | [p93](./rust/src/lc/p93.rs)              | `Vec`,`parse`                                    |
| 97. Interleaving String                                           | [p97](./rust/src/lc/p97.rs)              | `Vec`,`String`                                   |
| 118. Pascal's Triangle                                            | [p118](./rust/src/lc/p118.rs)            | `Vec`                                            |
| 119. Pascal's Triangle II                                         | [p119](./rust/src/lc/p119.rs)            | `Vec`                                            |
| 125. Valid Palindrome                                             | [p125](./rust/src/lc/p125/Solution.rs)   | `String`,`Rev`,`Iterator`,`Option`               |
| 136. Single Number                                                | [p136](./rust/src/lc/p136.rs)            | `Vec`,`^`                                        |
| 137. Single Number II                                             | [p137](./rust/src/lc/p137.rs)            | `Vec`,`>>`,`<<`                                  |
| 166. Fraction to Recurring Decimal                                | [p166](./rust/src/lc/p166/Solution.rs)   | `String`,`Vec`,`char`                            |
| 167. Two Sum II - Input Array Is Sorted                               | [p167](./go/p167.go)   |                          |
| 169. Majority Element                                             | [p169](./rust/src/lc/p169.rs)            | `Vec`,`Iterator`                                 |
| 172. Factorial Trailing Zeroes                                    | [p172](./rust/src/lc/p172.rs)            ||
| 187. Repeated DNA Sequences                                       | [p187](./rust/src/lc/p187/Solution.rs)   | `String`,`Vec`,`HashMap`                         |
| 190. Reverse Bits                                                 | [p190](./rust/src/lc/p190.rs)            | `>>`, `<<`                                       |
| 191. Number of 1 Bits                                             | [p191](./rust/src/lc/p191.rs)            | `u32`,`i32`                                      |
| 200. Number of Islands                                            | [p200](./rust/src/lc/p200.rs)            | `Vec`                                            |
| 201. Bitwise AND of Numbers Range                                 | [p201](./rust/src/lc/p201.rs)            | `>>`,`<<`,`!`                                    |
| 206. Reverse Linked List                                          | [p206](./rust/src/lc/p206/Solution.rs)   | `Option`,`Box`                                   |
| 211. Design Add and Search Words Data Structure                   | [p211](./rust/src/lc/p211.rs)            | `Default`,`HashMap`                              |
| 219. Contains Duplicate II                                        | [p219](./rust/src/lc/p219.rs)            | Vec`                                             |
| 228. Summary Ranges                                               | [p228](./rust/src/lc/p228.rs)            | `Vec`,`String`                                   |
| 229. Majority Element II                                          | [p229](./rust/src/lc/p229.rs)            | `Vec`,`Iterator`                                 |
| 230. Kth Smallest Element in a BST                                | [p230](./rust/src/lc/p230.rs)            | `Rc`,`RefCell`,`Vec`                             |
| 231. Power of Two                                                 | [p231](./rust/src/lc/p231.rs)            | `&`                                              |
| 232. Implement Queue using Stacks                                 | [p232](./rust/src/lc/p232.rs)            | `Vec`                                            |
| 238. Product of Array Except Self                                 | [p238](./rust/src/lc/p238.rs)            | `Vec`                                            |
| 240. Search a 2D Matrix II                                        | [p240](./rust/src/lc/p240.rs)            | `Vec`                                            |
| 241. Different Ways to Add Parentheses                            | [p241](./go/p241.go)            |                                           |
| 242. Valid Anagram                                                | [p242](./rust/src/lc/p242.rs)            | `String`                                         |
| 258. Add Digits                                                   | [p258](./rust/src/lc/p258.rs)            ||
| 260. Single Number III                                            | [p260](./rust/src/lc/p260.rs)            | `Vec`, `^`, `>>`,`<<`                            |
| 268. Missing Number                                               | [p268](./rust/src/lc/p268.rs)            | `Vec`                                            |
| 273. Integer to English Words                                     | [p273](./rust/src/lc/p273/Solution.rs)   | `String`                                         |
| 278. First Bad Version                                     | [p278](./go/p278.go)   |                                      |
| 282. Expression Add Operators                                     | [p282](./rust/src/lc/p282.rs)            | `Vec`,`String`                                   |
| 299. Bulls and Cows                                               | [p299](./rust/src/lc/p299.rs)            | `HashMap`,`BTreeSet`,`format!`                   |
| 300. Longest Increasing Subsequence                               | [p300](./rust/src/lc/p300.rs)            | `Vec`,`max`                                      |
| 301. Remove Invalid Parentheses                                   | [p301](./rust/src/lc/p301.rs)            | `Tuple`,`String`,`HashSet`                       |
| 306. Additive Number                                              | [p306](./rust/src/lc/p306.rs)            | `String`,`char`,`Slice`                          |
| 307. Range Sum Query - Mutable                                    | [p307](./rust/src/lc/p307.rs)            ||
| 310. Minimum Height Trees                                         | [p310](./rust/src/lc/p310.rs)            | `HashMap`                                        |
| 318. Maximum Product of Word Lengths                              | [p318](./rust/src/lc/p318.rs)            | `Vec`,`String`                                   |
| 319. Bulb Switcher                                                | [p319](./rust/src/lc/p319.rs)            | `sqrt`                                           |
| 322. Coin Change                                                  | [p322](./rust/src/lc/p322.rs)            | `Vec`                                            |
| 324. Wiggle Sort II                                               | [p324](./go/p324.go)            |                                            |
| 334. Increasing Triplet Subsequence                               | [p334](./rust/src/lc/p334.rs)            | `Vec`                                            |
| 335. Self Crossing                                                | [p335](./rust/src/lc/p335.rs)            | `Vec`                                            |
| 352. Data Stream as Disjoint Intervals                            | [p352](./rust/src/lc/p352/Solution.rs)   | `struct`,`Vec`,`method`,`impl`                   |
| 357. Count Numbers with Unique Digits                             | [p357](./rust/src/lc/p357.rs)            ||
| 367. Valid Perfect Square                                         | [p367](./rust/src/lc/p367.rs)            | `Vec`                                            |
| 372. Super Pow                                                    | [p372](./rust/src/lc/p372.rs)            | `Vec`                                            |
| 373. Find K Pairs with Smallest Sums                              | [p373](./rust/src/lc/p373.rs)            | `Vec`,`BinaryHeap`                               |
| 374. Guess Number Higher or Lower                                 | [p374](./rust/src/lc/p374.rs)            | `i32`                                            |
| 375. Guess Number Higher or Lower II                              | [p375](./rust/src/lc/p375.rs)            | `Vec`,`min`,`max`                                |
| 382. Linked List Random Node                                      | [p382](./rust/src/lc/p382.rs)            | `Box`,`rand`                                     |
| 383. Ransom Note                                                  | [p383](./rust/src/lc/p383.rs)            | `Vec`                                            |
| 384. Shuffle an Array                                             | [p384](./rust/src/lc/p384.rs)            | `Vec`,`clone`,`rand`                             |
| 385. Mini Parser                                                  | [p385](./rust/src/lc/p385.rs)            ||
| 386. Lexicographical Numbers                                      | [p386](./rust/src/lc/p386.rs)            ||
| 390. Elimination Game                                             | [p390](./rust/src/lc/p390.rs)            | `match`                                          |
| 393. UTF-8 Validation                                             | [p393](./rust/src/lc/p393.rs)            | `Vec`                                            |
| 397. Integer Replacement                                          | [p397](./rust/src/lc/p397.rs)            | `min`                                            |
| 398. Random Pick Index                                            | [p398](./rust/src/lc/p398.rs)            | `HashMap`                                        |
| 391. Perfect Rectangle                                            | [p391](./rust/src/lc/p391.rs)            | `HashMap`,`unwrap_or`,`struct`                   |
| 400. Nth Digit                                                    | [p400](./rust/src/lc/p400.rs)            | `pow`                                            |
| 405. Convert a Number to Hexadecimal                              | [p405](./rust/src/lc/p405/Solution.rs)   | `String`,`Range`,`char`                          |
| 407. Trapping Rain Water II                                       | [p407](./rust/src/lc/p407.rs)            | `Vec`,`BinaryHeap`,`std::cmp::Reverse`,`std::cmp::max` |
| 412. Fizz Buzz                                                    | [p412](./rust/src/lc/p412.rs)            | `String`, `formate!`                             |
| 414. Third Maximum Number                                         | [p414](./rust/src/lc/p414/Solution.rs)   | `Vec`, `if-let`                                  |
| 417. Pacific Atlantic Water Flow                                  | [p417](./rust/src/lc/p417.rs)            ||
| 419. Battleships in a Board                                       | [p419](./rust/src/lc/p419.rs)            | `Vec`                                            |
| 423. Reconstruct Original Digits from English                     | [p423](./rust/src/lc/p423.rs)            | `HashMap`,`Array`                                |
| 433. Minimum Genetic Mutation                                     | [p433](./rust/src/lc/p433.rs)            ||
| 434. Number of Segments in a String                               | [p434](./rust/src/lc/p434/Solution.rs)   | `String`                                         |
| 436. Find Right Interval                                          | [p434](./rust/src/lc/p436.rs)            | `sort_by`                                         |
| 438. Find All Anagrams in a String                                | [p438](./rust/src/lc/p438.rs)            | `HashMap`,`Vec`                                  |
| 440. K-th Smallest in Lexicographical Order                       | [p440](./rust/src/lc/p440.rs)            ||
| 441. Arranging Coins                                              | [p441](./rust/src/lc/p441/Solution.rs)   | `as`                                             |
| 442. Find All Duplicates in an Array                              | [p442](./rust/src/lc/p442.rs)            ||
| 453. Minimum Moves to Equal Array Elements                        | [p453](./rust/src/lc/p453.rs)            | `Vec`                                            |
| 458. Poor Pigs                                                    | [p458](./rust/src/lc/p458.rs)            | `log2`,`ceil`                                    |
| 461. Hamming Distance                                             | [p461](./rust/src/lc/p461.rs)            | `count_ones1,`^`                                 |
| 462. Minimum Moves to Equal Array Elements II                     | [p462](./rust/src/lc/p42.rs)             | |
| 464. Can I Win                                                    | [p464](./rust/src/lc/p464.rs)            | `HashMap`|
| 467. Unique Substrings in Wraparound String                       | [p467](./rust/src/lc/p467.rs)            | |
| 468. Validate IP Address                                          | [p468](./rust/src/lc/p468.rs)            |`match` |
| 473. Matchsticks to Square                                        | [p473](./rust/src/lc/p473.rs)            | `<<`,`!`                                            |
| 475. Heaters                                                      | [p475](./rust/src/lc/p475.rs)            | `Vec`                                            |
| 476. Number Complement                                            | [p476](./rust/src/lc/p476.rs)            | `>>`,`<<`                                        |
| 478. Generate Random Point in a Circle                                            | [p478](./go/p478/p478.go)            |                                       |
| 479. Largest Palindrome Product                                   | [p479](./rust/src/lc/p479.rs)            | `pow`                                            |
| 481. Magical String | [p481](./go/p481.go)   | |
| 482. License Key Formatting                                       | [p482](./rust/src/lc/p482/Solution.rs)   | `String`,`slice`                                 |
| 488. Zuma Game                                                    | [p488](./rust/src/lc/p488.rs)            | `HashSet`                                        |
| 492. Construct the Rectangle                                      | [p492](./rust/src/lc/p492.rs)            | `sqrt`,`Vec`                                     |
| 495. Teemo Attacking                                              | [p495](./rust/src/lc/p495.rs)            | `Vec`                                            |
| 496. Next Greater Element I                                       | [p496](./rust/src/lc/p496.rs)            | `HashMap`,`Vec`                                  |
| 497. Random Point in Non-overlapping Rectangles                   | [p497](./go/p497/p497.go)            |                                  |
| 498. Diagonal Traverse                   | [p498](./go/p498.go)            |                                  |
| 500. Keyboard Row                                                 | [p500](./rust/src/lc/p500.rs)            | `Vec`,`String`,`const`                           |
| 504. Base 7                                                       | [p504](./rust/src/lc/p504.rs)            | `Vec.reverse`                                    |
| 506. Relative Ranks                                               | [p506](./rust/src/lc/p506.rs)            | `Vec`,`binary_search_by`,`sort_by`,`HashMap`     |
| 507. Perfect Number                                               | [p507](./rust/src/lc/p507.rs)            | `sqrt`                                           |
| 508. Most Frequent Subtree Sum                                               | [p508](./go/p508.go)            |                                           |
| 513. Find Bottom Left Tree Value  | [p513](./go/p513.go)            |                                            |
| 515. Find Largest Value in Each Tree Row  | [p515](./go/p515.go)            |                                            |
| 518. Coin Change 2                                                | [p518](./rust/src/lc/p518.rs)            | `Vec`                                            |
| 519. Random Flip Matrix                                           | [p519](./rust/src/lc/p519.rs)            | `Vec`,`rand`                                     |
| 520. Detect Capital                                               | [p520](./rust/src/lc/p520.rs)            | `String`                                         |
| 521. Longest Uncommon Subsequence I                               | [p521](./rust/src/lc/p521.rs)            ||
| 522. Longest Uncommon Subsequence II                               | [p522](./go/p522.go)            ||
| 532. K-diff Pairs in an Array                               | [p532](./rust/src/lc/p532.rs)            |`HashMap`|
| 537. Complex Number Multiplication                                | [p537](./rust/src/lc/p537.rs)            | `String.split`                                   |
| 539. Minimum Time Difference                                      | [p539](./rust/src/lc/p539.rs)            | `Vec`                                            |
| 540. Single Element in a Sorted Array                             | [p540](./rust/src/lc/p540.rs)            ||
| 553. Optimal Division                                             | [p553](./rust/src/lc/p553.rs)            | `format!`                                        |
| 556. Next Greater Element III          | [p556](./go/p556.go)            |                                        |
| 558.  Logical OR of Two Binary Grids Represented as Quad-Trees | [p558](./go/p558/p558.go)            |                                        |
| 563. Binary Tree Tilt                                             | [p563](./rust/src/lc/p563.rs)            | `Option`,`Rc`,`RefCell`,`abs`                    |
| 564. Find the Closest Palindrome                                  | [p564](./rust/src/lc/p564.rs)            | `FromStr`,`Vec.extend`,`Vec.extend_from_slice`   |
| 565. Array Nesting| [p565](./go/p566.go)            | |
| 575. Distribute Candies                                           | [p575](./rust/src/lc/p575.rs)            | `HashMap`,`std::cmp::min`                        |
| 592. Fraction Addition and Subtraction         [p592](./go/p592.go)            |                       |
| 594. Longest Harmonious Subsequence                               | [p594](./rust/src/lc/p594.rs)            | `BTreeMap`,`max`                                 |
| 598. Range Addition II                                            | [p598](./rust/src/lc/p598.rs)            | `Vec`,`min`                                      |
| 606. Construct String from Binary Tree                            | [p606](./rust/src/lc/p606.rs)            | `Rc`,`RefCell`                                   |
| 622. Design Circular Queue | [p622](./go/p622/p622.go)            |                                           |
| 623. Add One Row to Tree | [p623](./go/p623.go)            |                                           |
| 629. K Inverse Pairs Array                                        | [p629](./rust/src/lc/p629.rs)            | `Vec`                                            |
| 630. Course Schedule III                                          | [p630](./rust/src/lc/p630.rs)            | `Vec`,`sort_by`,`BinaryHeap`                     |
| 638. Shopping Offers                                              | [p638](./rust/src/lc/p638.rs)            | `Vec`,`Reference`,`std::cmp::min`                |
| 640. Solve the Equation  | [p640](./go/p640.go)            | |
| 641. Design Circular Deque  | [p641](./go/p641/p641.go)            | |
| 646. Maximum Length of Pair Chain  | [p646](./go/p646.go)            | |
| 648. Replace Words                                              | [p648](./go/p648.go)            | |
| 652. Find Duplicate Subtrees       | [p652](./go/p652.go)            | |
| 654. Maximum Binary Tree        | [p654](./go/p654.go)            | |
| 655. Print Binary Tree        | [p655](./go/p655.go)            | |
| 658. Find K Closest Elements        | [p658](./go/p658.go)            | |
| 661. Image Smoother                                               | [p661](./rust/src/lc/p661.rs)            | `const`                                          |
| 662. Maximum Width of Binary Tree | [p662](./go/p662.go)            |   |
| 667. Beautiful Arrangement II | [p667](./go/p667.go)            |   |
| 668. Kth Smallest Number in Multiplication Table                  | [p668](./rust/src/lc/p668.rs)            |                                                  |
| 669. Trim a Binary Search Tree | [p669](./go/p669.go)            |   |
| 670. Maximum Swap | [p670](./go/p670.go)            |             |
| 676. Implement Magic Dictionary | [p676](./go/p676/p676.go)            |             |
| 677. Map Sum Pairs                                                | [p677](./rust/src/lc/p677.rs)            | `HashMap`,`PartialEq`,`Eq`,`Default`             |
| 682. Baseball Game                                                | [p682](./rust/src/lc/p682.rs)            ||
| 686. Repeated String Match                                        | [p686](./rust/src/lc/p686.rs)            | `String`,`contains`                              |
| 687. Longest Univalue Path                                        | [p687](./go/p687.go)            ||
| 689. Maximum Sum of 3 Non-Overlapping Subarrays                   | [p689](./rust/src/lc/p689.rs)            | `Vec`                                            |
| 693. Binary Number with Alternating Bits                          | [p693](./rust/src/lc/p693.rs)            | `>>`                                             |
| 698. Partition to K Equal Sum Subsets                          | [p698](./go/p698.go)            |  |
| 699. Falling Squares                                              | [p699](./rust/src/lc/p699.rs)            |                                                  |
| 700. Search in a Binary Search Tree                               | [p700](./rust/src/lc/p700.rs)            | `Rc`,`RefCell`,`Option`                          |
| 707. Design Linked List   | [p707](./go/p707/p707.go)            |  |
| 709. To Lower Case                                                | [p709](./rust/src/lc/p709.rs)            | `match`                                          |
| 710. Random Pick with Blacklist  | [p710](./go//p710/p710.go)            |                                          |
| 717. 1-bit and 2-bit Characters                                   | [p717](./rust/src/lc/p717.rs)            ||
| 719. Find K-th Smallest Pair Distance                                   | [p719](./rust/src/lc/p719.rs)            |`BTreeMap`|
| 720. Longest Word in Dictionary                                   | [p720](./rust/src/lc/p720.rs)            | `HashSet`,`String.gt`                            |
| 728. Self Dividing Numbers                                        | [p728](./rust/src/lc/p728.rs)            | `HashSet`,`String.gt`                            |
| 729. My Calendar I                                        | [p729](./go/p729/p729.go)            |  |
| 730. Count Different Palindromic Subsequences                                       | [p730](./go/p730.rs)            |                            |
| 731. My Calendar II                                     | [p731](./rust/src/lc/p731.rs)            |  |
| 732. My Calendar III                                        | [p732](./rust/src/lc/p732.rs)            |  |
| 735. Asteroid Collision  | [p735](./g0/p735.go)            |  |
| 744. Find Smallest Letter Greater Than Target                     | [p744](./rust/src/lc/p744.rs)            ||
| 745. Prefix and Suffix Search                     | [p745](./go/p745/p745.go)            ||
| 747. Largest Number At Least Twice of Others                      | [p747](./rust/src/lc/p747.rs)            | `Vec`                                            |
| 748. Shortest Completing Word                                     | [p748](./rust/src/lc/p748.rs)            | `Vec`,`match`                                    |
| 754. Reach a Number        | [p754](./go/p754.go)            |                                     |
| 757. Set Intersection Size At Least Two         | [p757](./rust/src/lc/p757.rs)            |                                     |
| 762. Prime Number of Set Bits in Binary Representation            | [p762](./rust/src/lc/p762.rs)            ||
| 768. Max Chunks To Make Sorted II            | [p768](./go/p768.go)            ||
| 769. Max Chunks To Make Sorted            | [p769](./go/p769/p769.go)            ||
| 775. Global and Local Inversions| [p775](./go/p775.go)            ||
| 777. Swap Adjacent in LR String  | [p777](./go/p777.go)            ||
| 779. K-th Symbol in Grammar  | [p779](./go/p779.go)            ||
| 780. Reaching Points                                              | [p780](./rust/src/lc/p780.rs)            ||
| 784. Letter Case Permutation  | [p784](./go/p784.go)            ||
| 786. K-th Smallest Prime Fraction                                 | [p786](./rust/src/lc/p786.rs)            | `sort_by`,`PartialOrd`,`Ord`,`PartialEq`,`BinaryHeap`,`Reverse` |
| 788. Rotated Digits     |[p788](./go/p788.go)            | |
| 790. Domino and Tromino Tiling|[p790](./go/p790.go)            | |
| 791. Custom Sort String|[p791](./go/p791.go)            | |
| 792. Number of Matching Subsequences|[p792](./go/p792.go)            | |
| 794. Valid Tic-Tac-Toe State                                      | [p794](./rust/src/lc/p794.rs)            | `Vec`                                            |
| 795. Number of Subarrays with Bounded Maximum| [p795](./go/p795.go) |  |
| 796. Rotate String                                                | [p796](./rust/src/lc/p796.rs)            ||
| 798. Smallest Rotation with Highest Score                         | [p798](./rust/src/lc/p798.rs)            | `Vec`                                            |
| 799. Champagne Tower| [p799](./go/p799.go)            | |
| 804. Unique Morse Code Words                                      | [p804](./rust/src/lc/p804.rs)            | `HashSet`                                        |
| 806. Number of Lines To Write String                              | [p806](./rust/src/lc/p806.rs)            ||
| 807. Max Increase to Keep City Skyline                            | [p807](./rust/src/lc/p807.rs)            | `Vec`                                            |
| 809. Expressive Words| [p809](./go/p809.go)            | |
| 811. Subdomain Visit Count  | [p811](./go/p811.go)            | |
| 812. Largest Triangle Area                                        | [p812](./rust/src/lc/p812.rs)            | `f64`                                            |
| 813. Largest Sum of Averages| [p813](./go/p813.go)            |                                        |
| 814. Binary Tree Pruning                                       | [p814](./go/p814.go)            |                                        |
| 816. Ambiguous Coordinates | [p816](./go/p816.go)            |                                        |
| 817. Linked List Components     | [p817](./go/p817.go)            |                                        |
| 819. Most Common Word                                             | [p819](./rust/src/lc/p819.rs)            | `HashMap`                                        |
| 825. Friends Of Appropriate Ages                                  | [p825](./rust/src/lc/p825.rs)            | `Vec`                                            |
| 827. Making A Large Island      | [p827](./go/p827/p827.go)            ||
| 829. Consecutive Numbers Sum                                  | [p829](./rust/src/lc/p829.rs)            |  
| 831. Masking Personal Information  | [p831](./cpp/p831.h)            |                                        |                                             |
| 838. Push Dominoes                                                | [p838](./rust/src/lc/p838.rs)            | `Vec`                                            |
| 846. Hand of Straights                                            | [p846](./rust/src/lc/p846.rs)            | `BTreeMap`,`VecDeque`,`FromIterator`             |
| 851. Loud and Rich                                                | [p851](./rust/src/lc/p851.rs)            | `Vec`                                            |
| 852. Peak Index in a Mountain Array                               | [p852](./rust/src/lc/p852.rs)            | `Vec`                                            |
| 854. K-Similar Strings| [p854](./go/p854.go)            |  |
| 855. Exam Room| [p855](./cpp/p855.h)            |  |
| 856. Score of Parentheses| [p856](./go/p856.go)            |  |
| 859. Buddy Strings                                                | [p859](./rust/src/lc/p859.rs)            | `String`                                         |
| 868. Binary Gap                                                   | [p868](./rust/src/lc/p868.rs)            ||
| 869. Reordered Power of 2                                         | [p869](./rust/src/lc/p869.rs)            | `HashSet`,`BinaryHeap`,`Reverse`                 |
| 873. Length of Longest Fibonacci Subsequence    | [p873](./go/p873.go)            |               |
| 875. Koko Eating Bananas                                         | [p875](./go/p875.go)            |               |
| 878. Nth Magical Number| [p878](./go/p878.go)            |               |
| 883. Projection Area of 3D Shapes                                 | [p883](./rust/src/lc/p883.rs)            | `HashMap`                                        |
| 884. Uncommon Words from Two Sentences                            | [p884](./rust/src/lc/p884.rs)            | `HashMap`,`String`,`split`                       |
| 886. Possible Bipartition| [p886](./go/p886.go)            | |
| 890. Find and Replace Pattern                           | [p890](./go/p890.go)            |                       |
| 895. Maximum Frequency Stack| [p895](./go/p895.go)            |                       |
| 899. Orderly Queue                           | [p899](./rust/src/lc/p899.rs)            |                       |
| 897. Increasing Order Search Tree                           | [p897](./go/p897.go)            |                       |
| 901. Online Stock Span| [p901](./go/p901/p901.go)            ||
| 904. Fruit Into Baskets| [p904](./go/p904.go)            ||
| 905. Sort Array By Parity                                         | [p905](./rust/src/lc/p905.rs)            ||
| 907. Sum of Subarray Minimums | [p907](./go/p907.go)            ||
| 908. Smallest Range I                                             | [p908](./rust/src/lc/p908.rs)            ||
| 911. Online Election                                              | [p911](./rust/src/lc/p911.rs)            | `Vec`,`HashMap`,`binary_search`,`match`          |
| 915. Partition Array into Disjoint Intervals| [p915](./go/p915.go)            |  |
| 917. Number Of Ways To Reconstruct A Tree                         | [p917](./rust/src/lc/p917.rs)            | `Vec.swap`                                       |
| 919. Complete Binary Tree Inserter                         | [p919](./go/p919.go)            |                                      |
| 921. Minimum Add to Make Parentheses Valid   | [p921](./go/p921.go)            |                                        |
| 926. Flip String to Monotone Increasing                        | [p926](./go/p926.go)            |                                        |
| 927. Three Equal Parts   | [p927](./go/p927.go)            |                                        |
| 929. Unique Email Addresses                        | [p929](./go/p929.go)            |                                        |
| 935. Knight Dialer                                                | [p935](./rust/src/lc/p935.rs)            | `Vec`                                            |
| 937. Reorder Data in Log Files                                    | [p937](./rust/src/lc/p937.rs)            | `Vec`,`sort_by`                                  |
| 940. Distinct Subsequences II| [p940](./go/p940.go)            ||
| 942. DI String Match                                              | [p942](./rust/src/lc/p942.rs)            ||
| 944. Delete Columns to Make Sorted                                | [p944](./rust/src/lc/p944.rs)            ||
| 946. Validate Stack Sequences                                | [p944](./go/p946.go)            ||
| 953. Verifying an Alien Dictionary                                | [p953](./rust/src/lc/p953.rs)            | `HashMap`                                        |
| 954. Array of Doubled Pairs                                       | [p954](./rust/src/lc/p954.rs)            | `HashMap`                                        |
| 961. N-Repeated Element in Size 2N Array                          | [p961](./rust/src/lc/p961.rs)            |                                                |
| 965. Univalued Binary Tree                                        | [p965](./rust/src/lc/p965.rs)            |                                                |
| 969. Pancake Sorting                                              | [p969](./rust/src/lc/p969.rs)            | `Vec`,`Slice`                                    |
| 997. Find the Town Judge                                          | [p997](./rust/src/lc/p997.rs)            | `Vec`                                            |
| 982. Triples with Bitwise AND Equal To Zero| [p982](./cpp//p982.h)            ||
| 998. Maximum Binary Tree II                                          | [p998](./go//p998.go)            ||
| 1004. Max Consecutive Ones III                                    | [p1004](./rust/src/lc/p1004.rs)          ||
| 1005. Maximize Sum Of Array After K Negations                     | [p1005](./rust/src/lc/p1005.rs)          | `Vec`,`sort`,`binary_search`,`min`               |
| 1009. Complement of Base 10 Integer                               | [p1009](./rust/src/lc/p1009.rs)          | `>>`,`<<`                                        |
| 1017. Convert to Base -2| [p1017](./cpp/p1017.h)          |     |
| 1019. Next Greater Node In Linked List| [p1019](./cpp/p1019.h)          |     |
| 1020. Number of Enclaves                                          | [p1020](./rust/src/lc/p1020.rs)          | `Vec`                                            |
| 1021. Remove Outermost Parentheses                                | [p1021](./rust/src/lc/p1021.rs)          |                                                  |
| 1023. Camelcase Matching| [p1023](./cpp/p1023.h)          |                                             |
| 1034. Coloring A Border                                           | [p1034](./rust/src/lc/p1034.rs)          | `Vec`                                            |
| 1037. Valid Boomerang                                         | [p1037](./go/p1037.go)          |                                             |
| 1039. Minimum Score Triangulation of Polygon| [p1039](./cpp/p1039.h)          |                                             |
| 1041. Robot Bounded In Circle| [p1041](./cpp/p1041.h)          |                                             |
| 1042. Flower Planting With No Adjacent| [p1042](./cpp/p1042.h)          |                                             |
| 1051. Height Checker                                       | [p1051](./go/p1051.go)          |                                             |
| 1053. Previous Permutation With One Swap| [p1051](./cpp/p1053.h)          |                                             |
| 1078. Occurrences After Bigram                                    | [p1078](./rust/src/lc/p1078.rs)          | `Vec`,`String`,`match-if`                        |
| 1089. Duplicate Zeros                                    | [p1089](./go/p1089.go)          |     |
| 1106.  Parsing A Boolean Expression| [p1106](./go/p1106.go)          |     |
| 1108. Defanging an IP Address                                    | [p1108](./go/p1108.go)          |     |
| 1125. Smallest Sufficient Team| [p1125](./cpp/p1125.h)          |     |
| 1138. Alphabet Board Path                                  | [p1138](./cpp/p1138.h)          |     |
| 1144. Decrease Elements To Make Array Zigzag| [p1144](./cpp/p1144.h)          |     |
| 1145. Binary Tree Coloring Game| [p1145](./cpp/p1145.h)          |     |
| 1154. Day of the Year                                             | [p1154](./rust/src/lc/p1154.rs)          | `match`,`match-if`,`const`                       |
| 1161. Maximum Level Sum of a Binary Tree                                             | [p1161](./go/p1161.go)          |                        |
| 1175. Prime Arrangements                       | [p1175](./go/p1175.go)          | |
| 1184. Distance Between Bus Stops   | [p1184](./go/p1184.go)          |                                        |
| 1185. Day of the Week                                             | [p1185](./rust/src/lc/p1185.rs)          | `const`                                          |
| 1189. Maximum Number of Balloons                                  | [p1189](./rust/src/lc/p1189.rs)          | `HashMap`                                        |
| 1200. Minimum Absolute Difference                                  | [p1200](./go/p1200.go)          | |
| 1217. Minimum Cost to Move Chips to The Same Position          | [p1218](./go/p1217.go)          ||
| 1218. Longest Arithmetic Subsequence of Given Difference          | [p1218](./rust/src/lc/p1218.rs)          | `Vec`,`max`,`HashMap`                            |
| 1219. Path with Maximum Gold                                      | [p1219](./rust/src/lc/p1219.rs)          | `Vec`                                            |
| 1223. Remove Sub-Folders from the Filesystem| [p1223](./cpp/p1223.rs)          | |
| 1224. Maximum Equal Frequency    | [p1224](./rust/src/lc/p1224.rs)          | `HashMap`                                            |
| 1235. Maximum Profit in Job Scheduling| [p1235](./go/p1235.go)          |  |
| 1237. Find Positive Integer Solution for a Given Equation|[p1237](./cpp/p1237.h)          |     |
| 1238. Circular Permutation in Binary Representation|[p1238](./cpp/p1238.h)          |     |
| 1247. Minimum Swaps to Make Strings Equal |[p1247](./cpp/p1247.h)          |     |
| 1252. Cells with Odd Values in a Matrix   | [p1252](./go/p1252.go)          |                                          |
| 1260. Shift 2D Grid   | [p1260](./go/p1260.go)          |                                          |
| 1282. Group the People Given the Group Size They Belong To  | [p1282](./go/p1282.go)          |                                          |
| 1302. Deepest Leaves Sum  | [p1302](./go/p1302.go)          |                                          |
| 1331. Rank Transform of an Array  | [p1331](./go/p1331.go)          |                                          |
| 1332. Remove Palindromic Subsequences                             | [p1332](./rust/src/lc/p1332.rs)          | `String`                                         |
| 1342. Number of Steps to Reduce a Number to Zero                  | [p1342](./rust/src/lc/p1342.rs)          ||
| 1345. Jump Game IV                                                | [p1345](./rust/src/lc/p1345.rs)          | `HashMap`,`HashSet`,`VecDeque`                   |
| 1351. Count Negative Numbers in a Sorted Matrix                                               | [p1351](./go/p1351.go)          | |
| 1374. Generate a String With Characters That Have Odd Counts   | [p1374](./go/p1374.go)          | |
| 1376. Time Needed to Inform All Employees | [p1376](./cpp/p1376.h)          | |
| 1380. Lucky Numbers in a Matrix                                   | [p1380](./rust/src/lc/p1380.rs)          | `Vec`                                            |
| 1385.  Find the Distance Value Between Two Arrays                                   | [p1385](./go/p1385.go)          |                                            |
| 1403. Minimum Subsequence in Non-Increasing Order    | [p1403](./go/p1403.go)          ||
| 1405. Longest Happy String                                        | [p1405](./rust/src/lc/p1405.rs)          | `BinaryHeap`,`Vec`                               |
| 1408. String Matching in an Array  | [p1408](./go/p1408.go)          | |
| 1413. Minimum Value to Get Positive Step by Step Sum| [p1413](./go/p1413.go)          ||
| 1414. Find the Minimum Number of Fibonacci Numbers Whose Sum Is K | [p1414](./rust/src/lc/p1414.rs)          | `binary_search`                                  |
| 1417. Reformat The String | [p1417](./go/p1417.go)          |                                |
| 1422. Maximum Score After Splitting a String | [p1422](./go/p1422.go)          |                                |
| 1436. Destination City                                            | [p1436](./rust/src/lc/p1436/Solution.rs) | `String`,`Vec`,`HashMap`                         |
| 1441. Build an Array With Stack Operations  | [p1441](./go/p1441.go)          | |
| 1446. Consecutive Characters                                      | [p1446](./rust/src/lc/p1446.rs)          | `String`,`max`                                   |
| 1447. Simplified Fractions                                        | [p1447](./rust/src/lc/p1447.rs)          | `format!`                                        |
| 1450. Number of Students Doing Homework at a Given Time   | [p1450](./go/p1450.go)          |    |
| 1455. Check If a Word Occurs As a Prefix of Any Word in a Sentence   | [p1455](./go/p1455.go)          |    |
| 1460. Make Two Arrays Equal by Reversing Sub-arrays   | [p1460](./go/p1460.go)          |    |
| 1464. Maximum Product of Two Elements in an Array   | [p1464](./go/p1464.go)          |    |
| 1470. Shuffle the Array   | [p1470](./go/p1470.go)          |    |
| 1475. Final Prices With a Special Discount in a Shop   | [p1475](./go/p1475.go)          |    |
| 1487. Making File Names Unique | [p1487](./cpp/p1487.h)          |    |
| 1518. Water Bottles                                               | [p1518](./rust/src/lc/p1518.rs)          ||
| 1539. Kth Missing Positive Number                               | [p1539](./go/p1539.go)          ||
| 1574. Shortest Subarray to be Removed to Make Array Sorted| [p1574](./cpp/p1574.h)          ||
| 1576. Replace All ?'s to Avoid Consecutive Repeating Characters   | [p1576](./rust/src/lc/p1576.rs)          ||
| 1582. Special Positions in a Binary Matrix   | [p1582](./go/p1582.go)          ||
| 1590. Make Sum Divisible by P | [p1590](./cpp/p1590.h)          ||
| 1592. Rearrange Spaces Between Words  | [p1592](./go/p1592.go)          ||
| 1598. Crawler Log Folder  | [p1598](./go/p1598.go)          ||
| 1599. Maximum Profit of Operating a Centennial Wheel | [p1599](./cpp/p1599.h)          ||
| 1601. Maximum Number of Achievable Transfer Requests              | [p1601](./rust/src/lc/p1601.rs)          | `count_ones`,`iter.find`                         |
| 1604. Alert Using Same Key-Card Three or More Times in a One Hour Period|[p1604](./cpp/p1604.h)          |                             |
| 1605. Find Valid Matrix Given Row and Column Sums| [p1605](./cpp/p1605.h)          |   |
| 1608. Special Array With X Elements Greater Than or Equal X       |[p1608](./go/p1608.go)          |                             |
| 1609. Even Odd Tree                                               | [p1609](./rust/src/lc/p1609.rs)          | `Rc`,`RefCell`,`Vec`                             |
| 1610. Maximum Number of Visible Points                            | [p1610](./rust/src/lc/p1610.rs)          | `Vec`,`sort_by`,`std::f64::consts::PI`,`partial_cmp`,`atan2` |
| 1614. Maximum Nesting Depth of the Parentheses                    | [p1614](./rust/src/lc/p1614.rs)          | `Vec(stack)`                                     |
| 1615. Maximal Network Rank| [p1615](./cpp/p1615.h)          |   |
| 1616. Split Two Strings to Make Palindrome| [p1616](./cpp/p1616.h)          |     |
| 1619. Mean of Array After Removing Some Elements| [p1619](./go/p1619.go)          |                                 |
| 1620. Coordinate With Maximum Network Quality| [p1620](./go/p1620.go)          |                                 |
| 1624. Largest Substring Between Two Equal Characters| [p1624](./go/p1624.go)          |                                 |
| 1626. Best Team With No Conflicts| [p1626](./cpp/p1626.h)          |                                 |
| 1629. Slowest Key                                                 | [p1629](./rust/src/lc/p1629.rs)          | `Vec`,`String`                                   |
| 1630. Arithmetic Subarrays | [p1630](./cpp/p1630.h)          |  |
| 1631. Path With Minimum Effort                                    | [p1631](./rust/src/lc/p1631.rs)          | `Vec`,`BinaryHeap`,`min`,`max`,`Reverse`,`Tuple` |
| 1636. Sort Array by Increasing Frequency  | [p1636](./go/p1636.go)          |  |
| 1637. Widest Vertical Area Between Two Points Containing No Points | [p1637](./cpp/p1637.h)          |  |
| 1638. Count Substrings That Differ by One Character| [p1638](./cpp/p1638.h)          |  |
| 1640. Check Array Formation Through Concatenation  | [p1640](./go/p1640.go)          |  |
| 1641. Count Sorted Vowel Strings| [p1641](./cpp/p1641.h)          |  |
| 1652. Defuse the Bomb| [p1652](./go/p1652.go)          | |
| 1653. Minimum Deletions to Make String Balanced| [p1654](./cpp/p1653.h)          | |
| 1656. Design an Ordered Stream | [p1656](./go/p1656.go)          | |
| 1658. Minimum Operations to Reduce X to Zero| [p1658](./cpp/p1658.h)          | |
| 1662. Check If Two String Arrays are Equivalent| [p1662](./go/p1662.go)          | |
| 1663. Smallest String With A Given Numeric Value| [p1663](./cpp/p1663.h)          | |
| 1664. Ways to Make a Fair Array| [p1664](./cpp/p1664.h)          | |
| 1668. Maximum Repeating Substring| [p1668](./go/p1668.go)          | |
| 1669. Merge In Between Linked Lists| [p1669](./cpp/p1669.h)          | |
| 1672. Richest Customer Wealth                                     | [p1672](./rust/src/lc/p1672.rs)          | `Iter.map`,`Iter.sum`,`Iter.max`                 |
| 1684. Count the Number of Consistent Strings| [p1684](./go/p1684.go)          ||
| 1688. Count of Matches in Tournament                              | [p1688](./rust/src/lc/p1688.rs)          ||
| 1694. Reformat Phone Number| [p1694](./go/p1694.go)          ||
| 1700. Number of Students Unable to Eat Lunch | [p1700](./go/p1700.go)          | |
| 1704. Determine if String Halves Are Alike| [p1704](./go/p1704.go)          | |
| 1705. Maximum Number of Eaten Apples                              | [p1705](./rust/src/lc/p1705.rs)          | `Vec`,`Reverse`                                  |
| 1706. Where Will the Ball Fall                                    | [p1706](./rust/src/lc/p1706.rs)          | `Vec`                                            |
| 1710. Maximum Units on a Truck| [p1710](./go/p1710.go)          ||
| 1716. Calculate Money in Leetcode Bank                            | [p1716](./rust/src/lc/p1716.rs)          ||
| 1725. Number Of Rectangles That Can Form The Largest Square       | [p1725](./rust/src/lc/p1725.rs)          | `Vec`                                            |
| 1732. Find the Highest Altitude| [p1732](./go/p1732.go)  ||
| 1742. Maximum Number of Balls in a Box| [p1742](./go/p1742.go)          |   |
| 1748. Sum of Unique Elements                                      | [p1748](./rust/src/lc/p1748.rs)          | `Vec`                                            |
| 1752. Check if Array Is Sorted and Rotated| [p1752](./go/p1752.go)          |  |
| 1750. Minimum Length of String After Deleting Similar Ends| [p1750](./cpp/p1750.h)          |  |
| 1753. Maximum Score From Removing Stones| [p1753](./cpp/p1753.h)          |  |
| 1754. Largest Merge Of Two Strings| [p1754](./cpp/p1754.h)          |  |
| 1758. Minimum Changes To Make Alternating Binary String| [p1758](./go/p1758/p1758.go)          |  |
| 1759. Count Number of Homogenous Substrings| [p1759](./cpp/p1759.h)          |  |
| 1760. Minimum Limit of Balls in a Bag| [p1760](./cpp/p1760.h)          |  |
| 1763. Longest Nice Substring                                      | [p1763](./rust/src/lc/p1763.rs)          | `String`,`Slice`                                 |
| 1764. Form Array by Concatenating Subarrays of Another Array | [p1764](./go/p1764.go)          | |
| 1765. Map of Highest Peak                                         | [p1765](./rust/src/lc/p1765.rs)          | `VecDeque`                                       |
| 1768. Merge Strings Alternately | [p1768](./go/p1768.go)          | |
| 1769. Minimum Number of Operations to Move All Balls to Each Box| [p1769](./go/p17769.go)          | |
| 1773. Count Items Matching a Rule| [p1773](./go/p1773.go)          | |
| 1774. Closest Dessert Cost| [p1774](./go/p1774.go)          | |
| 1775. Equal Sum Arrays With Minimum Number of Operations| [p1775](./go/p1775.go)          | |
| 1779. Find Nearest Point That Has the Same X or Y Coordinate| [p1779](./go/p1779.go)          | |
| 1780. Check if Number is a Sum of Powers of Three                 | [p1780](./rust/src/lc/p1780.rs) [p1780-go](./go/p1780.go)         | `pow`                                            |
| 1781. Sum of Beauty of All Substrings| [p1781](./go/p1781.go)    |  |
| 1784. Check if Binary String Has at Most One Segment of Ones| [p1780](./go/p1784.go)          |                                          |
| 1785. Minimum Elements to Add to Form a Given Sum| [p1785](./cpp/p1785.h)          |                                          |
| 1790. Check if One String Swap Can Make Strings Equal| [p1790](./go/p1790.go)          ||
| 1791. Find Center of Star Graph                                   | [p1791](./rust/src/lc/p1791.rs)          ||
| 1792. Maximum Average Pass Ratio| [p1792](./cpp/p1792.h)          ||
| 1796. Second Largest Digit in a String| [p1796](./go/p1796.go)          ||
| 1797. Design Authentication Manager| [p1797](./cpp/p1797.h)          ||
| 1798. Maximum Number of Consecutive Values You Can Make| [p1798](./cpp/p1798.h)          ||
| 1800. Maximum Ascending Subarray Sum   | [p1800](./go/p1800.go)          |                                         |
| 1801. Number of Orders in the Backlog| [p1801](./cpp/p1801.h)          |                                         |
| 1802. Maximum Value at a Given Index in a Bounded Array| [p1802](./cpp/p1802.h)          |  |
| 1803. Count Pairs With XOR in a Range| [p1803](./cpp/p1803.h)          |  |
| 1805. Number of Different Integers in a String| [p1805](./cpp/p1805.h)          |                                         |
| 1806. Minimum Number of Operations to Reinitialize a Permutation| [p1806](./cpp/p1806.h)          |                                         |
| 1807. Evaluate the Bracket Pairs of a String| [p1807](./cpp/p1807.h)          |                                         |
| 1812. Determine Color of a Chessboard Square| [p1812](./go/p1812.go)          |                                         |
| 1813. Sentence Similarity III| [p1813](./go/p1813.go)          |                                         |
| 1814. Count Nice Pairs in an Array| [p1814](./go/p1814.go)          |                                         |
| 1816. Truncate Sentence                                           | [p1816](./rust/src/lc/p1816.rs)          | `String`                                         |
| 1817. Finding the Users Active Minutes| [p1817](./cpp/p1817.h)          |                                         |
| 1819. Number of Different Subsequences GCDs| [p1819](./cpp/p1819.h)          |  |
| 1823. Find the Winner of the Circular Game                        | [p1823](./rust/src/lc/p1823.rs)          ||
| 1824. Minimum Sideway Jumps| [p1824](./cpp/p1824.h)          |  |
| 1827. Minimum Operations to Make the Array Increasing| [p1827](./cpp/p1827.h)          ||
| 1828. Queries on Number of Points Inside a Circle| [p1828](./cpp/p1828.h)          ||
| 1832. Check if the Sentence Is Pangram| [p1832](./cpp/p1832.h)          ||
| 1882. Sign of the Product of an Array| [p1882](./go/p1882.go)          ||
| 1945. Sum of Digits of String After Convert   | [p1945](./cpp/p1945.h)          |  |
| 1971. Find if Path Exists in Graph| [p1971](./go/p1971.go)          ||
| 1984. Minimum Difference Between Highest and Lowest of K Scores   | [p1984](./rust/src/lc/p1984.rs)          | `Vec`                                            |
| 1995. Count Special Quadruplets                                   | [p1995](./rust/src/lc/p1995.rs)          | `String`                                         |
| 1996. The Number of Weak Characters in the Game                   | [p1996](./rust/src/lc/p1996.rs)          | `Vec`,`sort_by`                                  |
| 2000. Reverse Prefix of Word                                      | [p2000](./rust/src/lc/p2000.rs)          ||
| 2006. Count Number of Pairs With Absolute Difference K            | [p2006](./rust/src/lc/p2006.rs)          ||
| 2011. Final Value of Variable After Performing Operations | [p2011](./cpp/p2011.h) |  |
| 2013. Detect Squares                                              | [p2013](./rust/src/lc/p2013.rs)          | `HashMap`                                        |
| 2016. Maximum Difference Between Increasing Elements              | [p2016](./rust/src/lc/p2016.rs)          | `Vec`                                            |
| 2022. Convert 1D Array Into 2D Array                              | [p2022](./rust/src/lc/p2022.rs)          | `Vec`                                            |
| 2024. Maximize the Confusion of an Exam                           | [p204](./rust/src/lc/p2024.rs)           |                                                |
| 2027. Minimum Moves to Convert String | [p2027](./cpp/p2027.h)  | |
| 2028. Find Missing Observations                                   | [p2028](./rust/src/lc/p2028.rs)          | `Vec`                                            |
| 2029. Stone Game IX                                               | [p2022](./rust/src/lc/p2029.rs)          | `Vec`                                            |
| 2032. Two Out of Three| [p2032](./cpp/p2032.h)  | |
| 2034. Stock Price Fluctuation                                     | [p2034](./rust/src/lc/p2034.rs)          | `BinaryHeap`,`HashMap`,`Reverse`                 |
| 2037. Minimum Number of Moves to Seat Everyone| [p2037](./cpp/p2037.h)  | |
| 2038. Remove Colored Pieces if Both Neighbors are the Same Color  | [p2038](./rust/src/lc/p2038.rs)          ||
| 2039. The Time When the Network Becomes Idle                      | [p2039](./rust/src/lc/p2039.rs)          | `VecDeque`                                       |
| 2040. Most Frequent Even Element| [p2040](./cpp/p2040.h)  | |
| 2042. Check if Numbers Are Ascending in a Sentence| [p2042](./cpp/p2042.h)  | |
| 2043. Simple Bank System                                          | [p2043](./rust/src/lc/p2043.rs)          ||
| 2044. Count Number of Maximum Bitwise-OR Subsets                  | [p2044](./rust/src/lc/p2044.rs)          | `HashMap`,`HashSet`                              |
| 2047. Number of Valid Words in a Sentence                         | [p2047](./rust/src/lc/p2047.rs)          | `BinaryHeap`,`HashMap`,`Reverse`                 |
| 2049. Count Nodes With the Highest Score                          | [p2049](./rust/src/lc/p2049.rs)          | `BTreeMap`,`HashMap`,`VecDeque`                  |
| 2055. Plates Between Candles                                      | [p2055](./rust/src/lc/p2055.rs)          | `Vec.binary_search`                              |
| 2100. Find Good Days to Rob the Bank                              | [p2100](./rust/src/lc/p2100.rs)          | `Vec`                                            |
| 2104. Sum of Subarray Ranges                                      | [p2104](./rust/src/lc/p2104.rs)          ||
| 2180. Count Integers With Even Digit Sum| [p2180](./cpp/p2180.h)          ||
| 2283. Check if Number Has Equal Digit Count and Digit Value| [p2283](./cpp/p2283.h)          ||
| 2287. Rearrange Characters to Make Target String| [p2287](./cpp/p2287.h)          ||
| 2293. Min Max Game| [p2293](./cpp/p2293.h)          ||
| 2299. Strong Password Checker II| [p2299](./cpp/p2299.h)          ||
| 2303. Calculate Amount Paid in Taxes|[p2303](./cpp/p2303.h)  | |
| 2309. Greatest English Letter in Upper and Lower Case|[p2309](./cpp/p2309.h)  | |
| 2315. Count Asterisks|[p2315](./cpp/p2315.h)  | |
| 2319. Check if Matrix Is X-Matrix|[p2319](./cpp/p2319.h)  | |
| 2325. Decode the Message|[p2325](./cpp/p2325.h)  | |
| 2331. Evaluate Boolean Binary Tree|[p2331](./cpp/p2331.h)  | |
| 2335. Minimum Amount of Time to Fill Cups|[p2335](./cpp/p2335.h)  | |
| 2351. First Letter to Appear Twice |[p2351](./cpp/p2351.h)  | |
| 2347. Best Poker Hand|[p2347](./cpp/p2347.h)  | |
| 2357. Make Array Zero by Subtracting Equal Amounts|[p2357](./cpp/p2357.h)  | |
| 2363. Merge Similar Items |[p2363](./cpp/p2363.h)  | |
| 2367. Number of Arithmetic Triplets |[p2367](./cpp/p2367.h)  | |
| 2373. Largest Local Values in a Matrix |[p2373](./cpp/p2373.h)  | |
| 2379. Minimum Recolors to Get K Consecutive Black Blocks |[p2379](./cpp/p2379.h)  | |
| 2383. Minimum Hours of Training to Win a Competition|[p2383](./cpp/p2383.h)  | |
| 2389. Longest Subsequence With Limited Sum|[p2389](./cpp/p2389.h)  | |
| 2395. Find Subarrays With Equal Sum|[p2395](./cpp/p2395.h)  | |
| 2399. Check Distances Between Same Letters|[p2399](./cpp/p2399.h)  | |
| 2427. Number of Common Factors|[p2427](./cpp/p2427.h)  | |
| 2469. Convert the Temperature|[p2469](./cpp/p2469.h)  | |
| 2488. Count Subarrays With Median K|[p2488](./cpp/p2488.h)  | |

### Cracking the Coding Interview, 6th Edition
| Problem              | Solution                          |Related Rust keywords|
|----------------------|-----------------------------------|---------|
| 01.02. Check Permutation LCCI | [p0105](./go/lcci0102.go) ||
| 01.05. One Away LCCI | [p0105](./rust/src/interview/p0105.rs) ||
| 01.08. Zero Matrix LCCI | [p0108](./go/lcci0108.go) ||
| 05.02. Binary Number to String LCCI| [p0502](./cpp/lcci0502.h) ||
| 17.05. Find Longest Subarray LCCI | [p1705](./cpp/lcci1705.h) ||
| 17.11. Find Closest LCCI | [p1711](./rust/src/interview/p1711.rs) ||
| 17.19. Missing Two LCCI | [p1719](./go/lcci1711.go) ||

### 《剑指 Offer》题目
|Problem|Solution|Related Rust keywords|
|-------|---------|---------|
|剑指 Offer II 001. 整数除法|[p001](./rust/src/offer/p001.rs)|`abs`|
|剑指 Offer 03. 数组中重复的数字|[offer03](./go/offer03.go)||
|剑指 Offer 04. 二维数组中的查找|[offer04](./go/offer04.go)||
|剑指 Offer 06. 从尾到头打印链表|[offer06](./go/offer06.go)||
|剑指 Offer 47. 礼物的最大价值|[offer47](./cpp/offer47.h)||

|剑指 Offer II 029. 排序的循环链表|[offer029](./go/offer029.go)||
|剑指 Offer II 041. 滑动窗口的平均值|[offerII041](./go/offerII041.go)||
|剑指 Offer II 091. 粉刷房子|[p091](./go/offer091.go)||
|剑指 Offer II 115. 重建序列|[p115](./rust/src/offer/offerII115.rs)||
