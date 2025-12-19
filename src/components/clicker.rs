use crate::components::icon::Icon;
use leptos::html::Div;
use leptos::prelude::*;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};

static PUPPY_LANG: [&'static str; 8] = ["woof", "bark", "wif", "awo", "waf", "rawr", "ow", "grr"];
#[island]
pub fn Clicker() -> impl IntoView {
  let mut hash = RandomState::new().build_hasher();

  let (phrase, set_phrase) = signal(String::new());
  let (count, set_count) = signal(1u8);
  let (reps, set_reps) = signal(0u8);
  let divref = NodeRef::<Div>::new();
  let on_click = move |_| {
    set_count.update(|c| {
      if *c < u8::MAX {
        *c += 1;
      } else {
        set_reps.update(|r| *r += 1);
        *c = 0;
      }

      if let Some(el) = divref.get() {
        let r = el.class_list().remove_1("pop-anim");
        let r2 = el.offset_width();
        let r3 = el.class_list().add_1("pop-anim");
      }
    });

    set_phrase.update(|p| {
      if count.get() >= u8::MAX {
        *p = String::new();
      } else {
        let idx = hash.finish() as usize % PUPPY_LANG.len();
        *p = p.to_owned() + " " + PUPPY_LANG[idx];
      }

      hash.write(p.as_bytes());
    });
  };

  view! {
    <div class="widget clicker">
      <div class="counter" node_ref=divref>{count}x</div>
      <div class="headline">woof?</div>
      <button class="btn" on:click=on_click>bark!</button>
      <div class="yap">{phrase}</div>
    </div>
  }
}
