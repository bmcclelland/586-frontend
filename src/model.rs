use yew::prelude::*;
use yew::services::fetch::*;
use yew::services::storage::*;
use crate::authservice::*;
use crate::locservice::*;
use crate::views::*;
use crate::domain::*;

extern crate askama;
use askama::Template;
use stdweb::web::Node;
use yew::virtual_dom::VNode;

#[derive(Default)]
pub struct Temp {
    pub task: Option<TaskDetails>,
}

pub struct Model {
    pub auth: AuthService,
    pub loc: LocService,
    pub fetcher: FetchService,
    pub storage: StorageService,
    pub link: ComponentLink<Model>,
    pub task: Option<FetchTask>,
    pub scene: Scene,
    pub temp: Temp,
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
            temp: Temp::default(),
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
