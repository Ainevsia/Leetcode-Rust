use std::{arch::x86_64::_MM_FROUND_CUR_DIRECTION, vec};

fn main() {
    Solution::length_longest_path("a".to_owned());
}

struct Solution {}

impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        let mut res = 1;
        let mut cur_dir = vec![];
        for s in input.split('\n') {
            let entry_len = s.split('\t').last().unwrap().len() as i32 + 1;
            let depth = s.len() + 1 - entry_len as usize;
            while cur_dir.len() > depth { cur_dir.pop(); }
            if s.contains(".") {
                let cur_path_len = cur_dir.iter().sum::<i32>() + entry_len;
                if cur_path_len > res { res = cur_path_len }
            } else {
                cur_dir.push(entry_len);
            }
        }
        res - 1
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::length_longest_path("dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext".to_owned()),20);
        assert_eq!(Solution::length_longest_path("dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext".to_owned()),32);
        assert_eq!(Solution::length_longest_path("a".to_owned()),0);
        assert_eq!(Solution::length_longest_path("file1.txt\nfile2.txt\nlongfile.txt".to_owned()),12);
        assert_eq!(Solution::length_longest_path("a.t".to_owned()),3);
        assert_eq!(Solution::length_longest_path("dir\n        file.txt".to_owned()),16);
        // dir
        //         file.txt

    }
}