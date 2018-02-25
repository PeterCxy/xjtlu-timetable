use stdweb::unstable::TryInto;
use stdweb::web::{IElement, INode, IParentNode, Element, NodeList};
use util::{ToElement, ElementAttribute};

#[derive(Clone, Debug)]
pub struct ClassTime {
    pub hour: usize,
    pub half: bool
}

#[derive(Debug)]
pub struct Class {
    pub title: String,
    pub lecturer: String,
    pub location: String,
    pub day: usize, // Day in a week
    pub weeks: [bool; 14], // Specify whether a class is available on week x
    pub start: ClassTime, 
    pub len: usize // length in half-hours
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
        let time_split: Vec<_> = coltitle.trim().split(":").collect();
        if time_split.len() != 2 {
            return Err(format!("Invalid time at row {}", row_index));
        }
        let current_start_time = ClassTime {
            hour: time_split[0].parse().map_err(|_| format!("Invalid hour at row {}", row_index))?,
            half: time_split[1] == "30"
        };

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
            // and have a `rowspan` attribute that corresponds to the half-hours
            // a class has.
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

                // Parse the class information
                /*let text = column_elem.text_content()
                    .ok_or(format!("Class empty at {}:{}", row_index, col_index))?;*/
                ret.push(parse_class_content(row_index, col_index, current_start_time.clone(), rowspan_value, &column_elem)?);
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

/*
 * Parse text in a cell representing a class
 * format:
 * >  title
 * >  lecturers
 * >  location
 * >  Week: x-y, z-w, t, ...
 */
fn parse_class_content(row_index: usize, day: usize, class_start: ClassTime, len: usize, content: &Element) -> Result<Class, String> {
    // Get all the lines from the current cell
    // We can't use text_content() and just split()
    // because different browsers have different logic on
    // how to add line breaks
    let lines = content.query_selector_all("tr.inR")
        .map_err(|_| format!("Invalid cell at {}:{}", row_index, day))?
        .iter()
        .map(|l| l.text_content().ok_or(format!("Invalid cell at {}:{}", row_index, day)))
        .fold(Ok(Vec::new()), |x, y| {
            if let Ok(mut v) = x {
                if let Ok(line) = y {
                    v.push(line.replace("\n", "").trim().to_string());
                    return Ok(v);
                }
            }
            Err(format!("Invalid cell at {}:{}", row_index, day))
        })?;

    if lines.len() != 4 || !lines[3].starts_with("Week:") {
        return Err("Information corrupted".to_string());
    }

    // If the class is available on week x, we later set week[x - 1] = true
    let mut weeks = [false; 14];

    // Parse the week range string
    let _weeks_text = lines[3].replace("Week:", "");
    let weeks_text = _weeks_text.trim().split(",");
    for week_range in weeks_text {
        let start_end: Vec<_> = week_range.trim().split("-").collect();
        if start_end.len() == 1 {
            // Just one week
            let w: usize = start_end[0].parse().map_err(|_| "Invalid week string".to_string())?;
            weeks[w - 1] = true;
            continue;
        }
        if start_end.len() != 2 {
            return Err("Information corrupted".to_string());
        }
        let start: usize = start_end[0].parse().map_err(|_| "Invalid week string".to_string())?;
        let end: usize = start_end[1].parse().map_err(|_| "Invalid week string".to_string())?;
        if !(start > 0 && end > 0 && end > start) {
            return Err("Information corrupted".to_string());
        }
        for i in (start - 1)..end {
            weeks[i] = true;
        }
    }

    Ok(Class {
        title: lines[0].to_string(),
        lecturer: lines[1].to_string(),
        location: lines[2].to_string(),
        day,
        weeks,
        start: class_start,
        len
    })
}