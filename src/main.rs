use std::env;

use log::{info, Level};
use logram::{self};
mod replies;

fn main() {
    logram::init(
        Level::Info,
        String::from(env::var("TELEGRAM_TOKEN").unwrap()),
        String::from(env::var("TELEGRAM_CHAT_ID").unwrap()),
        None,
    )
    .unwrap();

    let holidays = env::var("HOLIDAYS").unwrap();
    let args = holidays.split(",").collect::<Vec<&str>>();

    let start_date = &args[0];
    let end_date = &args[1];
    let work_date = if args.len() > 2 { Some(args[2]) } else { None };

    let message = replies::get_out_of_office_replies(start_date, end_date, work_date);
    info!("\n{}", message);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_holidays() {
        assert_eq!("I will be out of the office from Thursday, January 23, 2020 to Friday, January 24, 2020. I will respond to your email when I return to work on Monday.", replies::get_out_of_office_replies("2020-1-23", "2020-1-24", Some("2020-1-27")));
    }

    #[test]
    fn test_long_holidays() {
        assert_eq!("I will be out of office from Thursday, January 23, 2020 through Wednesday, January 29, 2020. I will be back to office on Thursday, January 30.\n\nDuring this period I will have limited access to my email. I will try to respond to your email as soon as I can.", replies::get_out_of_office_replies("2020-1-23", "2020-1-29", None));
    }
}
