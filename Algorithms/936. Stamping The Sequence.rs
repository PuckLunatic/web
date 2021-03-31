impl Solution {
    pub fn moves_to_stamp(stamp: String, mut target: String) -> Vec<i32> {
        fn can_replace(t: &[u8], p: usize, s: &[u8]) -> bool {
            for i in 0..s.len() {
                if t[i + p] != b'*' && t[i + p] != s[i] {
                    return false;
                }
            }
            true
        }

        fn do_replace(t: &mut [u8], p: usize, len: usize) -> usize {
            let mut count = 0;
            for i in 0..len {
                if t[i + p] != b'*' {
                    t[i + p] = b'*';
                    count += 1;
                }
            }
            count
        }

        let s = stamp.as_bytes();
        let t = unsafe { target.as_bytes_mut() };
        if t.len() < s.len() {
            return vec![];
        }
        let mut ans = vec![];

        let mut visited = vec![false; t.len()];
        let mut stars = 0;

        while stars < t.len() {
            let mut replace_done = false;
            for i in 0..=t.len() - s.len() {
                if !visited[i] && can_replace(t, i, s) {
                    stars += do_replace(t, i, s.len());
                    replace_done = true;
                    visited[i] = true;
                    ans.push(i as i32);
                    if stars == t.len() {
                        break;
                    }
                }
            }
            if !replace_done {
                return vec![];
            }
        }

        ans.reverse();
        ans
    }
}
