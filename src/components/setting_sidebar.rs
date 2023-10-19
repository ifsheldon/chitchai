use dioxus::prelude::*;
use futures_util::StreamExt;

#[derive(Debug, Clone, PartialEq)]
enum GPTService {
    AzureOpenAI,
    OpenAI(String),
}

#[derive(Debug, Clone, PartialEq)]
enum SettingEvent {
    ToggleEnableGroupChat,
    SelectService(Option<GPTService>),
}

async fn setting_event_handler(mut rx: UnboundedReceiver<SettingEvent>,
                               enable_group_chat: UseState<bool>,
                               gpt_service: UseState<Option<GPTService>>) {
    while let Some(event) = rx.next().await {
        log::info!("setting_event_handler {:?}", event);
        match event {
            SettingEvent::ToggleEnableGroupChat => enable_group_chat.modify(|e| !*e),
            SettingEvent::SelectService(service) => gpt_service.set(service),
        }
    }
    log::error!("setting_event_handler exited");
}


pub fn SettingSidebar(cx: Scope) -> Element {
    let gpt_service = use_state(cx, || None::<GPTService>);
    let enable_group_chat = use_state(cx, || false);
    use_coroutine(cx, |rx| setting_event_handler(rx,
                                                 enable_group_chat.to_owned(),
                                                 gpt_service.to_owned()));
    render! {
        aside {
            class: "flex",
            div {
                class: "relative h-screen w-full overflow-y-auto border-l border-slate-300 bg-slate-50 py-8 dark:border-slate-700 dark:bg-slate-900 sm:w-full",
                div {
                    class: "mb-4 flex items-center gap-x-2 px-2 text-slate-800 dark:text-slate-200",
                    CloseSettingButton {},
                    h2 {
                        class: "text-lg font-medium",
                        "Settings"
                    }
                }
                SelectServiceSection {}
                Toggle {}
                AdvanceSettings {
                    gpt_service: gpt_service
                }
            }
        }
    }
}

fn CloseSettingButton(cx: Scope) -> Element {
    render! {
        button {
            class: "inline-flex rounded-lg p-1 hover:bg-slate-700",
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                class: "h-6 w-6",
                stroke_width: "2",
                stroke: "currentColor",
                fill: "none",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path {
                    stroke: "none",
                    d: "M0 0h24v24H0z",
                    fill: "none",
                }
                path {
                    d: "M4 4m0 2a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2z",
                }
                path {
                    d: "M9 4v16",
                }
                path {
                    d: "M14 10l2 2l-2 2",
                }
            }
            span {
                class: "sr-only",
                "Close settings sidebar"
            }
        }
    }
}

fn SelectServiceSection(cx: Scope) -> Element {
    let setting_event_handler = use_coroutine_handle::<SettingEvent>(cx).unwrap();
    render! {
        div {
            class: "px-2 py-4 text-slate-800 dark:text-slate-200",
            label {
                r#for: "select-service",
                class: "px-2 text-sm font-medium",
                "Services"
            }
            select {
                name: "select-service",
                id: "select-service",
                onchange: |select| {
                    let value = select.data.value.as_str();
                    match value {
                        "AzureOpenAI" => setting_event_handler.send(SettingEvent::SelectService(Some(GPTService::AzureOpenAI))),
                        "OpenAI" => setting_event_handler.send(SettingEvent::SelectService(Some(GPTService::OpenAI(String::new())))),
                        "Select a GPT service" => setting_event_handler.send(SettingEvent::SelectService(None)),
                        _ => log::error!("Unknown select-service value: {}", value),
                    }
                },
                class: "mt-2 w-full cursor-pointer rounded-lg border-r-4 border-transparent bg-slate-200 py-3 pl-1 text-sm focus:outline-none focus:ring-2 focus:ring-blue-600 dark:bg-slate-800",
                option {
                    value: "",
                    "Select a GPT service"
                }
                option {
                    value: "AzureOpenAI",
                    "Azure OpenAI"
                }
                option {
                    value: "OpenAI",
                    "OpenAI"
                }
            }
        }
    }
}

pub fn Toggle(cx: Scope) -> Element {
    let setting_event_handler = use_coroutine_handle::<SettingEvent>(cx).unwrap();
    render! {
        div {
            class: "px-2 py-4",
            label {
                class: "relative flex cursor-pointer items-center",
                input {
                    r#type: "checkbox",
                    onclick: |_| setting_event_handler.send(SettingEvent::ToggleEnableGroupChat),
                    value: "",
                    class: "peer sr-only",
                }
                div {
                    class: "peer h-6 w-11 rounded-full bg-slate-200 after:absolute after:left-[2px] after:top-[2px] after:h-5 after:w-5 after:rounded-full after:border after:border-slate-300 after:bg-white after:transition-all after:content-[''] peer-checked:bg-blue-600 peer-checked:after:translate-x-full peer-checked:after:border-white peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-blue-300 dark:border-slate-600 dark:bg-slate-700 dark:peer-focus:ring-blue-800",
                }
                span {
                    class: "ml-3 text-sm font-medium text-slate-800 dark:text-slate-200",
                    "Enable Group Chat"
                }
            }
        }
    }
}

#[derive(Props)]
struct AdvanceSettingsProps<'a> {
    #[props(! optional)]
    gpt_service: &'a Option<GPTService>,
}

