use crate::app::widget::CompositeWidget;

pub trait Component: CompositeWidget {
    fn on_select(self: &mut Self);
    fn on_deselect(self: &mut Self);

    fn insert_one(self: &mut Self);
    fn insert_zero(self: &mut Self);
    fn remove_one(self: &mut Self);
}
