use crate::Solution;

impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut counts = [0; 1001];

        for trip in trips {
            let passenger_count = trip[0];
            let from = trip[1] as usize;
            let to = trip[2] as usize;

            counts[from] += passenger_count;
            counts[to] -= passenger_count;
        }
        let mut current_passenger = 0;
        for count in counts {
            current_passenger += count;
            if current_passenger > capacity {
                return false;
            }
        }
        true
    }
}
