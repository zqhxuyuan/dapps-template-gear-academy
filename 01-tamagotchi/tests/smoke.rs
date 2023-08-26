use tmg1_io::*;

use gtest::{Log, Program, System};
use gmeta::{In, InOut, Metadata};
use gstd::debug;

#[test]
fn smoke_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);

    // TODO: 8️⃣ Test the program initialization and message handling
    let res = program.send(2, String::from("zqhxuyuan"));
    assert!(!res.main_failed());

    let expected_log = Log::builder()
        .dest(2)
        .payload(String::from("zqhxuyuan"));
    assert!(res.contains(&expected_log));

    let res = program.send(
        2,
        TmgAction::Name,
    );
    let expected_log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Name(String::from("zqhxuyuan1")));
    // TODO: change the name why still success?
    assert!(res.contains(&expected_log));

    // let res = program.send(
    //     2,
    //     TmgAction::Age,
    // );
    // let expected_log = Log::builder()
    //     .dest(2)
    //     .payload(TmgEvent::Age(1));
    // assert!(res.contains(&expected_log));

    let mut state: <ProgramMetadata as Metadata>::State = program.read_state().unwrap();
    // debug!("Read state of Tamagotchi name:{:?} age:{:?}", state.name, state.date_of_birth);
    assert_eq!(state.name, String::from("zqhxuyuan"));
}
