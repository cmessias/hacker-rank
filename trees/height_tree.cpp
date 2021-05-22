// https://www.hackerrank.com/challenges/tree-height-of-a-binary-tree/
// Tree declaration and provided by Hacker Rank

#include <bits/stdc++.h>

using namespace std;

class Node {
    public:
        int data;
        Node *left;
        Node *right;
        Node(int d) {
            data = d;
            left = NULL;
            right = NULL;
        }
};

class Solution {
    public:
  	Node* insert(Node* root, int data) {
            if(root == NULL) {
                return new Node(data);
            } else {
                Node* cur;
                if(data <= root->data) {
                    cur = insert(root->left, data);
                    root->left = cur;
                } else {
                    cur = insert(root->right, data);
                    root->right = cur;
                }

               return root;
           }
        }

        int height(Node* root) {
            return getHeight(root, 0);
        }
    
        int getHeight(Node* node, int height) {
            if (node == nullptr) {
                return height -1;
            } else {
                return max(
                    getHeight(node->left, height + 1), 
                    getHeight(node->right, height + 1)
                );
            }
        }

};

int main() {
    
    Solution myTree;
    Node* root = NULL;
    
    int t;
    int data;

    std::cin >> t;

    while(t-- > 0) {
        std::cin >> data;
        root = myTree.insert(root, data);
    }
  
    int height = myTree.height(root);
    std::cout << height;

    return 0;
}
