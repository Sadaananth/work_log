pub mod plot {
    use crate::{common::common::Entry};
    use textplots::{Chart, Plot, Shape};

    pub fn plot_entries(entries: Vec<Entry>) {
        let data: Vec<(f32, f32)> = entries.iter()
        .map(|entry| (entry.entry as f32, entry.exit as f32))
        .collect();

        Chart::new(180, 60, 0.0, 6.0)
        .lineplot(&Shape::Lines(&data))
        .display();
    }
}
