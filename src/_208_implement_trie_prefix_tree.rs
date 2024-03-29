/*
 * @lc app=leetcode id=208 lang=rust
 *
 * [208] Implement Trie (Prefix Tree)
 */

// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Default, Debug)]
struct Trie {
    can_end: bool,
    next: [Option<Rc<RefCell<Trie>>>; 26],
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, mut word: String) {
        let (base, base_depth) = self.point(&*word);
        if word.is_empty()
            || (base_depth == word.len()
                && base.is_some()
                && base.as_ref().unwrap().borrow().can_end)
        {
            return;
        }
        if base_depth == word.len() {
            return base.as_ref().unwrap().borrow_mut().can_end = true;
        }
        let mut branch = Trie::new();
        branch.can_end = true;
        for _ in base_depth..word.len() - 1 {
            let c = word.pop().unwrap();
            let mut prev = Trie::new();
            prev.extend(c, branch);
            branch = prev;
        }
        let c = word.pop().unwrap();
        if let Some(node) = base {
            node.borrow_mut().extend(c, branch);
        } else {
            self.extend(c, branch);
        }
    }

    fn search(&self, word: impl AsRef<str>) -> bool {
        let (Some(node), depth) = self.point(&word) else {
            return false;
        };
        node.borrow().can_end && depth == word.as_ref().len()
    }

    fn starts_with(&self, prefix: impl AsRef<str>) -> bool {
        let (_, depth) = self.point(&prefix);
        depth == prefix.as_ref().len()
    }

    fn point(&self, prefix: impl AsRef<str>) -> (Option<Rc<RefCell<Trie>>>, usize) {
        let mut chars = prefix.as_ref().chars();
        let Some(c0) = chars.next() else {
            return (None, 0);
        };
        let Some(node) = self.get(c0) else {
            return (None, 0);
        };
        let (node, depth) = chars.fold((node.clone(), 1), |(node, depth), c| {
            node.borrow()
                .get(c)
                .map_or((node.clone(), depth), |next| (next.clone(), depth + 1))
        });
        (Some(node), depth)
    }

    fn get(&self, key: char) -> Option<&Rc<RefCell<Self>>> {
        self.next[(key as u8 - b'a') as usize].as_ref()
    }

    fn extend(&mut self, key: char, other: Self) {
        self.next[(key as u8 - b'a') as usize] = Some(Rc::new(RefCell::new(other)));
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
