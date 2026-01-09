use leptos::prelude::*;

#[component]
pub fn Webring(
  #[prop(optional)] class: &'static str,
  #[prop(default = "webring")] home_class: &'static str,
  #[prop(default = "webring")] home_name: &'static str,
  #[prop(optional)] home_url: &'static str,
  #[prop(default = "prev")] prev_class: &'static str,
  #[prop(default = "prev")] prev_name: &'static str,
  #[prop(optional)] prev_url: &'static str,
  #[prop(default = "next")] next_class: &'static str,
  #[prop(default = "next")] next_name: &'static str,
  #[prop(optional)] next_url: &'static str,
) -> impl IntoView {
  view! {
    <div class:webring class=class>
      <a class:prev_class href=prev_url>{prev_name}</a>
      <a class:home_class href=home_url>{home_name}</a>
      <a class:next_class href=next_url>{next_name}</a>
    </div>
  }
}
