use std::cmp::*;
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included};

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
}
