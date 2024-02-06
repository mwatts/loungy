use gpui::*;

use crate::state::StateModel;
use crate::theme::Theme;

pub struct Workspace {
    state: StateModel,
}

impl Workspace {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        let view = cx.new_view(|cx| {
            let state = StateModel::init(cx);
            Workspace { state }
        });
        view
    }
}

impl Render for Workspace {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let item = self.state.inner.read(cx).stack.last().unwrap();
        div()
            .full()
            .flex()
            .flex_col()
            .bg(theme.base)
            //.rounded_xl()
            //.border_2()
            //.border_color(theme.crust)
            .font("Inter")
            .text_color(theme.text)
            .child(
                div()
                    .child(item.query.clone())
                    .text_lg()
                    .p_2()
                    .w_full()
                    .border_b_1()
                    .border_color(theme.mantle),
            )
            .child(div().child(item.view.clone()).p_2())
            .child(
                div()
                    .absolute()
                    .bottom_0()
                    .left_0()
                    .right_0()
                    .bg(theme.mantle)
                    .w_full()
                    .border_t_1()
                    .border_color(theme.crust)
                    .px_4()
                    .py_2()
                    .text_color(theme.subtext0)
                    .text_xs()
                    .flex()
                    .child(
                        div()
                            .mr_2()
                            .on_mouse_down(MouseButton::Left, |_ev, cx| {
                                Theme::change(catppuccin::Flavour::Latte, cx);
                            })
                            .child("Latte"),
                    )
                    .child(
                        div()
                            .mr_2()
                            .on_mouse_down(MouseButton::Left, |_ev, cx| {
                                Theme::change(catppuccin::Flavour::Mocha, cx);
                            })
                            .child("Mocha"),
                    )
                    .child(
                        div()
                            .mr_2()
                            .on_mouse_down(MouseButton::Left, |_ev, cx| {
                                Theme::change(catppuccin::Flavour::Frappe, cx);
                            })
                            .child("Frappe"),
                    )
                    .child(
                        div()
                            .mr_2()
                            .on_mouse_down(MouseButton::Left, |_ev, cx| {
                                Theme::change(catppuccin::Flavour::Macchiato, cx);
                            })
                            .child("Macchiato"),
                    )
                    .child(item.actions.inner.clone()),
            )
    }
}
