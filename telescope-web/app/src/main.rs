#![recursion_limit = "256"]

use anyhow::{Error};
use yew::prelude::*;
use yew::services::fetch::{FetchTask};

mod api;

pub enum Msg {
    RepoList(Result<Vec<model::Repo>, Error>) 
}

#[derive(Debug)]
pub struct Homepage {
    link: ComponentLink<Self>,
    repos: Vec<model::Repo>,
    repos_cb: Callback<Result<Vec<model::Repo>, Error>>,
    repos_task: Option<FetchTask>
}

impl Component for Homepage {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let cb = _link.callback(Msg::RepoList);

        Homepage {
            link: _link,
            repos: Vec::new(),
            repos_cb: cb,
            repos_task: None,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.repos_task = Some(api::get_repos(self.repos_cb.clone()));
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        match _msg {
            Msg::RepoList(Ok(repos)) => {
                self.repos = repos;
            },
            Msg::RepoList(Err(_)) => {}
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <main>
                <h1 style="text-align:center"> {"Telescope 🔭"} </h1>
                <a href="https://github.com/suborbital" style="text-align:left;color:white">{"Suborbital"}</a>
                <ul style="text-align:left">
                    { 
                        self.repos.iter().map(|r| {
                            html! {
                                <li>
                                    {format!("{}: {}", r.name.clone(), r.stargazers_count.clone())}
                                </li>
                            }
                        }).collect::<Html>() 
                    }
                </ul>
            </main>
        }
    }
}

fn main() {
    yew::start_app::<Homepage>();
}