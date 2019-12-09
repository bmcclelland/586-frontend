use crate::domain::*;
use askama::Template;

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
#[template(path = "assign_task.html")]
pub struct AssignTaskView {
    pub task: TaskDetails,
    pub workers: Vec<ListWorker>,
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
    AssignTask(AssignTaskView),
}

impl Scene {
    pub fn hash_path(&self) -> String {
        match self {
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
            Scene::AssignTask(view) =>
                format!("assign/{}", view.task.id),
        }
    }
}
    
