

fn rotate90(s: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let order = s[0].len();
    let mut ret = vec![vec![0; order]; order];

    for i in 0..order {
        for j in 0..order {
            ret[i][j] = s[order - j - 1][i];
        }
    }
    ret
}

fn print_matrix(m: &Vec<Vec<i32>>) {
    m.iter().for_each(|row| {
        let row_str = row.iter()
            .map(|n| format!("{:02}", n) )
            .collect::<Vec<String>>();
        println!("{}",row_str.join(","))
    })
}

fn main() {
    let a1 = [[4,9,2], [3,5,7], [8,1,5]];
    let s1: Vec<Vec<i32>> = a1.iter().map(|row| row.to_vec() ).collect();
    let rot = rotate90( &s1 );
    print_matrix(&rot)
}