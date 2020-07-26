use im::HashMap;
use std::sync::Arc;
use crate::definitions::AddressUniqueId;

pub type ArcWorkbookInfo = Arc<WorkbookInfo>;

#[derive(Debug, Clone)]
pub struct WorkbookInfo {
    sheets: HashMap<String, ArcSheetInfo>,
}

impl WorkbookInfo {
    // based on the name of a sheet, get the sheet
    pub fn sheet_for_name(&self, name: String) -> Option<ArcSheetInfo> {
        self.sheets.get(&name).map(|x| x.clone())
    }

    pub fn set_sheet(&self, name: String, info: ArcSheetInfo) -> WorkbookInfo {
        WorkbookInfo {
            sheets: self.sheets.update(name, info)
        }
    }

    pub fn new() -> WorkbookInfo {
        WorkbookInfo {
            sheets: HashMap::new()
        }
    }
}

pub type ArcSheetInfo = Arc<SheetInfo>;

#[derive(Debug, Clone)]
pub struct SheetInfo {
    names_to_rows: HashMap<String, AddressUniqueId>,
    names_to_cols: HashMap<String, AddressUniqueId>,
    row_order: Vec<AddressUniqueId>,
    col_order: Vec<AddressUniqueId>,
    col_id_to_info: HashMap<AddressUniqueId, (String, u32)>,
    row_id_to_info: HashMap<AddressUniqueId, (String, u32)>
}

