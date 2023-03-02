mod base;
mod evaluation;
mod interpreter;
mod node;

#[cfg(feature = "bincode")]
mod serialization;

#[cfg(test)]
mod tests;

use crate::action_tree::*;
use crate::card::*;
use crate::mutex_like::*;
use std::collections::BTreeMap;

#[cfg(feature = "bincode")]
use bincode::{Decode, Encode};

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "bincode", derive(Decode, Encode))]
enum State {
    ConfigError = 0,
    #[default]
    Uninitialized = 1,
    TreeBuilt = 2,
    MemoryAllocated = 3,
    Solved = 4,
}

/// A struct representing a postflop game.
#[derive(Default)]
pub struct PostFlopGame {
    // state
    state: State,

    // postflop game configurations
    card_config: CardConfig,
    tree_config: TreeConfig,
    added_lines: Vec<Vec<Action>>,
    removed_lines: Vec<Vec<Action>>,
    action_root: Box<MutexLike<ActionTreeNode>>,

    // computed from configurations
    num_combinations: f64,
    initial_weights: [Vec<f32>; 2],
    private_cards: [Vec<(u8, u8)>; 2],
    same_hand_index: [Vec<u16>; 2],
    valid_indices_flop: [Vec<u16>; 2],
    valid_indices_turn: Vec<[Vec<u16>; 2]>,
    valid_indices_river: Vec<[Vec<u16>; 2]>,
    hand_strength: Vec<[Vec<StrengthItem>; 2]>,
    turn_isomorphism_ref: Vec<u8>,
    turn_isomorphism_card: Vec<u8>,
    turn_isomorphism_swap: [SwapList; 4],
    river_isomorphism_ref: Vec<Vec<u8>>,
    river_isomorphism_card: Vec<Vec<u8>>,
    river_isomorphism_swap: [[SwapList; 4]; 4],

    // store options
    storage_mode: BoardState,
    target_storage_mode: BoardState,
    num_nodes: [u64; 3],
    is_compression_enabled: bool,
    num_storage: u64,
    num_storage_ip: u64,
    num_storage_chance: u64,
    misc_memory_usage: u64,

    // global storage
    node_arena: Vec<MutexLike<PostFlopNode>>,
    storage1: Vec<u8>,
    storage2: Vec<u8>,
    storage_ip: Vec<u8>,
    storage_chance: Vec<u8>,
    locking_strategy: BTreeMap<usize, Vec<f32>>,

    // result interpreter
    history: Vec<usize>,
    is_normalized_weight_cached: bool,
    current_node_index: usize,
    turn: u8,
    river: u8,
    chance_factor: i32,
    turn_swapped_suit: Option<(u8, u8)>,
    turn_swap: Option<u8>,
    river_swap: Option<(u8, u8)>,
    total_bet_amount: [i32; 2],
    prev_bet_amount: i32,
    weights: [Vec<f32>; 2],
    normalized_weights: [Vec<f32>; 2],
    cfvalues_cache: [Vec<f32>; 2],
}

/// A struct representing a node in a postflop game tree.
///
/// The nodes must be stored as `Vec<MutexLike<PostFlopNode>>`.
#[derive(Debug, Clone, Copy)]
pub struct PostFlopNode {
    prev_action: Action,
    player: u8,
    turn: u8,
    river: u8,
    is_locked: bool,
    amount: i32,
    children_offset: u32,
    num_children: u16,
    num_elements_ip: u16,
    storage1: *mut u8, // strategy
    storage2: *mut u8, // regrets or cfvalues
    storage3: *mut u8, // IP cfvalues
    num_elements: u32,
    scale1: f32,
    scale2: f32,
    scale3: f32,
}

unsafe impl Send for PostFlopGame {}
unsafe impl Sync for PostFlopGame {}

unsafe impl Send for PostFlopNode {}
unsafe impl Sync for PostFlopNode {}
