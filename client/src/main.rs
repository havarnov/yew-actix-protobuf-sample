#[macro_use]
extern crate yew;
extern crate protobuf;
#[macro_use]
extern crate failure;
extern crate protos;
#[macro_use]
extern crate stdweb;

use yew::format::Nothing;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

use protobuf::parse_from_bytes;

use failure::Error;

use protos::timestamp::TimestampResponse;

struct Context {
    fetch_service: FetchService,
}

struct Model {
    timestamp: Option<u64>,
    task: Option<FetchTask>,
}

enum Msg {
    Fetch,
    ResponseReady(Result<Option<u64>, Error>),
}

impl Component<Context> for Model {
    // Some details omitted. Explore the examples to get more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        Model {
            timestamp: None,
            task: None,
        }
    }

    fn update(&mut self, msg: Self::Message, env: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Fetch => {
                let callback = env.send_back(Msg::ResponseReady);
                let url = format!("http://localhost:8001/api/timestamp");
                let handler = move |response: Response<Result<Vec<u8>, Error>>| {
                    let (meta, data) = response.into_parts();
                    if meta.status.is_success() {
                        match data {
                            Ok(bytes) => match parse_from_bytes::<TimestampResponse>(&bytes) {
                                Ok(timestamp_response) => {
                                    if timestamp_response.has_timestamp() {
                                        callback.emit(Ok(Some(timestamp_response.get_timestamp())))
                                    } else {
                                        callback.emit(Err(format_err!(
                                            "server couldn't send timestamp."
                                        )))
                                    }
                                }
                                Err(_) => callback.emit(Err(format_err!(
                                    "error decoding bytes into TimestampResponse."
                                ))),
                            },
                            Err(_) => callback
                                .emit(Err(format_err!("couldn't read bytes from http response."))),
                        }
                    } else {
                        callback.emit(Err(format_err!(
                            "{}: error fetching timestamp.",
                            meta.status
                        )))
                    }
                };
                let request = Request::get(url.as_str())
                    .body(Nothing)
                    .expect("failed to create request.");
                let task = env.fetch_service.fetch_binary(request, handler.into());
                self.task = Some(task);
                true
            }
            Msg::ResponseReady(Ok(timestamp)) => {
                self.timestamp = timestamp;
                true
            }
            Msg::ResponseReady(Err(e)) => {
                js!{
                    console.error(@{format!("{}", e)})
                }

                false
            }
        }
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            // Render your model here
            <div>
            <h1>{ "time: " }{ if let Some(timestamp) = self.timestamp { format!("{}", timestamp) } else { format!("not fetched..") } }</h1>
            <button onclick=|_| Msg::Fetch,>{ "fetch timestamp" }</button>
            </div>
        }
    }
}

fn main() {
    yew::initialize();
    let app: App<_, Model> = App::new(Context {
        fetch_service: FetchService::new(),
    });
    app.mount_to_body();
    yew::run_loop();
}
