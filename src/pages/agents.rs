use dioxus::prelude::*;

pub fn Agents(cx: Scope) -> Element {
    render! {
        div {
            AgentGrid {}
        }
    }
}

pub fn AgentGrid(cx: Scope) -> Element {
    render! {
        div {
            class: "bg-slate-800 min-h-screen p-8",
            div {
                class: "container mx-auto",
                div {
                    class: "grid grid-cols-1 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-8",
                    (0..8).map(|i| rsx! {
                        AgentCard {
                            idx: i,
                            name: "Agent Name".to_string(),
                            description: "Agent Description".to_string(),
                            img_url: "https://dummyimage.com/128x128/354ea1/ffffff&text=A".to_string(),
                        }
                    })
                    AddAgentCard {
                        on_click: move |event| log::info!("clicked adding agent!")
                    }
                }
            }
        }
    }
}

#[derive(Props)]
pub struct AddAgentCardProps<'a> {
    pub on_click: EventHandler<'a, MouseEvent>,
}

pub fn AddAgentCard<'a>(cx: Scope<'a, AddAgentCardProps<'a>>) -> Element<'a> {
    render! {
        div {
            class: "flex justify-center items-center w-full h-full max-w-sm mx-auto rounded-3xl bg-white p-6 shadow-lg cursor-pointer \
            hover:text-blue-500 text-gray-400 ring-1 ring-slate-300 \
            dark:bg-slate-900 dark:text-slate-200 dark:ring-slate-300/20",
            onclick: move |event| cx.props.on_click.call(event),
            span {
                class: "text-6xl",
                "+"
            }
        }
    }
}

#[derive(Props, PartialEq, Debug)]
pub struct AgentCardProps {
    idx: u16,
    name: String,
    description: String,
    img_url: String,
}

pub fn AgentCard(cx: Scope<AgentCardProps>) -> Element {
    //
    render! {
        div {
            class: "flex w-full max-w-md flex-col rounded-3xl bg-slate-50 p-8 text-slate-900 ring-1 ring-slate-300 \
            dark:bg-slate-900 dark:text-slate-200 dark:ring-slate-300/20",
            div {
                class: "flex justify-between items-center",
                h3 {
                    class: "text-lg font-semibold leading-8",
                    "{cx.props.name}"
                }
                img {
                    class: "h-16 w-16 rounded-full object-cover",
                    src: cx.props.img_url.as_str(),
                    alt: cx.props.name.as_str(),
                }
            }
            div {
                class: "mt-4",
                p {
                    class: "text-sm leading-6 text-slate-700 dark:text-slate-400",
                    "{cx.props.description}"
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