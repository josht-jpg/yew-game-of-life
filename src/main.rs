use stylist::css;
use wasm_bindgen::*;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

const DEFAULT_BOARD_SIZE: i32 = 32;

fn step(tiles: &[bool], board_size: i32) -> Vec<bool> {
    tiles
        .iter()
        .enumerate()
        .map(|(index, is_alive)| -> bool {
            let neighbor_to_integer = |i: i32| {
                if i < 0 || i >= tiles.len() as i32 {
                    0
                } else {
                    tiles[i as usize] as i32
                }
            };

            let number_of_live_neighbors = neighbor_to_integer((index as i32) - board_size - 1)
                + neighbor_to_integer((index as i32) - board_size)
                + neighbor_to_integer((index as i32) - board_size + 1)
                + neighbor_to_integer((index as i32) - 1)
                + neighbor_to_integer((index as i32) + 1)
                + neighbor_to_integer((index as i32) + board_size - 1)
                + neighbor_to_integer((index as i32) + board_size)
                + neighbor_to_integer((index as i32) + board_size + 1);

            if !is_alive {
                return number_of_live_neighbors == 3;
            }

            if number_of_live_neighbors < 2 || number_of_live_neighbors > 3 {
                return !is_alive;
            }

            if number_of_live_neighbors == 2 || number_of_live_neighbors == 3 {
                return *is_alive;
            }

            return true;
        })
        .collect()
}

#[function_component(App)]
fn app() -> Html {
    let tiles = use_state(|| vec![false; (DEFAULT_BOARD_SIZE * DEFAULT_BOARD_SIZE) as usize]);
    let board_size = use_state(|| DEFAULT_BOARD_SIZE);
    let is_mouse_down = use_state(|| false);
    let is_running = use_state(|| false);

    let on_mouse_down = {
        let is_mouse_down = is_mouse_down.clone();
        let is_running = is_running.clone();
        Callback::from(move |_| {
            if *is_running {
                return;
            }

            is_mouse_down.set(true)
        })
    };
    let on_mouse_up = {
        let is_mouse_down = is_mouse_down.clone();
        let is_running = is_running.clone();
        Callback::from(move |_| {
            if *is_running {
                return;
            }
            is_mouse_down.set(false)
        })
    };

    let on_mouse_over_cell = |index: usize| {
        let tiles = tiles.clone();
        let is_mouse_down = is_mouse_down.clone();
        let is_running = is_running.clone();
        Callback::from(move |_event: MouseEvent| {
            if *is_running {
                return;
            }
            tiles.set(if *is_mouse_down {
                tiles
                    .iter()
                    .enumerate()
                    .map(|(i, tile)| if index == i { true } else { *tile })
                    .collect::<Vec<bool>>()
            } else {
                tiles.to_vec()
            })
        })
    };

    {
        let tiles = tiles.clone();
        let is_running = is_running.clone();
        let board_size = board_size.clone();
        use_effect(move || {
            if *is_running {
                let t = gloo_timers::callback::Timeout::new(40, move || {
                    tiles.set(step(&tiles, *board_size))
                });
                t.forget();
            }

            || {}
        })
    }

    let on_start_press = {
        let is_running = is_running.clone();
        Callback::from(move |_| {
            if *is_running {
                is_running.set(false)
            } else {
                is_running.set(true);
            }
        })
    };

    let on_next_click = {
        let tiles = tiles.clone();
        let is_running = is_running.clone();
        let board_size = board_size.clone();

        Callback::from(move |_| {
            if *is_running {
                return;
            }
            tiles.set(step(&tiles, *board_size))
        })
    };

    let on_clear_click = {
        let tiles = tiles.clone();
        let is_running = is_running.clone();
        let board_size = board_size.clone();
        Callback::from(move |_| {
            tiles.set(vec![false; (*board_size * *board_size) as usize]);
            is_running.set(false)
        })
    };

    let on_board_size_change = {
        let board_size = board_size.clone();
        let tiles = tiles.clone();

        Callback::from(move |event: Event| {
            let target: Option<EventTarget> = event.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                board_size.set(input.value_as_number() as i32);
                tiles.set(vec![
                    false;
                    (input.value_as_number() * input.value_as_number())
                        as usize
                ]);
            }
        })
    };
    let grid_template_columns = format!("repeat({}, minmax(0, 1fr))", *board_size);
    html! {
        <div class="container" onmousedown={on_mouse_down} onmouseup={on_mouse_up}>
        <div class={css!(
           display: grid;
           grid-template-columns: ${grid_template_columns};
           box-shadow: grey 0 0 8px;
           height: 70vh;
           width: 70vh;
        )}>
            {tiles.iter().enumerate().map(|(index,  is_alive)|
               html!{<span class={
                   format!("{}", if *is_alive { "alive-cell" } else { "cell" })
                } onmouseover={ {on_mouse_over_cell(index)}}/> }
            ).collect::<Html>()}
        </div>

        <div class="controls-container">
        <button class="control" onclick={on_start_press}>
        { if *is_running { "Pause" } else { "Start" } }
        </button>
        <button class="control" onclick={on_next_click} >
        {"Next"}
        </button>
        <button class="control" onclick={on_clear_click}>
        {"Clear"}
        </button>
        </div>
        <div class="board-size-input-container">
            <label class="board-size-label">{ format!("Board Size: {} x {}", *board_size, *board_size)}</label>
                <input
                    class="board-size-input"
                    type="range"
                    min={5}
                    max={96}
                    onchange={on_board_size_change}
                />
        </div>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
