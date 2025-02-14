use gloo::timers::callback::Interval;
use stylist::yew::Global;
use yew::{html, Component, Context, Html};
use yew::prelude::*;

pub enum Msg {
    StartTime,
    StopTime,
    CountTime,
    ResetTime,
    UpdateTime,
}

pub struct MyTimer {
    timer_csec: u32,
    last_timer_csec: u32,
    timer_handle: Option<Interval>,
    message: Vec<(u32, u32, u32)>,
    state: u8,
}

impl MyTimer {
    fn new(_ctx: &Context<Self>) -> Self {
        Self {
            timer_csec: 0,
            last_timer_csec: 0,
            timer_handle: None,
            message: vec![],
            state: 0,
        }
    }
    fn start_time(&mut self, ctx: &Context<Self>) -> bool {
        if self.state == 0 || self.state == 2 {
            let timer_handle = {
                let link = ctx.link().clone();
                Interval::new(10, move || link.send_message(Msg::UpdateTime))
            };
            self.timer_handle = Some(timer_handle);
            self.state = 1;
        }
        true
    }
    fn stop_time(&mut self) -> bool {
        if self.state == 1 {
            self.timer_handle = None;
            self.state = 2;
        }
        true
    }
    fn conut_time(&mut self) -> bool {
        let mut l = 1;
        if let Some(t) = self.message.first() {
            l = t.0 + 1;
        }
        if self.state == 1 {
            self.message.insert(
                0,
                (l, self.timer_csec - self.last_timer_csec, self.timer_csec),
            );
            if self.message.len() > 10 {
                self.message.pop();
            }
            self.last_timer_csec = self.timer_csec;
        }
        true
    }
    fn reset_time(&mut self) -> bool {
        if self.state == 2 {
            self.timer_csec = 0;
            self.last_timer_csec = 0;
            self.message.clear();
            self.state = 0;
        }
        true
    }
    fn tick(&mut self) -> bool {
        self.timer_csec += 1;
        true
    }
}

impl Component for MyTimer {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        MyTimer::new(ctx)
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StartTime => self.start_time(ctx),
            Msg::StopTime => self.stop_time(),
            Msg::CountTime => self.conut_time(),
            Msg::ResetTime => self.reset_time(),
            Msg::UpdateTime => self.tick(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut button1 = html! {
        <button class="button1_1" onclick={ctx.link().callback(|_| Msg::CountTime)}>{"计次"}</button> };
        let mut button2 = html! {
            <button class="button1_2" onclick={ctx.link().callback(|_| Msg::StartTime)}>{"开始"}</button>
        };
        if self.state == 1 {
            button1 = html! {
                <button class="button2_1" onclick={ctx.link().callback(|_| Msg::CountTime)}>{"计次"}</button>
            };
            button2 = html! {
                <button class="button2_2" onclick={ctx.link().callback(|_| Msg::StopTime)}>{"暂停"}</button>
            };
        } else if self.state == 2 {
            button1 = html! {
                <button class="button2_1" onclick={ctx.link().callback(|_| Msg::ResetTime)}>{"重置"}</button>
            };
            button2 = html! {
                <button class="button1_2" onclick={ctx.link().callback(|_| Msg::StartTime)}>{"继续"}</button>
            };
        }
        let format_time_fun =
            |b: u32| format!("{:0>2}:{:0>2}.{:0>2}", b / 6000, (b % 6000) / 100, b % 100);

        html! {
            <>
                <Global css=r#"
                    html, body {
                        font-family: sans-serif;
                        padding: 0;
                        margin: 0;
                        display: flex;
                        justify-content: space-between;
                        align-items: center;
                        min-height: 100vh;
                        flex-direction: column;
                    }
                    .time {
                        margin-top: 100px;
                        font-size: 8em;
                    }
                    .buttons {
                        margin-bottom: 500px;
                        display: flex;
                        justify-content: center;
                    }
                    .buttons button {
                        height: 60px;
                        width: 120px;
                        margin: 50px;
                        font-size: 1.3em;
                        border-width: 0px;
                        border-radius: 30px;
                    }
                    .button1_1 {
                        color: #00000080;
                        background-color: #efefef80;
                        disabled: true;
                    }
                    .button1_2 {
                        color: #ffffff;
                        background-color: blue;
                    }
                    .button2_1 {
                        color: #000000;
                        background-color: #efefef;
                    }
                    .button2_2 {
                        color: #ffffff;
                        background-color: red;
                    }
                    .message {
                        color: grey;
                        height: 400px;
                    }
                    .message table {
                        text-align: center;
                        border-collapse: collapse;
                    }
                    .message thead {
                        display: block;
                        border-bottom-style: solid;
                        border-bottom-width: 1px;
                        border-bottom-color: grey;
                    }
                    .message tbody {
                        display: block;
                    }
                    .message th {
                        padding: 15px;
                        min-width: 60px;
                    }
                    .message td {
                        padding: 2px 15px;
                        min-width: 60px;
                    }
                "# />
                <div class="time">
                    { format_time_fun(self.timer_csec) }
                </div>
                <div class="message" >
                    if self.message.len()!=0 {
                        <table>
                            <thead>
                                <tr>
                                    <th>{"计次"}</th>
                                    <th>{"单圈时间"}</th>
                                    <th>{"总时间"}</th>
                                </tr>
                            </thead>
                            <tbody>
                            { for self.message.iter().map(|m| html!{
                                <tr>
                                    <td>{ format!("{:0>2}",m.0) }</td>
                                    <td>{ format_time_fun(m.1) }</td>
                                    <td>{ format_time_fun(m.2) }</td>
                                </tr>
                            })}
                            </tbody>
                        </table>
                    }
                </div>
                <div class="buttons">
                    { button1 }
                    { button2 }
                </div>
            </>
        }
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <MyTimer />
    }
}
fn main() {
    yew::start_app::<Main>();
}
