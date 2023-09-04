use crate::app::widget::CompositeWidget;

pub trait Component: CompositeWidget {
    fn on_select(self: &mut Self);
    fn on_deselect(self: &mut Self);
}
