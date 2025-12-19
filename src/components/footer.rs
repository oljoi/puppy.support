use crate::components::{icon::Icon, webring::Webring};
use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
  view! {
    <div class="footer">
      <div class="credits">
        made in <a href="https://rust-lang.org"><Icon svg=simpleicons_rs::SIRUST.svg /></a>
          and <a href="https://webassembly.org"><Icon svg=simpleicons_rs::SIWEBASSEMBLY.svg /></a>
          with <a href="https://www.leptos.dev"><Icon svg=simpleicons_rs::SILEPTOS.svg /></a> by oljoi
      </div>
    </div>
  }
}
