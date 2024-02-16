use std::{thread::sleep, time::Duration};

use chrono::prelude::*;
use yew::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub struct Properties {
    #[prop_or(0)]
    pub hour: usize,
    #[prop_or(0)]
    pub minute: usize,
    #[prop_or(0)]
    pub second: usize,
}

#[function_component]
fn App(props: &Properties) -> Html {
    let hour_positions: [usize; 12] = [3, 4, 5, 6, 12, 13, 14, 15, 21, 22, 23, 24];
    let minute_positions: [usize; 11] = [0, 1, 2, 8, 9, 10, 17, 18, 19, 25, 26];
    let special_positions: [usize; 4] = [7, 11, 16, 20];

    let hour: usize = props.hour.to_owned();
    let minute: usize = props.minute.to_owned();
    let second: usize = props.second.to_owned();
    let mut clock: [char; 28] = ['○'; 28];
    if hour / 12 == 1 && second % 2 == 0 {
        clock[27] = '●';
    } else if second % 2 == 1 {
        clock[27] = '◉';
    }
    if hour != 0 {
        clock[hour_positions[hour % 12 - 1]] = '●';
    }
    if minute % 12 != 0 {
        clock[minute_positions[minute % 12 - 1]] = '●';
    }
    if minute / 12 != 0 {
        clock[special_positions[minute / 12 - 1]] = '●';
    }

    html! {<>
    <div style="display: grid; justify-content: center; font-size: xx-large; margin: auto; width: 50%;">
        <span>{ format!("{}:{}:{}",hour, minute, second) }</span>
        <div style="margin: auto;">
        <span>{ clock[0] }</span>
        <span>{ clock[1] }</span>
        <span>{ clock[2] }</span>
        </div>
        <div style="margin: auto;">
        <span>{ clock[3] }</span>
        <span>{ clock[4] }</span>
        <span>{ clock[5] }</span>
        <span>{ clock[6] }</span>
        </div>
        <div style="margin: auto;">
        <span>{ clock[7] }</span>
        <span>{ clock[8] }</span>
        <span>{ clock[9] }</span>
        <span>{ clock[10] }</span>
        <span>{ clock[11] }</span>
        </div>
        <div style="margin: auto;">
        <span>{ clock[12] }</span>
        <span>{ clock[13] }</span>
        <span>{ clock[14] }</span>
        <span>{ clock[15] }</span>
        </div>
        <div style="margin: auto;">
        <span>{ clock[16] }</span>
        <span>{ clock[17] }</span>
        <span>{ clock[18] }</span>
        <span>{ clock[19] }</span>
        <span>{ clock[20] }</span>
        </div>
        <div style="margin: auto;">
        <span>{ clock[21] }</span>
        <span>{ clock[22] }</span>
        <span>{ clock[23] }</span>
        <span>{ clock[24] }</span>
        </div>
        <div style="margin: auto;">
        <span>{ clock[25] }</span>
        <span>{ clock[26] }</span>
        <span>{ clock[27] }</span>
        </div>
        </div>
        <p style="display: grid; justify-content: center;">{ "Web Assembly application: Realtime updates not implemented yet. Refresh page to update clock" }</p>
        </>}
}

fn main() {
    let mut clock = yew::Renderer::<App>::new().render();
    let mut date_time: DateTime<Local> = Local::now();
    let mut properties: Properties = Properties{hour: date_time.hour() as usize, minute: date_time.minute() as usize, second: date_time.second() as usize};
    loop {
        date_time = Local::now();
        properties = Properties{hour: date_time.hour() as usize, minute: date_time.minute() as usize, second: date_time.second() as usize};
        clock.update(properties);
        sleep(Duration::from_secs(1));
    }
}
