use std::cmp::Ordering;
use std::fmt::Debug;

t sub-tree are larger than `v`.
#[derive(Debug)]
pub struct Tree<T>(Option<Box<Node<T>>>);

#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Tree<T>,
    right: Tree<T>,
}

#[derive(Debug)]
pub enum TreeOpError {
    NoValue,
    ValueAlreadyExists,
    NoneTree,
}

impl<T> Tree<T>
where
    T: Ord + Debug,
{
    pub fn new() -> Self {
        Tree(None)
    }

    fn leaf(value: T) -> Self {
        Tree(Some(Box::new(Node {
            value,
            left: Tree(None),
            right: Tree(None),
        })))
    }

    pub fn insert(&mut self, value: T) -> Result<(), TreeOpError> {
        match self.0 {
            Some(ref mut n) => match value.cmp(&n.value) {
                Ordering::Equal => Err(TreeOpError::ValueAlreadyExists),
                Ordering::Less => n.left.insert(value),
                Ordering::Greater => n.right.insert(value),
            },
            None => {
                *self = Tree::leaf(value);
                Ok(())
            }
        }
    }

    pub fn contains(&self, target: T) -> bool {
        match self.0 {
            Some(ref n) => match target.cmp(&n.value) {
                Ordering::Equal => true,
                Ordering::Less => n.left.contains(target),
                Ordering::Greater => n.right.contains(target),
            },
            None => false,
        }
    }

    pub fn add_to_end(&mut self, tree: Tree<T>) -> Result<(), TreeOpError> {
        
        let target = if tree.0.is_some() {
            &tree.0.as_ref().unwrap().value
        } else {
            return Err(TreeOpError::NoneTree);
        };

        match self.0 {
            Some(ref mut n) => match target.cmp(&n.value) {
                Ordering::Equal => Err(TreeOpError::ValueAlreadyExists),
                Ordering::Less => n.left.add_to_end(tree),
                Ordering::Greater => n.right.add_to_end(tree),
            },
            None => {
                *self = tree;
                Ok(())
            }
        }
    }

    pub fn delete(&mut self, target: &T) -> Result<(), TreeOpError> {
        let Node { value, left, right } = match &mut self.0 {
            Some(n) => n.as_mut(),
            None => return Err(TreeOpError::NoValue),
        };

        match target.cmp(value) {
            Ordering::Equal => {
                match left.inorder_predecessor() {
                    Some(predecessor) => *value = predecessor,
                    None => self.0 = right.0.take(),
                }
                Ok(())
            }
            Ordering::Less => left.delete(target),
            Ordering::Greater => right.delete(target),
        }
    }

    pub fn inorder_predecessor(&mut self) -> Option<T> {
        match &mut self.0 {
            Some(n) => match n.right.inorder_predecessor() {
                Some(child) => Some(child),
                None => {
                    let predecessor = self.0.take().unwrap();
                    self.0 = predecessor.left.0;
                    Some(predecessor.value)
                }
            },
            None => None,
        }
    }
}

impl<T> Default for Tree<T>
where
    T: Ord + Debug,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> PartialEq for Tree<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self.0.as_ref(), other.0.as_ref()) {
            (Some(a), Some(b)) => a.value == b.value,
            (None, None) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty() {
        let bst: Tree<i32> = Tree::new();
        assert_eq!(bst, Tree::<i32>::new());
    }

    #[test]
    fn inserts() {
        let mut bst = Tree::new();
        assert!(bst.insert(2).is_ok());
    }

    #[test]
    fn leaf() {
        let mut bst = Tree::new();
        bst.insert(2).expect("Failed to insert");
        assert_eq!(bst, Tree::leaf(2));
    }

    #[test]
    fn contain() {
        let mut bst = Tree::new();
        bst.insert(2).expect("Failed to insert");
        bst.insert(23).expect("Failed to insert");
        bst.insert(20).expect("Failed to insert");
        assert_eq!(bst.contains(23), true);
        assert_eq!(bst.contains(29), false);
    }

    #[test]
    fn remove() {
        let mut bst = Tree::new();
        bst.insert(2).expect("Failed to insert");
        bst.insert(23).expect("Failed to insert");
        bst.insert(20).expect("Failed to insert");
        assert!(bst.delete(&2).is_ok());
        assert!(bst.delete(&23).is_ok());
        assert!(bst.delete(&2).is_err());
    }
}
