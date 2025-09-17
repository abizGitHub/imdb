use crate::models::messages::*;

pub fn process_echo(req: &EchoRequest) -> EchoResponse {
    let echoed_lines = req
        .message
        .lines
        .iter()
        .map(|line| format!("!{}!: {}", req.from, line))
        .collect();

    EchoResponse {
        from: req.from.clone(),
        echoed_lines,
        message_id: req.message.id,
    }
}
