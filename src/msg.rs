#![allow(unused_imports)]
#![allow(unused_macros)]

use yew::prelude::*;
use yew::services::fetch::*;
use yew::format::Json;
use yew::format::nothing::*;
use failure::Error;
use serde::{Serialize};
use crate::model::*;
use crate::views::*;
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
    AddTask((TaskName,ProjectId)),
    AddWorker(WorkerName),
    PreViewAssignTask(TaskId),
    ViewAssignTask(TaskDetails),
    PostViewAssignTask(Vec<ListWorker>),
    AssignTask((TaskId,WorkerId)),
    UnassignTask(TaskId),
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
            move |rsp: Response<Json<Result<Result<_,String>,Error>>>| {
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

trait Requester {
    fn get(&self, action: &str) -> Request<Nothing>;
    fn post<'a, T: Serialize>(&self, action: &str, body: &'a T) 
        -> Request<Json<&'a T>>;
}

impl Requester for Model {
    fn get(&self, action: &str) -> Request<Nothing> {
        Request::get(format!("{}/api/{}", remote_host(), action))
            .add_auth(&self.auth_state)
            .body(Nothing)
            .unwrap()
    }

    fn post<'a, T: Serialize>(&self, action: &str, body: &'a T) 
        -> Request<Json<&'a T>> 
    {
        Request::post(format!("{}/api/{}", remote_host(), action))
            .header("Content-Type", "application/json")
            .add_auth(&self.auth_state)
            .body(Json(body))
            .expect("Failed to build request")
    }
}

fn remote_host() -> String
{
    js!( return api_url; )
    .try_into()
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
        ["assign", n]  => Msg::PreViewAssignTask(parse_with_default(n, 0)),
        _              => Msg::Null,
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

fn register_view_assign_task_js(model: &mut Model) {
    let cb = model.link.send_back(Msg::PreViewAssignTask);
    let cb = move |x: TaskId| cb.emit(x); 
    
    js!(
        view_assign_task = function(x) {
            console.log("view_assign_task(" + x + ")");
            @{cb}(x);
        };
    );
}

fn register_assign_task_js(model: &mut Model) {
    let cb = model.link.send_back(Msg::AssignTask);
    let cb = move |x: TaskId, y: WorkerId| cb.emit((x,y)); 
    
    js!(
        assign_task = function(x,y) {
            console.log("assign_task(" + x + "," + y + ")");
            @{cb}(x,y);
        };
    );
}

fn register_unassign_task_js(model: &mut Model) {
    let cb = model.link.send_back(Msg::UnassignTask);
    let cb = move |x: TaskId| cb.emit(x); 
    
    js!(
        unassign_task = function(x) {
            console.log("unassign_task(" + x + ")");
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
    register_view_assign_task_js(model);
    register_assign_task_js(model);
    register_unassign_task_js(model);
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
            model.loc.set_hash_path(scene.hash_path());
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
                Scene::AssignTask(view) 
                    => Msg::ViewAssignTask(view.task.clone()),
            };

            model.link.send_self(msg);
        }
        Msg::GetProjects => {
            log!("Msg::GetProjects");
            let req = model.get("get_projects");
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
            let req = model.get("get_workers");
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
            let req = model.get(&format!("get_project/{}", id));
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
            let req = model.get(&format!("get_worker/{}", id));
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
            let req = model.get(&format!("get_task/{}", id));
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
        Msg::PreViewAssignTask(id) => {
            log!("Msg::PreViewAssignTask({})", id);
            let req = model.get(&format!("get_task/{}", id));
            fetch!(model, req, |task: Option<TaskDetails>| {
                if let Some(task) = task {
                    Msg::ViewAssignTask(task)
                }
                else {
                    Msg::Null
                }
            });
        }
        Msg::ViewAssignTask(task) => {
            log!("Msg::ViewAssignTask({})", task.id);
            model.temp.task = Some(task);
            let req = model.get("get_workers");
            fetch!(model, req, move |workers: Vec<ListWorker>| {
                Msg::PostViewAssignTask(workers)
            });
        }
        Msg::PostViewAssignTask(workers) => {
            log!("Msg::PostViewAssignTask()");
            match model.temp.task.take() {
                None => { () }
                Some(task) => {
                    model.link.send_self(
                        Msg::ChangeScene(
                            Scene::AssignTask(
                                AssignTaskView{ task, workers }
                            )
                        )
                    );
                }
            }
        }
        Msg::AssignTask((task_id, worker_id)) => {
            log!("Msg::AssignTask({},{})", task_id, worker_id);
            let params = AssignTaskParams { task_id, worker_id };
            let req = model.post("assign_task", &params);
            fetch!(model, req, move |_: ()| {
                Msg::GetTask(task_id)
            });
        }
        Msg::UnassignTask(task_id) => {
            log!("Msg::UnassignTask({})", task_id);
            let params = UnassignTaskParams { task_id };
            let req = model.post("unassign_task", &params);
            fetch!(model, req, move |_: ()| {
                Msg::GetTask(task_id)
            });
        }
        Msg::AddProject(name) => {
            log!("Msg::AddProject");
            let params = AddProjectParams { name };
            let req = model.post("add_project", &params);
            fetch!(model, req, |_: ProjectId| {
                Msg::RefreshScene
            });
        }
        Msg::AddWorker(name) => {
            log!("Msg::AddWorker");
            let params = AddWorkerParams { name };
            let req = model.post("add_worker", &params);
            fetch!(model, req, |_: WorkerId| {
                Msg::RefreshScene
            });
        }
        Msg::AddTask((name,project_id)) => {
            log!("Msg::AddTask");
            let params = AddTaskParams { name, project_id };
            let req = model.post("add_task", &params);
            fetch!(model, req, |_: TaskId| {
                Msg::RefreshScene
            });
        }
        Msg::GetUsers => {
            log!("Msg::GetUsers");
            let req = model.get("get_users");
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
