
/*
In this challenge, you will be given a list of letter heights in the alphabet and a string. Using
the letter heights given, determine the area of the rectangle highlight in mm^2 assuming all
letters are 1mm wide.

For example, the highlighted  word=torn   Assume the heights of the letters are t=2, o=1, r=1 and
n=1. The tallest letter is 2 high and there are 4 letters. The hightlighted area will be
2*4 = 8mm^2 so the answer is 8
 */

/// find the height of the largest character in word, using the array of heights: h.  return
/// the largest height * words.len()
///
/// # Params
/// * h: an array of integers representing the heights of each letter (1 <= h[?]] <= 7 where ? is an english lowercase letter)
/// * word: a string (contains no more than 10 letters)
///
/// # Return
/// an integer representing the size of the highlighted area.
fn designer_pdf_viewer(h: &[u8], word: &str) -> usize {
    fn char_index(ch: &char) -> usize {
        *ch as usize - 'a' as usize
    }

    let max_height = word.chars()
        .map(|c| h[char_index(&c)] )
        .max()
        .unwrap();
    max_height as usize * word.len()
}




fn main() {
    let h: [u8; 26] = [1, 3, 1, 3, 1, 4, 1, 3, 2, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7];
    let word = "zaba";
    println!("{}", designer_pdf_viewer(&h[..], word));

    for c in word.chars() {
        println!("{}={}",c, c as u8);
    }
}