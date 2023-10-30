use gpui2::{div, white, ParentElement, Styled, View, VisualContext, WindowContext};

pub struct TextStory {
    text: View<()>,
}

impl TextStory {
    pub fn view(cx: &mut WindowContext) -> View<()> {
        cx.build_view(|cx| (), |_, cx| {
            div()
                .size_full()
                .bg(white())
                .child(concat!(
                    "The quick brown fox jumps over the lazy dog. ",
                    "Meanwhile, the lazy dog decided it was time for a change. ",
                    "He started daily workout routines, ate healthier and became the fastest dog in town.",
                ))
        })
    }
}
