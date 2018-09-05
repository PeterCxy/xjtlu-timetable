/*
 * Simple and naive ical file builder
 * only for use in this crate, so only GMT+8 (Asia/Shanghai)
 * can be processed correctly
 */
use constants::*;
use chrono::{Datelike, DateTime, Duration, Utc, FixedOffset, TimeZone};
use parser::Class;

/*
 * Convert a list of XJTLU classes to ical file (.ics)
 */
pub fn classes_to_ical(classes: &[Class]) -> ICalBuilder {
    let mut builder = ICalBuilder::new();
    let tz = offset_utc8();
    let mut current_date = tz.ymd(
        Utc::now().with_timezone(&tz).year(),
        SEMESTER_START.0,
        SEMESTER_START.1
    );
    let mut current_week = 0;
    while current_date.month() != SEMESTER_END.0 || current_date.day() != SEMESTER_END.1 {
        // Go to the next day
        current_date = current_date.succ();
        let day = current_date.weekday().num_days_from_monday();

        if day == 0 {
            if VACATION_WEEKS.contains(&(current_date.month(), current_date.day())) {
                for _ in 0..7 {
                    current_date = current_date.succ();
                }
            } else {
                current_week += 1;
            }
        }

        // Skip vacations
        if VACATION_DAYS.contains(&(current_date.month(), current_date.day())) {
            continue;
        }

        for class in classes {
            if !class.weeks[current_week - 1] || class.day != (day as usize) {
                continue;
            }

            let min = if class.start.half { 30 } else { 0 };
            let start = current_date.and_hms(class.start.hour as u32, min, 0);
            let end = start + Duration::minutes((class.len as i64) * 30);

            builder.add(ICalEvent {
                summary: class.title.clone(),
                start,
                end,
                location: class.location.clone(),
                description: format!("by {}", class.lecturer)
            })
        }
    }
    return builder;
}

pub trait ICalElement {
    fn serialize(&self) -> String;
}

pub struct ICalEvent {
    summary: String,
    start: DateTime<FixedOffset>,
    end: DateTime<FixedOffset>,
    location: String,
    description: String
}

impl ICalElement for ICalEvent {
    fn serialize(&self) -> String {
        format!(r#"
            BEGIN:VEVENT
            SUMMARY:{}
            DTSTART;TZID=Asia/Shanghai:{}
            DTEND;TZID=Asia/Shanghai:{}
            LOCATION:{}
            DESCRIPTION:{}
            END:VEVENT
        "#,
            escape(&self.summary),
            format_utc8_str(&self.start),
            format_utc8_str(&self.end),
            escape(&self.location),
            escape(&self.description)
        )
    }
}

pub struct ICalBuilder {
    elements: Vec<Box<ICalElement>>
}

impl ICalBuilder {
    fn new() -> ICalBuilder {
        ICalBuilder {
            elements: Vec::new()
        }
    }

    fn add<E: 'static + ICalElement>(&mut self, elem: E) {
        self.elements.push(Box::new(elem));
    }
}

impl ICalElement for ICalBuilder {
    fn serialize(&self) -> String {
        let elem_str: Vec<_> = self.elements.iter()
            .map(|elem| elem.serialize())
            .collect();
        trim_all_lines(format!(r#"
            BEGIN:VCALENDAR
            VERSION:2.0
            CALSCALE:GREGORIAN
            {}
            END:VCALENDAR
        "#, elem_str.concat()))
    }
}

fn escape(s: &str) -> String {
    s.replace(",", "\\,")
}

#[inline(always)]
fn offset_utc8() -> FixedOffset {
    FixedOffset::east(8 * 3600)
}

fn format_utc8_str(time: &DateTime<FixedOffset>) -> String {
    format!("{}", time.with_timezone(&offset_utc8())
        .format("%Y%m%dT%H%M%S"))
}

fn trim_all_lines(s: String) -> String {
    s.lines()
        .map(|l| l.trim())
        .filter(|l| l != &"")
        .map(|l| format!("{}\n", l))
        .collect()
}
