mod components;

use components::textarea::TextArea;
use yew::prelude::*;
use snailquote::unescape;
use serde_json::json;

fn convert_to_prettified_json(data: String) -> String {
  let json_string = unescape(&data).unwrap();
  let value_result = serde_json::from_str(&json_string);
  let value = match value_result {
    Ok(v) => v,
    Err(_) => json!({}),
  };

  let prettified_string_value = serde_json::to_string_pretty(&value);
  let prettified_string = match prettified_string_value {
    Ok(s) => s,
    Err(_) => "".to_string(),
  };

  return prettified_string
}

#[function_component(App)]
pub fn app() -> Html {
  let input = use_state(String::new);
  let output = use_state(String::new);

  let input_onchange = {
    let input = input.clone();
    let output = output.clone();
    Callback::from(move |new_value: String| {
      input.set(new_value.clone());
      output.set(convert_to_prettified_json(new_value))
    })
  };

  html! {
    <div>
      <h1>{ "Prettify JSON String" }</h1>
      <TextArea 
        name="input"
        onchange={input_onchange}
        label="JSON String"
        id="input"
        value={ (*input).clone() }
      />
      <br />
      <TextArea
        name="output"
        label="Prettified JSON"
        id="output"
        value={ (*output).clone() }
      />
      <br />
      <a href="https://github.com/willdavsmith/prettifyjsonstring">{ "Source" }</a>
    </div>
  }
}
