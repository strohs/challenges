use std::collections::{BTreeSet};
use std::io::{Read};
use std::io;
use std::fs::File;

fn climbing_leaderboard(scores: &Vec<i64>, alice: &Vec<i64>) -> Vec<usize> {
    let score_list = build_sorted_scores( scores );
    let mut res = vec![0; alice.len()];
    for (aidx, alice_score) in alice.iter().enumerate() {
        match score_list.binary_search(alice_score) {
            Ok(sidx) => {
                if sidx == 0 { res[aidx] = score_list.len(); }
                else { res[aidx] = score_list.len() - sidx; }
            },
            Err(idx) => {
                res[aidx] = (score_list.len() - idx) + 1;
            }
        }
    }
    res
}

fn build_sorted_scores(scores: &Vec<i64>) -> Vec<i64> {
    let mut score_set = BTreeSet::new();
    for score in scores {
        score_set.insert(*score);
    }
    score_set.into_iter().collect()
}

fn read_score_file(path: &str) -> Result<Vec<i64>, io::Error> {
    let mut res: Vec<i64> = vec![];
    let mut s = String::new();

    File::open(path)?.read_to_string(&mut s)?;
    s.split_whitespace()
        .for_each(|ns| res.push( ns.parse::<i64>().unwrap() ) );
    Ok(res)
}

fn main() {
    //let scores = vec![100,100,50,40,40,20,10];
    //let alice = vec![5,25,50,120];
    let score_path1 = "/home/cliff/idea-projects/hackerrank/kotlin-solutions/src/main/resources/leaderBoardTestScores1.txt";
    let alice_path1 = "/home/cliff/idea-projects/hackerrank/kotlin-solutions/src/main/resources/leaderBoardTestAlice1.txt";
    let scores = read_score_file(score_path1).unwrap();
    let alice = read_score_file(alice_path1).unwrap();

    let res = climbing_leaderboard(&scores, &alice);
    res.iter().for_each(|s| println!("{}",s));

}