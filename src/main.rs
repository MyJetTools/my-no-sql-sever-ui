use crate::{states::*, ui_command::UiCommand};
use dioxus::prelude::*;
use elements::*;
use futures::StreamExt;
mod actions;
mod elements;
mod states;
mod ui_command;
fn main() {
    dioxus_desktop::launch_cfg(
        app,
        dioxus_desktop::Config::new().with_custom_head(
            r#"
            <style>
            body{
                margin: 0;
                padding: 0;
                font-family: 'Tahoma', sans-serif;
            }

            #main{
                overflow: hidden;
            }

            .btn-menu{
                display: block;
                width:100%;
            }

            .table-item{
                padding:5px;
                color: gray;
                cursor: pointer;
            }

            .selected{
              color: black !important;
              border-radius: 5px;
              background-color: lightgray;
              box-shadow: 0 0 3px lightgray;
            }

            </style>"#
                .to_string(),
        ),
    );
}

fn app(cx: Scope) -> Element {
    let input_style = r"height: 100vh; text-align: center;";

    use_shared_state_provider(cx, || Tables::new());

    use_shared_state_provider(cx, || SelectedTable::new());

    use_shared_state_provider(cx, || RightPanelState::new());

    actions::get_list_of_tables(&cx);

    let selected_table = use_shared_state::<SelectedTable>(cx).unwrap();
    let selected_table_owned = selected_table.to_owned();

    let right_panel_state = use_shared_state::<RightPanelState>(cx).unwrap();
    let right_panel_state_owned = right_panel_state.to_owned();

    let main_routine = use_coroutine(cx, |mut rx: UnboundedReceiver<UiCommand>| async move {
        while let Some(msg) = rx.next().await {
            match msg {
                UiCommand::LoadPartitions(table_name) => {
                    selected_table_owned
                        .write()
                        .set_selected_table(table_name.clone());
                    let result = actions::get_list_of_partitions(table_name.as_str()).await;

                    let mut right_panel_state = right_panel_state_owned.write();
                    *right_panel_state = RightPanelState::LoadedPartitions(LoadedPartitions {
                        table_name: table_name,
                        partitions: result.data,
                        amount: result.amount,
                    });
                }

                UiCommand::LoadRows(partition_key) => {
                    let selected_table = {
                        let selected_table = selected_table_owned.read();
                        selected_table.get_selected_table().unwrap().clone()
                    };
                    let rows = actions::get_list_of_rows(&selected_table, &partition_key).await;

                    let mut right_panel_state = right_panel_state_owned.write();
                    right_panel_state.promote_to_loaded_rows(partition_key, rows);
                }
            }
        }
    });

    render!(
        div { style: "{input_style}",
            table { style: "height: 100vh; width:100%;",
                tr {
                    td { style: "width: 20%; height:100vh;  vertical-align: top; text-align: left;",
                        div { style: " height: 100vh;  overflow-y: auto;",
                            left_panel {
                                on_table_selected: |selected_table| {
                                    main_routine.send(UiCommand::LoadPartitions(selected_table));
                                }
                            }
                        }
                    }

                    td { style: "width: 80%; height:100vh; vertical-align: top; text-align: left;",
                        div { style: " height: 100vh;  overflow-y: auto;",
                            right_part {
                                on_partition_select: |partition_key| {
                                    main_routine.send(UiCommand::LoadRows(partition_key));
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}
