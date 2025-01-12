pub mod plot {
    use crate::common::common::Entry;
    use textplots::{Chart, LabelBuilder, LabelFormat, Plot, Shape};

    use chrono::{DateTime, Local, TimeZone, Timelike};

    fn to_hour_and_minute(timestamp: u64) -> Option<(u32, u32)> {
        let datetime: DateTime<Local> = Local.timestamp_opt(timestamp as i64, 0u32).single()?;
        Some((datetime.hour(), datetime.minute()))
    }

    pub fn plot_entries(entries: Vec<Entry>) {
        let mut increment = 0.0;
        let entry_data: Vec<(f32, f32)> = entries
            .iter()
            .filter_map(|value| {
                to_hour_and_minute(value.entry).and_then(|(hour, minute)| {
                    let hour = hour as f32;
                    let minute = minute as f32 / 60.0;
                    increment += 1.0;
                    Some((increment, hour + minute))
                })
            })
            .collect();
        increment = 0.0;
        let exit_data: Vec<(f32, f32)> = entries
            .iter()
            .filter_map(|value: &Entry| {
                to_hour_and_minute(value.exit).and_then(|(hour, minute)| {
                    let hour = hour as f32;
                    let minute = minute as f32 / 60.0;
                    increment += 1.0;
                    Some((increment, hour + minute))
                })
            })
            .collect();

        for val in &entry_data {
            println!("{:?}", val);
        }
        Chart::new(150, 40, 0.0, 5.0)
            .lineplot(&Shape::Steps(&entry_data))
            .x_label_format(LabelFormat::Custom(Box::new(move |val| {
                format!("{}", val as i64)
            })))
            .nice();
        Chart::new(150, 40, 0.0, 5.0)
            .lineplot(&Shape::Steps(&exit_data))
            .nice();
    }
}
