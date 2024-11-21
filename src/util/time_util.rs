use std::str::FromStr;

pub(crate) fn time_to_seconds(time: &str) -> u32 {
    let mut seconds = 0;

    for fragment in time.split_whitespace() {
        if let Some(hours) = fragment.strip_suffix("h") {
            if let Ok(hours_int) = u32::from_str(hours) {
                seconds += hours_int * 3600;
            }
        } else if let Some(minutes) = fragment.strip_suffix("m") {
            if let Ok(minutes_int) = u32::from_str(minutes) {
                seconds += minutes_int * 60;
            }
        }
    }

    seconds
}
