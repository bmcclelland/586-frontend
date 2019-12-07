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
    Null,
    Error(String),
    Init,
    AuthReady(AuthState),
    Route,
    Login,
    Logout,
    ChangeScene(Scene),
    RefreshScene,
    GetProjects,
    GetUsers,
    GetWorkers,
    GetProject(ProjectId),
    GetWorker(WorkerId),
    GetTask(TaskId),
    AddProject(ProjectName),
    AddTask((TaskName,WorkerId)),
    AddWorker(WorkerName),
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
        []             => Msg::Null,
        ["projects"]   => Msg::GetProjects,
        ["workers"]    => Msg::GetWorkers,
        ["users"]      => Msg::GetUsers,
        ["project", n] => Msg::GetProject(parse_with_default(n, 0)),
        ["worker", n]  => Msg::GetWorker(parse_with_default(n, 0)),
        ["task", n]    => Msg::GetTask(parse_with_default(n, 0)),
        _              => Msg::Null,
    }
}

fn hash_from_scene(scene: &Scene) -> String {
    match scene {
        Scene::Null          => "".into(),
        Scene::Projects(_)   => "projects".into(),
        Scene::Workers(_)    => "workers".into(),
        Scene::Users(_)      => "users".into(),
        Scene::ProjectDetails(view) => 
            format!("project/{}", view.project.id),
        Scene::WorkerDetails(view) => 
            format!("worker/{}", view.worker.id),
        Scene::TaskDetails(view) => 
            format!("task/{}", view.task.id),
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

fn register_add_worker_js(model: &mut Model) {
    let cb = model.link.send_back(Msg::AddWorker);
    let cb = move |x: WorkerName| cb.emit(x); 
    
    js!(
        add_worker = function(x) {
            console.log("add_worker(" + x + ")");
            document.getElementById("add_worker_name").value = "";
            @{cb}(x);
        };
    );
}

fn register_add_task_js(model: &mut Model) {
    let cb = model.link.send_back(Msg::AddTask);
    let cb = move |x: TaskName, y: ProjectId| cb.emit((x,y)); 
    
    js!(
        add_task = function(x,y) {
            console.log("add_task(" + x + "," + y + ")");
            document.getElementById("add_task_name").value = "";
            @{cb}(x,y);
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

fn register_get_worker_js(model: &mut Model) {
    let cb = model.link.send_back(Msg::GetWorker);
    let cb = move |x: WorkerId| cb.emit(x); 
    
    js!(
        get_worker = function(x) {
            console.log("get_worker(" + x + ")");
            @{cb}(x);
        };
    );
}

fn register_get_task_js(model: &mut Model) {
    let cb = model.link.send_back(Msg::GetTask);
    let cb = move |x: TaskId| cb.emit(x); 
    
    js!(
        get_task = function(x) {
            console.log("get_task(" + x + ")");
            @{cb}(x);
        };
    );
}
            
fn register_msg_js(model: &mut Model) {
    register_add_project_js(model);
    register_add_worker_js(model);
    register_add_task_js(model);
    register_get_project_js(model);
    register_get_worker_js(model);
    register_get_task_js(model);
}

pub fn update(model: &mut Model, msg: Msg) -> ShouldRender {
    match msg {
        Msg::Null => {
            log!("Msg::Null");
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
        Msg::ChangeScene(scene) => {
            log!("Msg::ChangeScene");
            model.loc.set_hash_path(hash_from_scene(&scene));
            model.scene = scene;
        }
        Msg::RefreshScene => {
            let msg = match &model.scene {
                Scene::Null 
                    => Msg::Null,
                Scene::Projects(_) 
                    => Msg::GetProjects,
                Scene::Workers(_)
                    => Msg::GetWorkers,
                Scene::Users(_)
                    => Msg::GetUsers,
                Scene::ProjectDetails(view) 
                    => Msg::GetProject(view.project.id),
                Scene::WorkerDetails(view) 
                    => Msg::GetWorker(view.worker.id),
                Scene::TaskDetails(view) 
                    => Msg::GetTask(view.task.id),
            };

            model.link.send_self(msg);
        }
        Msg::GetProjects => {
            log!("Msg::GetProjects");
            let req = get_request(&model.auth_state, "get_projects");
            fetch!(model, req, |projects: Vec<ListProject>| {
                Msg::ChangeScene(
                    Scene::Projects(
                        ProjectsView{projects}
                    )
                )
            });
        }
        Msg::GetWorkers => {
            log!("Msg::GetWorkers");
            let req = get_request(&model.auth_state, "get_workers");
            fetch!(model, req, |workers: Vec<ListWorker>| {
                Msg::ChangeScene(
                    Scene::Workers(
                        WorkersView{workers}
                    )
                )
            });
        }
        Msg::GetProject(id) => {
            log!("Msg::GetProject({:?})", id);
            let req = get_request(&model.auth_state, &format!("get_project/{}", id));
            fetch!(model, req, |project: Option<ProjectDetails>| {
                if let Some(project) = project {
                    Msg::ChangeScene(
                        Scene::ProjectDetails(
                            ProjectDetailsView{project}
                        )
                    )
                }
                else {
                    Msg::Null
                }
            });
        }
        Msg::GetWorker(id) => {
            log!("Msg::GetWorker({:?})", id);
            let req = get_request(&model.auth_state, &format!("get_worker/{}", id));
            fetch!(model, req, |worker: Option<WorkerDetails>| {
                if let Some(worker) = worker {
                    Msg::ChangeScene(
                        Scene::WorkerDetails(
                            WorkerDetailsView{worker}
                        )
                    )
                }
                else {
                    Msg::Null
                }
            });
        }
        Msg::GetTask(id) => {
            log!("Msg::GetTask({:?})", id);
            let req = get_request(&model.auth_state, &format!("get_task/{}", id));
            fetch!(model, req, |task: Option<TaskDetails>| {
                if let Some(task) = task {
                    Msg::ChangeScene(
                        Scene::TaskDetails(
                            TaskDetailsView{task}
                        )
                    )
                }
                else {
                    Msg::Null
                }
            });
        }
        Msg::AddProject(name) => {
            log!("Msg::AddProject");
            let params = AddProjectParams { name };
            let req = json_request(&params, &model.auth_state, "add_project");
            fetch!(model, req, |_: ProjectId| {
                Msg::RefreshScene
            });
        }
        Msg::AddWorker(name) => {
            log!("Msg::AddWorker");
            let params = AddWorkerParams { name };
            let req = json_request(&params, &model.auth_state, "add_worker");
            fetch!(model, req, |_: WorkerId| {
                Msg::RefreshScene
            });
        }
        Msg::AddTask((name,project_id)) => {
            log!("Msg::AddTask");
            let params = AddTaskParams { name, project_id };
            let req = json_request(&params, &model.auth_state, "add_task");
            fetch!(model, req, |_: TaskId| {
                Msg::RefreshScene
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
