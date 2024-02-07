/*
 * @lc app=leetcode id=208 lang=rust
 *
 * [208] Implement Trie (Prefix Tree)
 */

// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;

struct Trie {
    root: Vec<Rc<RefCell<Node>>>,
}

struct Node {
    char: char,
    next: Vec<Rc<RefCell<Node>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Self { root: vec![] }
    }

    fn insert(&mut self, mut word: String) {
        if word.is_empty() || self.search(&*word) {
            return;
        }
        word.push('\n');
        let mut next = vec![];
        while let Some(char) = word.pop() {
            let branch = Node { char, next };
            next = vec![Rc::new(RefCell::new(branch))];
            if let Some(node) = self.point(&*word) {
                return node.borrow_mut().next.append(&mut next);
            }
        }
        self.root.append(&mut next)
    }

    fn search(&self, word: impl AsRef<str>) -> bool {
        if let Some(node) = self.point(word) {
            node.as_ref()
                .borrow()
                .next
                .iter()
                .any(|node| node.as_ref().borrow().char == '\n')
        } else {
            false
        }
    }

    fn starts_with(&self, prefix: impl AsRef<str>) -> bool {
        self.point(prefix).is_some()
    }

    fn point(&self, prefix: impl AsRef<str>) -> Option<Rc<RefCell<Node>>> {
        let mut chars = prefix.as_ref().chars();
        let c0 = chars.next()?;
        let curr = self.root.iter().find(|rc| rc.as_ref().borrow().char == c0);
        Self::_point(&mut chars, curr)
    }

    fn _point(
        chars: &mut impl Iterator<Item = char>,
        curr: Option<&Rc<RefCell<Node>>>,
    ) -> Option<Rc<RefCell<Node>>> {
        let node = curr.as_ref()?.as_ref().borrow();
        let Some(c) = chars.next() else {
            return curr.cloned();
        };
        let curr = node.next.iter().find(|rc| rc.as_ref().borrow().char == c);
        Self::_point(chars, curr)
    }
}

/*
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // Input:
        // ["Trie","insert","search","search","startsWith","insert","search"]
        // [[],["apple"],["apple"],["app"],["app"],["app"],["app"]]
        // Output:
        // [null,null,true,false,true,null,true]
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
        assert_eq!(trie.search("apple"), true);
        assert_eq!(trie.search("app"), false);
        assert_eq!(trie.starts_with("app"), true);
        trie.insert("app".to_string());
        assert_eq!(trie.search("app"), true);
    }
}
