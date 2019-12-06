#![allow(unused_imports)]
#![allow(unused_macros)]

use yew::prelude::*;
use yew::services::fetch::*;
use yew::format::Json;
use yew::format::nothing::*;
use failure::Error;
use serde::{Serialize};
use crate::model::*;
use crate::domain::*;
use crate::authservice::*;
use stdweb::js;
use stdweb::unstable::TryInto;

pub enum Msg {
    Noop,
    Error(String),
    Init,
    AuthReady(AuthState),
    Route,
    Login,
    Logout,
    GetProjects,
    GetUsers,
    ChangeScene(Scene),
    GetProject(ProjectId),
    AddProject(ProjectName),
}
 
pub fn parse_with_default<T>(s: &str, d: T)
    -> T
    where T: std::str::FromStr
{
    match s.parse::<T>() {
        Ok(t) => t,
        Err(_) => d,
    }
}

macro_rules! fetch(
    ($self: ident, $req: ident, $do: expr) => {
        let callback = $self.link.send_back(
            |rsp: Response<Json<Result<Result<_,String>,Error>>>| {
                let (meta, Json(body)) = rsp.into_parts();
                if meta.status.is_success() {
                    match body {
                        Ok(Ok(s))  => {
                            $do(s)
                        }
                        Ok(Err(s)) => {
                            Msg::Error(s)
                        }
                        Err(_) => { 
                            Msg::Error("JSON error?".into())
                        }
                    }
                }
                else {
                    Msg::Error("Not 200".into())
                }
            }
        );

        $self.task = Some($self.fetcher.fetch(
            $req,
            callback,
        ));
    }
);
//Authorization: Bearer eyJhbGciOiJIUzI1NiIXVCJ9...TJVA95OrM7E20RMHrHDcEfxjoYZgeFONFh7HgQ

trait Authable
{
    fn add_auth(&mut self, auth: &AuthState) -> &mut Self;
}

impl Authable for http::request::Builder
{
    fn add_auth(&mut self, auth: &AuthState) -> &mut Self
    {
        match auth {
            AuthState::Yes(user) => {
                self.header("Authorization", format!("Bearer {}", user.token))
            }
            _ => {
                self
            }
        }
    }
}

fn remote_host() -> String
{
    js!( return api_url; )
    .try_into()
    .unwrap()
}


fn json_request<'a, T>(body: &'a T, auth: &AuthState, action: &str) 
    -> Request<Json<&'a T>> 
    where T: Serialize
{
    Request::post(format!("{}/api/{}", remote_host(), action))
        .header("Content-Type", "application/json")
        .add_auth(auth)
        .body(Json(body))
        .expect("Failed to build request")
}

fn get_request(auth: &AuthState, action: &str) 
    -> Request<Nothing> 
{
    Request::get(format!("{}/api/{}", remote_host(), action))
        .add_auth(auth)
        .body(Nothing)
        .unwrap()
}

macro_rules! log(
    ($s: literal $(,$x: expr)*) => { 
        let msg = format!($s $(, $x)*);
        js!{ console.log(@{msg}); }; 
    };
);

fn route(path: Vec<String>) -> Msg {
    let slices : Vec<&str> = path.iter().map(|s| s.as_ref()).collect();
    match slices.as_slice() {
        []             => Msg::Noop,
        ["projects"]   => Msg::GetProjects,
        ["users"]      => Msg::GetUsers,
        ["project", n] => Msg::GetProject(parse_with_default(n, 0)),
        _              => Msg::Noop,
    }
}

fn hash_from_scene(scene: &Scene) -> String {
    match scene {
        Scene::Projects(_) => "projects".into(),
        Scene::Project(view) => format!("project/{}", view.project.id),
        Scene::Users(_)    => "users".into(),
        Scene::Null        => "".into(),
    }
}
    
fn register_add_project_js(model: &mut Model) {
    let cb = model.link.send_back(Msg::AddProject);
    let cb = move |x: ProjectName| cb.emit(x); 
    
    js!(
        add_project = function(x) {
            console.log("add_project(" + x + ")");
            document.getElementById("add_project_name").value = "";
            @{cb}(x);
        };
    );
}

fn register_get_project_js(model: &mut Model) {
    let cb = model.link.send_back(Msg::GetProject);
    let cb = move |x: ProjectId| cb.emit(x); 
    
    js!(
        get_project = function(x) {
            console.log("get_project(" + x + ")");
            @{cb}(x);
        };
    );
}
            
fn register_msg_js(model: &mut Model) {
    register_add_project_js(model);
    register_get_project_js(model);
}

pub fn update(model: &mut Model, msg: Msg) -> ShouldRender {
    match msg {
        Msg::Noop => {
            log!("Msg::Noop");
            // Do absolutely nothing.
        }
        Msg::Error(s) => {
            log!("Msg::Error({})", s);
            model.task = None;
        }
        Msg::Init => {
            log!("Msg::Init");
            register_msg_js(model);
            model.loc.init(model.link.send_back(|_| Msg::Route));
            model.auth.init(model.link.send_back(Msg::AuthReady));
        }
        Msg::Login => {
            log!("Msg::Login");
            model.auth.login();
        }
        Msg::Logout => {
            log!("Msg::Logout");
            model.auth.logout();
        }
        Msg::Route => {
            log!("Msg::Route");
            model.link.send_self(
                route(model.loc.get_hash_path())
                );
        }
        Msg::AuthReady(auth_state) => {
            log!("Msg::AuthReady");
            model.auth_state = auth_state;
            model.link.send_self(Msg::Route);
        }
        Msg::GetProjects => {
            log!("Msg::GetProjects");
            let req = get_request(&model.auth_state, "get_projects");
            fetch!(model, req, |projects: Vec<Project>| {
                Msg::ChangeScene(
                    Scene::Projects(
                        ProjectsView{projects}
                    )
                )
            });
        }
        Msg::ChangeScene(scene) => {
            log!("Msg::ChangeScene");
            model.loc.set_hash_path(hash_from_scene(&scene));
            model.scene = scene;
        }
        Msg::GetProject(id) => {
            log!("Msg::GetProject({:?})", id);
            let req = get_request(&model.auth_state, &format!("get_project/{}", id));
            fetch!(model, req, |project: Option<Project>| {
                if let Some(project) = project {
                    Msg::ChangeScene(
                        Scene::Project(
                            ProjectView{project}
                        )
                    )
                }
                else {
                    Msg::Noop
                }
            });
        }
        Msg::AddProject(name) => {
            log!("Msg::AddProject");
            let params = AddProjectParams {
                name: name
            };
            let req = json_request(&params, &model.auth_state, "add_project");
            fetch!(model, req, |_project_id: ProjectId| {
                Msg::GetProjects
            });
        }
        Msg::GetUsers => {
            log!("Msg::GetUsers");
            let req = get_request(&model.auth_state, "get_users");
            fetch!(model, req, |users: Vec<User>| {
                Msg::ChangeScene(
                    Scene::Users(
                        UsersView{users}
                    )
                )
            });
        }
    }
    true
}
