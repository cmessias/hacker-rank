// https://www.hackerrank.com/challenges/ctci-linked-list-cycle
// List definition provided by Hacker Rank.

/*
Detect a cycle in a linked list. Note that the head pointer may be 'NULL' if the list is empty.

A Node is defined as: 
    struct Node {
        int data;
        struct Node* next;
    }
*/

bool has_cycle(Node* head) {
    if (head == nullptr) {
        return false;
    }
    
    for (Node* tortoise = head, * hare = head->next;
        hare != nullptr && hare->next != nullptr;
        tortoise = tortoise->next, hare = hare->next->next)
    {
        if (hare == tortoise) {
            return true;
        }
    }
    
    return false;
}
