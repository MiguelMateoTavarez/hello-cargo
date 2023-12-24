fn main() {
    //*Enums: can be out of main
    #[warn(dead_code)]
    enum WebEvent {
        WELoad,
        WEKeys(String, char),
        WEClick { x: i64, y: i64 },
    }

    #[derive(Debug)]
    struct KeyPress(String, char);

    #[derive(Debug)]
    struct MouseClick {
        x: i64,
        y: i64,
    }

    #[derive(Debug)]
    enum WebEvent {
        WELoad(bool),
        WEClick(MouseClick),
        WEKeys(KeyPress),
    }

    let click: MouseClick = MouseClick { x: 100, y: 250 };

    println!("Mouse click location: {}, {}", click.x, click.y);

    let keys: KeyPress = KeyPress(String::from("Ctrl+"), 'N');

    let we_load: WebEvent = WebEvent::WELoad(true);
    let we_click: WebEvent = WebEvent::WEClick(click);
    let we_key: WebEvent = WebEvent::WEKeys(keys);

    println!(
        "\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}",
        we_load, we_click, we_key
    );
}
