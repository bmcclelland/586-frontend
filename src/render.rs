use yew::prelude::*;
use crate::model::*;
use crate::domain::*;
//use stdweb::{js, Value};
use crate::msg::*;
use crate::authservice::*;

//fn view_buttons(model: &Model) -> Html<Model> {
//    html! {
//        <div>
//            <div>
//                <input
//                    placeholder="Project Name"
//                    value=&model.inputs.project_name.0
//                    oninput=|e| Msg::UpdateProjectInput(ProjectName(e.value))
//                />
//                <button onclick=|_| Msg::AddProject>{ "Add Project" }</button>
//            </div>
//            <div>
//                <input
//                    placeholder="Worker Name"
//                    value=&model.inputs.worker_name.0
//                    oninput=|e| Msg::UpdateWorkerInput(WorkerName(e.value))
//                />
//                <button onclick=|_| Msg::AddWorker>{ "Add Worker" }</button>
//            </div>
//            <button onclick=|_| Msg::GetProjects>{ "Get Projects" }</button>
//            <button onclick=|_| Msg::GetWorkers>{ "Get Workers" }</button>
//            <button onclick=|_| Msg::GetTasks>{ "Get Tasks" }</button>
//        </div>
//    }
//}
       
fn view_nav(_model: &Model) -> Html<Model> {
    html! {
        <span>
            <button onclick=|_| Msg::GetProjects>{ "Projects" }</button>
            <button onclick=|_| Msg::GetUsers>{ "Users" }</button>
        </span>
    }
}

fn view_view(model: &Model) -> Html<Model> {
    match &model.scene {
        Scene::Null=> html! {
            <div>
                <p>{ "(No view)" }</p>
            </div>
        },
        Scene::Projects(projects) => html! {
            <div>
                <div>
                    <input
                        placeholder="Project Name"
                        value=&model.inputs.project_name
                        oninput=|e| Msg::UpdateProjectName(e.value)
                    />
                    <button onclick=|_| Msg::AddProject>{ "Add Project" }</button>
                </div>
                <p>{"PROJECTS: "}</p>
                <ul>{ for projects.iter().map(view_list_project) }</ul>
            </div>
        },
        Scene::Project(project) => html! {
            <div>
                { view_project(project) }
            </div>
        },
        Scene::Users(users) => html! {
            <div>
                <p>{"USERS: "}</p>
                <ul>{ for users.iter().map(view_user) }</ul>
            </div>
        },
    }
}

fn view_project(project: &Project) -> Html<Model> {
    html! {
        <div>
            <h2>{ &project.name }</h2>
            { format!("ID: {}", project.id) }
            <br />
            { format!("Tasks: {:?}", project.tasks) }
        </div>
    }
}

fn view_list_project(project: &Project) -> Html<Model> {
    let project_id = project.id;
    html! {
        <li>
            <a href="javascript:void(0)" onclick=|_|Msg::GetProject(project_id)>
                { &project.name }
            </a>
        </li>
    }
}

fn view_user(user: &User) -> Html<Model> {
    let user_roles = format!("{:?}", user.roles);
    html! {
        <li>
            <span>{ &user.id }{": "}</span> 
            <span>{ user_roles }</span>
        </li>
    }
}

//fn view_worker(worker: &Worker) -> Html<Model> {
//    let worker_name = worker.name.clone();
//    let worker_id = worker.id;
//
//    html! {
//        <li>
//            <span>{ worker_id.0 }{": "}</span> 
//            <span>{ worker_name.0 }</span>
//            <button onclick=|_| Msg::DeleteWorker(worker_id)>{ "Delete" }</button>
//        </li>
//    }
//}

//fn view_task(task: &Task) -> Html<Model> {
//    html! {
//        <li>
//            <span>{ task.id.0 }{": "}</span> 
//            <span>{ task.name.0.clone() }{": "}</span>
//            <span>{ task.project_id.0 }</span>
//        </li>
//    }
//}
//      

fn view_auth_unknown(_model: &Model) -> Html<Model> {
    html! {
        <div>
        </div>
    }
}

fn view_auth_yes(model: &Model, user: &AuthUser) -> Html<Model> {
    html! {
        <div>
            <div>
                { format!("Logged in as {}", user.name)}
            </div>
            <div>
                { view_nav(model) }
                <button id="btn-logout" onclick=|_| Msg::Logout>{ "Log out" }</button>
            </div>
            {view_view(model)}
        </div>
    }
}

fn view_auth_no(model: &Model) -> Html<Model> {
    html! {
        <div>
            <div>
                <button id="btn-login" onclick=|_| Msg::Login>{ "Log in" }</button>
            </div>
            <div>
                { "Testing: " }
                { view_nav(model) }
            </div>
        </div>
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        match &self.auth_state {
            AuthState::Unknown   => view_auth_unknown(self),
            AuthState::Yes(user) => view_auth_yes(self, &user),
            AuthState::No        => view_auth_no(self),
        }
    }
}
