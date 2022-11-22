
use gloo::console::{log, externs::log}; //Para usar console log
use rand::{Rng};
use serde::{Deserialize, Serialize}; //Para mostrar imprimir JSON
use yew::prelude::*; //Framework //Math random

#[derive(Serialize, Deserialize)]


#[derive(Clone)]
struct Carta {
    valor: String,
    tipo: String,
    color: String,
    img: String,
}

// impl Copy for Carta{}
// impl Clone for Carta {
//     fn clone(&self)->Carta{
//         *self
//     }
// }

#[function_component(Game)]

fn game() -> Html {
    // let onclick2 = Callback::from(|mouse_event:MouseEvent|{
    //     log!("Probando");
    // });
    //let mazoRestante = use_state(|| Vec::new());
    let pila= use_state(|| [Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new()]);
    //let v: Vec<Vec<Carta>> = use_state(|| Vec::new());

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
                };
                mazo.push(carta);
            }
        }
        //log!(serde_json::to_string_pretty(&mazo).unwrap());
        mazo
    }

    fn mezclarMazo() -> Vec<Carta> {
        let mut rng = rand::thread_rng();
        let mut mazoRevuelto: Vec<Carta> = Vec::new();
        let mut g2: Vec<Carta> = Vec::new();
        let mut g3: Vec<Carta> = Vec::new();
        let mazo: Vec<Carta> = creaMazo();

        for i in mazo {
            let random = rng.gen_range(1..4);
            if random == 1 {
                mazoRevuelto.push(i);
            } else if random == 2 {
                g2.push(i);
            } else {
                g3.push(i);
            }
        }
        for i in g2 {
            mazoRevuelto.push(i);
            mazoRevuelto.reverse();
        }
        for i in g3 {
            mazoRevuelto.push(i);
        }

        //log!(serde_json::to_string_pretty(&mazoRevuelto).unwrap());
        mazoRevuelto
    }
    
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

    let llenarMazo = Callback::from(move |_| {
        pila.set(colocar());
        // m.get(0).unwrap().color;
        log!(serde_json::to_string_pretty(&*pila).unwrap());

    });


    html!(
        <div class="tablero">
        <button onclick={llenarMazo} class="empezar">{"Juego nuevo"}</button>
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
