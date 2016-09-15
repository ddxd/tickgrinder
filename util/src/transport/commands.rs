use serde_json;
use uuid::Uuid;
#[allow(unused_imports)]
use test;

/// Represents a command sent to the Tick Processor
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Command {
    Ping,
    Restart,
    Shutdown,
    AddSMA{period: f64},
    RemoveSMA{period: f64},
}

/// Represents a command bound to a unique identifier that can be
/// used to link it with a Response
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct WrappedCommand {
    pub uuid: Uuid,
    pub cmd: Command
}

/// Converts a String into a WrappedCommand
/// JSON Format: {"uuid": "xxxx-xxxx", "cmd": {"CommandName":{"arg": "val"}}}
pub fn parse_wrapped_command(cmd: String) -> WrappedCommand {
    serde_json::from_str::<WrappedCommand>(cmd.as_str())
        .expect("Unable to parse WrappedCommand from String")
}

/// Represents a response from the Tick Processor to a Command sent
/// to it at some earlier point.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Response {
    Ok,
    Error{status: String},
    Pong
}

/// A Response bound to a UUID
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct WrappedResponse {
    pub uuid: Uuid,
    pub res: Response
}

/// Parses a String into a WrappedResponse
pub fn parse_wrapped_response(raw_res: String) -> WrappedResponse {
    serde_json::from_str::<WrappedResponse>(raw_res.as_str())
        .expect("Unable to parse WrappedResponse from String")
}

#[test]
fn command_serialization() {
    let cmd_str = "{\"AddSMA\": {\"period\": 6.64} }";
    let cmd: Command = serde_json::from_str(cmd_str).unwrap();
    assert_eq!(cmd, Command::AddSMA{period: 6.64f64});
}

#[test]
fn command_deserialization() {
    let cmd = Command::RemoveSMA{period: 6.64f64};
    let cmd_string = serde_json::to_string(&cmd).unwrap();
    assert_eq!("{\"RemoveSMA\":{\"period\":6.64}}", cmd_string.as_str());
}

#[test]
fn response_serialization() {
    let res_str = "\"Ok\"";
    let res: Response = serde_json::from_str(res_str).unwrap();
    assert_eq!(res, Response::Ok);
}

#[test]
fn response_deserialization() {
    let res = Response::Ok;
    let res_string = serde_json::to_string(&res).unwrap();
    assert_eq!("\"Ok\"", res_string.as_str());
}

#[bench]
fn wrappedcmd_to_string(b: &mut test::Bencher) {
    let cmd = Command::AddSMA{period: 42.23423f64};
    let wr_cmd = WrappedCommand{uuid: Uuid::new_v4(), cmd: cmd};
    b.iter(|| {
        let wr_cmd = &wr_cmd;
        let _ = serde_json::to_string(wr_cmd);
    })
}

#[bench]
fn string_to_wrappedcmd(b: &mut test::Bencher) {
    let raw = "{\"uuid\":\"2f663301-5b73-4fa0-b201-09ab196ec5fd\",\"cmd\":{\"RemoveSMA\":{\"period\":5.2342}}}";
    b.iter(|| {
        let raw = &raw;
        let _: WrappedCommand  = serde_json::from_str(raw).unwrap();
    })
}

#[bench]
fn uuid_generation(b: &mut test::Bencher) {
    b.iter(|| {
        Uuid::new_v4();
    })
}