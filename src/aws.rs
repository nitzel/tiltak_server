use crate::board::{Board, Move};
use crate::search;
use board_game_traits::board::Board as EvalBoard;
use bytes::Bytes;
use lambda_runtime::error::HandlerError;
use lambda_runtime::Context;
use log::{debug, error, warn};
use rusoto_core::Region;
use rusoto_lambda::{InvocationRequest, Lambda, LambdaClient};
use serde::Deserialize;
use serde::Serialize;
use std::io;
use std::time::Duration;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Event {
    pub size: usize,
    pub moves: Vec<Move>,
    pub time_left: Duration,
    pub increment: Duration,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Output {
    pub best_move: Move,
    pub score: f32,
}

/// AWS serverside handler
pub fn handle_aws_event(e: Event, c: Context) -> Result<Output, HandlerError> {
    match e.size {
        4 => handle_aws_event_generic::<4>(e, c),
        5 => handle_aws_event_generic::<5>(e, c),
        s => panic!("Unsupported board size {}", s),
    }
}

pub fn handle_aws_event_generic<const S: usize>(
    e: Event,
    _c: Context,
) -> Result<Output, HandlerError> {
    let mut board = <Board<S>>::default();
    for mv in e.moves {
        board.do_move(mv);
    }

    let max_time = Duration::min(e.time_left / 40 + e.increment, Duration::from_secs(30));

    let (best_move, score) = search::play_move_time(board, max_time);

    Ok(Output { best_move, score })
}

/// Clientside function for receiving moves from AWS
pub fn best_move_aws(aws_function_name: &str, payload: &Event) -> io::Result<Output> {
    let client = LambdaClient::new(Region::UsEast2);

    let request = InvocationRequest {
        client_context: None,
        function_name: aws_function_name.to_string(),
        invocation_type: Some("RequestResponse".to_string()),
        log_type: None,
        payload: Some(Bytes::copy_from_slice(
            &serde_json::to_vec(payload).unwrap(),
        )),
        qualifier: None,
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async { client.invoke(request).await });
    match result {
        Ok(response) => {
            if let Some(status_code) = response.status_code {
                if status_code / 100 == 2 {
                    debug!("Got HTTP response {} from aws", status_code);
                } else {
                    error!("Got HTTP response {} from aws", status_code);
                }
            } else {
                warn!("AWS response contained no status code");
            }
            if let Some(payload) = response.payload {
                Ok(serde_json::from_str(
                    std::str::from_utf8(&payload)
                        .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?,
                )?)
            } else {
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "AWS response contained no payload",
                ))
            }
        }
        Err(err) => Err(io::Error::new(io::ErrorKind::Other, err)),
    }
}
