use druid::Widget;

use crate::widget::DebuggerWidget;

pub trait WidgetExt<T: druid::Data>: Widget<T> + Sized + 'static {
    fn debug(self, name: impl Into<String>) -> DebuggerWidget<T> {
        DebuggerWidget::new(Box::new(self), name.into())
    }
}

impl<T: druid::Data, W: Widget<T> + Sized + 'static> WidgetExt<T> for W {}
