#![no_std]

use asr::{
    future::next_tick, print_message, settings::Gui, string::ArrayCString, timer, watcher::Pair,
    Process,
};
use memory::{refresh_mem_values, ROOM_NAME_SIZE_CAP};
use rooms_info::{is_room_casino, is_room_restricted_to_split};

asr::async_main!(stable);
asr::panic_handler!();

mod memory;
mod rooms_info;
mod settings;

const MAIN_MODULE: &str = "ANTONBLAST.exe";

#[derive(Default)]
struct MemoryAddresses {
    main_address: Option<asr::Address>,
    room_id: Option<asr::Address>,
    room_names: Option<asr::Address>,
}

#[derive(Default)]
struct MemoryValues {
    room_id: Pair<i32>,
    room_name: Pair<ArrayCString<ROOM_NAME_SIZE_CAP>>,
}

async fn main() {
    let mut settings = settings::Settings::register();
    let mut mem_addresses = MemoryAddresses::default();
    let mut mem_values = MemoryValues::default();

    asr::print_message("Starting the ANTONBLAST autosplitter");

    loop {
        // check if settings GUI changes
        settings.update();

        let process = Process::wait_attach(MAIN_MODULE).await;
        mem_addresses.main_address = match process.get_module_address(MAIN_MODULE) {
            Ok(address) => Some(address),
            Err(_) => None,
        };

        process
            .until_closes(async {

                // init
                if let Ok(address) = memory::room_id_sigscan_start(&process, &mem_addresses) {
                    mem_addresses.room_id = Some(address);
                } else {
                    mem_addresses.room_id = None;
                }

                if mem_addresses.room_id.is_none() {
                    mem_addresses.room_id = Some(asr::Address::new(0x2795C90));
                    print_message("Room ID address not found with sigscan. Using hardcoded value for 1.1.2f...");
                }

                if mem_addresses.room_id.is_some() {
                    if let Ok(room_id_result) = process.read(mem_addresses.main_address.unwrap_or(asr::Address::default()).value() + mem_addresses.room_id.unwrap().value()) {
                        mem_values.room_id.current = room_id_result
                    } else {
                        mem_values.room_id.current = 0;
                        print_message("Could not read room ID before stall that waits for the game opening. Using 0");
                    }
                    if mem_values.room_id.current == 0 {
                        print_message("Waiting for the game to start...");
                    }
                    while mem_values.room_id.current == 0 {
                        if mem_values.room_id.current == 0 {
                            if let Ok(value) = process.read::<i32>(mem_addresses.main_address.unwrap_or(asr::Address::default()).value() + mem_addresses.room_id.unwrap().value()) {
                                mem_values.room_id.current = value;
                            } else {
                                break;
                            }
                        }
                    }
                }

                mem_addresses.room_names = match memory::room_name_array_sigscan_start(&process) {
                    Ok(address) => Some(address),
                Err(_) => None,
                };

                // ready for main loop
                if mem_addresses.room_names.is_some() {
                    loop {
                        settings.update();

                        if let Err(text) = refresh_mem_values(&process, &mem_addresses, &mut mem_values) {
                            print_message(text);
                            print_message("Exiting main loop and retrying...");
                            break;
                        }

                        let room_name_current = mem_values.room_name.current.validate_utf8().unwrap_or("(invalid utf8 string)");
                        let room_name_old = mem_values.room_name.old.validate_utf8().unwrap_or("(invalid utf8 string)");

                        // start
                        if settings.start_enable &&room_name_current == "rm_city_01_antonsHood" && room_name_old == "rm_city_01_antonsHood"  {                            
                            timer::start();
                        }

                        // split
                        if settings.splits_enable && (
                            (
                                // split on level exit
                                mem_values.room_name.changed() &&
                                is_room_casino(room_name_current) &&
                                !is_room_casino(room_name_old) &&
                                !is_room_restricted_to_split(room_name_old)
                            )
                            ||
                            // split on satan defeat
                            room_name_current == "rm_satanBeatdown" && room_name_old != "rm_satanBeatdown") {
                            timer::split();
                        }


                        // reset
                        if settings.reset_enable && room_name_current == "rm_characterSelect" && room_name_old != "rm_characterSelect" {
                            timer::reset();
                        }

                        next_tick().await;
                    }
                } else {
                    next_tick().await;
                }
            })
            .await;
    }
}
