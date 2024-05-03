use dioxus::signals::{GlobalSignal, Signal};

pub static COUNT: GlobalSignal<i32> = Signal::global(|| 0);
