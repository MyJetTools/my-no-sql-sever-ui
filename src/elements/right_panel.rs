use dioxus::prelude::*;

use crate::states::RightPanelState;

#[derive(Props)]
pub struct RightPanelModel<'a> {
    pub on_partition_select: EventHandler<'a, String>,
}

pub fn right_part<'s>(cx: Scope<'s, RightPanelModel<'s>>) -> Element<'s> {
    let right_panel_state = use_shared_state::<RightPanelState>(cx).unwrap();

    match right_panel_state.read().as_ref() {
        RightPanelState::Nothing => {
            render! { div {} }
        }
        RightPanelState::Loading => {
            render! { div { style: "padding:5px", "Loading..." } }
        }
        RightPanelState::LoadedPartitions(partitions) => {
            render! {
                partitions.partitions.iter().map(|partition_key| {

                        let partition_key = partition_key.to_string();

                        rsx! {
                            div { style: "padding:5px; border: 1px solid gray; border-radius: 3px; margin:2px; display: inline-block;  cursor:pointer",
                            onclick: move|_|{
                                cx.props.on_partition_select.call(partition_key.clone());
                            },
                             "{partition_key}"
                            }
                        }
                    })
            }
        }
        RightPanelState::NoPartitions(table_name) => {
            render! {
                div { style: "padding:5px", format!("No partitions found for table '{table_name}'") }
            }
        }
        RightPanelState::LoadedRows(rows) => {
            let headers = rows.get_list_of_headers();
            let amount = rows.get_amount();
            render! {
                div { style: "overflow-x:scroll",
                    select {
                        onchange: move |evn| {
                            cx.props.on_partition_select.call(evn.data.value.to_string());
                        },
                        rows.partitions.iter().map(|itm|{
                        rsx!{
                            option {
                                value: "{itm}",
                                selected: itm == &rows.partition_key, "{itm}",

                            }
                        }
                   })
                    }
                    table { style: "width:auto; font-size:10px;-webkit-border-vertical-spacing:0;-webkit-border-horizontal-spacing:0",
                        tr {
                            headers.iter().map(|header| {
                        rsx! {
                            th {style:" border: 1px solid;", "{header}"}
                        }
                    })
                        }
                        (0..amount).map(|no|{
                        let values = rows.get_values(no, &headers);
                        rsx!{
                            tr{

                                values.iter().map(|value|{
                                    match value{
                                        Some(value)=>{
                                            rsx!{
                                                td {style:" border: 1px solid;",
                                                div{ style:"width:200px; height:100px; overflow-y:auto; overflow-wrap:anywhere","{value}"}
                                            }
                                            }

                                        },
                                        None=>{
                                            rsx!{
                                                td {style:" border: 1px solid; width:100px",}
                                            }
                                        }
                                    }
                                })

                            }
                        }
                    })
                    }
                }
            }
        }
    }
}
