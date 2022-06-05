
struct TopVotedCandidate {
    times: Vec<i32>,
    leaders: Vec<i32>
}

/// #Vec, HashMap, binary_search, match
impl TopVotedCandidate {

    fn new(persons: Vec<i32>, times: Vec<i32>) -> Self {
        let mut map = std::collections::HashMap::new();
        let mut leaders = vec![];
        let mut leader:(usize, i32) = (0, 0);
        for i in 0..persons.len() {
            let count = map.entry(persons[i]).or_default();
            *count += 1;
            if *count >= leader.0 {
                leader = (*count, persons[i]);
            }
            leaders.push(leader.1);
        }
        TopVotedCandidate {
            times, leaders
        }
    }

    fn q(&self, t: i32) -> i32 {
        let idx = match self.times.binary_search(&t) {
            Ok(idx) => idx,
            Err(idx) => idx - 1
        };
        return self.leaders[idx];
    }
}

#[test]
fn test() {
    let obj = TopVotedCandidate::new(vec![0, 1, 1, 0, 0, 1, 0], vec![0, 5, 10, 15, 20, 25, 30]);
    assert_eq!(0, obj.q(3));
    assert_eq!(1, obj.q(12));
    assert_eq!(1, obj.q(25));
    assert_eq!(0, obj.q(15));
    assert_eq!(0, obj.q(24));
    assert_eq!(1, obj.q(8));
}