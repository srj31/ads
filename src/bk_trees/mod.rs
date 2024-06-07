use std::{cmp::min, collections::HashMap};

struct BKTreeNode {
    word: String,
    children: HashMap<u32, BKTreeNode>,
}

pub struct BKTree {
    head: BKTreeNode,
}

impl BKTree {
    fn new(root: String) -> BKTree {
        BKTree {
            head: BKTreeNode {
                word: root,
                children: HashMap::new(),
            },
        }
    }

    fn insert(&mut self, word: String) {
        let mut dist = calculate_distance(&self.head.word, &word);
        let mut current_node = &mut self.head;
        while current_node.children.contains_key(&dist) {
            current_node = current_node.children.get_mut(&dist).unwrap();
            dist = calculate_distance(&current_node.word, &word);
        }

        current_node.children.insert(
            dist,
            BKTreeNode {
                word,
                children: HashMap::new(),
            },
        );
    }

    fn search(&self, word: String, tolerance: u32) -> Vec<String> {
        let mut ans = vec![];
        let mut stack = vec![&self.head];

        while !stack.is_empty() {
            let cur = stack.pop().unwrap();
            let dist = calculate_distance(&cur.word, &word);
            if dist <= tolerance {
                ans.push(cur.word.clone());
            }
            println!("{}-{}: {} {}", word, cur.word, dist, tolerance);
            for child in &cur.children {
                if *child.0 <= dist + tolerance && child.0 + tolerance >= dist {
                    stack.push(child.1);
                }
            }
        }

        ans
    }
}

fn calculate_distance(a: &str, b: &str) -> u32 {
    let len_a = a.len();
    let len_b = b.len();
    let mut dp: Vec<Vec<u32>> = vec![vec![u32::MAX; len_b + 1]; len_a + 1];

    for i in 0..=len_a {
        dp[i][0] = i as u32;
    }

    for i in 1..=len_b {
        dp[0][i] = i as u32;
    }

    for i in 1..=len_a {
        for j in 1..=len_b {
            if a.chars().nth(i - 1) == b.chars().nth(j - 1) {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = min(min(dp[i - 1][j], dp[i][j - 1]), dp[i - 1][j - 1]) + 1;
            }
        }
    }

    println!(
        "{}
             ",
        dp[len_a][len_b]
    );
    dp[len_a][len_b]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_distance() {
        let a = "hello";
        let b = "hallo";
        assert_eq!(calculate_distance(a, b), 1);
    }

    #[test]
    fn test_calculate_distance2() {
        let a = "hello";
        let b = "hello";
        assert_eq!(calculate_distance(a, b), 0);
    }

    #[test]
    fn test_calculate_distance3() {
        let a = "hello";
        let b = "abcde";
        assert_eq!(calculate_distance(a, b), 5);
    }

    #[test]
    fn test_search() {
        let values = [
            "book", "books", "cake", "boo", "boon", "cook", "cape", "cart",
        ];

        let mut bk_tree = BKTree::new(values[0].to_string());

        for v in values.into_iter().skip(1) {
            bk_tree.insert(v.to_string());
        }

        let typed = "cool";
        let ans_1 = bk_tree.search(typed.to_string(), 1);
        let mut ans_2 = bk_tree.search(typed.to_string(), 2);
        ans_2.sort();

        assert_eq!(ans_1, vec!["cook"]);
        assert_eq!(ans_2, vec!["boo", "book", "boon", "cook"]);
    }
}
