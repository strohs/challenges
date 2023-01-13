use std::collections::{HashMap, HashSet, BinaryHeap};

struct Twitter {
    // keeps track of when a tweet was "created" so we can sort the tweets later on,
    // more recent tweets will have a higher number
    time: i32,
    // maps a user's id to the tweets they created, the tweets are a tuple of (time, tweet_id)
    user_tweets: HashMap<i32, Vec<(i32, i32)>>,
    // follows maps a user_id to a set of user_id that they follow
    follows: HashMap<i32, HashSet<i32>>,
}

impl Twitter {
    fn new() -> Self {
        Self {
            time: 0,
            user_tweets: HashMap::new(),
            follows: HashMap::new(),
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        let tweets = self.user_tweets
            .entry(user_id)
            .or_default();
        tweets.push((self.time, tweet_id));
        self.time += 1;
    }

    fn get_news_feed(&mut self, user_id: i32) -> Vec<i32> {
        // (time, tweetID, followeeID, index)
        let mut heap: BinaryHeap<(i32, i32, i32, i32)> = BinaryHeap::new();
        let mut feed: Vec<i32> = Vec::with_capacity(10);

        // user must follow themselves in order to get their own tweets
        self.follow(user_id, user_id);

        // 1. initially populate the heap with the most recent tweet of the user and each followee
        for followee_id in self.follows.get(&user_id).iter().flat_map(|e| e.iter()) {
            // get the most recent tweet of each followee and add them to the heap
            if let Some(followee_tweets) = self.user_tweets.get(followee_id) {
                let index = followee_tweets.len() as i32 - 1;
                if let Some((ftime, ftweet_id)) = followee_tweets.get(index as usize) {
                    heap.push((*ftime, *ftweet_id, *followee_id, index - 1));
                }
            }
        }

        // 2. keep adding most recent tweets to the heap until the heap is empty or we have reached 10 tweets in the feed
        while !heap.is_empty() && feed.len() < 10 {
            let (_time, tweet_id, followee_id, index) = heap.pop().unwrap();
            feed.push(tweet_id);
            if index >= 0 {
                let (time, tweet_id) = self.user_tweets[&followee_id][index as usize];
                heap.push((time, tweet_id, followee_id, index - 1));
            }
        }

        feed
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        let followees = self.follows.entry(follower_id)
            .or_insert(HashSet::new());
        followees.insert(followee_id);
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(followees) = self.follows.get_mut(&follower_id) {
            followees.remove(&followee_id);
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::Twitter;

    #[test]
    fn test_post_tweet() {
        let mut twitter = Twitter::new();
        twitter.post_tweet(1, 5);
        let feed = twitter.get_news_feed(1);
        assert_eq!(feed, vec![5]);
    }

    #[test]
    fn test2() {
        let mut twitter = Twitter::new();
        twitter.post_tweet(1, 5);
        twitter.follow(1,2);
        twitter.post_tweet(2,6);
        let mut feed = twitter.get_news_feed(1);
        assert_eq!(feed, vec![6, 5]);
        twitter.unfollow(1, 2);
        feed = twitter.get_news_feed(1);
        assert_eq!(feed, vec![5]);
    }
}