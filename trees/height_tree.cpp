/* https://www.hackerrank.com/challenges/tree-height-of-a-binary-tree

The tree node has data, left child and right child 
class Node {
    int data;
    Node* left;
    Node* right;
};
*/
    
int height(Node* node, int h = 0) {
    if (node == nullptr) return -1;
    return 1 + max(height(node->left, h + 1), height(node->right, h + 1));
}
