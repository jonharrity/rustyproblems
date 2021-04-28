impl Solution {
    pub fn reverse(input: i32) -> i32 {
        let mut x = input;
        let isNeg = if x < 0 {true} else {false};
        
        if isNeg {
            x *= -1;
        }
        
        let mut vec = Vec::new();
        
        let mut m = 0;
        let mut res = 0;
        let mut prev = res;
        while x > 0 {
            m = x % 10;
            vec.insert(0, m);
            x /= 10;
            res *= 10;
            res += m;
            if ((res - m) / 10) != prev { // buffer overflow
                return 0;    
            }
            prev = res;
        }
        
        if isNeg {
            res *= -1;
        }
        return res;
    }
}
