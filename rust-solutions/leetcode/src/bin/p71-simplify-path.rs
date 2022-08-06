/// # Problem 70 - Simplify Path
/// Given a string path, which is an absolute path (starting with a slash '/') to a file or
/// directory in a Unix-style file system, convert it to the simplified canonical path.
///
/// In a Unix-style file system, a period '.' refers to the current directory, a double
/// period '..' refers to the directory up a level, and any multiple consecutive slashes
/// (i.e. '//') are treated as a single slash '/'. For this problem, any other format of
/// periods such as '...' are treated as file/directory names.
///
/// The canonical path should have the following format:
///
///     The path starts with a single slash '/'.
///     Any two directories are separated by a single slash '/'.
///     The path does not end with a trailing '/'.
///     The path only contains the directories on the path from the root directory to the target file or directory (i.e., no period '.' or double period '..')
///
/// Return the simplified canonical path.


// rather than doing multiple find and replace, we'll build a separate canonical string
// so we only have to iterate once.
// walk through path, char by char, keeping track of last index of slash in canonical (lsi)
//  if '/', mark its pos (bi), and check for the following patterns:
//      if bi+1 == EOL then finished
//      if bi+1 == './' or .EOL, push a single '/' onto canonical,  and repeat starting from bi+2
//      if bi+1 == '../' or ..EOL, then:
//          if lsi == 0, do nothing cause at root,
//          else replace all chars in canonical from lsi.. with '', re-find and set the lsi, repeat from bi+3
//      if bi+1 == any_other_chars, push char, increment bi, and repeat main loop until next / is found
pub fn simplify_path(path: String) -> String {
    let mut canonical = String::new();
    let mut lsi: usize = 0;
    let mut bi = 0;

    let valid_dot = |i: usize|
        matches!(path.get(i..i+2), Some("./")) ||
            matches!(path.get(i..i+1), Some(".") if i + 1 >= path.len());

    let valid_dotdot = |i: usize|
        matches!(path.get(i..i+3), Some("../")) ||
            matches!(path.get(i..i+2), Some("..") if i + 2 >= path.len());


    while bi < path.len() {
        let cur_char = &path[bi..bi + 1];
        if cur_char == "/" {
            // check .. before .
            if valid_dotdot(bi+1) {
                canonical.replace_range(lsi.., "");
                lsi = canonical.rfind("/").unwrap_or(0);
                bi += 3;
            } else if valid_dot(bi+1) {
                bi += 2;
            } else if path.get(bi+1..bi+2) == Some("/") {
                // do nothing, next char is a "/"
                bi += 1;
            } else {
                // push the slash
                canonical.push_str(cur_char);
                lsi = canonical.len() - 1;
                bi += 1;
            }
        } else {
            // push any other chars
            canonical.push_str(cur_char);
            bi += 1;
        }
    }

    if canonical.len() == 0 {
        String::from("/")
    } else if canonical.len() > 1 && canonical.ends_with("/") {
        // get rid of any trailing /
        canonical.replace_range(canonical.len() - 1.., "");
        canonical
    } else {
        canonical
    }
}

fn main() {}


#[cfg(test)]
mod tests {
    use crate::simplify_path;

    #[test]
    fn tester() {
        let path = "abcde/.";
        assert!(true);
    }

    #[test]
    fn example1() {
        assert_eq!(simplify_path(String::from("/home/")), String::from("/home"));
    }

    #[test]
    fn example2() {
        assert_eq!(simplify_path(String::from("/../")), String::from("/"));
    }

    #[test]
    fn example3() {
        assert_eq!(simplify_path(String::from("/home//foo/")), String::from("/home/foo"));
    }

    #[test]
    fn example4() {
        assert_eq!(simplify_path(String::from("/home/./foo")), String::from("/home/foo"));
    }

    #[test]
    fn example5() {
        assert_eq!(simplify_path(String::from("/")), String::from("/"));
    }

    #[test]
    fn example6() {
        assert_eq!(simplify_path(String::from("///")), String::from("/"));
    }

    #[test]
    fn example7() {
        assert_eq!(simplify_path(String::from("/..")), String::from("/"));
    }

    #[test]
    fn example7_5() {
        assert_eq!(simplify_path(String::from("/...")), String::from("/..."));
    }

    #[test]
    fn example8() {
        assert_eq!(simplify_path(String::from("/../../..")), String::from("/"));
    }

    #[test]
    fn example9() {
        assert_eq!(simplify_path(String::from("/abc/...fab/../foo")), String::from("/abc/foo"));
    }

    #[test]
    fn example10() {
        assert_eq!(simplify_path(String::from("/abc/fab/..")), String::from("/abc"));
    }

    #[test]
    fn example11() {
        assert_eq!(simplify_path(String::from("/a/./b/../../c/")), String::from("/c"));
    }
}