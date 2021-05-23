//! Definitions for Mesa X
//!

// Copyright 2020 David Pollak
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

use im::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio_stream::Stream;

/// A generic unique identifier
pub type UniqueId = u64;

/// A unique identifier for addresses
pub type AddressUniqueId = UniqueId;

/// Address of a Cell or other "thing"
#[derive(Debug, Copy, Clone)]
pub struct Address {
    pub row_id: AddressUniqueId,
    pub col_id: AddressUniqueId,
}

/// A rectangular range of cells
#[derive(Debug, Copy, Clone)]
pub struct SingleRange {
    pub upper_left: Address,
    pub lower_right: Address,
}

/// An arbitrary range of cells
#[derive(Debug, Clone)]
pub struct Range {
    pub ranges: Vec<SingleRange>,
}

/// A range on a specific sheet
#[derive(Debug, Clone)]
pub struct SheetRange {
    pub sheet: SheetUniqueIdentifier,
    pub range: Range,
}

/// The unique identifier for a sheet
pub type SheetUniqueIdentifier = UniqueId;

/// the name/identifier structure that identifies a Sheet.
/// Why is this a `trait` rather than a struct? so it can
/// apply different structs
pub trait SheetIdentifier {
    fn get_name(&self) -> String;
    fn get_id(&self) -> SheetUniqueIdentifier;
}

///
pub trait ChangeEvent {
    fn get_sheet_id(&self) -> SheetUniqueIdentifier;
    fn get_range(&self) -> SheetRange;
}

/// A workbook is a collection
pub trait Workbook {
    fn get_sheets(&self) -> Vec<Arc<dyn SheetIdentifier>>;
    fn get_sheet(&self, sheet_id: SheetUniqueIdentifier) -> Option<Arc<dyn Worksheet>>;
    fn listen_for_changes(
        &self,
        to_listen: &Vec<SheetRange>,
    ) -> dyn Stream<Item = Arc<dyn ChangeEvent>>;
}

pub trait Cell {
    fn get_address(&self) -> Address;
    fn get_display_string(&self) -> String;
}

pub trait Worksheet: SheetIdentifier {}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Int(i128),
    Float(f64),
    Str(String),
    Date(Instant),
    Bool(bool),
    JSON(JsonValue),
    Error((String, u32)),
    Maybe(Option<Arc<Value>>),
    TypedJSON((JsonValue, Arc<JsonType>)),
    Other(Arc<OtherValue>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum JsonType {}

#[derive(Debug, PartialEq, Clone)]
pub enum JsonValue {
    Number(f64),
    IntNumber(i128),
    Str(String),
    Bool(bool),
    Null,
    Array(Vec<JsonValue>),
    Json(HashMap<String, JsonValue>),
}
#[derive(Debug, PartialEq, Clone)]
pub struct OtherValue {}
