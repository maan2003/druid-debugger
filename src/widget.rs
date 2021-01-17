use std::mem;

use druid::{Color, Target, Widget, WidgetExt, WidgetPod, WindowId, im::Vector, widget::prelude::*};

use crate::{
    data::{self, DebugItem, DebuggerData},
    EVENT, INSPECT, INSPECT_RESPONSE,
};

pub struct AppWrapper {
    pub inner: WidgetPod<DebuggerData, Box<dyn Widget<DebuggerData>>>,
    pub data: DebuggerData,
}

impl<T: Data> Widget<T> for AppWrapper {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut T, env: &Env) {
        if let Event::Command(cmd) = event {
            if let Some((widget_id, name)) = cmd.get(INSPECT_RESPONSE).cloned() {
                self.data.item = Some(DebugItem {
                    name,
                    events: Vector::new(),
                    widget_id,
                });
            }

            if let (Some((_widget_id, event)), Some(item)) =
                (cmd.get(EVENT).cloned(), &mut self.data.item)
            {
                item.events.push_back(data::Event(event));
                ctx.children_changed();
                // return;
            }
        }
        self.inner.event(ctx, event, &mut self.data, env);
        ctx.request_update();
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, _data: &T, env: &Env) {
        self.inner.lifecycle(ctx, event, &self.data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, _data: &T, env: &Env) {
        self.inner.update(ctx, &self.data, env)
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &T, env: &Env) -> Size {
        let size = self.inner.layout(ctx, bc, &self.data, env);
        self.inner.set_origin(ctx, &self.data, env, (0., 0.).into());
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &T, env: &Env) {
        self.inner.paint(ctx, &self.data, env)
    }
}

pub struct DebuggerWidget<T> {
    inner: Box<dyn Widget<T>>,
    is_selecting: bool,
    debug_name: String,
    attached: bool,
}

impl<T: Data> DebuggerWidget<T> {
    pub fn new(inner: Box<dyn Widget<T>>, debug_name: String) -> Self {
        Self {
            inner,
            debug_name,
            is_selecting: false,
            attached: false,
        }
    }
}

impl<T: Data> Widget<T> for DebuggerWidget<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        if let Event::Command(cmd) = event {
            if cmd.is(INSPECT) {
                self.is_selecting = true;
                ctx.request_paint();
            }
            if cmd.is(INSPECT_RESPONSE) {
                self.is_selecting = false;
                ctx.request_paint();
            }
        };
        match event {
            Event::MouseDown(_) => ctx.set_active(true),
            Event::MouseUp(_) if ctx.is_active() && self.is_selecting => {
                ctx.set_active(false);
                ctx.submit_command(
                    INSPECT_RESPONSE
                        .with((ctx.widget_id(), self.debug_name.clone()))
                        .to(Target::Global),
                );
                self.attached = true;
                return;
            }
            _ => {}
        }

        if self.attached {
            ctx.submit_command(
                EVENT
                    .with((ctx.widget_id(), event.clone()))
                    // totally not ashamed
                    .to(Target::Window(unsafe { mem::transmute(2u64) })),
            );
        }
        if !self.is_selecting || event.should_propagate_to_hidden() {
            self.inner.event(ctx, event, data, env);
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if let LifeCycle::HotChanged(c) = event {
            ctx.request_paint();
        }
        self.inner.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.inner.update(ctx, old_data, data, env)
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.inner.paint(ctx, data, env);
        if ctx.is_hot() && self.is_selecting {
            let rect = ctx.size().to_rect();
            ctx.fill(rect, &Color::GREEN.with_alpha(0.4));
        }
    }
}

// Taken from runebender (https://github.com/linebender/runebender)
use druid::widget::SizedBox;

/// A widget that switches between two possible child views, for `Data` that
/// is `Option<T>`.
pub struct Maybe<T> {
    some_maker: Box<dyn Fn() -> Box<dyn Widget<T>>>,
    none_maker: Box<dyn Fn() -> Box<dyn Widget<()>>>,
    widget: MaybeWidget<T>,
}

#[allow(clippy::large_enum_variant)]
enum MaybeWidget<T> {
    Some(WidgetPod<T, Box<dyn Widget<T>>>),
    None(WidgetPod<(), Box<dyn Widget<()>>>),
}

