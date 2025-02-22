use crate::READOUTS;
use bytesize::ByteSize;
use macchina_read::traits::{BatteryReadout, GeneralReadout, MemoryReadout, ReadoutError};

/// This function should return a new `String` constructed from the value \
/// returned by `READOUTS.general.uptime()`
pub fn uptime(shorthand: bool) -> Result<String, ReadoutError> {
    let mut formatted_uptime = String::new();
    let uptime: f32 = READOUTS.general.uptime()?.parse().unwrap();
    // Uptime is formatted to "x days, y hours, z minutes" if the system
    // has been up for longer than 60 seconds, and "x seconds" if not.

    // "x days", "y hours" or "z minutes" might not show up if their value is 0.
    // for example, if the system has been up for less than a day,
    // this function will return "y hours, z minutes".
    if uptime > 60.0 {
        let up_days = (uptime / 60.0 / 60.0 / 24.0).floor();
        let up_hours = (uptime / 60.0 / 60.0 % 24.0).floor();
        let up_minutes = (uptime / 60.0 % 60.0).floor();
        match shorthand {
            true => {
                if up_days != 0.0 {
                    formatted_uptime.push_str(&up_days.to_string());
                    formatted_uptime.push_str("d ");
                }
                if up_hours != 0.0 {
                    formatted_uptime.push_str(&up_hours.to_string());
                    formatted_uptime.push_str("h ");
                }
                if up_minutes != 0.0 {
                    formatted_uptime.push_str(&up_minutes.to_string());
                    formatted_uptime.push('m');
                }
            }
            false => {
                if up_days != 0.0 {
                    if (up_days - 1.0).abs() < 0.001 {
                        formatted_uptime.push_str(&up_days.to_string());
                        formatted_uptime.push_str(" day ");
                    } else {
                        formatted_uptime.push_str(&up_days.to_string());
                        formatted_uptime.push_str(" days ");
                    }
                }
                if up_hours != 0.0 {
                    if (up_hours - 1.0).abs() < 0.001 {
                        formatted_uptime.push_str(&up_hours.to_string());
                        formatted_uptime.push_str(" hour ");
                    } else {
                        formatted_uptime.push_str(&up_hours.to_string());
                        formatted_uptime.push_str(" hours ");
                    }
                }
                if up_minutes != 0.0 {
                    if (up_minutes - 1.0).abs() < 0.001 {
                        formatted_uptime.push_str(&up_minutes.to_string());
                        formatted_uptime.push_str(" minute");
                    } else {
                        formatted_uptime.push_str(&up_minutes.to_string());
                        formatted_uptime.push_str(" minutes");
                    }
                }
            }
        }
    }
    // Uptime is formatted to seconds only if the system has been up for fewer than 60 seconds
    else {
        let up_seconds = (uptime % 60.0).floor();
        if up_seconds != 0.0 {
            formatted_uptime = up_seconds.to_string();
            formatted_uptime.push('s');
        }
    }

    Ok(formatted_uptime.trim().to_string())
}

/// This function should return a new `String` constructed from the values \
/// returned by `READOUTS.general.username()` and `READOUTS.general.hostname()`
pub fn host() -> Result<String, ReadoutError> {
    let username = READOUTS.general.username()?;
    let hostname = READOUTS.general.hostname()?;

    Ok(format!("{}@{}", username, hostname))
}

/// This function should return a new `String` constructed from the values \
/// returned by `READOUTS.battery.percentage()` and `READOUTS.battery.status()`
pub fn battery() -> Result<String, ReadoutError> {
    let percentage = READOUTS.battery.percentage()?;
    let status_from_read_func = READOUTS.battery.status()?;
    if !percentage.is_empty() && !status_from_read_func.is_empty() {
        // Holds either "Charging" or "Discharging" values
        return if percentage != "100" {
            if status_from_read_func == "TRUE" {
                Ok(format!("{}% & Charging", percentage))
            } else {
                Ok(format!("{}% & Discharging", percentage))
            }
        } else {
            Ok(String::from("Full"))
        };
    }

    Err(ReadoutError::MetricNotAvailable)
}

/// This function should return a new `String` constructed from the values \
/// returned by `READOUTS.memory.total()` and `READOUTS.memory.used()`
pub fn memory() -> Result<String, ReadoutError> {
    let total = ByteSize::kb(READOUTS.memory.total()?);
    let used = ByteSize::kb(READOUTS.memory.used()?);

    Ok(format!("{}/{}", used, total))
}

/// This function should return a new `String` constructed from the values \
/// returned by `READOUTS.general.cpu_model_name()` and `num_cpus::get()`
pub fn cpu() -> Result<String, ReadoutError> {
    let cpu_model = READOUTS.general.cpu_model_name()?;

    Ok(format!("{} ({})", cpu_model, num_cpus::get())
        .replace("(TM)", "™")
        .replace("(R)", "®"))
}
