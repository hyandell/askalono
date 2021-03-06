// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License").
// You may not use this file except in compliance with the License.
// A copy of the License is located at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// or in the "license" file accompanying this file. This file is distributed
// on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
// express or implied. See the License for the specific language governing
// permissions and limitations under the License.

use std::cmp::min;
use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::collections::VecDeque;

#[derive(Debug, Serialize, Deserialize)]
pub struct NgramSet {
    map: HashMap<String, u32>,
    // once Rust supports it, it'd be nice to make this
    // a type parameter & specialize
    n: u8,
    size: usize,
}

impl<'a> NgramSet {
    pub fn new(n: u8) -> NgramSet {
        NgramSet {
            map: HashMap::new(),
            n: n,
            size: 0,
        }
    }

    pub fn from_str(s: &str, n: u8) -> NgramSet {
        let mut set = NgramSet::new(n);
        set.analyze(s);
        set
    }

    pub fn analyze(&mut self, s: &str) {
        let words = s.split(' ');

        let mut deque: VecDeque<&str> = VecDeque::with_capacity(self.n as usize);
        for w in words {
            deque.push_back(w);
            if deque.len() == self.n as usize {
                let parts = deque.iter().cloned().collect::<Vec<&str>>();
                self.add_gram(parts.join(" "));
                deque.pop_front();
            }
        }
    }

    fn add_gram(&mut self, gram: String) {
        let n = self.map.entry(gram).or_insert(0);
        *n += 1;
        self.size += 1;
    }

    pub fn get(&self, gram: &str) -> u32 {
        if let Some(count) = self.map.get(gram) {
            *count
        } else {
            0
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn dice(&self, other: &NgramSet) -> f32 {
        if other.n != self.n {
            return 0f32;
        }

        // choose the smaller map to iterate
        let (x, y) = if self.len() < other.len() {
            (self, other)
        } else {
            (other, self)
        };

        let mut matches = 0;
        for (gram, count) in x {
            matches += min(*count, y.get(gram));
        }

        (2.0 * matches as f32) / ((self.len() + other.len()) as f32)
    }
}

impl<'a> IntoIterator for &'a NgramSet {
    type Item = (&'a String, &'a u32);
    type IntoIter = Iter<'a, String, u32>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // this is a pretty banal test, but it's a starting point :P
    #[test]
    fn can_construct() {
        let set = NgramSet::new(2);
        assert_eq!(set.size, 0);
        assert_eq!(set.n, 2);
    }
}
