#![allow(unused_imports)]

use serde::{ Serialize, Deserialize };

pub type ProjectName = String;
pub type ProjectId = i32;

//#[derive(Serialize,Deserialize,Debug)]
//pub enum Perm {
//};

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Role {
    pub id: String,
    pub perms: Vec<i32>,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct User {
    pub id: String,
    pub roles: Vec<Role>,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Project {
    pub id: ProjectId,
    pub name: ProjectName,
    pub tasks: Vec<()>,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct AddProjectParams {
    pub name: ProjectName,
}

