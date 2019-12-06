use yew::prelude::*;
use crate::model::*;
use crate::msg::*;
use crate::authservice::*;

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        match &self.auth_state {
            AuthState::Yes(user) => view_auth_yes(self, &user),
            AuthState::No        => view_auth_no(self),
            AuthState::Unknown   => view_auth_no(self),
        }
    }
}

fn view_auth_yes(model: &Model, user: &AuthUser) -> Html<Model> {
    html! {
        <div>
            <div>
                { format!("Logged in as {}", user.name)}
            </div>
            <div>
                { view_nav(model) }
                <button id="btn-logout" onclick=|_| Msg::Logout>{ "Log out" }</button>
            </div>
            { view_scene(model) }
        </div>
    }
}

fn view_auth_no(model: &Model) -> Html<Model> {
    html! {
        <div>
            <div>
                <button id="btn-login" onclick=|_| Msg::Login>{ "Log in" }</button>
            </div>
            <div>
                { "Testing: " }
                { view_nav(model) }
            </div>
        </div>
    }
}

fn view_nav(_model: &Model) -> Html<Model> {
    html! {
        <span>
            <button onclick=|_| Msg::GetProjects>{ "Projects" }</button>
            <button onclick=|_| Msg::GetUsers>{ "Users" }</button>
        </span>
    }
}

fn view_scene(model: &Model) -> Html<Model> {
    match &model.scene {
        Scene::Null           => model.render_template(&NullView),
        Scene::Projects(view) => model.render_template(view),
        Scene::Project(view)  => model.render_template(view),
        Scene::Users(view)    => model.render_template(view),
    }
}

