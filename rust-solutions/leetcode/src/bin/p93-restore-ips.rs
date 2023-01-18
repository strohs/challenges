/// # Problem 93 - Restore IP Addresses
/// https://leetcode.com/problems/restore-ip-addresses/
///

pub fn restore_ip_addresses(s: String) -> Vec<String> {
    fn valid_ip(ip: &str) -> bool {
        if ip.len() > 3 || (ip.len() > 1 && ip.starts_with("0")) {
            false
        } else {
            return ip.parse::<u8>().map_or(false, |_n| true);
        }
    }

    /// depth = current recursion depth
    /// i = index of s that is being examined
    /// s = the ip address string itself
    /// cur = contains slices of valid ip addresses for the current branch, it will never be greater than 4
    /// finals = contains the final, valid, ip addresses from all recursions
    fn recurse(depth: u8, i: usize, s: &str, cur: Vec<&str>, finals: &mut Vec<String>) {
        if depth == 5 {
            if i >= s.len() {
                // no more chars to examine in s, cur will have a valid ip that should be pushed to finals
                finals.push(cur.join("."));
                return
            } else {
                // we are at depth 5, but there are more chars left, we can't possibly build a valid ip address from this recursive branch
                return
            }
        } else {
            // not at depth 5, continue collecting substrings and recurse on them
            if i < s.len() {
                let sub = &s[i..i+1];
                if valid_ip(sub) {
                    let mut new_cur = cur.clone();
                    new_cur.push(sub);
                    recurse(depth + 1, i + 1, s, new_cur, finals);
                }
            }
            if i + 1 < s.len() {
                let sub = &s[i..i+2];
                if valid_ip(sub) {
                    let mut new_cur = cur.clone();
                    new_cur.push(sub);
                    recurse(depth + 1, i + 2, s, new_cur, finals);
                }
            }
            if i + 2 < s.len() {
                let sub = &s[i..i+3];
                if valid_ip(sub) {
                    let mut new_cur = cur.clone();
                    new_cur.push(sub);
                    recurse(depth + 1, i + 3, s, new_cur, finals);
                }
            }
        }
    }

    if s.len() < 4 || s.len() > 12 {
        return vec![]
    }

    let mut finals: Vec<String> = Vec::new();
    recurse(1, 0, &s, vec![], &mut finals);

    finals
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::restore_ip_addresses;

    #[test]
    fn test_all_zeros() {
        let s = String::from("0000");
        let valids = restore_ip_addresses(s);
        dbg!(valids);
    }

    #[test]
    fn test1() {
        let s = String::from("25525511135");
        let valids = restore_ip_addresses(s);
        assert_eq!(valids.len(), 2);
        assert!(valids.contains(&"255.255.11.135".to_string()));
        assert!(valids.contains(&"255.255.111.35".to_string()));
    }

    #[test]
    fn test3() {
        let s = String::from("101023");
        let valids = restore_ip_addresses(s);
        assert_eq!(valids.len(), 5);
        dbg!(&valids);
    }

    #[test]
    fn test4() {
        let s = String::from("1231231231");
        let valids = restore_ip_addresses(s);
        dbg!(&valids);
    }
}