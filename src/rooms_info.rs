pub fn is_room_casino(current_room: &str) -> bool {
    [
        "rm_casinoWW",
        "rm_casino_entranceHall",
        "rm_casinoEW",
        "rm_casino_toHell",
        "rm_casino_hellsWaitingRoom",
        "rm_casinoTopFloor",
    ]
    .contains(&current_room)
}

pub fn is_room_restricted_to_split(current_room: &str) -> bool {
    [
        "rm_shopMenu",
        "rm_skinSelect",
        "rm_musicPlayer",
        "rm_peanutPark_00",
        "rm_nina",
        "rm_casino_brazil",
    ]
    .contains(&current_room)
}
