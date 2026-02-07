/// Converts a type to another type.
pub(crate) trait Convert<T> {
    #[must_use]
    fn convert(self) -> T;
}

impl<const N: usize, O, I: Convert<O>> Convert<[O; N]> for [I; N] {
    fn convert(self) -> [O; N] {
        self.map(I::convert)
    }
}

impl Convert<egui::Pos2> for lyon_path::math::Point {
    fn convert(self) -> egui::Pos2 {
        egui::Pos2::new(self.x, self.y)
    }
}
