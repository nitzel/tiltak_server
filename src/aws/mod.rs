use std::time;

use serde::Deserialize;
use serde::Serialize;

use crate::search::{ TimeControl, Action };
use crate::search::MoveInfo;

#[cfg(feature = "aws-lambda-client")]
pub mod client;
#[cfg(feature = "aws-lambda-runtime")]
pub mod server;


#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AwsWrappedEvent {
    pub body: String // base64 encoded json of the request body
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Event {
    pub size: usize,
    pub tps: Option<String>,
    pub moves: Vec<String>,
    pub time_control: TimeControl,
    pub komi: f64,
    pub dirichlet_noise: Option<f32>,
    pub rollout_depth: u16,
    pub rollout_temperature: f64,
    pub action: Action,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Output {
    SuggestMoves {
        moves: Vec<MoveInfo>,
        position: String,
        komi: f32,
        game_result: Option<String>,
    },
    SuggestMove {
        pv: Vec<String>,
        score: f32,
        nodes: u64,
        mem_usage: u64,
        time_taken: time::Duration,
    },
}
