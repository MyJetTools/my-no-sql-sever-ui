use std::collections::BTreeMap;

pub struct TablesList {
    pub selected_table: Option<String>,
    pub tables: Option<BTreeMap<String, ()>>,
    pub err: Option<String>,
}

impl TablesList {
    pub fn new() -> Self {
        Self {
            selected_table: None,
            tables: None,
            err: None,
        }
    }
    pub fn set_selected_table(&mut self, table: String) {
        self.selected_table = Some(table);
    }

    pub fn tables_are_loaded(&self) -> bool {
        self.tables.is_some() || self.err.is_some()
    }

    pub fn get_selected_table(&self) -> Option<String> {
        self.selected_table.clone()
    }

    pub fn set_loaded_tables(&mut self, src: Vec<String>) {
        let mut tables = BTreeMap::new();

        for table in src {
            tables.insert(table, ());
        }

        self.tables = Some(tables);
    }

    pub fn get_tables(&self) -> Option<Vec<String>> {
        let result: Vec<String> = self
            .tables
            .as_ref()?
            .keys()
            .map(|table| table.to_string())
            .collect();
        Some(result)
    }

    pub fn get_err(&self) -> Option<String> {
        self.err.clone()
    }

    pub fn set_error(&mut self, err: String) {
        self.err = Some(err);
    }

    pub fn reset(&mut self) {
        self.selected_table = None;
        self.tables = None;
        self.err = None;
    }
}
