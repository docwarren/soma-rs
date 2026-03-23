use crate::models::coordinates::CoordinateSystem;


pub trait Feature: std::fmt::Display {
    fn get_chromosome(&self) -> String;
    fn get_begin(&self) -> u32;
    fn get_end(&self) -> u32;
    fn get_length(&self) -> u32;
    fn get_id(&self) -> String;
    fn coordinate_system(&self) -> CoordinateSystem;

    fn to_canonical(&self) -> (u32, u32) {
        self.coordinate_system().to_canonical(self.get_begin(), self.get_end())
    }

    fn overlaps(&self, other: &dyn Feature) -> bool {
        let (begin, end) = self.to_canonical();
        let (other_begin, other_end) = other.to_canonical();
        self.get_chromosome() == other.get_chromosome() && begin < other_end && end > other_begin
    }
}
