/*
 * Values in this file correspond to the school calendar.
 * This file should be updated once a new school calendar is released
 * i.e. each semester.
 */

/*
 * Duration of the current semester.
 */
pub const SEMESTER_START: (u32, u32) = (9, 9); // The day before the semester starts
pub const SEMESTER_END: (u32, u32) = (12, 21); // The day the semester ends

/*
 * Days on which we will not have any classes
 */
pub const VACATION_DAYS: &'static [(u32, u32)] = &[
    (9, 24)
];

/*
 * Weeks that do not count into weeks in a semester
 */
pub const VACATION_WEEKS: &'static [(u32, u32)] = &[
    // Nothing here yet.
    (10, 1)
];
