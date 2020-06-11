use std::collections::HashMap;

fn main() {
    let input = vec!["eat".to_string(), "tea".to_string(), "tan".to_string(), "ate".to_string(), "nat".to_string(), "bat".to_string()];
    let result = Solution::group_anagrams(input);

    println!("{:?}", result);
}



struct Solution {}

impl Solution {

    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut pair = HashMap::new(); 
        let mut pair_return = vec![];

        for str in strs.iter() {
            pair.entry(Self::sort_string(str)).or_insert_with(|| Vec::new()).push(str.to_string());
        }
        
        for (_, p) in pair.iter_mut() {
            p.sort();
            pair_return.push(p.to_vec());
        }

        pair_return.sort_by(|a, b| a.len().cmp(&b.len()).reverse());

        pair_return
    }

    pub fn sort_string(original_string: &String) -> String {
        let mut chars: Vec<char> = original_string.chars().collect();
        chars.sort();
        chars.into_iter().collect()
    }
}


#[test]
fn group_anagrams_should_work() {
    let input = vec!["eat".to_string(), "tea".to_string(), "tan".to_string(), "ate".to_string(), "nat".to_string(), "bat".to_string()];
    let ans = vec![
        vec!["ate".to_string(),"eat".to_string(),"tea".to_string()],
        vec!["nat".to_string(),"tan".to_string()],
        vec!["bat".to_string()]
    ];

    assert_eq!(ans, Solution::group_anagrams(input));
}