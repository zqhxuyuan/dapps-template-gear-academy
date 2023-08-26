#![no_std]

use codec::{Decode, Encode};
use gmeta::{In, InOut, Metadata};
use gstd::prelude::*;
use scale_info::TypeInfo;

#[derive(Default, Encode, Decode, TypeInfo)]
pub struct Tamagotchi {
    // TODO: 1️⃣ Add `name` and `age` fields
    pub name: String,
    pub date_of_birth: u64,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum TmgAction {
    // TODO: 2️⃣ Add `Name` and `Age` actions that set the name and age
    Name,
    Age,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum TmgEvent {
    // TODO: 3️⃣ Add `Name` and `Age` events that return the name and age
    Name(String),
    Age(u64),
}

pub struct ProgramMetadata;

// TODO: 4️⃣ Fill `Init`, `Handle`, and `State` types
impl Metadata for ProgramMetadata {
    type Init = In<String>;
    type Handle = InOut<TmgAction, TmgEvent>;
    type State = Tamagotchi;
    type Reply = ();
    type Others = ();
    type Signal = ();
}
