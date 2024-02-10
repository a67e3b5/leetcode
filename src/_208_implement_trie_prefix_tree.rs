/*
 * @lc app=leetcode id=208 lang=rust
 *
 * [208] Implement Trie (Prefix Tree)
 */

// @lc code=start
#[derive(Default)]
struct Trie {
    can_end: bool,
    next: [Option<Box<Trie>>; 26],
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
        let (base, base_depth) = self.point_mut(&*word);
        let is_prefix_of_already_inserted = base_depth == word.len();
        if word.is_empty() || (is_prefix_of_already_inserted && base.can_end) {
            return;
        }
        if is_prefix_of_already_inserted {
            return base.can_end = true;
        }
        let mut branch = Trie::new();
        branch.can_end = true;
        for _ in base_depth..word.len() - 1 {
            let c = word.pop().unwrap();
            let mut prev = Trie::new();
            prev.next[(c as u8 - b'a') as usize] = Some(Box::new(branch));
            branch = prev;
        }
        let c = word.pop().unwrap();
        base.next[(c as u8 - b'a') as usize] = Some(Box::new(branch));
    }

    fn search(&self, word: impl AsRef<str>) -> bool {
        let (node, depth) = self.point(&word);
        depth == word.as_ref().len() && node.can_end
    }

    fn starts_with(&self, prefix: impl AsRef<str>) -> bool {
        let (_, depth) = self.point(&prefix);
        depth == prefix.as_ref().len()
    }

    fn point(&self, prefix: impl AsRef<str>) -> (&Trie, usize) {
        let mut node = self;
        let mut depth = 0;
        for c in prefix.as_ref().chars() {
            let Some(ref next) = node.next[(c as u8 - b'a') as usize] else {
                break;
            };
            node = next.as_ref();
            depth += 1;
        }
        (node, depth)
    }

    fn point_mut(&mut self, prefix: impl AsRef<str>) -> (&mut Trie, usize) {
        let mut node = self;
        let mut depth = 0;
        for c in prefix.as_ref().chars() {
            let Some(ref mut next) = node.next[(c as u8 - b'a') as usize] else {
                break;
            };
            node = next.as_mut();
            depth += 1;
        }
        (node, depth)
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
