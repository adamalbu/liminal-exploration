use crate::map::Room;

struct LevelNumber {
    main: u16,
    sub: Option<u16>,
}

impl LevelNumber {
    const fn new_main(main: u16) -> Self {
        Self { main, sub: None }
    }

    const fn new_sub(main: u16, sub: u16) -> Self {
        Self {
            main,
            sub: Some(sub),
        }
    }
}

pub struct Level<T: Room> {
    number: LevelNumber,
    name: String,
    room: T,
}

impl<T: Room> Level<T> {
    pub fn new_main(main_num: u16, name: String, room: T) -> Self {
        let number = LevelNumber::new_main(main_num);
        Self { number, name, room }
    }

    pub fn new_sub(main_num: u16, sub_num: u16, name: String, room: T) -> Self {
        let number = LevelNumber::new_sub(main_num, sub_num);
        Self { number, name, room }
    }
}
