/*
https://www.hackerrank.com/challenges/ctci-is-binary-search-tree/

The Node struct is defined as follows:
    struct Node {
        int data;
        Node* left;
        Node* right;
    }
*/

#include <climits>

bool checkBST(Node* n, int min = INT_MIN, int max = INT_MAX) {
    if (n == nullptr) return true;
    if (n->data <= min || n->data >= max) return false;
    return checkBST(n->left, min, n->data) && checkBST(n->right, n->data, max);
}
