use leptos::prelude::*;

#[component]
pub fn Webring(
  #[prop(optional)] class: &'static str,
  #[prop(default = "webring")] home: &'static str,
  #[prop(optional)] home_url: &'static str,
  #[prop(default = "prev")] prev: &'static str,
  #[prop(optional)] prev_url: &'static str,
  #[prop(default = "next")] next: &'static str,
  #[prop(optional)] next_url: &'static str,
) -> impl IntoView {
  view! {
    <div class:webring class=class>
      <a class:prev href=prev_url>{prev}</a>
      <a class:home href=home_url>{home}</a>
      <a class:next href=next_url>{next}</a>
    </div>
  }
}
