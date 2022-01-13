
/*
Brie’s Drawing teacher asks her class to open their books to a page number. Brie can either start
turning pages from the front of the book or from the back of the book. She always turns pages one
at a time. When she opens the book, page 1 is always on the right side.

When she flips page 1, she sees pages 2 and 3. Each page except the last page will always be
printed on both sides. The last page may only be printed on the front, given the length of the book.
If the book is pages n long, and she wants to turn to page p, what is the minimum number of pages
she will turn? She can start at the beginning or the end of the book.

Given n and p, find and print the minimum number of pages Brie must turn in order to arrive
at page p.
*/

use std::cmp::min;

/// returns the minimum number of page turns needed to get to the specified page: p, of a book.
/// The turns can start from the first page of the book, or the last page
///
/// # Arguments
/// * `n` - the total number of pages in the book
/// * `p` - the page number you want to turn to
///
fn page_count(n :u32, p: u32) -> u32 {
    let from_front = || -> u32 { p / 2 };
    let from_back = || -> u32 {(n / 2) - (p / 2)};
    min( from_front(), from_back() )
}

fn main() {
    println!("{}", page_count(6,2));
    println!("{}", page_count(8,2));
    println!("{}", page_count(6,6));
    println!("{}", page_count(5,5));
    println!("{}", page_count(12,4));
    println!("{}", page_count(12,9));
}
