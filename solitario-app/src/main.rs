use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

#[function_component(Game)]
fn game()->Html{
    html!(
        <div class="tablero">
        <button class="empezar">{"Juego nuevo"}</button>
        <br/>
        <br/>
        <button class="empezar">{"Salir"}</button>
        <div class="superior">
            <div class="mazo">
                <div class="espacio-carta">
                    <div id="pila-inicial"></div>
                    <div class="carta-mazo">
                        <img src="./img/card back red.png" width="98" height="152"/>
                    </div>
                </div>
                <div class="espacio-carta" id="seleccionada"></div>
            </div>
            <div class="casa">
                <div class="espacio-carta" id="casa-0"></div>
                <div class="espacio-carta" id="casa-1"></div>
                <div class="espacio-carta" id="casa-2"></div>
                <div class="espacio-carta" id="casa-3"></div>
            </div>
        </div>
        <div class="inferior">
            <div class="espacio-carta" id="pila-0"></div>
            <div class="espacio-carta" id="pila-1"></div>
            <div class="espacio-carta" id="pila-2"></div>
            <div class="espacio-carta" id="pila-3"></div>
            <div class="espacio-carta" id="pila-4"></div>
            <div class="espacio-carta" id="pila-5"></div>
            <div class="espacio-carta" id="pila-6"></div>
        </div>
    </div>
    )
}


fn main() {
    yew::start_app::<Game>();

    
}