//Richard Osvaldo León Chinchilla 2019003759

use gloo::{console::{log, externs::log}, utils::{document, document_element, window}, timers::callback}; //Para usar console log
use rand::{Rng};
use serde::{Deserialize, Serialize}; //Para mostrar imprimir JSON
use yew::{prelude::*}; //Framework //Math random
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{console,Document,HtmlElement,Element,Event,EventTarget};

#[derive(Serialize, Deserialize)]


#[derive(Clone)]
struct Carta {
    valor: String,
    tipo: String,
    color: String,
    img: String,
    volteada:bool,
    id:isize
}


//Juego
#[function_component(Game)]

fn game() -> Html {
    let pila:UseStateHandle<[Vec<Carta>; 8]>= use_state(|| [Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new()]);

    let primeraCartaClick:UseStateHandle<Carta> =use_state(|| Carta { valor: "".to_string(), tipo: "".to_string(), color: "".to_string(), img: "".to_string(), volteada: false, id: 2 });

//Funcion que se encarga de crear las cartas del solitario
    fn creaMazo() -> Vec<Carta> {
        let mut mazo: Vec<Carta> = Vec::new();
        let tipos = ["trebol", "corazon", "diamante", "espada"];
        let nums = [
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "Z", "J", "Q", "K",
        ];
        for i in nums {
            for j in tipos {
                let mut colores = "rojo";
                if j.to_string().eq("trebol") || j.to_string().eq("espada") {
                    colores = "negro";
                }
                let imgs = format!("./img/{}_de_{}.png", i.to_string(), j.to_string());
                let carta = Carta {
                    valor: i.to_string(),
                    color: colores.to_string(),
                    tipo: j.to_string(),
                    img: imgs.to_string(),
                    volteada:true,
                    id:0

                };
                mazo.push(carta);
            }
        }
        //log!(serde_json::to_string_pretty(&mazo).unwrap());
        mazo
    }

//Funcion que se encarga de revolver las cartas creadas
    fn mezclarMazo() -> Vec<Carta> {
        let mut rng = rand::thread_rng();
        let mut mazoRevuelto: Vec<Carta> = Vec::new();
        let mut mazo: Vec<Carta> = creaMazo();
        while mazo.len()>0{
            let random = rng.gen_range(0..mazo.len());
            mazoRevuelto.push(mazo.get(random).unwrap().clone());
            mazo.remove(random);

        }

        //log!(serde_json::to_string_pretty(&mazoRevuelto).unwrap());
        mazoRevuelto
    }
    
//Funcion que se encarga de colocar las cartas en las pilas [1..7]
    fn colocar()->[Vec<Carta>;8]{
        //En la posicion 8 estará alojado el restante del mazo
        let mut pilas:[Vec<Carta>;8] = [Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new()];
        let mut mazoRevuelto:Vec<Carta> = mezclarMazo();
        for i in 0..7{
           for j in 0..(i+1){
            let carta = mazoRevuelto[0].clone();
            mazoRevuelto.remove(0);
            pilas[i].push(carta);
           
           }
        }
        pilas[7] = mazoRevuelto;
        //log!(serde_json::to_string_pretty(&pilas).unwrap());
        pilas

    }

    fn comprobarClickCarta(carta:Carta){
        log!("Si funcionoo");
        // if(primeraCartaClik){

        // }
        // else{

        // }
    }




  //Funcion que coloca las cartas de la pila, en las pilas del tablero (HTML) 
    let colocarCartasPilas = |pilas:UseStateHandle<[Vec<Carta>;8]>|{
        let document = document();

        for i in 0..8  {
            let mut j = i;
            if i==7 {
                j=j-1;
            }
            let idMazo = "#pila-inicial".to_string();
            let mazo = document.query_selector(&idMazo).unwrap().unwrap();
            let id = format!("#pila-{}",j.to_string());

            let pila = document.query_selector(&id).unwrap().unwrap();
            for j in 0..pilas[i].len(){
                let carta = pilas[i].get(j).unwrap().clone();
                let cartaHTML = document.create_element("div").unwrap();
                let imagen = document.create_element("img").unwrap();
                if carta.volteada && j!=pilas[i].len()-1 && i != 7 {
                    imagen.set_attribute("src","./img/card back red.png");
                }
                else {
                    imagen.set_attribute("src",&carta.img);
                }
                
                imagen.set_attribute("width", "98");
                imagen.set_attribute("height", "152");
                cartaHTML.set_class_name("pila-inicial");
                let espacio = j*30;
                let style = format!("top:{}px",espacio.to_string());
                if i!=7{
                    cartaHTML.set_attribute("style", &style);
                    cartaHTML.append_child(&imagen);
                    let f = Closure::wrap(Box::new(move || { 
                      //  primeraCartaClick.set(carta.clone());
                        
                    }) as Box<dyn FnMut()>);
                    cartaHTML.add_event_listener_with_callback("click", f.as_ref().unchecked_ref());
                    f.forget();
                    let node = cartaHTML.into();
                    pila.append_child(&node);
                }
                else{
                   cartaHTML.append_child(&imagen);
                   let node = cartaHTML.into();
                   mazo.append_child(&node);
               }


                
            }
        }

    };

    
//Funcion que se ejecuta cuando se le da click, da inicio al juego
    let llenarMazo = Callback::from(move |_| {
        pila.set(colocar());

        log!(serde_json::to_string_pretty(&*pila).unwrap());

         colocarCartasPilas(pila.clone());

    });

    //HTML del tablero
    html!(
        <div class="tablero">
        <button onclick={llenarMazo} class="empezar">{"Juego nuevo"}</button>
        <br/>
        <br/>
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

//Funcion principal
fn main() {
    yew::start_app::<Game>();
}
