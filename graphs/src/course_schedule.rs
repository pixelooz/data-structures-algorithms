use crate::Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut adj: HashMap<i32, Vec<i32>> = HashMap::new();

        for pre in prerequisites {
            let course = pre[0];
            let prereq = pre[1];
            adj.entry(prereq).or_default().push(course);
        }
        let mut visiting = HashSet::new();
        let mut visited = HashSet::new();

        for i in 0..num_courses {
            if !Self::course_schedule_dfs(i, &adj, &mut visiting, &mut visited) {
                return false;
            }
        }
        true
    }

    fn course_schedule_dfs(
        index: i32,
        adj: &HashMap<i32, Vec<i32>>,
        visiting: &mut HashSet<i32>,
        visited: &mut HashSet<i32>,
    ) -> bool {
        if visiting.contains(&index) {
            return false;
        }
        if visited.contains(&index) {
            return true;
        }
        visiting.insert(index);
        if let Some(neighbors) = adj.get(&index) {
            for &index in neighbors {
                if !Self::course_schedule_dfs(index, adj, visiting, visited) {
                    return false;
                }
            }
        }
        visited.insert(index);
        visiting.remove(&index);
        true
    }
}
