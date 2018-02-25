use stdweb::unstable::TryInto;
use stdweb::web::{alert, IElement, INode, IParentNode, Element, NodeList};
use util::{ToElement, ElementAttribute};

pub enum Weekdays {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday
}

pub struct Class {
    title: String,
    lecturer: String,
    location: String,
    day: Weekdays,
    start: String, 
    len: usize // length in half-hours
}

/*
 * Try to parse the content of an element as XJTLU class timetable
 * If it is not, an Err will be returned
 */
pub fn parse(elem: &Element) -> Result<Vec<Class>, String> {
    // For XJTLU class timetables, all the content
    // Are inside a table whose class is `.maintable`
    // First find all the rows of that table
    elem.query_selector_all(".maintable > tbody > tr")
        .map_err(|_| "Failed to find the timetable element".to_string())
        .and_then(|rows| {
            // Parse the row list
            parse_rows(rows)
        })
}

#[derive(Clone)]
struct RowSpanCell {
    cell_index: usize,
    remaining_rows: usize
}

/*
 * Parse the list of rows selected from the main table
 * of XJTLU class timetable.
 * All the information should be in this list.
 */
fn parse_rows(rows: NodeList) -> Result<Vec<Class>, String> {
    let mut ret: Vec<Class> = Vec::new();
    let mut row_span_cells: Vec<RowSpanCell> = Vec::new();
    let mut pending_span_cells: Vec<RowSpanCell> = Vec::new();

    for (row_index, r) in rows.iter().enumerate() {
        // A row must be an Element
        let row_elem = r.to_element()
            .ok_or(format!("Invalid row {}", row_index))?;

        // A row must have a cell whose class is `coltitle` which
        // indicates the corresponding class time of this row.
        let coltitle = row_elem.query_selector("td.coltitle")
            .map_err(|_| "")
            .and_then(|r| r.ok_or(""))
            .and_then(|elem| elem.text_content().ok_or(""))
            .unwrap_or("".to_string());
        if coltitle == "" {
            continue;
        }

        // Find all the grid cells
        let row_columns = row_elem.query_selector_all("td.gridcell")
                .map_err(|_| format!("Invalid row {}", row_index))?;

        // All the cells representing classes in XJTLU class timetable
        // will span over multiple rows, using the `rowspan` attribute.
        // Thus row may contain implicit hidden cells (i.e. not in `td` list)
        // that is a cell from a former row.
        // For example, if the former row has a cell at column 0
        // whose `rowspan` is 2, then the first cell of the current row
        // is actually the cell at colum 1 instead of column 0.
        // So we need an `offset` state to keep track of this for each row.
        let mut offset = 0;
        for (j, c) in row_columns.iter().enumerate() {
            let mut col_index = j + offset;

            // Loop over all the previously known cells that have a `rowspan` attribute
            for span_cell in &row_span_cells {
                // Increase offset by 1 if there is a cell that have a `rowspan`
                // previously at the current column.
                if span_cell.cell_index == col_index {
                    col_index += 1;
                    offset += 1;
                }
            }

            // A column must always be an element too.
            let column_elem = c.to_element()
                .ok_or(format!("Invalid column {}:{}", row_index, col_index))?;

            // If the cell is a `nonemptycell` then it represents a class
            // and have a `rowspan` attribute
            if column_elem.class_list().contains("nonemptycell") {
                let rowspan_value: usize = column_elem.get_attribute("rowspan").try_into()
                    .map_err(|_| ())
                    .and_then(|s: String| s.parse()
                        .map_err(|_| ()))
                    .unwrap_or(0);
                if rowspan_value > 0 {
                    // Record the rowspan attribute of this cell
                    // but it should be pending until we finish
                    // this row.
                    pending_span_cells.push(RowSpanCell {
                        cell_index: col_index,
                        remaining_rows: rowspan_value
                    })
                } else {
                    // As far as I am concerned, there is no half-an-hour classes.
                    return Err(format!("Invalid class at {}:{}", row_index, col_index));
                }

                // TODO: finish parsing
                alert(&format!("{}:{}: {:?}", row_index, col_index, column_elem.text_content()));
            }
        }

        // Append all pending cells into the span cells list
        row_span_cells.append(&mut pending_span_cells);
        
        // Decrease all `remaining_rows` by 1
        for span_cell in &mut row_span_cells {
            span_cell.remaining_rows -= 1;
        }

        // Remove the span cells that do not have any remining rows
        row_span_cells.retain(|cell| cell.remaining_rows != 0);
    }
    Ok(ret)
}