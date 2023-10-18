use dioxus::prelude::*;

pub fn SettingSidebar(cx: Scope) -> Element {
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
                SelectModeSection {}
                Toggle {}
                AdvanceSettings {}
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

fn SelectModeSection(cx: Scope) -> Element {
    render! {
        div {
            class: "px-2 py-4 text-slate-800 dark:text-slate-200",
            label {
                r#for: "select-mode",
                class: "px-2 text-sm font-medium",
                "Mode"
            }
            select {
                name: "select-mode",
                id: "select-mode",
                class: "mt-2 w-full cursor-pointer rounded-lg border-r-4 border-transparent bg-slate-200 py-3 pl-1 text-sm focus:outline-none focus:ring-2 focus:ring-blue-600 dark:bg-slate-800",
                option {
                    value: "",
                    "Select a mode"
                }
                option {
                    value: "Complete",
                    "Complete"
                }
                option {
                    value: "Chat",
                    "Chat"
                }
                option {
                    value: "Insert",
                    "Insert"
                }
                option {
                    value: "Edit",
                    "Edit"
                }
            }
        }
    }
}

pub fn Toggle(cx: Scope) -> Element {
    render! {
        div {
            class: "px-2 py-4",
            label {
                class: "relative flex cursor-pointer items-center",
                input {
                    r#type: "checkbox",
                    value: "",
                    class: "peer sr-only",
                }
                div {
                    class: "peer h-6 w-11 rounded-full bg-slate-200 after:absolute after:left-[2px] after:top-[2px] after:h-5 after:w-5 after:rounded-full after:border after:border-slate-300 after:bg-white after:transition-all after:content-[''] peer-checked:bg-blue-600 peer-checked:after:translate-x-full peer-checked:after:border-white peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-blue-300 dark:border-slate-600 dark:bg-slate-700 dark:peer-focus:ring-blue-800",
                }
                span {
                    class: "ml-3 text-sm font-medium text-slate-800 dark:text-slate-200",
                    "Toggle"
                }
            }
        }
    }
}

fn AdvanceSettings(cx: Scope) -> Element {
    render! {
        div {
            class: "my-4 border-t border-slate-300 px-2 py-4 text-slate-800 dark:border-slate-700 dark:text-slate-200",
            label {
                class: "px-2 text-xs uppercase text-slate-500 dark:text-slate-400",
                "Advanced"
            }
            SecretInputs {}
            SelectModel {}
            ModelConfigs {}
            button {
                r#type: "button",
                class: "mt-4 block w-full rounded-lg bg-slate-200 p-2.5 text-xs font-semibold hover:bg-blue-600 hover:text-slate-200 focus:outline-none focus:ring-2 focus:ring-blue-600 dark:bg-slate-800 dark:hover:bg-blue-600",
                "Save changes"
            }
        }
    }
}

fn SecretInputs(cx: Scope) -> Element {
    render! {
        div {
            label {
                r#for: "api-key",
                class: "mb-2 mt-4 block px-2 text-sm font-medium",
                "API Key"
            }
            input {
                r#type: "password",
                id: "api-key",
                class: "block w-full rounded-lg bg-slate-200 p-2.5 text-xs focus:outline-none focus:ring-2 focus:ring-blue-600 dark:bg-slate-800 dark:placeholder-slate-400 dark:focus:ring-blue-600",
                placeholder: "4sNhFQ******ffyt",
            }
            label {
                r#for: "base-url",
                class: "mb-2 mt-4 block px-2 text-sm font-medium",
                "Base URL"
            }
            input {
                r#type: "url",
                id: "base-url",
                class: "block w-full rounded-lg bg-slate-200 p-2.5 text-xs focus:outline-none focus:ring-2 focus:ring-blue-600 dark:bg-slate-800 dark:placeholder-slate-400 dark:focus:ring-blue-600",
                placeholder: "https://api.openai.com",
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
                    value: "gpt-4",
                    "gpt-4"
                }
                option {
                    value: "gpt-4-0314",
                    "gpt-4-0314"
                }
                option {
                    value: "gpt-4-32k",
                    "gpt-4-32k"
                }
                option {
                    value: "gpt-4-32k-0314",
                    "gpt-4-32k-0314"
                }
                option {
                    value: "gpt-3.5-turbo-0301",
                    "gpt-3.5-turbo-0301"
                }
            }
        }
    }
}

fn ModelConfigs(cx: Scope) -> Element {
    render! {
        div {
            label {
                r#for: "max-tokens",
                class: "mb-2 mt-4 block px-2 text-sm font-medium",
                "Max tokens"
            }
            input {
                r#type: "number",
                id: "max-tokens",
                class: "block w-full rounded-lg bg-slate-200 p-2.5 text-xs focus:outline-none focus:ring-2 focus:ring-blue-600 dark:bg-slate-800 dark:placeholder-slate-400 dark:focus:ring-blue-600",
                placeholder: "2048",
            }
            label {
                r#for: "max-tokens",
                class: "mb-2 mt-4 block px-2 text-sm font-medium",
                "Temperature"
            }
            input {
                r#type: "number",
                id: "max-tokens",
                class: "block w-full rounded-lg bg-slate-200 p-2.5 text-xs focus:outline-none focus:ring-2 focus:ring-blue-600 dark:bg-slate-800 dark:placeholder-slate-400 dark:focus:ring-blue-600",
                placeholder: "0.7",
            }
            label {
                r#for: "top-p",
                class: "mb-2 mt-4 block px-2 text-sm font-medium",
                "Top P"
            }
            input {
                r#type: "number",
                id: "top-p",
                class: "block w-full rounded-lg bg-slate-200 p-2.5 text-xs focus:outline-none focus:ring-2 focus:ring-blue-600 dark:bg-slate-800 dark:placeholder-slate-400 dark:focus:ring-blue-600",
                placeholder: "1",
            }
        }
    }
}