#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct PiGraphProps {
    width: u64,
    height: u64,
    pi_data: Vec<PiData>,
}

#[derive(Props, Clone, PartialEq)]
pub struct PiData {
    pub name: String,
    pub value: u64,
}

#[cfg(feature = "web")]
#[wasm_bindgen(module = "./javascript/pi_chart.js")]
extern "C" {
    #[wasm_bindgen(catch, js_name = piChart)]
    fn pi_chart(width: u64, height: u64, dataset: Vec<PiData>) -> Result<JsValue, JsValue>;
}

#[component]
pub fn pi_graph(props: PiGraphProps) -> Element {
    use_before_render(move || {
        #[cfg(feature = "web")]
        pi_chart(props.width, props.height).expect("get pi chart failed");
    });

    rsx! {
        div {
            class: "flex flex-col w-full h-[100px] justify-start items-start",
            div {
                id: "pie",
                class: "flex flex-row w-full h-[50px] justify-start items-start",
            }
            // div {
            //     "Hel;lo234"
            // }
        }
    }
}
