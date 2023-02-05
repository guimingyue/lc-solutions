/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution {
public:
    ListNode* mergeInBetween(ListNode* list1, int a, int b, ListNode* list2) {
        ListNode newHead(-1);
        newHead.next = list1;
        ListNode* p = &newHead;
        int idx = 0;
        while (idx < a) {
            idx++;
            p = p->next;
        }
        ListNode* q = p;
        while (idx <= b && p != nullptr) {
            idx++;
            p = p->next;
        }
        q->next = list2;
        ListNode* r = list2;
        while (r->next != nullptr) {
            r = r->next;
        }
        r->next = p->next;
        return newHead.next;
    }
};