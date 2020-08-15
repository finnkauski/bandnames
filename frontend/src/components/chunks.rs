use yew::prelude::*;

pub struct ProductCard {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub id: i32,
    pub name: String,
    pub which: String,
}

impl Component for ProductCard {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="product_card_container">
                <Anchor route=Route::ProductDetail(self.props.product.id) classes="product_card_anchor">
                    <img class="product_card_image" src={&self.props.product.image}/>
                    <div class="product_card_name">{&self.props.product.name}</div>
                    <div class="product_card_price">{"$"}{&self.props.product.price}</div>
                </Anchor>
                <button class="product_atc_button" onclick=onclick>{"Add To Cart"}</button>
            </div>
        }
    }
}
