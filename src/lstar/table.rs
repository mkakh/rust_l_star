use super::dfa::{State, Symbol};
use std::collections::HashMap;

type Symbols = Vec<Symbol>;

#[derive(Debug)]
pub(crate) struct ObservationTable {
    rows: Vec<Symbols>,
    columns: Vec<Symbols>,
    cells: HashMap<(Symbols, Symbols), bool>,
}

impl ObservationTable {
    pub(crate) fn new() -> Self {
        ObservationTable {
            rows: vec![vec!['ε'.to_string()]],
            columns: vec![vec!['ε'.to_string()]],
            cells: HashMap::new(),
        }
    }

    pub(crate) fn get_rows(&self) -> &[Symbols] {
        &self.rows
    }

    pub(crate) fn get_columns(&self) -> &[Symbols] {
        &self.columns
    }

    pub(crate) fn get_cell(&self, prefix: &Symbols, suffix: &Symbols) -> Option<&bool> {
        self.cells.get(&(prefix.clone(), suffix.clone()))
    }

    pub(crate) fn get_value(&self, prefix: &Symbols) -> Result<Vec<bool>, String> {
        let mut v: Vec<bool> = vec![];
        for suffix in self.columns.iter() {
            if let Some(value) = self.get_cell(prefix, suffix) {
                v.push(*value);
            } else {
                return Err(format!(
                    "Value not found for prefix '{:?}' and suffix '{:?}'",
                    prefix, suffix
                ));
            }
        }
        Ok(v)
    }

    pub(crate) fn get_value_as_state(&self, prefix: &Symbols) -> Result<State, String> {
        self.get_value(prefix).map(Self::vecbool_to_state)
    }

    pub(crate) fn fill_cell(&mut self, row: &Symbols, column: &Symbols, value: bool) {
        self.cells.insert((row.clone(), column.clone()), value);
    }

    pub(crate) fn is_filled(&self, row: &Symbols, column: &Symbols) -> bool {
        self.cells.contains_key(&(row.clone(), column.clone()))
    }

    pub(crate) fn add_rows(&mut self, prefix: Symbols) {
        if !self.rows.contains(&prefix) {
            self.rows.push(prefix);
        }
    }

    pub(crate) fn add_columns(&mut self, suffix: Symbols) {
        if !self.columns.contains(&suffix) {
            self.columns.push(suffix);
        }
    }

    pub(crate) fn get_states(&self) -> Result<Vec<State>, String> {
        let mut states = Vec::new();
        for prefix in self.get_rows().iter() {
            if let Ok(value) = self.get_value(prefix) {
                states.push(Self::vecbool_to_state(value));
            } else {
                return Err(format!("Cells are not filles for prefix '{:?}'", prefix));
            }
        }
        Ok(states)
    }

    fn vecbool_to_state(vb: Vec<bool>) -> State {
        let mut v = Vec::new();
        for b in vb {
            if b {
                v.push('1');
            } else {
                v.push('0');
            }
        }
        v.iter().collect()
    }
}

impl std::fmt::Display for ObservationTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut column_widths = Vec::with_capacity(self.columns.len() + 1);

        column_widths.push(4);
        for column in &self.columns {
            let column_len = format!("{:?}", column).len();
            column_widths.push(column_len + 2);
        }

        for row in &self.rows {
            let row_width = format!("{:?}", row).len() + 2;
            if column_widths[0] < row_width {
                column_widths[0] = row_width;
            }
        }

        for ((prefix, suffix), cell_value) in &self.cells {
            let prefix_len = format!("{:?}", prefix).len() + 2;
            if column_widths[0] < prefix_len {
                column_widths[0] = prefix_len;
            }
            if let Some(index) = self.columns.iter().position(|c| c == suffix) {
                let cell_len = cell_value.to_string().len() + 2;
                if column_widths[index + 1] < cell_len {
                    column_widths[index + 1] = cell_len;
                }
            }
        }

        write!(f, "{:<width$}", "", width = column_widths[0])?;
        for (i, column) in self.columns.iter().enumerate() {
            write!(
                f,
                "{:width$}",
                format!("{:?}", column),
                width = column_widths[i + 1]
            )?;
        }
        writeln!(f)?;

        for row in &self.rows {
            write!(
                f,
                "{:width$}",
                format!("{:?}", row),
                width = column_widths[0]
            )?;
            for (i, column) in self.columns.iter().enumerate() {
                let value = self.get_cell(row, column).map_or("-".to_string(), |v| {
                    if *v {
                        "1".to_string()
                    } else {
                        "0".to_string()
                    }
                });
                write!(f, "{:width$}", value, width = column_widths[i + 1])?;
            }
            writeln!(f)?;
        }

        let total_width: usize = column_widths.iter().sum::<usize>() + column_widths.len() - 1;
        writeln!(f, "{}", "-".repeat(total_width))?;

        let mut extra_rows = vec![];
        for ((prefix, _), _) in &self.cells {
            if !self.rows.contains(prefix) && !extra_rows.contains(prefix) {
                extra_rows.push(prefix.clone());
            }
        }

        for row in extra_rows {
            write!(
                f,
                "{:width$}",
                format!("{:?}", row),
                width = column_widths[0]
            )?;
            for (i, column) in self.columns.iter().enumerate() {
                let value = self.get_cell(&row, column).map_or("-".to_string(), |v| {
                    if *v {
                        "1".to_string()
                    } else {
                        "0".to_string()
                    }
                });
                write!(f, "{:width$}", value, width = column_widths[i + 1])?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