impl<T: Data> Maybe<T> {
    /// Create a new `Maybe` widget with a `Some` and a `None` branch.
    pub fn new<W1, W2>(
        // we make these generic so that the caller doesn't have to explicitly
        // box. We don't technically *need* to box, but it seems simpler.
        some_maker: impl Fn() -> W1 + 'static,
        none_maker: impl Fn() -> W2 + 'static,
    ) -> Maybe<T>
    where
        W1: Widget<T> + 'static,
        W2: Widget<()> + 'static,
    {
        let widget = MaybeWidget::Some(WidgetPod::new(some_maker().boxed()));
        Maybe {
            some_maker: Box::new(move || some_maker().boxed()),
            none_maker: Box::new(move || none_maker().boxed()),
            widget,
        }
    }

    /// Create a new `Maybe` widget where the `None` branch is an empty widget.
    #[allow(dead_code)]
    pub fn or_empty<W1: Widget<T> + 'static>(some_maker: impl Fn() -> W1 + 'static) -> Maybe<T> {
        Self::new(some_maker, SizedBox::empty)
    }

    fn rebuild_widget(&mut self, is_some: bool) {
        if is_some {
            self.widget = MaybeWidget::Some(WidgetPod::new((self.some_maker)()));
        } else {
            self.widget = MaybeWidget::None(WidgetPod::new((self.none_maker)()));
        }
    }
}

impl<T: Data> Widget<Option<T>> for Maybe<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Option<T>, env: &Env) {
        if data.is_some() == self.widget.is_some() {
            match data.as_mut() {
                Some(d) => self.widget.with_some(|w| w.event(ctx, event, d, env)),
                None => self.widget.with_none(|w| w.event(ctx, event, &mut (), env)),
            };
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &Option<T>,
        env: &Env,
    ) {
        if data.is_some() != self.widget.is_some() {
            // possible if getting lifecycle after an event that changed the data,
            // or on WidgetAdded
            self.rebuild_widget(data.is_some());
        }
        assert_eq!(data.is_some(), self.widget.is_some(), "{:?}", event);
        match data.as_ref() {
            Some(d) => self.widget.with_some(|w| w.lifecycle(ctx, event, d, env)),
            None => self.widget.with_none(|w| w.lifecycle(ctx, event, &(), env)),
        };
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &Option<T>, data: &Option<T>, env: &Env) {
        if old_data.is_some() != data.is_some() {
            self.rebuild_widget(data.is_some());
            ctx.children_changed();
        } else {
            match data {
                Some(new) => self.widget.with_some(|w| w.update(ctx, new, env)),
                None => self.widget.with_none(|w| w.update(ctx, &(), env)),
            };
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &Option<T>,
        env: &Env,
    ) -> Size {
        match data.as_ref() {
            Some(d) => self.widget.with_some(|w| {
                let size = w.layout(ctx, bc, d, env);
                w.set_layout_rect(ctx, d, env, size.to_rect());
                size
            }),
            None => self.widget.with_none(|w| {
                let size = w.layout(ctx, bc, &(), env);
                w.set_layout_rect(ctx, &(), env, size.to_rect());
                size
            }),
        }
        .unwrap_or_default()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Option<T>, env: &Env) {
        match data.as_ref() {
            Some(d) => self.widget.with_some(|w| w.paint(ctx, d, env)),
            None => self.widget.with_none(|w| w.paint(ctx, &(), env)),
        };
    }
}

impl<T> MaybeWidget<T> {
    fn is_some(&self) -> bool {
        match self {
            Self::Some(_) => true,
            Self::None(_) => false,
        }
    }

    fn with_some<R, F: FnOnce(&mut WidgetPod<T, Box<dyn Widget<T>>>) -> R>(
        &mut self,
        f: F,
    ) -> Option<R> {
        match self {
            Self::Some(widget) => Some(f(widget)),
            Self::None(_) => {
                // log::warn!("Maybe::with_some called on none value");
                None
            }
        }
    }

    fn with_none<R, F: FnOnce(&mut WidgetPod<(), Box<dyn Widget<()>>>) -> R>(
        &mut self,
        f: F,
    ) -> Option<R> {
        match self {
            Self::None(widget) => Some(f(widget)),
            Self::Some(_) => {
                // log::warn!("Maybe::with_none called on none value");
                None
            }
        }
    }
}
