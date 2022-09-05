// Copyright (c) 2022 Kaushik Kulkarni
// Copyright (c) 2009-2013 Andreas Kloeckner (for original code in pytools)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::iter::IntoIterator;
use std::str::FromStr;
use std::string::ToString;

lazy_static! {
    static ref RE_COUNTER_MATCH: Regex =
        Regex::new(r"^(?P<based_on>\w+)_(?P<counter>\d+)$").unwrap();
}

pub struct UniqueNameGenerator {
    existing_names: HashSet<String>,
    prefix_to_counter: HashMap<String, u32>,
}

impl UniqueNameGenerator {
    pub fn is_name_conflicting<T: ToString>(&self, name: T) -> bool {
        return self.existing_names.contains(&name.to_string());
    }
    pub fn add_name<T: ToString>(&mut self, name: T) {
        if self.is_name_conflicting(name.to_string()) {
            panic!("Name '{}' conflicts with existing names.", name.to_string())
        }
        self.existing_names.insert(name.to_string());
    }

    pub fn add_names<I: ToString, T: IntoIterator<Item = I>>(&mut self, names: T) {
        for name in names.into_iter() {
            self.add_name(name.to_string());
        }
    }

    pub fn get<T: ToString>(&mut self, based_on: T) -> String {
        let new_based_on_x_counter: Option<(String, u32)> = match self.prefix_to_counter
                                                                      .get(&based_on.to_string())
        {
            Some(x) => Some((based_on.to_string(), *x)),
            None => {
                if !RE_COUNTER_MATCH.is_match(&(based_on.to_string()[..])) {
                    None
                } else {
                    let based_on_string = based_on.to_string();
                    let captures_iter = RE_COUNTER_MATCH.captures_iter(based_on_string.as_str());
                    let captures_iter_vec: Vec<_> = captures_iter.collect();
                    assert_eq!(captures_iter_vec.len(), 1); // this should be a single match
                    let capture = &captures_iter_vec[0];
                    Some((capture.name("based_on").unwrap().as_str().to_string(),
                          u32::from_str(capture.name("counter").unwrap().as_str()).unwrap()))
                }
            }
        };

        let unique_name = match new_based_on_x_counter {
            None => {
                self.prefix_to_counter.insert(based_on.to_string(), 0);
                format!("{}", based_on.to_string())
            }
            Some((based_on, counter)) => {
                let mut icounter = counter;
                while self.is_name_conflicting(format!("{}_{}", based_on.to_string(), icounter)) {
                    icounter += 1;
                }
                self.prefix_to_counter
                    .insert(based_on.to_string(), icounter + 1);
                format!("{}_{}", based_on.to_string(), icounter)
            }
        };

        self.add_name(unique_name.clone());
        return unique_name;
    }
}

pub fn make_unique_name_gen<T: IntoIterator<Item = String>>(existing_names: T)
                                                            -> UniqueNameGenerator {
    UniqueNameGenerator { existing_names: existing_names.into_iter().collect(),
                          prefix_to_counter: HashMap::new() }
}
