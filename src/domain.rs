#![allow(unused_imports)]

use serde::{ Serialize, Deserialize };

pub type ProjectName = String;
pub type ProjectId = i32;
pub type TaskName = String;
pub type TaskId = i32;

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

    pub struct Task {
        pub id: TaskId,
        pub name: TaskName,
    }

    pub struct Project {
        pub id: ProjectId,
        pub name: ProjectName,
        pub tasks: Vec<Task>,
    }
}

#[derive(Serialize,Deserialize,Debug)]
pub struct AddProjectParams {
    pub name: ProjectName,
}