fn AdvanceSettings<'a>(cx: Scope<'a, AdvanceSettingsProps>) -> Element<'a> {
    render! {
        div {
            class: "my-4 border-t border-slate-300 px-2 py-4 text-slate-800 dark:border-slate-700 dark:text-slate-200",
            label {
                class: "px-2 text-xs uppercase text-slate-500 dark:text-slate-400",
                "Advanced"
            }
            if let Some(gpt_service) = cx.props.gpt_service {
                match gpt_service {
                    GPTService::AzureOpenAI => render! {
                            SecretInputs {
                            gpt_service: gpt_service,
                        }
                    },
                    GPTService::OpenAI(_) => render!{
                        SecretInputs {
                            gpt_service: gpt_service,
                        }
                        SelectModel {}
                    },
                }
            }
            ModelConfigs {}
            button {
                r#type: "button",
                class: "mt-4 block w-full rounded-lg bg-slate-200 p-2.5 text-xs font-semibold hover:bg-blue-600 hover:text-slate-200 focus:outline-none focus:ring-2 focus:ring-blue-600 dark:bg-slate-800 dark:hover:bg-blue-600",
                "Save changes"
            }
        }
    }
}

#[derive(Props)]
struct SecretInputsProps<'a> {
    gpt_service: &'a GPTService,
}

fn SecretInputs<'a>(cx: Scope<'a, SecretInputsProps>) -> Element<'a> {
    const LABEL_STYLE: &str = "mb-2 mt-4 block px-2 text-sm font-medium";
    const INPUT_STYLE: &str = "block w-full rounded-lg bg-slate-200 p-2.5 text-xs focus:outline-none focus:ring-2 focus:ring-blue-600 dark:bg-slate-800 dark:placeholder-slate-400 dark:focus:ring-blue-600";
    match cx.props.gpt_service {
        GPTService::OpenAI(_) => render! {
            div {
                // API Key
                label {
                    r#for: "api-key",
                    class: "{LABEL_STYLE}",
                    "API Key"
                }
                input {
                    r#type: "password",
                    id: "api-key",
                    class: "{INPUT_STYLE}",
                    placeholder: "Required"
                }
                // Base URL
                label {
                    r#for: "base-url",
                    class: "{LABEL_STYLE}",
                    "Base URL / API Base (Optional)"
                }
                input {
                    r#type: "url",
                    id: "base-url",
                    class: "{INPUT_STYLE}",
                    placeholder: "https://api.openai.com",
                }
                // Org ID
                label {
                    r#for: "org-id",
                    class: "{LABEL_STYLE}",
                    "Org ID (Optional)"
                }
                input {
                    r#type: "text",
                    id: "org-id",
                    class: "{INPUT_STYLE}",
                }
            }
        },
        GPTService::AzureOpenAI => render! {
            div {
                // API Key
                label {
                    r#for: "api-key",
                    class: "{LABEL_STYLE}",
                    "API Key"
                }
                input {
                    r#type: "password",
                    id: "api-key",
                    class: "{INPUT_STYLE}",
                    placeholder: "Required"
                }
                // Base URL
                label {
                    r#for: "base-url",
                    class: "{LABEL_STYLE}",
                    "Base URL / API Base"
                }
                input {
                    r#type: "url",
                    id: "base-url",
                    class: "{INPUT_STYLE}",
                    placeholder: "Required",
                }
                // Deployment ID
                label {
                    r#for: "deployment-id",
                    class: "{LABEL_STYLE}",
                    "Deployment ID"
                }
                input {
                    r#type: "text",
                    id: "deployment-id",
                    class: "{INPUT_STYLE}",
                    placeholder: "Required",
                }
                // API Version
                label {
                    r#for: "api-version",
                    class: "{LABEL_STYLE}",
                    "API Version"
                }
                input {
                    r#type: "text",
                    id: "api-version",
                    class: "{INPUT_STYLE}",
                    placeholder: "Required",
                }
            }
        }
    }
}

fn SelectModel(cx: Scope) -> Element {
    render! {
        div {
            label {
                r#for: "select-model",
                class: "mb-2 mt-4 block px-2 text-sm font-medium",
                "Model"
            }
            select {
                name: "select-model",
                id: "select-model",
                class: "block w-full cursor-pointer rounded-lg border-r-4 border-transparent bg-slate-200 py-3 pl-1 text-xs focus:outline-none focus:ring-2 focus:ring-blue-600 dark:bg-slate-800 dark:placeholder-slate-400 dark:focus:ring-blue-600",
                option {
                    value: "gpt-3.5-turbo",
                    "gpt-3.5-turbo"
                }
                option {
                    value: "gpt-3.5-turbo-16k",
                    "gpt-3.5-turbo-16k"
                }
                option {
                    value: "gpt-4",
                    "gpt-4"
                }
                option {
                    value: "gpt-4-32k",
                    "gpt-4-32k"
                }
            }
        }
    }
}

fn ModelConfigs(cx: Scope) -> Element {
    const LABEL_STYLE: &str = "mb-2 mt-4 block px-2 text-sm font-medium";
    const INPUT_STYLE: &str = "block w-full rounded-lg bg-slate-200 p-2.5 text-xs focus:outline-none focus:ring-2 focus:ring-blue-600 dark:bg-slate-800 dark:placeholder-slate-400 dark:focus:ring-blue-600";
    render! {
        div {
            label {
                r#for: "max-tokens",
                class: "{LABEL_STYLE}",
                "Max tokens"
            }
            input {
                r#type: "number",
                id: "max-tokens",
                class: "{INPUT_STYLE}",
                placeholder: "2048",
            }
            label {
                r#for: "model-temperature",
                class: "{LABEL_STYLE}",
                "Temperature"
            }
            input {
                r#type: "number",
                id: "model-temperature",
                class: "{INPUT_STYLE}",
                placeholder: "0.7",
            }
        }
    }
}