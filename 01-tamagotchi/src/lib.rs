#![no_std]

use tmg1_io::*;
use gstd::{exec, msg, debug, prelude::*};

static mut NAME: Option<String> = None;
static mut AGE: Option<u64> = None;

#[no_mangle]
extern "C" fn init() {
    // TODO: 5️⃣ Initialize the Tamagotchi program
    let name: String = msg::load()
        .expect("Can't decode an init message");
    debug!("Program was initialized with message {:?}", name);
    let age: u64 = exec::block_timestamp();
    debug!("Get current timestamp {:?}", age);

    unsafe { NAME = Some(name) };
    unsafe { AGE = Some(age) };
    msg::reply(age.to_string(), 0)
            .expect("Unable to reply");
}

#[no_mangle]
extern "C" fn handle() {
    // TODO: 6️⃣ Add handling of `Name` and `Age` actions
    let input_message: TmgAction = msg::load()
        .expect("Error in loading InputMessages");
    let name = unsafe {
        NAME
            .as_mut()
            .expect("The contract is not initialized")
    };
    let age = unsafe {
        AGE
            .as_mut()
            .expect("The contract is not initialized")
    };
    match input_message {
        TmgAction::Name => {
            debug!("Program receive name message: {:?}", name);
            msg::reply(TmgEvent::Name(name.clone()), 0)
                .expect("Error in sending reply");
        }
        TmgAction::Age => {
            debug!("Program receive age message: {:?}", age);
            msg::reply(TmgEvent::Age(age.clone()), 0)
                .expect("Error in sending reply");
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    // TODO: 7️⃣ Return the Tamagotchi state
    let name = unsafe {
        NAME
            .as_ref()
            .expect("The contract is not initialized")
    };
    let age = unsafe {
        AGE
            .as_mut()
            .expect("The contract is not initialized")
    };
    let tamagotchi = Tamagotchi {
        name: name.clone(),
        date_of_birth: age.clone()
    };
    msg::reply(tamagotchi, 0).expect("Failed to share state");
}
