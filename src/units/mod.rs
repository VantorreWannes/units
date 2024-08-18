pub mod metric;
pub mod imperial;

pub trait Unit where Self::UnitType: PartialOrd {
    type UnitType;

    fn unit(&self) -> Self::UnitType;

    fn grow_type(&mut self);

    fn shrink_type(&mut self);

    fn convert(&mut self, unit: Self::UnitType) {
        while self.unit() < unit {
            self.grow_type();
        }
        while self.unit() > unit {
            self.shrink_type();
        }
    }
}