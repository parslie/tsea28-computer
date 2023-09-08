pub mod value5;
pub mod value7;
pub mod value8;
pub mod value16;
pub mod list7;
pub mod list16;
pub mod list25;

use crate::app::widget::CompositeWidget;

pub trait Component: CompositeWidget {
    fn on_select(self: &mut Self);
    fn on_deselect(self: &mut Self);

    fn insert_one(self: &mut Self);
    fn insert_zero(self: &mut Self);
    fn remove_one(self: &mut Self);
}
