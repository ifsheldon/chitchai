use std::fmt::{Display, Formatter};

use dioxus::prelude::*;
use futures_util::StreamExt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GPTService {
    AzureOpenAI,
    OpenAI,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OpenAIModel {
    GPT35,
    GPT35_16k,
    GPT4,
    GPT4_32k,
}

impl Display for OpenAIModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            OpenAIModel::GPT35 => "gpt-3.5-turbo",
            OpenAIModel::GPT35_16k => "gpt-3.5-turbo-16k",
            OpenAIModel::GPT4 => "gpt-4",
            OpenAIModel::GPT4_32k => "gpt-4-32k",
        })
    }
}

impl PartialEq<str> for OpenAIModel {
    fn eq(&self, other: &str) -> bool {
        let other = other.trim().to_lowercase();
        match self {
            OpenAIModel::GPT35 => other == "gpt-3.5-turbo",
            OpenAIModel::GPT35_16k => other == "gpt-3.5-turbo-16k",
            OpenAIModel::GPT4 => other == "gpt-4",
            OpenAIModel::GPT4_32k => other == "gpt-4-32k",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum SettingEvent {
    ToggleEnableGroupChat,
    SelectService(Option<GPTService>),
    SelectModel(Option<OpenAIModel>),
}

async fn setting_event_handler(mut rx: UnboundedReceiver<SettingEvent>,
                               enable_group_chat: UseState<bool>,
                               gpt_service: UseState<Option<GPTService>>,
                               openai_model: UseState<Option<OpenAIModel>>) {
    while let Some(event) = rx.next().await {
        log::info!("setting_event_handler {:?}", event);
        match event {
            SettingEvent::ToggleEnableGroupChat => enable_group_chat.modify(|e| !*e),
            SettingEvent::SelectService(service) => {
                if !service.is_none() && service.unwrap().eq(&GPTService::AzureOpenAI) {
                    openai_model.set(None);
                }
                gpt_service.set(service)
            }
            SettingEvent::SelectModel(selected) => openai_model.set(selected),
        }
    }
    log::error!("setting_event_handler exited");
}


pub fn SettingSidebar(cx: Scope) -> Element {
    let gpt_service = use_state(cx, || None::<GPTService>);
    let enable_group_chat = use_state(cx, || false);
    let openai_model = use_state(cx, || None::<OpenAIModel>);
    use_coroutine(cx, |rx| setting_event_handler(rx,
                                                 enable_group_chat.to_owned(),
                                                 gpt_service.to_owned(),
                                                 openai_model.to_owned()));
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
                Toggle {}
                ServiceConfigs {
                    gpt_service: **gpt_service
                }
                ModelParameters {}
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
    const NULL_OPTION: &str = "Select AI Provider";
    let setting_event_handler = use_coroutine_handle::<SettingEvent>(cx).unwrap();

    render! {
        div {
            class: "px-2 py-4 text-slate-800 dark:text-slate-200",
            select {
                name: "select-service",
                id: "select-service",
                onchange: |select| {
                    let value = select.data.value.as_str();
                    match value {
                        "AzureOpenAI" => setting_event_handler.send(SettingEvent::SelectService(Some(GPTService::AzureOpenAI))),
                        "OpenAI" => setting_event_handler.send(SettingEvent::SelectService(Some(GPTService::OpenAI))),
                        NULL_OPTION => setting_event_handler.send(SettingEvent::SelectService(None)),
                        _ => log::error!("Unknown select-service value: {}", value),
                    }
                },
                class: "mt-2 w-full cursor-pointer rounded-lg border-r-4 border-transparent bg-slate-200 py-3 pl-1 text-sm focus:outline-none focus:ring-2 focus:ring-blue-600 dark:bg-slate-800",
                option {
                    value: "",
                    "{NULL_OPTION}"
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

#[derive(Props, PartialEq)]
struct ServiceConfigsProps {
    #[props(! optional)]
    gpt_service: Option<GPTService>,
}

fn ServiceConfigs(cx: Scope<ServiceConfigsProps>) -> Element {
    render! {
        div {
            class: "my-4 border-t border-slate-300 px-2 py-4 text-slate-800 dark:border-slate-700 dark:text-slate-200",
            label {
                class: "px-2 text-xs uppercase text-slate-500 dark:text-slate-400",
                "Service Configurations"
            }
            SelectServiceSection {}
            if let Some(gpt_service) = cx.props.gpt_service {
                match gpt_service {
                    GPTService::AzureOpenAI => render! {
                            SecretInputs {
                                gpt_service: gpt_service,
                            }
                            button {
                                r#type: "button",
                                class: "mt-4 block w-full rounded-lg bg-slate-200 p-2.5 text-xs font-semibold hover:bg-blue-600 hover:text-slate-200 focus:outline-none focus:ring-2 focus:ring-blue-600 dark:bg-slate-800 dark:hover:bg-blue-600",
                                "Save Configs"
                            }
                    },
                    GPTService::OpenAI => render!{
                        SecretInputs {
                            gpt_service: gpt_service,
                        }
                        SelectModel {}
                        button {
                            r#type: "button",
                            class: "mt-4 block w-full rounded-lg bg-slate-200 p-2.5 text-xs font-semibold hover:bg-blue-600 hover:text-slate-200 focus:outline-none focus:ring-2 focus:ring-blue-600 dark:bg-slate-800 dark:hover:bg-blue-600",
                            "Save Configs"
                        }
                    },
                }
            }
        }
    }
}

#[derive(Props, PartialEq)]
struct SecretInputsProps {
    gpt_service: GPTService,
}

fn SecretInputs(cx: Scope<SecretInputsProps>) -> Element {
    const LABEL_STYLE: &str = "mb-2 mt-4 block px-2 text-sm font-medium";
    const INPUT_STYLE: &str = "block w-full rounded-lg bg-slate-200 p-2.5 text-xs focus:outline-none focus:ring-2 focus:ring-blue-600 dark:bg-slate-800 dark:placeholder-slate-400 dark:focus:ring-blue-600";
    match cx.props.gpt_service {
        GPTService::OpenAI => render! {
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
    const ALL_MODELS: [OpenAIModel; 4] = [
        OpenAIModel::GPT35,
        OpenAIModel::GPT35_16k,
        OpenAIModel::GPT4,
        OpenAIModel::GPT4_32k,
    ];
    const NULL_OPTION: &str = "Select a model";
    let setting_event_handler = use_coroutine_handle::<SettingEvent>(cx).unwrap();
    render! {
        div {
            label {
                r#for: "select-model",
                class: "mb-2 mt-4 block px-2 text-sm font-medium",
                "Model"
            }
            select {
                name: "select-model",
                onchange: |change|{
                    let model = change.data.value.as_str();
                    if model == NULL_OPTION {
                        setting_event_handler.send(SettingEvent::SelectModel(None));
                    } else {
                        match ALL_MODELS.iter().find(|m| (*m).eq(model)).cloned() {
                            Some(m) => setting_event_handler.send(SettingEvent::SelectModel(Some(m))),
                            None => log::error!("Unknown model: {}", model),
                        }
                    }
                },
                id: "select-model",
                class: "block w-full cursor-pointer rounded-lg border-r-4 border-transparent bg-slate-200 py-3 pl-1 text-xs focus:outline-none focus:ring-2 focus:ring-blue-600 dark:bg-slate-800 dark:placeholder-slate-400 dark:focus:ring-blue-600",
                option {
                    value: "",
                    "{NULL_OPTION}"
                }
                ALL_MODELS.iter().map(|model| rsx! {
                    option {
                        value: "{model}",
                        "{model}"
                    }
                })
            }
        }
    }
}

fn ModelParameters(cx: Scope) -> Element {
    const LABEL_STYLE: &str = "mb-2 mt-4 block px-2 text-sm font-medium";
    const INPUT_STYLE: &str = "block w-full rounded-lg bg-slate-200 p-2.5 text-xs focus:outline-none focus:ring-2 focus:ring-blue-600 dark:bg-slate-800 dark:placeholder-slate-400 dark:focus:ring-blue-600";
    render! {
        div {
            class: "my-4 border-t border-slate-300 px-2 py-4 text-slate-800 dark:border-slate-700 dark:text-slate-200",
            label {
                class: "px-2 text-xs uppercase text-slate-500 dark:text-slate-400",
                "Model Configurations"
            }
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
            button {
                r#type: "button",
                class: "mt-4 block w-full rounded-lg bg-slate-200 p-2.5 text-xs font-semibold hover:bg-blue-600 hover:text-slate-200 focus:outline-none focus:ring-2 focus:ring-blue-600 dark:bg-slate-800 dark:hover:bg-blue-600",
                "Save Parameters"
            }
        }
    }
}