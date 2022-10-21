use wasm_bindgen::JsCast;
use yew::prelude::*;
use web_sys::HtmlTextAreaElement;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
  pub name: String,
  pub value: String,
  pub onchange: Option<Callback<String>>,
  pub label: String,
  pub id: String,
}

#[function_component(TextArea)]
pub fn textarea(props: &Props) -> Html {
  let onchange = {
    let props_onchange = props.onchange.clone();
    match props_onchange {
      Some(props_onchange) => Callback::from(move |event: Event| {
        let change = event
          .target()
          .unwrap()
          .unchecked_into::<HtmlTextAreaElement>()
          .value();
        props_onchange.emit(change.clone());
      }),
      None => Callback::noop(),
    }
  };

  html! {
    <div>
      <label for={props.id.clone()}>{&props.label}</label>
      <br />
      <textarea
        rows="50"
        cols="100"
        id={props.id.clone()}
        value={props.value.clone()}
        {onchange}
      />
    </div>
  }
}
