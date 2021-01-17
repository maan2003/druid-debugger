use druid::{
    widget::{Button, Flex, Label, List},
    Target, Widget, WidgetExt, WidgetPod,
};

use crate::{
    data::{DebugItem, DebuggerData, Event},
    widget::{AppWrapper, Maybe},
    INSPECT,
};

pub fn ui_builder<T: druid::Data>() -> impl Widget<T> {
    AppWrapper {
        inner: WidgetPod::new(
            Maybe::new(widget_page, selector_page)
                .lens(DebuggerData::item)
                .boxed(),
        ),
        data: DebuggerData { item: None },
    }
}

fn selector_page() -> impl Widget<()> {
    Button::new("Inspect")
        .on_click(|ctx, _, _| ctx.submit_command(INSPECT.to(Target::Global)))
        .center()
}

fn widget_page() -> impl Widget<DebugItem> {
    Flex::column()
        .with_child(
            Label::dynamic(|data: &DebugItem, _| data.name.clone())
                .with_text_size(24.0)
                .center(),
        )
        .with_spacer(20.)
        .with_child(Label::new("Events").with_text_size(18.))
        .with_default_spacer()
        .with_child(List::new(event).with_spacing(5.).lens(DebugItem::events))
}

fn event() -> impl Widget<Event> {
    Label::dynamic(|x, _| format!("{:?}", x))
}
