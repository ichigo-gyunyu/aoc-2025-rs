use std::cmp::*;
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interval {
    l: u64,
    r: u64,
}

#[derive(Debug)]
pub struct IntervalSet {
    // Stores disjoint, merged intervals [l, r] (inclusive)
    // Invariant: intervals are non-overlapping and sorted by l
    intervals: BTreeMap<u64, u64>,

    // Total length covered by all intervals
    total_length: u64,
}

impl IntervalSet {
    pub fn new() -> Self {
        Self {
            intervals: BTreeMap::new(),
            total_length: 0,
        }
    }

    /// Returns the length of interval [l, r]
    fn length(l: u64, r: u64) -> u64 {
        r - l + 1
    }

    /// Adds interval [l, r] into the interval set.
    ///
    /// Merges any overlapping or touching intervals in the process.
    pub fn add(&mut self, l: u64, r: u64) {
        assert!(l <= r, "interval start must be <= interval end");

        // Collect all intervals that overlap or touch [l, r] and compute the new interval to add
        // [new_l, new_r]
        let mut to_remove = Vec::new();
        let mut new_l = l;
        let mut new_r = r;

        // Step 1: Check previous interval (if any) - the interval with the largest start ll <= l
        if let Some((&ll, &rr)) = self.intervals.range(..=l).next_back() {
            // If it overlaps or touches [l, r], merge it
            // rr = l - 1 -- touches
            // rr > l - 1 -- overlaps
            if rr >= l.saturating_sub(1) {
                to_remove.push(ll);
                new_l = min(new_l, ll);
                new_r = max(new_r, rr);
            }
        }

        // Step 2: Scan existing intervals that overlap [l, r] or touch r (intervals touching l was
        // handled in step 1).
        for (&ll, &rr) in self
            .intervals
            .range((Excluded(l), Included(r.saturating_add(1))))
        {
            to_remove.push(ll);
            new_l = min(new_l, ll);
            new_r = max(new_r, rr);
        }

        // Step 3: Remove all intervals touching or overlapping with [new_l, new_r]. Also update
        // total_length.
        for ll in to_remove {
            let rr = self
                .intervals
                .remove(&ll)
                .expect("interval should have been removed.");
            self.total_length = self.total_length - Self::length(ll, rr);
        }

        // Step 4: Add interval [new_l, new_r] and update total_length.
        self.intervals.insert(new_l, new_r);
        self.total_length = self.total_length + Self::length(new_l, new_r);
    }

    /// Overload for add(l, r)
    pub fn add_interval(&mut self, interval: Interval) {
        self.add(interval.l, interval.r);
    }

    /// Returns the (merged) interval that fully covers [l, r] or None if it doesn't exist
    pub fn covered_by(&mut self, l: u64, r: u64) -> Option<Interval> {
        self.intervals
            .range(..=l)
            .next_back()
            .filter(|&(_, &rr)| rr >= r)
            .map(|(&ll, &rr)| Interval { l: ll, r: rr })
    }

    pub fn covered_by_interval(&mut self, interval: Interval) -> Option<Interval> {
        self.covered_by(interval.l, interval.r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intervalset_add() {
        let mut s = IntervalSet::new();

        // 1. Add first interval
        s.add(6, 7);
        assert_eq!(s.intervals, BTreeMap::from([(6, 7)]));
        assert_eq!(s.total_length, 2);

        // 2. Add non-overlapping interval
        s.add(10, 13);
        assert_eq!(s.intervals, BTreeMap::from([(6, 7), (10, 13)]));
        assert_eq!(s.total_length, 6);

        // 3. Add touching interval (touching on both ends)
        s.add(8, 9);
        assert_eq!(s.intervals, BTreeMap::from([(6, 13)]));
        assert_eq!(s.total_length, 8);

        // 4. Add touching interval (left end touching)
        s.add(14, 17);
        assert_eq!(s.intervals, BTreeMap::from([(6, 17)]));
        assert_eq!(s.total_length, 12);

        // 5. Add touching interval (left end touching, but in between another interval)
        s.add(30, 40);
        s.add(18, 20);
        assert_eq!(s.intervals, BTreeMap::from([(6, 20), (30, 40)]));
        assert_eq!(s.total_length, 26);

        // 6. Add touching interval (right end touching)
        s.add(3, 5);
        assert_eq!(s.intervals, BTreeMap::from([(3, 20), (30, 40)]));
        assert_eq!(s.total_length, 29);

        // 7. Add touching interval (right end touching, but in between another interval)
        s.add(27, 29);
        assert_eq!(s.intervals, BTreeMap::from([(3, 20), (27, 40)]));
        assert_eq!(s.total_length, 32);

        // 8. Add overlapping interval (overlapping on both ends)
        s.add(5, 35);
        assert_eq!(s.intervals, BTreeMap::from([(3, 40)]));
        assert_eq!(s.total_length, 38);

        // 9. Add overlapping interval (left end overlapping)
        s.add(33, 52);
        assert_eq!(s.intervals, BTreeMap::from([(3, 52)]));
        assert_eq!(s.total_length, 50);

        // 10. Add overlapping interval (left end overlapping, but in between another interval)
        s.add(67, 80);
        s.add(45, 60);
        assert_eq!(s.intervals, BTreeMap::from([(3, 60), (67, 80)]));
        assert_eq!(s.total_length, 72);

        // 11. Add overlapping interval (right end overlapping)
        s.add(2, 49);
        assert_eq!(s.intervals, BTreeMap::from([(2, 60), (67, 80)]));
        assert_eq!(s.total_length, 73);

        // 12. Add overlapping interval (right end overlapping, but in between another interval)
        s.add(65, 77);
        assert_eq!(s.intervals, BTreeMap::from([(2, 60), (65, 80)]));
        assert_eq!(s.total_length, 75);

        // 13. Add overlapping interval (multiple intervals overlapping)
        s.add(90, 94);
        s.add(1, 100);
        assert_eq!(s.intervals, BTreeMap::from([(1, 100)]));
        assert_eq!(s.total_length, 100);

        // 14. Add interval that is fully embedded in the interval set (no-op)
        s.add(67, 67);
        assert_eq!(s.intervals, BTreeMap::from([(1, 100)]));
        assert_eq!(s.total_length, 100);
    }

    #[test]
    fn test_intervalset_coveredby() {
        let mut s = IntervalSet::new();
        s.add(10, 20);
        s.add(40, 50);
        s.add(70, 80);

        assert_eq!(s.covered_by(15, 16), Some(Interval { l: 10, r: 20 }));
        assert_eq!(s.covered_by(80, 80), Some(Interval { l: 70, r: 80 }));
        assert_eq!(s.covered_by(40, 51), None);
        assert_eq!(s.covered_by(40, 80), None);
    }
}
