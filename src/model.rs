use yew::prelude::*;
use yew::services::fetch::*;
use yew::services::storage::*;
use crate::authservice::*;
use crate::locservice::*;
use crate::domain::*;

extern crate askama;
use askama::Template;
use stdweb::web::Node;
use yew::virtual_dom::VNode;

#[derive(Template)]
#[template(path = "null.html")]
pub struct NullView;

#[derive(Template)]
#[template(path = "projects.html")]
pub struct ProjectsView {
    pub projects: Vec<ListProject>,
}

#[derive(Template)]
#[template(path = "workers.html")]
pub struct WorkersView {
    pub workers: Vec<ListWorker>,
}

#[derive(Template)]
#[template(path = "users.html")]
pub struct UsersView {
    pub users: Vec<User>,
}

#[derive(Template)]
#[template(path = "project_details.html")]
pub struct ProjectDetailsView {
    pub project: ProjectDetails,
}

#[derive(Template)]
#[template(path = "worker_details.html")]
pub struct WorkerDetailsView {
    pub worker: WorkerDetails,
}

#[derive(Template)]
#[template(path = "task_details.html")]
pub struct TaskDetailsView {
    pub task: TaskDetails,
}

pub enum Scene {
    Null,
    Projects(ProjectsView),
    Workers(WorkersView),
    Users(UsersView),
    ProjectDetails(ProjectDetailsView),
    WorkerDetails(WorkerDetailsView),
    TaskDetails(TaskDetailsView),
}

pub struct Model {
    pub auth: AuthService,
    pub loc: LocService,
    pub fetcher: FetchService,
    pub storage: StorageService,
    pub link: ComponentLink<Model>,
    pub task: Option<FetchTask>,
    pub scene: Scene,
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
        }
    }

    pub fn render_template(&self, template: &impl Template) -> Html<Self> {
        VNode::VRef(
            Node::from_html(
                &template.render().unwrap()
                ).unwrap()
            )
    }
}
