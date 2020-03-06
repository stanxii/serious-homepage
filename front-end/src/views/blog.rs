use yew::prelude::*;

pub struct Blog {}

impl Component for Blog {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Blog {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h2>{ "Everyone needs a blog right?" }</h2>
                <p>
                    { "Some very wise words... "}
                </p>
            </div>
        }
    }
}
