use dioxus::prelude::*;
use dioxus_logger::tracing::debug;
use dioxus_sdk::storage::*;
use dioxus_signals::Signal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct State {
    pub something: Option<String>,
}

impl State {
    /// LocalStorage key.
    const LS_KEY: &'static str = "dioxus-05-test-storage_something";

    pub fn load_from_localstorage() -> Signal<Self> {
        let state_signal = use_synced_storage::<LocalStorage, State>(Self::LS_KEY.into(), || State::default());
        debug!(">>> [load_from_localstorage] Loaded {:?}", state_signal());
        state_signal
    }

    pub fn save_to_localstorage(&self) {
        LocalStorage::set(Self::LS_KEY.into(), self);
        debug!(">>> [save_to_localstorage] Saved {:?}", self);
    }

    pub fn new(something: String) -> Self {
        Self {
            something: Some(something),
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
