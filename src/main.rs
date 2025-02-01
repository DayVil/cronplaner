mod configuration;
mod view;

use anyhow::Result;
use configuration::TimeSlotsConfig;
use prettytable::{row, Table};
use terminal_size::{terminal_size, Width};

fn main() -> Result<()> {
    let time_slots = TimeSlotsConfig::new()?;

    let diffs = time_slots.compare_to_today();

    let mut table = Table::new();

    table.add_row(row!["Name", "Due Date", "Remaining"]);

    for diff in diffs {
        let remaining = if diff.less_one_day() {
            diff.to_hours().to_string()
        } else {
            format!("{} d", diff.to_days())
        };

        table.add_row(row![diff.name, diff.due, remaining]);
    }

    // Convert the table to a string
    let table_string = table.to_string();

    // Get the terminal width
    let terminal_width = if let Some((Width(w), _)) = terminal_size() {
        w as usize
    } else {
        80 // Default width if terminal size cannot be determined
    };

    // Calculate padding to center the table
    let table_width = table_string.lines().next().map_or(0, |line| line.len());
    let padding = (terminal_width.saturating_sub(table_width)) / 2;

    // Print the table with padding
    for line in table_string.lines() {
        println!("{:>width$}", line, width = padding + line.len());
    }

    time_slots.write_back_to_file()?;
    Ok(())
}
