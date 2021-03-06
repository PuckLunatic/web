impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        let mut set: Vec<bool> = vec![false; bound as usize + 1];
        let mut i = 0;
        while x.pow(i) < bound {
            let mut j = 0;
            while y.pow(j) < bound {
                let sum = x.pow(i) + y.pow(j);
                if sum <= bound {
                    set[sum as usize] = true;
                }
                j += 1;
                if y == 1 {
                    break;
                }
            }
            i += 1;
            if x == 1 {
                break;
            }
        }
        
        let mut res = vec![];
        for i in 0..=bound {
            if set[i as usize] {
                res.push(i);
            }
        }
        res
    }
}
