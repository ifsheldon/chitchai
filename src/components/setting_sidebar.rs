use dioxus::prelude::*;
use futures_util::StreamExt;
use transprompt::async_openai::Client;
use transprompt::async_openai::config::{AzureConfig, OpenAIConfig};

use crate::app::{AppEvents, AuthedClient};
use crate::utils::auth::Auth;
use crate::utils::settings::{GPTService, OpenAIModel};
use crate::utils::storage::StoredStates;

const API_KEY: &str = "api-key";
const API_BASE: &str = "base-url";
const ORG_ID: &str = "org-id";
const API_VERSION: &str = "api-version";
const DEPLOYMENT_ID: &str = "deployment-id";

#[derive(Debug, Clone, PartialEq)]
enum SettingEvent {
    ToggleEnableGroupChat,
    SelectService(Option<GPTService>),
    SaveServiceConfig,
}


#[derive(Debug, Clone, PartialEq, Default)]
struct ServiceSettings {
    api_key: Option<String>,
    api_base: Option<String>,
    org_id: Option<String>,
    api_version: Option<String>,
    deployment_id: Option<String>,
    openai_model: Option<OpenAIModel>,
}

async fn setting_event_handler(mut rx: UnboundedReceiver<SettingEvent>,
                               enable_group_chat: UseState<bool>,
                               authed_client: UseSharedState<AuthedClient>,
                               service_settings: UseSharedState<ServiceSettings>,
                               global: UseSharedState<StoredStates>) {
    while let Some(event) = rx.next().await {
        log::info!("setting_event_handler {:?}", event);
        match event {
            SettingEvent::ToggleEnableGroupChat => enable_group_chat.modify(|e| !*e),
            SettingEvent::SelectService(service) => {
                match service.as_ref() {
                    None => *service_settings.write() = ServiceSettings::default(),
                    Some(s) => {
                        if *s == GPTService::AzureOpenAI {
                            service_settings.write().openai_model = None;
                        }
                    }
                };
                log::info!("Selected service: {:?}", service);
                global.write().selected_service = service;
            }
            SettingEvent::SaveServiceConfig => {
                let gpt_service = global.read().selected_service.clone();
                log::info!("Saving service configs for {:?}", gpt_service);
                match gpt_service {
                    None => unreachable!(),
                    Some(gpt_service) => {
                        let service_settings = service_settings.read();
                        // check fields first
                        match gpt_service {
                            GPTService::AzureOpenAI => {
                                if service_settings.api_key.is_none() {
                                    log::error!("API Key is required");
                                    continue;
                                }
                                if service_settings.api_base.is_none() {
                                    log::error!("API Base is required");
                                    continue;
                                }
                                if service_settings.deployment_id.is_none() {
                                    log::error!("Deployment ID is required");
                                    continue;
                                }
                                if service_settings.api_version.is_none() {
                                    log::error!("API Version is required");
                                    continue;
                                }
                            }
                            GPTService::OpenAI => {
                                if service_settings.api_key.is_none() {
                                    log::error!("API Key is required");
                                    continue;
                                }
                            }
                        }
                        // save configs
                        let (new_auth, new_authed_client): (Auth, Client) = match gpt_service {
                            GPTService::AzureOpenAI => {
                                let auth = Auth::AzureOpenAI {
                                    api_version: service_settings.api_version.to_owned().unwrap(),
                                    deployment_id: service_settings.deployment_id.to_owned().unwrap(),
                                    api_base: service_settings.api_base.to_owned().unwrap(),
                                    api_key: service_settings.api_key.to_owned().unwrap(),
                                };
                                let client = Client::with_config::<AzureConfig>(auth.clone().into());
                                (auth, client)
                            }
                            GPTService::OpenAI => {
                                let auth = Auth::OpenAI {
                                    api_key: service_settings.api_key.to_owned().unwrap(),
                                    org_id: service_settings.org_id.to_owned(),
                                    api_base: service_settings.api_base.to_owned(),
                                };
                                let client = Client::with_config::<OpenAIConfig>(auth.clone().into());
                                (auth, client)
                            }
                        };
                        let mut global = global.write();
                        global.auth.replace(new_auth);
                        authed_client.write().replace(new_authed_client);
                        global.save();
                        // TODO: remove this after testing
                        log::info!("Saved new auth: {:?}", global.auth);
                    }
                }
            }
        }
    }
    log::error!("setting_event_handler exited");
}


