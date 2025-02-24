//       ___           ___           ___           ___
//      /\__\         /\  \         /\  \         /\__\
//     /:/  /         \:\  \        \:\  \       /::|  |
//    /:/__/           \:\  \        \:\  \     /:|:|  |
//   /::\  \ ___       /::\  \       /::\  \   /:/|:|__|__
//  /:/\:\  /\__\     /:/\:\__\     /:/\:\__\ /:/ |::::\__\
//  \/__\:\/:/  /    /:/  \/__/    /:/  \/__/ \/__/~~/:/  /
//       \::/  /    /:/  /        /:/  /            /:/  /
//       /:/  /     \/__/         \/__/            /:/  /
//      /:/  /                                    /:/  /
//      \/__/                                     \/__/
//
// Copyright (c) 2023, Robert Swinford <robert.swinford<...at...>gmail.com>
//
// For the full copyright and license information, please view the LICENSE file
// that was distributed with this source code.

// this module is a re-implementation of the into_group_map() and into_group_map_by()
// methods for Iterator by Rust Itertools team, for the purpose of using the same
// hashbrown hashmap used elsewhere in httm.  this was/is done for both performance
// and binary size reasons.
//
// though I am fairly certain this re-implementation of their API is fair use
// I've reproduced their license, as of 11/25/2022, verbatim below:

// "Copyright (c) 2015
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE."

use std::{hash::Hash, iter::Iterator};

use hashbrown::HashMap;

pub trait HttmIter: Iterator {
    fn into_group_map<K, V>(self) -> HashMap<K, Vec<V>>
    where
        Self: Iterator<Item = (K, V)> + Sized,
        K: Hash + Eq,
    {
        group_map::into_group_map(self)
    }

    fn into_group_map_by<K, V, F>(self, f: F) -> HashMap<K, Vec<V>>
    where
        Self: Iterator<Item = V> + Sized,
        K: Hash + Eq,
        F: Fn(&V) -> K,
    {
        group_map::into_group_map_by(self, f)
    }
}

impl<T: ?Sized> HttmIter for T where T: Iterator {}

pub mod group_map {
    use std::{hash::Hash, iter::Iterator};

    use hashbrown::HashMap;

    pub fn into_group_map<I, K, V>(iter: I) -> HashMap<K, Vec<V>>
    where
        I: Iterator<Item = (K, V)>,
        K: Hash + Eq,
    {
        let mut lookup: HashMap<K, Vec<V>> = HashMap::with_capacity(iter.size_hint().0);

        iter.for_each(|(key, val)| match lookup.get_mut(&key) {
            Some(vec_val) => {
                vec_val.push(val);
            }
            None => {
                lookup.insert_unique_unchecked(key, [val].into());
            }
        });

        lookup
    }

    pub fn into_group_map_by<I, K, V>(iter: I, f: impl Fn(&V) -> K) -> HashMap<K, Vec<V>>
    where
        I: Iterator<Item = V>,
        K: Hash + Eq,
    {
        into_group_map(iter.map(|v| (f(&v), v)))
    }
}
