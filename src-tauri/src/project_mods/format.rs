use core::time;

#[derive(Debug)]
pub struct DurationFormat {
  hours: u64,
  minutes: u64,
  seconds: u64,
}

#[allow(dead_code)]
pub trait DurationFormatTrait {
  fn new(hours: u64, minutes: u64, seconds: u64) -> Self;
  fn new_from_seconds(total_seconds: u64) -> Self;
  fn new_from_duration(duration: time::Duration) -> Self;
  fn as_string(&self) -> String;
  fn hours(&self) -> u64;
  fn minutes(&self) -> u64;
  fn seconds(&self) -> u64;
}

impl DurationFormatTrait for DurationFormat {
  fn new(hours: u64, minutes: u64, seconds: u64) -> Self {
    DurationFormat {
      hours,
      minutes,
      seconds,
    }
  }
  fn new_from_seconds(total_seconds: u64) -> Self {
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    DurationFormat {
      hours,
      minutes,
      seconds,
    }
  }
  fn new_from_duration(duration: time::Duration) -> Self {
    let total_seconds = duration.as_secs() as u64;
    Self::new_from_seconds(total_seconds)
  }
  fn hours(&self) -> u64 {
    self.hours
  }
  fn minutes(&self) -> u64 {
    self.minutes
  }
  fn seconds(&self) -> u64 {
    self.seconds
  }

  fn as_string(&self) -> String {
    format!("{:02}:{:02}:{:02}", self.hours, self.minutes, self.seconds)
  }
}