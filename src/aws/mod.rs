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
    pub komi: f64, // "Main" komi setting, used to determine the game result at terminal nodes
    pub eval_komi: Option<f64>, // Komi used for heuristic evaluation. Default to the main komi, but not all komis are supported
    pub dirichlet_noise: Option<f32>,
    pub rollout_depth: u16,
    pub rollout_temperature: f64,
    pub action: Action,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Output {
    SuggestMoves(Vec<MoveInfo>),
    SuggestMove {
        pv: Vec<String>,
        score: f32,
        nodes: u32,
        mem_usage: u64,
        time_taken: time::Duration,
    },
}
