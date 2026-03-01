/// Represents a pane layout where each column can have a different number of rows.
#[derive(Debug, PartialEq)]
pub struct Layout {
    /// Number of rows in each column. columns.len() = number of columns.
    pub columns: Vec<u32>,
}

impl Layout {
    /// Total number of columns.
    pub fn num_cols(&self) -> usize {
        self.columns.len()
    }

    /// Total number of panes.
    pub fn total_panes(&self) -> u32 {
        self.columns.iter().sum()
    }
}

/// Parses a layout argument string into a `Layout`.
///
/// Accepts three formats (tried in order):
/// 1. Comma-separated custom layout (e.g. "1,3")
/// 2. Grid spec with 'x' (e.g. "2x3")
/// 3. Plain number (e.g. "4")
pub fn parse_layout(arg: &str) -> Result<Layout, String> {
    // 1. Custom layout: comma-separated column row counts
    if arg.contains(',') {
        return parse_custom_layout(arg);
    }

    // 2. Grid spec: CxR format
    if arg.contains('x') {
        return parse_grid_spec(arg);
    }

    // 3. Numeric spec
    parse_numeric(arg)
}

/// Parses comma-separated custom layout (e.g. "1,3" -> columns: [1, 3]).
fn parse_custom_layout(arg: &str) -> Result<Layout, String> {
    let parts: Vec<&str> = arg.split(',').collect();

    let mut columns = Vec::with_capacity(parts.len());
    for part in &parts {
        if part.is_empty() {
            return Err(format!("Invalid custom layout: '{}'", arg));
        }
        let rows = part
            .parse::<u32>()
            .map_err(|_| format!("Invalid custom layout: '{}'", arg))?;
        if rows == 0 {
            return Err(format!(
                "Each column must have >= 1 row, got 0 in: '{}'",
                arg
            ));
        }
        columns.push(rows);
    }

    let layout = Layout { columns };
    if layout.total_panes() < 2 {
        return Err("Total panes must be >= 2".to_string());
    }
    Ok(layout)
}

/// Parses grid spec (e.g. "2x3" -> 2 columns, each with 3 rows).
fn parse_grid_spec(arg: &str) -> Result<Layout, String> {
    let (cols_str, rows_str) = arg
        .split_once('x')
        .ok_or_else(|| format!("Invalid grid format: '{}'", arg))?;
    let cols = cols_str
        .parse::<u32>()
        .map_err(|_| format!("Invalid grid format: '{}'", arg))?;
    let rows = rows_str
        .parse::<u32>()
        .map_err(|_| format!("Invalid grid format: '{}'", arg))?;
    if cols == 0 || rows == 0 {
        return Err(format!("Grid dimensions must be >= 1, got: '{}'", arg));
    }
    if cols * rows < 2 {
        return Err("Total panes must be >= 2".to_string());
    }
    Ok(Layout {
        columns: vec![rows; cols as usize],
    })
}

/// Parses a plain number and factorizes into near-square grid.
fn parse_numeric(arg: &str) -> Result<Layout, String> {
    let n = arg.parse::<u32>().map_err(|_| {
        format!(
            "Invalid argument: '{}'. Expected a number or grid spec (e.g. 4, 2x3)",
            arg
        )
    })?;
    if n < 2 {
        return Err("Number of panes must be >= 2".to_string());
    }

    // Factorize into near-square: pick the factor pair closest to sqrt(N)
    let sqrt = (n as f64).sqrt().ceil() as u32;
    let mut cols = sqrt;
    while n % cols != 0 {
        cols += 1;
    }
    let rows = n / cols;
    // Ensure cols >= rows
    let (cols, rows) = if cols >= rows {
        (cols, rows)
    } else {
        (rows, cols)
    };

    Ok(Layout {
        columns: vec![rows; cols as usize],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_layout_valid_numeric() {
        let cases: &[(&str, &[u32])] = &[
            ("2", &[1, 1]),
            ("3", &[1, 1, 1]),
            ("4", &[2, 2]),
            ("5", &[1, 1, 1, 1, 1]), // prime => single row per column
            ("6", &[2, 2, 2]),
            ("9", &[3, 3, 3]),
        ];
        for (input, expected_columns) in cases {
            assert_eq!(
                parse_layout(input),
                Ok(Layout {
                    columns: expected_columns.to_vec()
                }),
                "input: {}",
                input
            );
        }
    }

    #[test]
    fn parse_layout_valid_grid() {
        let cases: &[(&str, &[u32])] = &[("2x3", &[3, 3]), ("3x2", &[2, 2, 2]), ("1x4", &[4])];
        for (input, expected_columns) in cases {
            assert_eq!(
                parse_layout(input),
                Ok(Layout {
                    columns: expected_columns.to_vec()
                }),
                "input: {}",
                input
            );
        }
    }

    #[test]
    fn parse_layout_valid_custom() {
        let cases: &[(&str, &[u32])] = &[("1,3", &[1, 3]), ("2,1,3", &[2, 1, 3]), ("1,1", &[1, 1])];
        for (input, expected_columns) in cases {
            assert_eq!(
                parse_layout(input),
                Ok(Layout {
                    columns: expected_columns.to_vec()
                }),
                "input: {}",
                input
            );
        }
    }

    #[test]
    fn parse_layout_invalid_cases() {
        let cases = [
            "abc", "0", "1", "0x3", "2x0", "1x1", "axb", ",3", "3,", "1,,3", "0,3", "1,0", "a,b",
        ];
        for input in cases {
            assert!(parse_layout(input).is_err(), "input: {}", input);
        }
    }

    #[test]
    fn parse_layout_custom_single_column_too_few_panes() {
        // "1" via custom layout is not reachable (no comma), but total panes < 2 check
        // is tested via numeric "1" above. Here we ensure a single-pane custom layout fails.
        // Not possible with comma format since at least two parts are needed.
    }

    #[test]
    fn parse_layout_grid_and_custom_equivalence() {
        // "2,2" (custom) and "2x2" (grid) should produce the same Layout
        let custom = parse_layout("2,2").unwrap();
        let grid = parse_layout("2x2").unwrap();
        assert_eq!(custom, grid);
    }

    #[test]
    fn layout_num_cols() {
        let cases: &[(&[u32], usize)] = &[(&[1, 3], 2), (&[2, 1, 3], 3), (&[4], 1)];
        for (columns, expected) in cases {
            let layout = Layout {
                columns: columns.to_vec(),
            };
            assert_eq!(layout.num_cols(), *expected);
        }
    }

    #[test]
    fn layout_total_panes() {
        let cases: &[(&[u32], u32)] = &[(&[1, 3], 4), (&[2, 1, 3], 6), (&[4], 4), (&[2, 2], 4)];
        for (columns, expected) in cases {
            let layout = Layout {
                columns: columns.to_vec(),
            };
            assert_eq!(layout.total_panes(), *expected);
        }
    }
}
