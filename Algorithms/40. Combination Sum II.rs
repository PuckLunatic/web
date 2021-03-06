impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort();
        let mut combination: Vec<i32> = vec![];
        let mut res: Vec<Vec<i32>> = vec![];
        Self::_combination_sum2(&mut candidates, target, &mut res, &mut combination, 0);
        res
    }
    
    fn _combination_sum2(candidates: &mut Vec<i32>, target: i32, res: &mut Vec<Vec<i32>>, combination: &mut Vec<i32>, cursor: usize) {
        if target == 0 {
            res.push(combination.to_vec());
            return;
        }
        
        let mut i = cursor;
        
        while i < candidates.len() && target >= candidates[i] {
            if i == cursor || candidates[i] != candidates[i - 1] {
                combination.push(candidates[i]);
                Self::_combination_sum2(candidates, target - candidates[i], res, combination, i + 1);
                combination.pop();
            }
            i += 1;
        }
    }
}
