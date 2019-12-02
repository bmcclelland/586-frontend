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
    AddProject,
    UpdateProjectName(String),
    UpdateProjectId(String),
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

fn remote_host() -> &'static str
{
//    "http://cs586:8001"
    "http://localhost:8001"
}


fn json_request<'a, T>(body: &'a T, auth: &AuthState, action: &str) 
    -> Request<Json<&'a T>> 
    where T: Serialize
{
    Request::post(format!("{}/api/{}", remote_host(), action)) // TODO
        .header("Content-Type", "application/json")
        .add_auth(auth)
        .body(Json(body))
        .expect("Failed to build request")
}

fn get_request(auth: &AuthState, action: &str) 
    -> Request<Nothing> 
{
    Request::get(format!("{}/api/{}", remote_host(), action)) // TODO
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
        Scene::Project(p)  => format!("project/{}", p.id),
        Scene::Users(_)    => "users".into(),
        Scene::Null        => "".into(),
    }
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
            model.loc.init(
                model.link.send_back(|_| Msg::Route)
                );
            model.auth.init(
                model.link.send_back(Msg::AuthReady)
                );
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
                Msg::ChangeScene(Scene::Projects(projects))
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
                    Msg::ChangeScene(Scene::Project(project))
                }
                else {
                    Msg::Noop
                }
            });
        }
        Msg::AddProject => {
            log!("Msg::AddProject");
            let params = AddProjectParams {
                name: model.inputs.project_name.clone(),
            };
            let req = json_request(&params, &model.auth_state, "add_project");
            fetch!(model, req, |_project_id: ProjectId| {
                Msg::GetProjects
            });
            model.inputs.project_name.clear();
        }
        Msg::GetUsers => {
            log!("Msg::GetUsers");
            let req = get_request(&model.auth_state, "get_users");
            fetch!(model, req, |users: Vec<User>| {
                Msg::ChangeScene(
                    Scene::Users(users)
                )
            });
        }
//        Msg::AddWorker => {
//            model.debug.push("AddWorker");
//            let params = AddWorkerParams {
//                name: model.inputs.worker_name.clone(),
//            };
//            let req = json_request(&params, "add_worker");
//            fetch!(model, req, |_worker_id: WorkerID| {
//                Msg::GetWorkers
//            });
//            model.inputs.worker_name.0.clear();
//        }
//        Msg::DeleteProject(project_id) => {
//            model.debug.push("DeleteProject");
//            let params = DeleteProjectParams { project_id };
//            let req = json_request(&params, "delete_project");
//            fetch!(model, req, |_success: bool| {
//                Msg::GetProjects
//            });
//        }
//        Msg::DeleteWorker(worker_id) => {
//            model.debug.push("DeleteWorker");
//            let params = DeleteWorkerParams { worker_id };
//            let req = json_request(&params, "delete_worker");
//            fetch!(model, req, |_success: bool| {
//                Msg::GetWorkers
//            });
//        }
//        Msg::AddTask => {
//            unimplemented!()
//        }
        Msg::UpdateProjectName(x) => {
            model.inputs.project_name = x;
        }
        Msg::UpdateProjectId(x) => {
            model.inputs.project_id = x;
        }
//        Msg::UpdateWorkerInput(s) => {
//            model.debug.push("UpdateWorkerInput");
//            model.inputs.worker_name = s;
//        }
//        Msg::UpdateTaskInput(_,_) => {
//            unimplemented!();
//        }
//        Msg::ViewProjects(projects) => {
//            model.debug.push("ViewProjects");
//            model.view = Scene::Projects(projects);
//            model.task = None;
//        }
//        Msg::ViewWorkers(body) => {
//            model.debug.push("ViewWorkers");
//            model.view = Scene::Workers(body);
//            model.task = None;
//        }
//        Msg::ViewTasks(body) => {
//            model.debug.push("ViewTasks");
//            model.view = Scene::Tasks(body);
//            model.task = None;
//        }
//        Msg::GetProjects => {
//            model.debug.push("GetProjects");
//            let params = GetProjectsParams;
//            let req = json_request(&params, "get_projects");
//            fetch!(model, req, |projects: Vec<Project>| {
//                Msg::ViewProjects(projects)
//            });
//        }
//        Msg::GetWorkers => {
//            model.debug.push("GetWorkers");
//            let params = GetWorkersParams;
//            let req = json_request(&params, "get_workers");
//            fetch!(model, req, |workers: Vec<Worker>| {
//                Msg::ViewWorkers(workers)
//            });
//        }
//        Msg::GetTasks => {
//            model.debug.push("GetTasks");
//            let params = GetTasksParams;
//            let req = json_request(&params, "get_tasks");
//            fetch!(model, req, |tasks: Vec<Task>| {
//                Msg::ViewTasks(tasks)
//            });
//        }
    }
    true
}
