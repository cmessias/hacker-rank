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

bool checkBST(Node* root) {
    return checkBST(root, INT_MIN, INT_MAX);
}

bool checkBST(Node* node, int minData, int maxData) {
    if (node == nullptr) return true;
    if (node->data < minData || node->data > maxData) return false;
    return checkBST(node->left, minData, node->data-1)
        && checkBST(node->right, node->data+1, maxData);
}
