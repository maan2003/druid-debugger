use druid::{Selector, WidgetId};

mod data;
mod ui;
mod widget;
mod widget_ext;

pub const INSPECT: Selector<()> = Selector::new("druid-debugger.inspect");
pub const INSPECT_RESPONSE: Selector<WidgetId> = Selector::new("druid-debugger.inspect-response");

pub use widget_ext::WidgetExt;
pub use ui::ui_builder;
