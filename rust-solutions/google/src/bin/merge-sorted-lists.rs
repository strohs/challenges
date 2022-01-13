
/// merge two sorted vectors into a new sorted vector
fn merge_lists<T: Ord + Copy>(ls: &Vec<T>, ms: &Vec<T>) -> Vec<T> {

    let mut ts: Vec<T> = Vec::new();
    let mut li = 0;
    let mut mi = 0;

    while li < ls.len() && mi < ms.len() {
        if ls[li] <= ms[mi] {
            ts.push(ls[li]);
            li += 1;
        } else {
            ts.push(ms[mi]);
            mi += 1;
        }
    }

    while li < ls.len() {
        ts.push(ls[li]);
        li += 1;
    }

    while mi < ms.len() {
        ts.push(ms[mi]);
        mi += 1;
    }

    ts
}


fn main() {
    let l1 = vec![2,4,6];
    let l2 = vec![1,3,5,7,9];
    let l3 = vec![10, 20, 30, 40, 50, 60, 70];
    let l4 = vec![11,22,33,44,55];


    dbg!(merge_lists(&l1, &l2));
    dbg!(merge_lists(&l3,&l4));

}