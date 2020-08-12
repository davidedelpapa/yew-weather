use yew::{html, Html, Properties};
use yewtil::{Pure, PureComponent};

pub type Image = Pure<PureImage>;

#[derive(Clone, PartialEq, Properties)]
pub struct PureImage {
    pub img: String,
    pub caption: String,
}

impl PureComponent for PureImage {
    fn render(&self) -> Html {
        html! {
            <div>
                <img src=&self.img alt=&self.caption />
                <p>{ &self.caption }</p>
            </div>
        } 
    }
}