pub fn SettingSidebar(cx: Scope) -> Element {
    // get global states
    let global = use_shared_state::<StoredStates>(cx).unwrap();
    let authed_client = use_shared_state::<AuthedClient>(cx).unwrap();
    // setup local states
    let enable_group_chat = use_state(cx, || false);
    // setup shared states
    use_shared_state_provider(cx, ServiceSettings::default);
    let service_settings = use_shared_state::<ServiceSettings>(cx).unwrap();
    use_coroutine(cx, |rx| setting_event_handler(rx,
                                                 enable_group_chat.to_owned(),
                                                 authed_client.to_owned(),
                                                 service_settings.to_owned(),
                                                 global.to_owned()));
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
                    gpt_service: global.read().selected_service.clone(),
                    enable_group_chat: *enable_group_chat.get(),
                }
                ModelParameters {}
            }
        }
    }
}

fn CloseSettingButton(cx: Scope) -> Element {
    let app_event_handler = use_coroutine_handle::<AppEvents>(cx).unwrap();
    render! {
        button {
            class: "inline-flex rounded-lg p-1 hover:bg-slate-700",
            onclick: |_| app_event_handler.send(AppEvents::ToggleSettingsSidebar),
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
    enable_group_chat: bool,
}

enum ServiceEvent {
    SaveConfigs,
    SelectOpenAIModel(Option<OpenAIModel>),
}


fn ServiceConfigs(cx: Scope<ServiceConfigsProps>) -> Element {
    // TODO: when the component is opened, display the stored configs if any
    let service_settings = use_shared_state::<ServiceSettings>(cx).unwrap();
    let setting_event_handler = use_coroutine_handle::<SettingEvent>(cx).unwrap();
    let service_event_handler = use_coroutine(cx, |mut rx| {
        let service_settings = service_settings.to_owned();
        let setting_event_handler = setting_event_handler.to_owned();
        async move {
            while let Some(event) = rx.next().await {
                match event {
                    ServiceEvent::SaveConfigs => setting_event_handler.send(SettingEvent::SaveServiceConfig),
                    ServiceEvent::SelectOpenAIModel(model) => service_settings.write().openai_model = model,
                }
            }
        }
    });
    render! {
        div {
            class: "my-4 border-t border-slate-300 px-2 py-4 text-slate-800 dark:border-slate-700 dark:text-slate-200",
            label {
                class: "px-2 text-xs uppercase text-slate-500 dark:text-slate-400",
                "Service Configurations"
            }
            SelectServiceSection {}
            if let Some(gpt_service) = cx.props.gpt_service {
                rsx! {
                    SecretInputs {
                        gpt_service: gpt_service,
                    }
                    if gpt_service == GPTService::OpenAI {
                        rsx! {
                            SelectOpenAIModel {
                                enable_group_chat: cx.props.enable_group_chat,
                            }
                        }
                    }
                    button {
                        r#type: "button",
                        class: "mt-4 block w-full rounded-lg bg-slate-200 p-2.5 text-xs font-semibold hover:bg-blue-600 hover:text-slate-200 focus:outline-none focus:ring-2 focus:ring-blue-600 dark:bg-slate-800 dark:hover:bg-blue-600",
                        onclick: |_| {
                            service_event_handler.send(ServiceEvent::SaveConfigs)
                        },
                        "Save Configs"
                    }
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
    let service_settings = use_shared_state::<ServiceSettings>(cx).unwrap();
    const LABEL_STYLE: &str = "mb-2 mt-4 block px-2 text-sm font-medium";
    const INPUT_STYLE: &str = "block w-full rounded-lg bg-slate-200 p-2.5 text-xs focus:outline-none focus:ring-2 focus:ring-blue-600 dark:bg-slate-800 dark:placeholder-slate-400 dark:focus:ring-blue-600";
    match cx.props.gpt_service {
        GPTService::OpenAI => render! {
            div {
                // API Key
                label {
                    r#for: "{API_KEY}",
                    class: "{LABEL_STYLE}",
                    "API Key"
                }
                input {
                    r#type: "password",
                    id: "{API_KEY}",
                    class: "{INPUT_STYLE}",
                    onchange: |c| {
                        let value = &c.data.value;
                        if value.is_empty() {
                            service_settings.write().api_key = None;
                        } else {
                            service_settings.write().api_key = Some(value.to_string());
                        }
                    },
                    placeholder: "Required"
                }
                // Base URL
                label {
                    r#for: "{API_BASE}",
                    class: "{LABEL_STYLE}",
                    "Base URL / API Base (Optional)"
                }
                input {
                    r#type: "url",
                    id: "{API_BASE}",
                    class: "{INPUT_STYLE}",
                    onchange: |c| {
                        let value = &c.data.value;
                        if value.is_empty() {
                            service_settings.write().api_base = None;
                        } else {
                            service_settings.write().api_base = Some(value.to_string());
                        }
                    },
                    placeholder: "https://api.openai.com",
                }
                // Org ID
                label {
                    r#for: "{ORG_ID}",
                    class: "{LABEL_STYLE}",
                    "Org ID (Optional)"
                }
                input {
                    r#type: "text",
                    id: "{ORG_ID}",
                    class: "{INPUT_STYLE}",
                    onchange: |c| {
                        let value = &c.data.value;
                        if value.is_empty() {
                            service_settings.write().org_id = None;
                        } else {
                            service_settings.write().org_id = Some(value.to_string());
                        }
                    },
                }
            }
        },
        GPTService::AzureOpenAI => render! {
            div {
                // API Key
                label {
                    r#for: "{API_KEY}",
                    class: "{LABEL_STYLE}",
                    "API Key"
                }
                input {
                    r#type: "password",
                    id: "{API_KEY}",
                    class: "{INPUT_STYLE}",
                    placeholder: "Required",
                    onchange: |c| {
                        let value = &c.data.value;
                        if value.is_empty() {
                            service_settings.write().api_key = None;
                        } else {
                            service_settings.write().api_key = Some(value.to_string());
                        }
                    },
                }
                // Base URL
                label {
                    r#for: "{API_BASE}",
                    class: "{LABEL_STYLE}",
                    "Base URL / API Base"
                }
                input {
                    r#type: "url",
                    id: "{API_BASE}",
                    class: "{INPUT_STYLE}",
                    placeholder: "Required",
                    onchange: |c| {
                        let value = &c.data.value;
                        if value.is_empty() {
                            service_settings.write().api_base = None;
                        } else {
                            service_settings.write().api_base = Some(value.to_string());
                        }
                    },
                }
                // Deployment ID
                label {
                    r#for: "{DEPLOYMENT_ID}",
                    class: "{LABEL_STYLE}",
                    "Deployment ID"
                }
                input {
                    r#type: "text",
                    id: "{DEPLOYMENT_ID}",
                    class: "{INPUT_STYLE}",
                    placeholder: "Required",
                    onchange: |c| {
                        let value = &c.data.value;
                        if value.is_empty() {
                            service_settings.write().deployment_id = None;
                        } else {
                            service_settings.write().deployment_id = Some(value.to_string());
                        }
                    },
                }
                // API Version
                label {
                    r#for: "{API_VERSION}",
                    class: "{LABEL_STYLE}",
                    "API Version"
                }
                input {
                    r#type: "text",
                    id: "{API_VERSION}",
                    class: "{INPUT_STYLE}",
                    placeholder: "Required",
                    onchange: |c| {
                        let value = &c.data.value;
                        if value.is_empty() {
                            service_settings.write().api_version = None;
                        } else {
                            service_settings.write().api_version = Some(value.to_string());
                        }
                    },
                }
            }
        }
    }
}

#[inline_props]
fn SelectOpenAIModel(cx: Scope, enable_group_chat: bool) -> Element {
    const NULL_OPTION: &str = "Select a model";
    let service_event_handler = use_coroutine_handle::<ServiceEvent>(cx).unwrap();
    let usable_models = if *enable_group_chat {
        OpenAIModel::gpt4_models()
    } else {
        OpenAIModel::all_models()
    };
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
                        service_event_handler.send(ServiceEvent::SelectOpenAIModel(None));
                    } else {
                        match usable_models.iter().find(|m| (*m).eq(model)).cloned() {
                            Some(m) => service_event_handler.send(ServiceEvent::SelectOpenAIModel(Some(m))),
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
                usable_models.iter().map(|model| rsx! {
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