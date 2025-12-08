use leptos::prelude::*;

#[component]
pub fn Icon(
  svg: impl Into<&'static str>,
  #[prop(optional)] width: &'static str,
  #[prop(optional)] height: &'static str,
  #[prop(optional)] class: &'static str,
  #[prop(optional)] align: &'static str,
  #[prop(optional)] fill: &'static str,
) -> impl IntoView {
  view! {
    <span
      class=class
      class:icon
      style:fill=move || if fill.is_empty() { None } else { Some(fill) }
      style:vertical-align=move || if align.is_empty() { None } else { Some(align) }
      style:width=move || if width.is_empty() { None } else { Some(width) }
      style:height=move || if height.is_empty() { None } else { Some(height) }
      inner_html=svg.into() />
  }
}
