package leetcode.Problem355;

import java.util.*;

class Twitter {

    static class Tweet {
        int time;
        int tweetId;

        public Tweet(int time, int tweetId) {
            this.time = time;
            this.tweetId = tweetId;
        }

        @Override
        public boolean equals(Object o) {
            if (this == o) return true;
            if (o == null || getClass() != o.getClass()) return false;
            Tweet tweet = (Tweet) o;
            return time == tweet.time;
        }

        @Override
        public int hashCode() {
            return Objects.hash(time);
        }
    }

    static class FeedRec implements Comparable<FeedRec> {
        int time;
        int tweetId;
        int followeeId;
        int index;

        public FeedRec(int time, int tweetId, int followeeId, int index) {
            this.time = time;
            this.tweetId = tweetId;
            this.followeeId = followeeId;
            this.index = index;
        }

        @Override
        public boolean equals(Object o) {
            if (this == o) return true;
            if (o == null || getClass() != o.getClass()) return false;
            FeedRec feedRec = (FeedRec) o;
            return time == feedRec.time;
        }

        @Override
        public int hashCode() {
            return Objects.hash(time);
        }

        @Override
        public int compareTo(FeedRec o) {
            return Integer.compare(this.time, o.time);
        }
    }

    int time;
    Map<Integer, List<Tweet>> userTweets;

    Map<Integer, Set<Integer>> follows;

    public Twitter() {
        this.time = 0;
        this.userTweets = new HashMap<>();
        this.follows = new HashMap<>();
    }

    public void postTweet(int userId, int tweetId) {
        List<Tweet> tweets = this.userTweets.getOrDefault(userId, new ArrayList<>());
        tweets.add(new Tweet(this.time, tweetId));
        this.userTweets.put(userId, tweets);
        this.time += 1;
    }

    public List<Integer> getNewsFeed(int userId) {
        TreeSet<FeedRec> heap = new TreeSet<>();
        ArrayList<Integer> feed = new ArrayList<>(10);

        // use always follows themselves
        this.follow(userId, userId);

        for (Integer followeeId : this.follows.getOrDefault(userId, new HashSet<>())) {
            List<Tweet> followeeTweets = this.userTweets.get(followeeId);
            if (followeeTweets != null) {
                int index = followeeTweets.size() - 1;
                if (index >= 0) {
                    Tweet ftweet = followeeTweets.get(index);
                    heap.add(new FeedRec(ftweet.time, ftweet.tweetId, followeeId, index - 1));
                }
            }
        }

        while (!heap.isEmpty() && feed.size() < 10) {
            FeedRec frec = heap.pollLast();
            feed.add(frec.tweetId);
            if (frec.index >= 0) {
                Tweet tweet = this.userTweets.get(frec.followeeId).get(frec.index);
                heap.add(new FeedRec(tweet.time, tweet.tweetId, frec.followeeId, frec.index - 1));
            }
        }
        return feed;
    }

    public void follow(int followerId, int followeeId) {
        Set<Integer> followees = this.follows.getOrDefault(followerId, new HashSet<>());
        followees.add(followeeId);
        this.follows.put(followerId, followees);
    }

    public void unfollow(int followerId, int followeeId) {
        Set<Integer> followees = this.follows.get(followerId);
        if (followees != null) {
            followees.remove(followeeId);
        }
    }

    public static void main(String[] args) {
        Twitter t = new Twitter();
        t.postTweet(1, 5);
        List<Integer> feed = t.getNewsFeed(1);
        t.follow(1, 2);
        t.postTweet(2, 6);
        feed = t.getNewsFeed(1); // [6, 5]
        System.out.println(feed);
        t.unfollow(1, 2);
        feed = t.getNewsFeed(1); // [5]
        System.out.println(feed);
    }
}

/**
 * Your Twitter object will be instantiated and called as such:
 * Twitter obj = new Twitter();
 * obj.postTweet(userId,tweetId);
 * List<Integer> param_2 = obj.getNewsFeed(userId);
 * obj.follow(followerId,followeeId);
 * obj.unfollow(followerId,followeeId);
 */
