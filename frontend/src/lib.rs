use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod components;

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {

        <div id="content">
          <InputForm alertNewData={setNewData} />
          <Chunks
            data={data}
            dataSetter={setData}
            newData={newData}
            newDataSetter={setNewData}
          />
        </div>
            }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Home>::new().mount_to_body();
}
