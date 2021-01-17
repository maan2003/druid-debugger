use druid::{
    widget::{Flex, Label, TextBox},
    AppDelegate, AppLauncher, Data, Widget, WindowDesc,
};
use druid_debugger::WidgetExt;

fn main() {
    let window = WindowDesc::new(ui);
    AppLauncher::with_window(window)
        .use_simple_logger()
        .delegate(Delegate { init: false })
        .launch(String::new())
        .unwrap();
}

struct Delegate {
    init: bool,
}

impl<T: Data> AppDelegate<T> for Delegate {
    fn window_added(
        &mut self,
        id: druid::WindowId,
        data: &mut T,
        env: &druid::Env,
        ctx: &mut druid::DelegateCtx,
    ) {
        if !self.init {
            let window = WindowDesc::<T>::new(druid_debugger::ui_builder);
            ctx.new_window(window);
            self.init = true;
        }
    }
}

fn ui() -> impl Widget<String> {
    Flex::column()
        .with_child(Label::new("Hello wrold").debug("hello world label"))
        .with_default_spacer()
        .with_child(TextBox::new().debug("Text Box"))
}
