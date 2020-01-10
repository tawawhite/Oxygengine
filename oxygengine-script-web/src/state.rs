use crate::interface::WebScriptInterface;
use core::{
    ecs::World,
    state::{State, StateChange},
};
use js_sys::{Function, JsString, Reflect};
use wasm_bindgen::{JsCast, JsValue};

pub struct WebScriptBootState {
    initial_state_name: String,
}

impl WebScriptBootState {
    pub fn new(initial_state_name: &str) -> Self {
        Self {
            initial_state_name: initial_state_name.to_owned(),
        }
    }
}

impl State for WebScriptBootState {
    fn on_process(&mut self, _: &mut World) -> StateChange {
        if WebScriptInterface::is_ready() {
            return if let Some(result) = WebScriptInterface::build_state(&self.initial_state_name) {
                StateChange::Swap(Box::new(WebScriptState::new(result)))
            } else {
                StateChange::Pop
            };
        }
        StateChange::None
    }
}

pub(crate) struct WebScriptState {
    context: JsValue,
    on_enter: Option<Function>,
    on_process: Option<Function>,
    on_exit: Option<Function>,
}

impl WebScriptState {
    pub fn new(context: JsValue) -> Self {
        let on_enter = if let Ok(m) = Reflect::get(&context, &JsValue::from_str("onEnter")) {
            m.dyn_ref::<Function>().cloned()
        } else {
            None
        };
        let on_process = if let Ok(m) = Reflect::get(&context, &JsValue::from_str("onProcess")) {
            m.dyn_ref::<Function>().cloned()
        } else {
            None
        };
        let on_exit = if let Ok(m) = Reflect::get(&context, &JsValue::from_str("onExit")) {
            m.dyn_ref::<Function>().cloned()
        } else {
            None
        };
        Self {
            context,
            on_enter,
            on_process,
            on_exit,
        }
    }
}

impl State for WebScriptState {
    fn on_enter(&mut self, world: &mut World) {
        WebScriptInterface::set_world(world);

        if let Some(on_enter) = &self.on_enter {
            drop(on_enter.call0(&self.context));
        }
    }

    fn on_process(&mut self, world: &mut World) -> StateChange {
        WebScriptInterface::run_systems();
        WebScriptInterface::maintain_entities(world);

        if let Some(on_process) = &self.on_process {
            if let Ok(result) = on_process.call0(&self.context) {
                if let Some(result) = result.dyn_ref::<JsString>() {
                    let name = String::from(result);
                    if let Some(result) = WebScriptInterface::build_state(&name) {
                        return StateChange::Swap(Box::new(WebScriptState::new(result)));
                    }
                }
            }
        }
        StateChange::None
    }

    fn on_exit(&mut self, _: &mut World) {
        WebScriptInterface::unset_world();

        if let Some(on_exit) = &self.on_exit {
            drop(on_exit.call0(&self.context));
        }
    }
}
