/// Represents a grid layout with columns and rows.
#[derive(Debug, PartialEq)]
pub struct Layout {
    /// Number of columns in the grid.
    pub cols: u32,
    /// Number of rows in the grid.
    pub rows: u32,
}

/// Parses a layout argument string into a `Layout`.
///
/// Accepts either a plain number (e.g. "4") or a grid spec (e.g. "2x3").
pub fn parse_layout(arg: &str) -> Result<Layout, String> {
    // グリッド指定: CxR 形式
    if let Some((cols_str, rows_str)) = arg.split_once('x') {
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
        return Ok(Layout { cols, rows });
    }

    // 数値指定
    let n = arg.parse::<u32>().map_err(|_| {
        format!(
            "Invalid argument: '{}'. Expected a number or grid spec (e.g. 4, 2x3)",
            arg
        )
    })?;
    if n < 2 {
        return Err("Number of panes must be >= 2".to_string());
    }

    // 正方形に近い因数分解: √N に最も近い因数ペアを選択
    let sqrt = (n as f64).sqrt().ceil() as u32;
    let mut cols = sqrt;
    while n % cols != 0 {
        cols += 1;
    }
    let rows = n / cols;
    // cols >= rows になるよう調整
    let (cols, rows) = if cols >= rows {
        (cols, rows)
    } else {
        (rows, cols)
    };

    Ok(Layout { cols, rows })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_layout_valid_cases() {
        let cases = [
            // 数値指定
            ("2", 2, 1),
            ("3", 3, 1),
            ("4", 2, 2),
            ("5", 5, 1), // 素数は横一列
            ("6", 3, 2),
            ("9", 3, 3),
            // グリッド指定
            ("2x3", 2, 3),
            ("3x2", 3, 2),
            ("1x4", 1, 4),
        ];
        for (input, cols, rows) in cases {
            assert_eq!(
                parse_layout(input),
                Ok(Layout { cols, rows }),
                "input: {}",
                input
            );
        }
    }

    #[test]
    fn parse_layout_invalid_cases() {
        let cases = ["abc", "0", "1", "0x3", "2x0", "1x1", "axb"];
        for input in cases {
            assert!(parse_layout(input).is_err(), "input: {}", input);
        }
    }
}
