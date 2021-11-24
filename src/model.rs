use chrono::DateTime;
use chrono::Datelike;
use chrono::Duration;
use chrono::Local;
use chrono::NaiveDate;
use chrono::TimeZone;
use chrono::Timelike;
use chrono::Utc;
use now::DateTimeNow;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Deserialize, Serialize, Debug)]
pub struct Workspace {
  pub id: u64,
  pub name: String,
  pub premium: bool,
  pub admin: bool,
  pub default_hourly_rate: f64,
  pub default_currency: String,
  pub only_admins_may_create_projects: bool,
  pub only_admins_see_billable_rates: bool,
  pub rounding: i8,
  pub rounding_minutes: i8,
  pub at: DateTime<Utc>,
  pub logo_url: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Project {
  pub id: u64,
  pub name: String,
  pub wid: u64,
  pub cid: Option<u64>,
  pub active: bool,
  pub is_private: bool,
  pub template: bool,
  pub template_id: Option<u64>,
  pub billable: bool,
  pub auto_estimates: bool,
  pub estimated_hours: Option<u64>,
  pub at: DateTime<Utc>,
  pub color: String,
  pub rate: Option<f64>,
  pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserData {
  pub id: u64,
  pub api_token: String,
  pub default_wid: u64,
  pub email: String,
  pub fullname: String,
  pub jquery_timeofday_format: String,
  pub jquery_date_format: String,
  pub timeofday_format: String,
  pub date_format: String,
  pub store_start_and_stop_time: bool,
  pub beginning_of_week: u8,
  pub language: String,
  pub image_url: String,
  pub sidebar_piechart: bool,
  pub at: DateTime<Utc>,

  #[serde(default)]
  pub new_blog_post: HashMap<String, String>,
  pub send_product_emails: bool,
  pub send_weekly_report: bool,
  pub send_timer_notifications: bool,
  pub openid_enabled: bool,
  pub timezone: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SinceWith<T> {
  pub since: u64,
  pub data: T,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DataWith<T> {
  pub data: T,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TimeEntry {
  pub id: u64,
  pub wid: u64,
  pub pid: u64,
  pub billable: bool,
  pub start: DateTime<Utc>,
  pub stop: Option<DateTime<Utc>>,
  pub duration: i64,
  pub description: Option<String>,

  #[serde(default)]
  pub tags: Vec<String>,
  pub duronly: bool,
  pub at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Client {
  pub id: u64,
  pub name: String,
  pub wid: u64,
  pub notes: Option<String>,

  // This shouldn't be an Option:
  // https://github.com/toggl/toggl_api_docs/blob/master/chapters/clients.md#create-a-client
  pub at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy)]
pub enum Range {
  Today,
  Yesterday,
  ThisWeek,
  LastWeek,
  ThisMonth,
  LastMonth,
  FromTo(NaiveDate, NaiveDate),
  Date(NaiveDate),
}

impl Range {
  pub fn as_range(self) -> (DateTime<Local>, DateTime<Local>) {
    match self {
      Range::Today => {
        let now = Local::now();
        let start = Local
          .ymd(now.year(), now.month(), now.day())
          .and_hms(0, 0, 0);
        let end = start + Duration::days(1);

        (start, end)
      }
      Range::Yesterday => {
        let now = Local::now() - Duration::days(1);

        let start = Local
          .ymd(now.year(), now.month(), now.day())
          .and_hms(0, 0, 0);
        let end = start + Duration::days(1);

        (start, end)
      }
      Range::ThisWeek => {
        let now = Local::now();

        (now.beginning_of_week(), now.end_of_week())
      }
      Range::LastWeek => {
        let now = Local::now() - Duration::weeks(1);

        (now.beginning_of_week(), now.end_of_week())
      }
      Range::ThisMonth => {
        let now = Local::now();

        (now.beginning_of_month(), now.end_of_month())
      }
      Range::LastMonth => {
        let now = Local::now();
        let date = Local.ymd(now.year(), now.month() - 1, now.day()).and_hms(
          now.hour(),
          now.minute(),
          now.second(),
        );

        (date.beginning_of_month(), date.end_of_month())
      }
      Range::FromTo(start_date, end_date) => {
        let start = start_date.and_hms(0, 0, 0);
        let end = end_date.and_hms(0, 0, 0) + Duration::days(1);

        (
          Local.from_local_datetime(&start).unwrap(),
          Local.from_local_datetime(&end).unwrap(),
        )
      }
      Range::Date(date) => {
        let start = Local
          .ymd(date.year(), date.month(), date.day())
          .and_hms(0, 0, 0);
        let end = start + Duration::days(1);

        (start, end)
      }
    }
  }
}

impl FromStr for Range {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "today" => Ok(Range::Today),
      "yesterday" => Ok(Range::Yesterday),
      "this-week" => Ok(Range::ThisWeek),
      "last-week" => Ok(Range::LastWeek),
      "this-month" => Ok(Range::ThisMonth),
      "last-month" => Ok(Range::LastMonth),
      from_to_or_date => match from_to_or_date.find('|') {
        Some(index) => Ok(Range::FromTo(
          NaiveDate::parse_from_str(&from_to_or_date[..index], "%Y-%m-%d")?,
          NaiveDate::parse_from_str(&from_to_or_date[index + 1..], "%Y-%m-%d")?,
        )),
        None => Ok(Range::Date(from_to_or_date.parse()?)),
      },
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Start {
  Now,
  Date(DateTime<Local>),
}

impl Start {
  pub fn as_date_time(self) -> DateTime<Local> {
    match self {
      Start::Now => Local::now(),
      Self::Date(date) => date,
    }
  }
}

impl FromStr for Start {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "now" => Ok(Start::Now),
      date => Ok(Start::Date(DateTime::<Local>::from_str(date)?)),
    }
  }
}
#[cfg(test)]
mod tests {
  use std::str::FromStr;

  use super::Start;
  use chrono::{DateTime, Local};
  use pretty_assertions::assert_eq;

  #[ctor::ctor]
  fn setup() {
    std::env::set_var("TZ", "Europe/Berlin");
  }

  #[ctor::dtor]
  fn teardown() {
    std::env::remove_var("TZ");
  }

  #[test]
  fn test_start() -> anyhow::Result<()> {
    let expected_date_time =
      DateTime::<Local>::from_str("2021-11-21T22:58:09+01:00")?;

    assert_eq!(
      Start::from_str("2021-11-21T22:58:09+01:00")?,
      Start::Date(expected_date_time)
    );

    Ok(())
  }
}
