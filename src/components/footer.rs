use crate::components::{icon::Icon, webring::Webring};
use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
  view! {
    <div class="footer">
      <div class="credits">
        made in <Icon svg=simpleicons_rs::SIRUST.svg />
        and <Icon svg=simpleicons_rs::SIWEBASSEMBLY.svg />
        with <Icon svg=simpleicons_rs::SILEPTOS.svg /> by oljoi
      </div>
      <hr />
      <Webring />
    </div>
  }
}
