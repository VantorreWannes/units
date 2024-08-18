use units::{metric::{MetricUnit, MetricUnitVariant}, Unit};

pub mod units;

fn main() {
    println!("Hello, world!");
    let mut metric = MetricUnit::new(1.0, MetricUnitVariant::Meter);
    println!("{}", metric);
    metric.convert(MetricUnitVariant::Millimeter);
    println!("{}", metric);
}
