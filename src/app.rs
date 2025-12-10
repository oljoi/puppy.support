use leptos::html::Div;
use leptos::prelude::*;
use leptos_meta::{Link, MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
  StaticSegment,
  components::{Route, Router, Routes},
};
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};

use crate::components::{footer::Footer, header::Header, icon::Icon, webring::Webring};

pub fn shell(options: LeptosOptions) -> impl IntoView {
  view! {
    <!DOCTYPE html>
    <html lang="en">
        <head>
            <meta charset="utf-8"/>
            <meta name="viewport" content="width=device-width, initial-scale=1"/>
            <AutoReload options=options.clone() />
            <HydrationScripts options islands=true/>
            <MetaTags/>
        </head>
        <body>
            <App/>
        </body>
    </html>
  }
}

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();

  view! {
    <Stylesheet id="leptos" href="/pkg/puppy-support.css"/>

    <Title text="oljoi"/>

    <Link rel="icon" href="blahaj.gif" type_="image/gif" />

    <Router>
      <Header />

      <div class="main">
        <Widgets>
          <div class="widget wip">
            work in progress
          </div>

         <Clicker />
        </Widgets>

        <main class="content">
        <Routes fallback=PageNotFound>
            <Route path=StaticSegment("") view=HomePage/>
            <Route path=StaticSegment("/about") view=NotHomePage/>
          </Routes>
        </main>
      </div>

      <Footer />

    </Router>
  }
}

#[component]
fn PageNotFound() -> impl IntoView {
  view! {
    <div class="file notfound">
      <img src="sad-puppy.webp" class="notfound-image"/>
      <p>Page not found ":("</p>
    </div>
  }
}

static PUPPY_LANG: [&'static str; 8] = ["woof", "bark", "wif", "awo", "waf", "rawr", "ow", "grr"];
#[island]
fn Clicker() -> impl IntoView {
  let mut hash = RandomState::new().build_hasher();

  let phrase = RwSignal::new(String::from(""));
  let count = RwSignal::new(1u8);
  let on_click = move |_| {
    count.update(|c| {
      if *c < (u8::MAX - 1) {
        *c += 1;
      } else {
        *c = 0;
      }
    });
    phrase.update(|p| {
      let idx = hash.finish() as usize % PUPPY_LANG.len();
      *p = p.to_owned() + " " + PUPPY_LANG[idx];
      hash.write(p.as_bytes());
    });
  };

  let divref = NodeRef::<Div>::new();

  Effect::new(move |_| {
    let count = count.get();

    if let Some(el) = divref.get() {
      let r = el.class_list().remove_1("pop-anim");
      let r2 = el.offset_width();
      let r3 = el.class_list().add_1("pop-anim");
    }
  });

  view! {
    <div class="widget clicker">
      <div class="counter" node_ref=divref>{count}x</div>
      <div class="headline">"woof?"</div>
      <button on:click=on_click>"Bark!"</button>
      <div class="yap">{phrase}</div>
    </div>
  }
}

#[component]
fn Widgets(children: Children) -> impl IntoView {
  view! {
    <div class="widgets">
      { children() }
    </div>
  }
}

#[component]
fn HomePage() -> impl IntoView {
  /*let count = RwSignal::new(1);
  let on_click = move |_| {
    count.update(|c| {
      *c += 1;
    });
  };
  let barks = move || "bark ".repeat(count.get().try_into().unwrap());*/

  view! {
    <div class="file">
      pipi pupu<br />
      1<br />
      2<br />
      3<br />
      4<br />
    </div>
    <div class="file">
      chat
    </div>
    //<h1>"woof :3"</h1>
    //<button on:click=on_click>"Bark!"</button>
    //<br />
    //<span>{barks}</span>
  }
}

#[component]
fn NotHomePage() -> impl IntoView {
  view! {
    <div class="file">
      puki kaki
      2
      3
      4
    </div>
  }
}
