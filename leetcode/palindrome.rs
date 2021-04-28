// https://leetcode.com/problems/palindrome-number/

impl Solution {
    pub fn is_palindrome(input: i32) -> bool {
        let mut x = input;
        
        if x < 0 {
            return false;
        }
        
        if x < 10 {
            return true;
        }
        
        let mut vec = Vec::new();
        let mut m = 0;
        while x > 0 {
            m = x % 10;
            vec.push(m);
            x /= 10;
        }
                
        let size = vec.len();
        let mut i = 0;
        let mut j = size - i - 1;
        while i < j {
            if vec[i] != vec[j] {
                return false;
            }
            i += 1;
            j = size - i - 1;
        }
        
        return true;
    }
}
