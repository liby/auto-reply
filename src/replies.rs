use chrono::offset::Utc;
use chrono::{Date, Duration, NaiveDate};

pub fn get_out_of_office_replies(
    start_date: &str,
    end_date: &str,
    work_date: Option<&str>,
) -> String {
    let start_naive_date = NaiveDate::parse_from_str(start_date, "%Y-%m-%d").unwrap();
    let end_naive_date = NaiveDate::parse_from_str(end_date, "%Y-%m-%d").unwrap();

    let start_utc_date = Date::<Utc>::from_utc(start_naive_date, Utc);
    let end_utc_date = Date::<Utc>::from_utc(end_naive_date, Utc);
    let work_utc_date = if let Some(date) = work_date {
        let work_naive_date = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
        Date::<Utc>::from_utc(work_naive_date, Utc)
    } else {
        end_utc_date + Duration::days(1)
    };

    let duration = end_utc_date.signed_duration_since(start_utc_date);
    if duration.num_days() > 3 {
        format!(
            "I will be out of office from {start_date} \
            through {end_date}. \
            I will be back to office on {work_date}.\n\n\
            During this period I will have limited access to my email. \
            I will try to respond to your email as soon as I can.",
            start_date = start_utc_date.format("%A, %B %d, %Y"),
            end_date = end_utc_date.format("%A, %B %d, %Y"),
            work_date = work_utc_date.format("%A, %B %d"),
        )
    } else {
        format!(
            "I will be out of the office from {start_date} \
            to {end_date}. \
            I will respond to your email when I return to work on {work_date}.",
            start_date = start_utc_date.format("%A, %B %d, %Y"),
            end_date = end_utc_date.format("%A, %B %d, %Y"),
            work_date = work_utc_date.format("%A"),
        )
    }
}
