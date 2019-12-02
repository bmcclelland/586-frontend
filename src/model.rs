use yew::prelude::*;
use yew::services::fetch::*;
use yew::services::storage::*;
use crate::authservice::*;
use crate::locservice::*;
use crate::domain::*;

pub enum Scene {
    Null,
    Projects(Vec<Project>),
    Project(Project),
    Users(Vec<User>),
//    Workers(Vec<Worker>),
//    Tasks(Vec<Task>),
}

pub struct Inputs {
    pub project_name: String,
    pub project_id:   String,
//    pub worker_name:  WorkerName,
//    pub task_name:    TaskName,
}

pub struct Model {
    pub auth: AuthService,
    pub loc: LocService,
    pub fetcher: FetchService,
    pub storage: StorageService,
    pub link: ComponentLink<Model>,
    pub task: Option<FetchTask>,
    pub scene: Scene,
    pub inputs: Inputs,
    pub auth_state: AuthState,
}

impl Model {
    pub fn new(link: ComponentLink<Self>) -> Self {
        Self {
            auth: AuthService::new(),
            loc: LocService::new(),
            fetcher: FetchService::new(),
            storage: StorageService::new(Area::Local),
            link,
            auth_state: AuthState::Unknown,
            task: None,
            scene: Scene::Null,
            inputs: Inputs {
                project_name: "".into(),
                project_id:   "".into(),
//                worker_name:  WorkerName("".into()),
//                task_name:    TaskName("".into()),
            },
        }
    }
}
