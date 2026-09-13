use std::collections::{BinaryHeap, HashMap, HashSet};

struct Twitter {
    time: usize,
    tweets: HashMap<i32, Vec<(usize, i32)>>,
    following: HashMap<i32, HashSet<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {
    fn new() -> Self {
        Self {
            time: 0,
            tweets: HashMap::new(),
            following: HashMap::new(),
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.time += 1;
        self.tweets
            .entry(user_id)
            .or_default()
            .push((self.time, tweet_id));
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut feed = Vec::new();
        let mut max_heap = BinaryHeap::new();

        let mut users_to_poll = HashSet::new();
        users_to_poll.insert(user_id);

        if let Some(followers) = self.following.get(&user_id) {
            users_to_poll.extend(followers.iter());
        }
        for author_id in users_to_poll {
            if let Some(author_tweets) = self.tweets.get(&author_id) {
                if let Some(&(time, tweet_id)) = author_tweets.last() {
                    let index = author_tweets.len() - 1;
                    max_heap.push((time, tweet_id, author_id, index));
                }
            }
        }
        while let Some((_, tweet_id, author_id, index)) = max_heap.pop() {
            feed.push(tweet_id);
            if feed.len() == 10 {
                break;
            }
            if index > 0 {
                let prev_idx = index - 1;
                let (prev_time, prev_tweet_id) = self.tweets[&author_id][prev_idx];
                max_heap.push((prev_time, prev_tweet_id, author_id, prev_idx));
            }
        }
        feed
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.following
            .entry(follower_id)
            .or_default()
            .insert(followee_id);
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(followees) = self.following.get_mut(&follower_id) {
            followees.remove(&followee_id);
        }
    }
}
