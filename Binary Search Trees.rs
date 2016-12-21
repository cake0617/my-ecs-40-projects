#[derive(PartialEq, Debug)]
pub struct TreeNode<T>{
    key:T,

    l: Option<Box<TreeNode<T>>>,

    r: Option<Box<TreeNode<T>>>,
}

#[derive(PartialEq, Debug)]
pub struct Tree<T> {
    root: Option<Box<TreeNode<T>>>,
}

impl<T: Ord+ PartialEq> Tree<T> {
    /// Creates an empty tree
    pub fn new() -> Self {
        Tree{
            root: None
        }
    }

    /// Returns `false` if `key` already exists in the tree, and `true` otherwise.
    pub fn insert(&mut self, key: T) -> bool
    {

        match self.root{
            None => {
                self.root = Some(TreeNode::new(key).into());
                return true
            },

            Some(ref mut a) => {
                a.insert(key)
            }
        }

    }

    /// Returns `true` if `key` exists in the tree, and `false` otherwise.
    pub fn find(&self, key: &T) -> bool {
        match self.root{
            None => return false,
            Some(ref a) => a.find(key),
        }
    }

    /// Returns the preorder traversal of the tree.
    pub fn preorder(&self) -> Vec<&T> {

        match self.root{
            None => panic!("no!"),
            Some(ref a) => a.preorder(),
        }
    }

    /// Returns the inorder traversal of the tree.
    pub fn inorder(&self) -> Vec<&T> {
        match self.root{
            None => panic!("no!"),
            Some(ref a) => a.inorder(),
        }
    }

    /// Returns the postorder traversal of the tree.
    pub fn postorder(&self) -> Vec<&T> {

        match self.root{
            None => panic!("no!"),
            Some(ref a) => a.postorder(),
        }
    }
}

impl<T: Ord> TreeNode<T> {

    pub fn new(key:T) -> Self {
        TreeNode{
            key : key,
            l: None,
            r: None,
        }
    }
    pub fn insert(&mut self, key: T) -> bool {
        if self.key == key {
            return false
        }

        if key > self.key {
            match self.r {
                None => {
                    self.r = Some(TreeNode::new(key).into());
                    return true
                },
                Some(ref mut b) => {
                    b.insert(key)
                }
            }
        } else {
            match self.l {
                None => {
                    self.l = Some(TreeNode::new(key).into());
                    return true
                },
                Some(ref mut b) => {
                    b.insert(key)
                }
            }
        }
    }

    /// Returns `true` if `key` exists in the tree, and `false` otherwise.
    pub fn find(&self, key: &T) -> bool {
        if self.key == *key {
            return true
        }

        if key > &self.key {
            match self.r {
                None => {
                    return false
                },
                Some(ref b) => {
                    b.find(key)
                }
            }
        } else {
            match self.l {
                None => {
                    return false
                },
                Some(ref b) => {
                    b.find(key)
                },
            }
        }

    }

    /// Returns the preorder traversal of the tree.
    pub fn preorder(&self) -> Vec<&T> {
        let mut stack = Vec::new();

        stack.push(&self.key);

        match self.l {
            None => (),
            Some(ref a) => {
                stack.extend(a.preorder().iter().clone());
            },
        }

        match self.r {
            None => (),
            Some(ref a) => {
                stack.extend(a.preorder().iter().clone());
            },
        }

        return stack
    }

    /// Returns the inorder traversal of the tree.
    pub fn inorder(&self) -> Vec<&T> {
        let mut stack = Vec::new();

        match self.l{
            None => (),
            Some(ref a ) =>{
                stack.extend(a.inorder().iter().clone());
            }
        }

        stack.push(&self.key);

        match self.r {
            None => (),
            Some(ref a) => {
                stack.extend(a.inorder().iter().clone());
            },
        }

        return stack


    }

    /// Returns the postorder traversal of the tree.
    pub fn postorder(&self) -> Vec<&T> {
        let mut stack = Vec::new();

        match self.l{
            None => (),
            Some(ref a ) =>{
                stack.extend(a.postorder().iter().clone());
            }
        }

        match self.r {
            None => (),
            Some(ref a) => {
                stack.extend(a.postorder().iter().clone());
            },
        }

        stack.push(&self.key);

        return stack
    }
}
