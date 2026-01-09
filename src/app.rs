use leptos::html::Div;
use leptos::prelude::*;
use leptos_meta::{Link, Meta, MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
  StaticSegment,
  components::{Route, Router, Routes},
};
use std::collections::{BTreeMap, HashMap, hash_map::RandomState};
use std::hash::{BuildHasher, Hasher};

use crate::components::{
  clicker::Clicker, footer::Footer, header::Header, icon::Icon, webring::Webring,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
  view! {
    <!DOCTYPE html>
    <html lang="en">
        <head>
            <meta charset="utf-8"/>
            <meta name="viewport" content="width=device-width, initial-scale=1"/>
            <AutoReload options=options.clone() />
            <HydrationScripts options islands=true/>
            <MetaTags />
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

  let mut frens: BTreeMap<&'static str, &'static str> = BTreeMap::new();
  frens.insert("https://nikableh.moe/88x31.png", "https://nikableh.moe/");

  view! {
    <Stylesheet id="leptos" href="/pkg/puppy-support.css"/>

    <Title text="oljoi"/>

    <Link rel="icon" href="blahaj.gif" type_="image/gif" />

    <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

    <Router>
      <Header />

      <div class="main">
        <Widgets>
          <div class="widget wip">
            work in progress<br />
            im kinda still developing my site so content is from my old aboutme, sry
            "><"
          </div>

         <Clicker />
         <X8831 list=frens/>
        </Widgets>

        <main class="content">
        <Routes fallback=PageNotFound>
            <Route path=StaticSegment("") view=HomePage/>
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
      <img src="sad-puppy.webp" class="notfound-image" loading="lazy" alt="sad puppy"/>
      <p>Page not found ":("</p>
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
fn X8831(list: BTreeMap<&'static str, &'static str>) -> impl IntoView {
  view! {
    <div class="widget buttons">
      <div class="title">frens "<3"</div>
      <div class="content">
        {list.into_iter()
          .map(|(img, link)| view! {
            <div class="x8831">
            <a href={link}><img src={img} loading="lazy" alt={link} /></a>
            </div>
          })
          .collect_view()}
      </div>
      <div class="title">mine</div>
      <div class="content">
        <a href="https://puppy.support/88x31/shutup.gif">
          <img src="https://puppy.support/88x31/shutup.gif" loading="lazy"/>
        </a>
      </div>
    </div>
  }
}

#[component]
fn HomePage() -> impl IntoView {
  view! {
    <div class="info file">
      <p>haiii :3</p>
      <p>i am <b>oljoi</b> (she/them)<br />
      stupid silly puppy and sometimes "embedded/system" engineer</p>
      <p>
      <b>some info about me:</b><br />
      "i mainly like tinkering with microcontrollers and microprocessors, robots, (micro/nano)electronics and other hardware things ><"<br />
      <br />
      "also i enjoy physics, electrical engineering, operating systems, networking, homelabing"<br />
      "and kissing girls,,,"</p>
      <p>"i code mainly in rust but also in c, c++, c#, x86/riscv asm, lua, java, kotlin, ruby, python, javascript and typescript"<br />
      <br />
      "and talk in russian (native), english (c1). learning: suomi(a1), karelian"</p>
      <b>contact me:</b>
      <table class="contacts">
        <tr>
          <td>tg channel</td>
          <td><a href="https://t.me/bottompuppydisorder">"@bottompuppydisorder"</a></td>
        </tr>
        <tr>
          <td>fedi</td>
          <td><a href="https://blahaj.love/@oljoi">"@oljoi@blahaj.love"</a></td>
        </tr>
        <tr>
          <td>message me at </td>
          <td><a href="https://t.me/olj0i">"@olj0i"</a></td>
        </tr>
      </table>
    </div>
  }
}
