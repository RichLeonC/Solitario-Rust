use std::{path::StripPrefixError, ops::RangeInclusive};

use gloo::console::{externs::log, log}; //Para usar console log
use rand::{Rng, random};
use serde::{Deserialize, Serialize}; //Para mostrar imprimir JSON
use yew::prelude::*; //Framework //Math random

#[derive(Serialize, Deserialize)]
struct Carta {
    valor: String,
    tipo: String,
    color: String,
    img: String,
}

#[function_component(Game)]

fn game() -> Html {
    // let onclick2 = Callback::from(|mouse_event:MouseEvent|{
    //     log!("Probando");
    // });
    let mut mazo: Vec<Carta> = Vec::new();
    let creaMazo= ||{
        
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
        log!(serde_json::to_string_pretty(&mazo).unwrap());
       
    };


    let llenarMazo =Callback::from(|mouse_event:MouseEvent|{
        creaMazo;
        
        
    });
    
    // let mezclarMazo: () = {
    //     let mut rng = rand::thread_rng();
    //     let mut mazoRevuelto:Vec<Carta> = Vec::new();
    //     let mut g2:Vec<Carta> = Vec::new();
    //     let mut g3:Vec<Carta> = Vec::new();

    //     for i in mazo{
    //         let random = rng.gen_range(1..4);
    //         log!(random);
    //         if random == 1{
    //             mazoRevuelto.push(i);
    //         }
    //         else if random == 2{
    //             g2.push(i);
    //         }
    //         else{
    //             g3.push(i);
    //         }
    //     }
    //     for i in g2{
    //         mazoRevuelto.push(i);
    //         mazoRevuelto.reverse();
            
    //     }
    //     for i in g3{
    //         mazoRevuelto.push(i);
    //     }
 

    //     log!(serde_json::to_string_pretty(&mazoRevuelto).unwrap());

    // };

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
