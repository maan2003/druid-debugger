use druid::{im, Data, KeyEvent, Lens, MouseEvent, WidgetId};
use im::Vector;

#[derive(Data, Clone, Lens)]
pub struct DebuggerData {
    pub item: Option<DebugItem>,
}

#[derive(Data, Clone, Lens)]
pub struct DebugItem {
    pub name: String,
    pub events: im::Vector<Event>,
    #[data(same_fn = "PartialEq::eq")]
    pub widget_id: WidgetId,
}

#[derive(Clone, Debug)]
pub struct Event(pub druid::Event);

impl Data for Event {
    fn same(&self, other: &Self) -> bool {
        let mouse_ev = |a: &MouseEvent, b: &MouseEvent| {
            a.pos == b.pos
                && a.window_pos == b.window_pos
                && a.button == b.button
                && a.buttons == b.buttons
                && a.count == b.count
                && a.focus == b.focus
                && a.mods == b.mods
                && a.wheel_delta == b.wheel_delta
        };
        let key_ev = |a: &KeyEvent, b: &KeyEvent| {
            a.code == b.code
                && a.is_composing == b.is_composing
                && a.key == b.key
                && a.location == b.location
                && a.mods == b.mods
                && a.repeat == b.repeat
                && a.state == b.state
        };
        match (&self.0, &other.0) {
            (druid::Event::WindowConnected, druid::Event::WindowConnected) => true,
            (druid::Event::WindowCloseRequested, druid::Event::WindowCloseRequested) => true,
            (druid::Event::WindowDisconnected, druid::Event::WindowDisconnected) => true,
            (druid::Event::WindowSize(a), druid::Event::WindowSize(b)) => a == b,
            (druid::Event::MouseDown(a), druid::Event::MouseDown(b)) => mouse_ev(a, b),
            (druid::Event::MouseUp(a), druid::Event::MouseUp(b)) => mouse_ev(a, b),
            (druid::Event::MouseMove(a), druid::Event::MouseMove(b)) => mouse_ev(a, b),
            (druid::Event::Wheel(a), druid::Event::Wheel(b)) => mouse_ev(a, b),
            (druid::Event::KeyDown(a), druid::Event::KeyDown(b)) => key_ev(a, b),
            (druid::Event::KeyUp(a), druid::Event::KeyUp(b)) => key_ev(a, b),
            (druid::Event::Paste(a), druid::Event::Paste(b)) => true,
            (druid::Event::Zoom(_), druid::Event::Zoom(_)) => true,
            (druid::Event::Timer(_), druid::Event::Timer(_)) => true,
            (druid::Event::AnimFrame(_), druid::Event::AnimFrame(_)) => true,
            (druid::Event::Command(_), druid::Event::Command(_)) => true,
            (druid::Event::Notification(_), druid::Event::Notification(_)) => true,
            (druid::Event::Internal(_), druid::Event::Internal(_)) => true,
            _ => false,
        }
    }
}
