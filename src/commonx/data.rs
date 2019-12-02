use serde::{Serialize,Deserialize};
use failure::{Fail,Error};
use crate::schema::*;
use diesel::prelude::*;

pub trait Inc {
    fn inc(&mut self);
}

#[derive(Serialize,Deserialize,Fail,Debug)]
pub enum AppError {
    #[fail(display = "not found")]
    NotFound,
}

pub type AppResult<T> = Result<T,Error>;

newtype!(WorkerName:  String);
newtype!(TaskName:    String);
newtype!(ProjectName: String);

newtype_id!(WorkerID:    i32);
newtype_id!(TaskID:      i32);
newtype_id!(ProjectID:   i32);

#[derive(Debug,Clone,PartialEq,Queryable,Serialize,Deserialize,Associations,Identifiable, AsChangeset, Insertable)]
pub struct Project {
    pub id:   ProjectID,
    pub name: ProjectName,
}

#[derive(Debug,Clone,PartialEq,Queryable,Serialize,Deserialize,Associations,Identifiable, AsChangeset, Insertable)]
#[belongs_to(Project)]
#[belongs_to(Worker)]
pub struct Task {
    pub id:    TaskID,
    pub name:  TaskName,
    pub project_id: ProjectID,
    pub worker_id:  Option<WorkerID>,
}

#[derive(Debug,Clone,PartialEq,Queryable,Serialize,Deserialize,Associations,Identifiable, AsChangeset, Insertable)]
pub struct Worker {
    pub id:   WorkerID,
    pub name: WorkerName,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct AddProjectParams {
    pub name: ProjectName,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct AddWorkerParams {
    pub name: WorkerName,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct AddTaskParams {
    pub name: TaskName,
    pub project_id: ProjectID,
    pub worker_id: Option<WorkerID>,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct GetTasksParams;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct GetProjectsParams;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct GetWorkersParams;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct DeleteProjectParams {
    pub project_id: ProjectID,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct DeleteWorkerParams {
    pub worker_id: WorkerID,
}
