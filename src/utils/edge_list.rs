use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::GraphError;

/// Reads an edge list from a file and returns a vector of edges.
/// Each edge is represented as a tuple `(u, v, Option<weight>)`.
///
/// # Arguments
/// * `file_path` - Path to the file containing the edge list.
///
/// # Returns
/// A vector of edges or an error if the file could not be read.
///
/// # Errors
/// This function returns a `GraphError` in the following cases:
/// - If the file cannot be opened.
/// - If a line cannot be read from the file.
/// - If a line has an invalid format (e.g., less than two elements).
/// - If a vertex cannot be parsed into a `usize`.
/// - If a weight (if present) cannot be parsed into an `i32`.
pub fn parse_edge_list(file_path: &str) -> Result<Vec<(usize, usize, Option<i32>)>, GraphError> {
    let path = Path::new(file_path);
    let file = File::open(path)
        .map_err(|_| GraphError::InvalidOperation("Failed to open file".to_string()))?;
    let reader = io::BufReader::new(file);

    let mut edges = Vec::new();

    for (line_number, line) in reader.lines().enumerate() {
        let line =
            line.map_err(|_| GraphError::InvalidOperation("Failed to read line".to_string()))?;

        if line.trim().is_empty() || line.starts_with('#') {
            continue; // Skip empty lines and comments
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            return Err(GraphError::InvalidOperation(format!(
                "Invalid format at line {}: {}",
                line_number + 1,
                line
            )));
        }

        let u = parts[0].parse::<usize>().map_err(|_| {
            GraphError::InvalidOperation(format!(
                "Invalid vertex format at line {}: {}",
                line_number + 1,
                line
            ))
        })?;
        let v = parts[1].parse::<usize>().map_err(|_| {
            GraphError::InvalidOperation(format!(
                "Invalid vertex format at line {}: {}",
                line_number + 1,
                line
            ))
        })?;
        let weight = if parts.len() == 3 {
            Some(parts[2].parse().map_err(|_| {
                GraphError::InvalidOperation(format!(
                    "Invalid weight at line {}: {}",
                    line_number + 1,
                    line
                ))
            })?)
        } else {
            None
        };

        edges.push((u, v, weight));
    }

    Ok(edges)
}
