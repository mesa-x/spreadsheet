//! The implementation of a Worksheet (single sheet) for Mesa X
//!

// Copyright 2021 David Pollak
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::definitions::Value as DValue;
use arc_swap::ArcSwap;
use im::{HashMap, Vector};
use std::sync::Arc;

pub type CellHolder = HashMap<SimpleAddress, Arc<DValue>>;

pub trait Worksheet: Send + Sync {
    type Address: Send + Sync + Clone;
    type Value: Send + Sync + Clone;

    fn has_cell(&self, addr: &Self::Address) -> bool;
    fn get_cell_value(&self, addr: &Self::Address) -> Option<Arc<Self::Value>>;
    fn clear_cell(&self, addr: &Self::Address) -> Option<Arc<Self::Value>>;
    fn set_cell(&self, addr: &Self::Address, value: &Arc<Self::Value>);
}

#[derive(Debug)]
pub struct SimpleWorksheet {
    cells: ArcSwap<CellHolder>,
    history: ArcSwap<Vector<Arc<CellHolder>>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SimpleAddress {
    pub row: i32,
    pub col: i32,
}

impl SimpleAddress {
    pub fn to_string(&self) -> String {
        format!("{}{}", self.format_column(), self.row)
    }

    pub fn format_column(&self) -> String {
        format_column(self.col)
    }
}

pub fn format_column(mut c: i32) -> String {
    let mut buf = [0 as u8; 20];
    if c < 0 {
        return "A".into();
    }

    let mut pos = 0;

    loop {
        let ch: u8 = (c % 26) as u8;
        let mut to_append: u8 = 'A' as u8;
        if pos > 0 && c < 26 {
            to_append -= 1;
        }
        to_append += ch;
        buf[pos] = to_append;
        if c < 26 {
            break;
        }
        c = c / 26;
        pos += 1;
    }

    let mut ret = String::with_capacity(pos);

    loop {
        ret.push(buf[pos as usize] as char);
        if pos == 0 {
            break;
        }
        pos -= 1;
    }

    ret
}

#[test]
fn test_column_format() {
    assert_eq!(format_column(0), "A".to_string());
    assert_eq!(format_column(1), "B".to_string());
    assert_eq!(format_column(25), "Z".to_string());
    assert_eq!(format_column(26 * 2), "BA".to_string());
    assert_eq!(format_column(26), "AA".to_string());
    assert_eq!(format_column(27), "AB".to_string());
    assert_eq!(format_column(26 * 26 + 1), "AAB".to_string());
    assert_eq!(format_column(26 * 26 + 25), "AAZ".to_string());
    assert_eq!(format_column(26 * 26 + (26 * 2) + 25), "ACZ".to_string());
    assert_eq!(format_column(i32::MAX), "FYTISYX".to_string());
}

impl Worksheet for SimpleWorksheet {
    type Address = SimpleAddress;
    type Value = DValue;

    fn has_cell(&self, addr: &Self::Address) -> bool {
        self.cells.load().contains_key(addr)
    }

    fn get_cell_value(&self, addr: &Self::Address) -> Option<Arc<Self::Value>> {
        match self.cells.load().get(addr) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }

    fn clear_cell(&self, addr: &Self::Address) -> Option<Arc<Self::Value>> {
        let ret = self.get_cell_value(addr);
        self.cells.rcu(|t| t.without(addr));
        ret
    }

    fn set_cell(&self, addr: &Self::Address, value: &Arc<Self::Value>) {
        let mut last_gen: Option<Arc<CellHolder>> = None;
        // FIXME -- this is not doing the generational history thing the right way... sigh
        self.cells.rcu(|t| {
            last_gen = Some(t.clone());
            t.update(addr.clone(), value.clone())
        });

        match last_gen {
            Some(old_cells) => {
                self.history.rcu(|h| {
                    // deref from the Arc
                    let q: &Vector<Arc<CellHolder>> = h;

                    // an O(1) clone of the vector
                    let mut h2 = q.clone();

                    // do operations
                    h2.push_back(old_cells.clone());
                    // only keep the last 100 changes FIXME -- make variable?
                    if h2.len() > 100 {
                        h2.pop_front();
                    }

                    // return the new Arc of the vector
                    Arc::new(h2)
                });
                ()
            }
            None => (),
        }
    }
}

impl SimpleWorksheet {
    pub fn new() -> Arc<SimpleWorksheet> {
        Arc::new(SimpleWorksheet {
            cells: ArcSwap::new(Arc::new(HashMap::new())),
            history: ArcSwap::new(Arc::new(Vector::new())),
        })
    }
}
