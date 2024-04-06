use dioxus::prelude::*;

pub fn Agents() -> Element {
    rsx! {
        div {
            AgentGrid {}
        }
    }
}

pub fn AgentGrid() -> Element {
    rsx! {
        div {
            class: "bg-slate-800 min-h-screen p-8",
            div {
                class: "container mx-auto",
                div {
                    class: "grid grid-cols-1 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-8",
                    {
                        (0..8).map(|i| rsx! {
                            AgentCard {
                                idx: i,
                                name: "Agent Name".to_string(),
                                description: "Agent Description".to_string(),
                                img_url: "https://dummyimage.com/128x128/354ea1/ffffff&text=A".to_string(),
                            }
                        })
                    }
                    AddAgentCard {
                        on_click: move |_| log::info!("clicked adding agent!")
                    }
                }
            }
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct AddAgentCardProps {
    pub on_click: EventHandler<MouseEvent>,
}

pub fn AddAgentCard(props: AddAgentCardProps) -> Element {
    rsx! {
        div {
            class: "flex justify-center items-center w-full h-full max-w-sm mx-auto rounded-3xl bg-white p-6 shadow-lg cursor-pointer \
            hover:text-blue-500 text-gray-400 ring-1 ring-slate-300 \
            dark:bg-slate-900 dark:text-slate-200 dark:ring-slate-300/20",
            onclick: move |event| props.on_click.call(event),
            span {
                class: "text-6xl",
                "+"
            }
        }
    }
}


#[component]
pub fn AgentCard(idx: u16, name: String, description: String, img_url: String) -> Element {
    //
    rsx! {
        div {
            class: "flex w-full max-w-md flex-col rounded-3xl bg-slate-50 p-8 text-slate-900 ring-1 ring-slate-300 \
            dark:bg-slate-900 dark:text-slate-200 dark:ring-slate-300/20",
            div {
                class: "flex justify-between items-center",
                h3 {
                    class: "text-lg font-semibold leading-8",
                    "{name}"
                }
                img {
                    class: "h-16 w-16 rounded-full object-cover",
                    src: img_url.as_str(),
                    alt: name.as_str(),
                }
            }
            div {
                class: "mt-4",
                p {
                    class: "text-sm leading-6 text-slate-700 dark:text-slate-400",
                    "{description}"
                }
            }
            button {
                class: "mt-4 rounded-md bg-blue-600 text-white px-4 py-2 text-sm font-semibold leading-5 \
                hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-600 focus:ring-opacity-50",
                // onclick: todo!(),
                "Edit"
            }
        }
    }
}