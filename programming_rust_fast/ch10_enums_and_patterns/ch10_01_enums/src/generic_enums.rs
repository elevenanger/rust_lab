use crate::generic_enums::generic_enums::BinaryTree;

pub mod generic_enums {
    use crate::generic_enums::generic_enums::BinaryTree::{Empty, NonEmpty};

    /// 泛型枚举，
    /// 二叉树
    #[derive(Debug)]
    pub enum BinaryTree<T> {
        Empty,
        NonEmpty(Box<TreeNode<T>>),
    }

    #[derive(Debug)]
    pub struct TreeNode<T> {
        element: T,
        left: BinaryTree<T>,
        right: BinaryTree<T>,
    }

    impl<T:Ord> BinaryTree<T> {
        pub fn add(&mut self, value: T) {
            match *self {
                Empty => {
                    *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                        element: value,
                        left: BinaryTree::Empty,
                        right: BinaryTree::Empty
                    }))
                },
                NonEmpty(ref mut node) => {
                    if value <= node.element {
                        node.left.add(value);
                    } else {
                        node.right.add(value);
                    }
                }
            }
        }
    }

    #[test]
    fn test_binary_tree() {
        use self::BinaryTree;
        let jupiter_tree = NonEmpty(Box::new(TreeNode {
            element: "Jupiter",
            left: Empty,
            right: Empty
        }));

        let mercury_tree = NonEmpty(Box::new(TreeNode {
            element: "Mercury",
            left: Empty,
            right: Empty
        }));

        let mars_tree = NonEmpty(Box::new(TreeNode {
            element: "Mars",
            left: jupiter_tree,
            right: mercury_tree
        }));

        println!("mars_tree => {:?}", mars_tree);
    }
}

#[test]
fn test_add() {
    number_tree.add(10);
    number_tree.add(20);
    number_tree.add(5);
    number_tree.add(3);
    number_tree.add(6);

    println!("number tree => {:?}", number_tree);
}