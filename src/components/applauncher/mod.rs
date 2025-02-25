use crate::{example, example::ExamplePage};

use patternfly_yew::*;
use yew::prelude::*;

#[function_component(AppLauncherExample)]
pub fn app_launcher_example() -> Html {
    let toaster = use_toaster();

    let toast = move |message: &str| {
        if let Some(toaster) = &toaster {
            toaster.toast(Toast {
                title: message.into(),
                r#type: AlertType::Success,
                ..Default::default()
            });
        } else {
            log::info!("Missing toaster");
        }
    };

    let example1 = example! ("AppLauncher" => "applauncher.1.example");

    html! {
        <>
            <ExamplePage title="AppLauncher">
                {example1}
            </ExamplePage>
        </>
    }
}
