use std::fmt::Debug;

use dioxus_logger::tracing::debug;

// use dioxus_sdk::storage::*;
// use dioxus_signals::Signal;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct State {
    pub something: Option<String>,

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    localstorage: Option<web_sys::Storage>,
}

impl State {
    /// The key used to store data in LocalStorage.
    const LS_KEY: &'static str = "dioxus-05-test-storage_something";

    // -----------------------------------------------------------------------------------------------------------
    // The version that uses dioxus_sdk.
    //
    // pub fn load_from_localstorage() -> Signal<Self> {
    //     let state_signal = use_synced_storage::<LocalStorage, State>(Self::LS_KEY.into(), || State::default());
    //     debug!(">>> [load_from_localstorage] Loaded {:?}", state_signal());
    //     state_signal
    // }
    //
    // pub fn save_to_localstorage(&self) {
    //     LocalStorage::set(Self::LS_KEY.into(), self);
    //     debug!(">>> [save_to_localstorage] Saved {:?}", self);
    // }
    // -----------------------------------------------------------------------------------------------------------

    // -----------------------------------------------------------------------------------------------------------
    // The version that uses `web_sys`.

    fn new() -> Result<Self, String> {
        let window = web_sys::window().expect("no global `window` exists");
        if let Ok(Some(storage)) = window.local_storage() {
            let state = State {
                something: None,
                localstorage: Some(storage),
            };
            Ok(state)
        } else {
            debug!(">>> [State::new] Error: No local storage found!");
            Err("No local storage found".into())
        }
    }

    pub fn load_from_localstorage() -> Result<Self, String> {
        let mut state = State::new()?;
        if let Ok(Some(value)) = state.localstorage.as_ref().unwrap().get(Self::LS_KEY) {
            debug!(">>> [State::load_from_localstorage] Loaded value={:?}", value);
            state.something = Some(value);
        } else {
            debug!(">>> [State::load_from_localstorage] No value exists in localstorage.");
        }
        Ok(state)
    }

    pub fn save_to_localstorage(&self) {
        //
        if self.something.is_some() {
            self.localstorage
                .as_ref()
                .unwrap()
                .set_item(Self::LS_KEY, self.something.as_ref().unwrap())
                .unwrap();
            debug!(">>> [save_to_localstorage] Saved {:?}", self.something);
        } else {
            self.localstorage.as_ref().unwrap().remove_item(Self::LS_KEY).unwrap();
            debug!(">>> [save_to_localstorage] Removed {:?} key from localstorage.", Self::LS_KEY);
        }
    }

    // -----------------------------------------------------------------------------------------------------------
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
