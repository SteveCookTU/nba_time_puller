use dioxus::core::to_owned;
use dioxus::prelude::*;
use futures::StreamExt;
use nba_time_puller::get_nba_times;
use nba_time_puller::timezone::Timezone;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let table_rows: &UseState<Option<Vec<String>>> = use_state(&cx, || None);
    let date = use_state(&cx, || "2022-05-19".to_string());
    let loading = use_state(&cx, || false);
    let timezone = use_state(&cx, || Timezone::EDT);

    let text_routine = use_coroutine(&cx, |mut rx: UnboundedReceiver<(String, Timezone)>| {
        to_owned![table_rows, loading];
        async move {
            while let Some((date, timezone)) = rx.next().await {
                table_rows.set(Some(get_nba_times(&date, timezone).await));
                loading.set(false);
            }
        }
    });

    let table = if let Some(rows) = table_rows.get() {
        let rows = rows.iter().map(|row| {
            let mut row = row.split(',');
            let title = row.next().unwrap();
            let date = row.next().unwrap();
            let start = row.next().unwrap();
            let end = row.next().unwrap();
            rsx! {
                tr {
                    td {
                        "{title}"
                    }
                    td {
                        class: "center",
                        "{date}"
                    }
                    td {
                        class: "center",
                        "{start}"
                    }
                    td {
                        class: "center",
                        "{end}"
                    }
                }
            }
        });
        rsx! {
            style { ["td { padding: 3px 10px;} .center { text-align: center; } th { padding: 0px 15px; }"] }
            table {
                tr {
                    th {
                        "Game"
                    }
                    th {
                        "Date"
                    }
                    th {
                        "Converted Start Time"
                    }
                    th {
                        "Converted End Time"
                    }
                }
                rows
            }
        }
    } else {
        rsx! {
            div {
                "Search for results"
            }
        }
    };

    cx.render(rsx! {
        button {
            style: "margin-right: 5px;",
            onclick: move |_| {
                loading.set(true);
                text_routine.send((date.get().to_string(), *timezone.get()))
            },
            disabled: "{loading}",
            "Load Data"
        }
        input {
            style: "margin-right: 5px;",
            r#type: "date",
            value: "{date}",
            oninput: move |evt| date.set(evt.value.clone())
        }
        p {
            "Convert to Timezone:",
            select {
                style: "margin-left: 5px;",
                onchange: move |evt| {
                    let i = evt.value.parse::<i8>().unwrap();
                    timezone.set(Timezone::try_from(i).unwrap());
                },
                option {
                    value: "-4",
                    "EDT"
                }
                option {
                    value: "-5",
                    "CDT"
                }
                option {
                    value: "-6",
                    "MDT"
                }
                option {
                    value: "-7",
                    "PDT"
                }
            }
        }
        table
    })
}
