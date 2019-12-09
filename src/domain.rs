#![allow(unused_imports)]

use serde::{ Serialize, Deserialize };

pub type ProjectName = String;
pub type ProjectId = i32;
pub type TaskName = String;
pub type TaskId = i32;
pub type WorkerName = String;
pub type WorkerId = i32;

//#[derive(Serialize,Deserialize,Debug)]
//pub enum Perm {
//};

macro_rules! domain_derive {
    ($($i: item)+) => {
        $(#[derive(Serialize,Deserialize,Debug,Clone)]
        $i)+
    }
}

domain_derive! {
    pub struct Role {
        pub id: String,
        pub perms: Vec<i32>,
    }
    
    pub struct User {
        pub id: String,
        pub roles: Vec<Role>,
    }

    pub struct ListTask {
        pub id: TaskId,
        pub name: TaskName,
    }

    pub struct ListProject {
        pub id: ProjectId,
        pub name: ProjectName,
    }
    
    pub struct ListWorker {
        pub id: WorkerId,
        pub name: WorkerName,
    }

    pub struct ProjectDetails {
        pub id: ProjectId,
        pub name: ProjectName,
        pub tasks: Vec<ListTask>,
    }
    
    pub struct WorkerDetails {
        pub id: WorkerId,
        pub name: WorkerName,
        pub tasks: Vec<ListTask>,
    }
    
    pub struct TaskDetails {
        pub id: TaskId,
        pub name: TaskName,
        pub project: ListProject,
        pub worker: Option<ListWorker>,
    }
    
    pub struct AddProjectParams {
        pub name: ProjectName,
    }
    
    pub struct AddWorkerParams {
        pub name: WorkerName,
    }
    
    pub struct AddTaskParams {
        pub name: TaskName,
        pub project_id: ProjectId,
    }
    
    pub struct AssignTaskParams {
        pub task_id: TaskId,
        pub worker_id: WorkerId,
    }
    
    pub struct UnassignTaskParams {
        pub task_id: TaskId,
    }
}
