//! # Michael Saylor Quotes
//!
//! `saylor-quotes` is a collection of  [Michael Saylor](https://x.com/saylor)
//! quotes with reactivity by the all-mighty [`dioxus`](https://dioxuslabs.com)
//! (no YAVASCRIPT!)
//!
//! It also has plain-text API GET endpoints at `/` that returns a Michael Saylor
//! quote for terminal users with `curl` and `wget`.

#![allow(non_snake_case)]

use dioxus::launch;
#[cfg(debug_assertions)]
use dioxus::logger::tracing::Level;

#[cfg(debug_assertions)]
use log::info;

mod app;
mod component;
mod data;

use app::App;

pub fn main() {
    #[cfg(debug_assertions)]
    {
        // init logger for Dioxus
        dioxus::logger::init(Level::INFO).expect("failed to init logger");
    }
    // launch the web app
    #[cfg(debug_assertions)]
    info!("Launching Saylor Quotes app");
    launch(App);
}
