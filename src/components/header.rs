use crate::components::icon::Icon;
use leptos::prelude::*;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};

static SVG_GRADIENTS: [&'static str; 4] = [
  // transfem flag gradient
  "<linearGradient id=\"nixlogo-gradient\" gradientTransform=\"rotate(90)\">
    <stop offset=\"0%\" stop-color=\"#74dfff\" />
    <stop offset=\"15%\" stop-color=\"#74dfff\" />

    <stop offset=\"15%\" stop-color=\"#ffe1ed\" />
    <stop offset=\"30%\" stop-color=\"#ffe1ed\" />

    <stop offset=\"30%\" stop-color=\"#ffb5d8\" />
    <stop offset=\"45%\" stop-color=\"#ffb5d8\" />

    <stop offset=\"45%\" stop-color=\"#fd8dbf\" />
    <stop offset=\"60%\" stop-color=\"#fd8dbf\" />

    <stop offset=\"60%\" stop-color=\"#ffb5d8\" />
    <stop offset=\"75%\" stop-color=\"#ffb5d8\" />

    <stop offset=\"75%\" stop-color=\"#ffe1ed\" />
    <stop offset=\"90%\" stop-color=\"#ffe1ed\" />

    <stop offset=\"90%\" stop-color=\"#74dfff\" />
    <stop offset=\"100%\" stop-color=\"#74dfff\" />
  </linearGradient>",
  // nonbinary flag gradient
  "<linearGradient id=\"nixlogo-gradient\" gradientTransform=\"rotate(90)\">
    <stop offset=\"0%\" stop-color=\"#fcf431\" />
    <stop offset=\"25%\" stop-color=\"#fcf431\" />

    <stop offset=\"25%\" stop-color=\"#fcfcfc\" />
    <stop offset=\"50%\" stop-color=\"#fcfcfc\" />

    <stop offset=\"50%\" stop-color=\"#9d59d2\" />
    <stop offset=\"75%\" stop-color=\"#9d59d2\" />

    <stop offset=\"75%\" stop-color=\"#282828\" />
    <stop offset=\"100%\" stop-color=\"#282828\" />
  </linearGradient>",
  // transgender flag gradient
  "<linearGradient id=\"nixlogo-gradient\" gradientTransform=\"rotate(90)\">
    <stop offset=\"0%\" stop-color=\"#5bcffb\" />
    <stop offset=\"20%\" stop-color=\"#5bcffb\" />

    <stop offset=\"20%\" stop-color=\"#f5abb9\" />
    <stop offset=\"40%\" stop-color=\"#f5abb9\" />

    <stop offset=\"40%\" stop-color=\"#ffffff\" />
    <stop offset=\"60%\" stop-color=\"#ffffff\" />

    <stop offset=\"60%\" stop-color=\"#f5abb9\" />
    <stop offset=\"80%\" stop-color=\"#f5abb9\" />

    <stop offset=\"80%\" stop-color=\"#5bcffb\" />
    <stop offset=\"100%\" stop-color=\"#5bcffb\" />
  </linearGradient>",
  // lesbian flag gradient
  "<linearGradient id=\"nixlogo-gradient\" gradientTransform=\"rotate(90)\">
    <stop offset=\"0%\" stop-color=\"#d52d00\" />
    <stop offset=\"15%\" stop-color=\"#d52d00\" />

    <stop offset=\"15%\" stop-color=\"#ef7627\" />
    <stop offset=\"30%\" stop-color=\"#ef7627\" />

    <stop offset=\"30%\" stop-color=\"#ff9a56\" />
    <stop offset=\"45%\" stop-color=\"#ff9a56\" />

    <stop offset=\"45%\" stop-color=\"#ffffff\" />
    <stop offset=\"60%\" stop-color=\"#ffffff\" />

    <stop offset=\"60%\" stop-color=\"#d162a4\" />
    <stop offset=\"75%\" stop-color=\"#d162a4\" />

    <stop offset=\"75%\" stop-color=\"#b55690\" />
    <stop offset=\"90%\" stop-color=\"#b55690\" />

    <stop offset=\"90%\" stop-color=\"#a30262\" />
    <stop offset=\"100%\" stop-color=\"#a30262\" />
  </linearGradient>",
];

static CSS_GRADIENTS: [&'static str; 4] = [
  "linear-gradient(to right,
      #74dfff 0%,
      #74dfff 15%,
      #ffe1ed 15%,
      #ffe1ed 30%,
      #ffb5d8 30%,
      #ffb5d8 45%,
      #fd8dbf 45%,
      #fd8dbf 60%,
      #ffb5d8 60%,
      #ffb5d8 75%,
      #ffe1ed 75%,
      #ffe1ed 90%,
      #74dfff 90%,
      #74dfff 100%
    )",
  "linear-gradient(to right,
      #fcf431 0%,
      #fcf431 25%,
      #fcfcfc 25%,
      #fcfcfc 50%,
      #9d59d2 50%,
      #9d59d2 75%,
      #282828 75%,
      #282828 100%
    )",
  "linear-gradient(to right,
      #5bcffb 0%,
      #5bcffb 20%,
      #f5abb9 20%,
      #f5abb9 40%,
      #ffffff 40%,
      #ffffff 60%,
      #f5abb9 60%,
      #f5abb9 80%,
      #5bcffb 80%,
      #5bcffb 100%
    )",
  "linear-gradient(to right,
      #d52d00 0%,
      #d52d00 15%,
      #ef7627 15%,
      #ef7627 30%,
      #ff9a56 30%,
      #ff9a56 45%,
      #ffffff 45%,
      #ffffff 60%,
      #d162a4 60%,
      #d162a4 75%,
      #b55690 75%,
      #b55690 90%,
      #a30262 90%,
      #a30262 100%
    )",
];

#[component]
pub fn Header() -> impl IntoView {
  let hash = RandomState::new().build_hasher();
  let idx = hash.finish() as usize % SVG_GRADIENTS.len();

  view! {
    <div class:header>
      <div>
        <svg
          style="width:0;height:0;position:absolute;"
          aria-hidden="true"
          focusable="false"
          inner_html=SVG_GRADIENTS[idx]
          >
        </svg>
        <a href="/"><Icon class="nix-icon" svg=simpleicons_rs::SINIXOS.svg /></a>
        <a href="/"
          style:color="transparent"
          style:background-clip="text"
          style:background-image=CSS_GRADIENTS[idx]
        >oljoi</a>
      </div>
      <div class="links">
        <a href="/">main</a>
        <a href="/contacts">contacts</a>
        <a href="/projects">projects</a>
        <a href="/blog">blog</a>
      </div>
    </div>
  }
}
