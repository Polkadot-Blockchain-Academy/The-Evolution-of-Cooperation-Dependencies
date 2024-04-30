/*
 * Copyright (C) 2024 Polkadot Blockchain Academy
 *  See the LICENSE.md file distributed with this work for additional
 *  information regarding copyright ownership.
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *      http://www.apache.org/licenses/LICENSE-2.0
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 */

pub mod submission_macro;
pub mod utils;

#[macro_use]
extern crate enum_display_derive;

use core::fmt::{Debug, Display};
use std::cell::RefCell;
use std::hash::*;
use std::rc::Rc;
pub use std::sync::{Arc, Mutex};

use serde::Serialize;
pub use urandom::rng::Xoshiro256;

use Move::Z;
pub use named::Named;

use crate::Move::{X, Y};

/// This is the trait that needs to be implemented and submitted
pub trait Strategy: Named + Sync {
    /// Determines the next move for the strategy, taking into account the strategy owner's favored move.
    ///
    /// # Arguments
    ///
    /// * `favoured_move` - A move that might be favored by the opponent or game conditions.
    ///
    /// # Returns
    ///
    /// The move that the strategy chooses to play.
    fn play_for_favoured_move(&mut self, favoured_move: Move) -> Move;

    /// Handles the last round of the game, taking into account the strategy owner's favored move.
    ///
    /// # Arguments
    ///
    /// * `round` - The last round of the game
    /// * `favoured_move` - The strategy owner's favored move
    fn handle_last_round(&mut self, round: Round, favoured_move: Move);
}

#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash, Named, Ord, PartialOrd, Serialize, Display)]
pub enum Move {
    X,
    Y,
    Z,
}

/// Return the opposite of a `Move` such that:
///
/// `X` -> `Y`
///
/// `Y` -> `X`
///
/// `Z` -> `Z`
///
pub trait Opposite {
    fn opposite(self) -> Self;
}

impl Opposite for Move {
    fn opposite(self) -> Self {
        match self {
            X => Y,
            Y => X,
            Z => Z,
        }
    }
}

/// The result of a round
#[derive(Clone, Copy, Debug)]
pub struct Round {
    /// The move that the participant made
    pub my_move: Move,
    /// The move that the opponent made
    pub opponent_move: Move,
}

impl Round {
    pub fn of(my_move: Move, opponent_move: Move) -> Self {
        Round {
            my_move,
            opponent_move,
        }
    }
}

pub type ParticipantName = &'static str;
pub type ParticipantPubName = &'static str;

/// Represents a participant in the game.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Participant {
    /// The type of the participant (e.g., System, Remote, Onsite).
    pub participant_type: ParticipantType,
    /// The internal, unique name of the participant.
    pub name: ParticipantName,
    /// The public-facing name of the participant.
    pub pub_name: ParticipantPubName,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Display, Eq, PartialEq, Hash)]
pub enum ParticipantType {
    System,
    Remote,
    Onsite,
}

impl Participant {
    /// Creates a new participant.
    pub fn new(
        participant_type: ParticipantType,
        name: ParticipantName,
        pub_name: ParticipantPubName,
    ) -> Self {
        Self {
            participant_type,
            name,
            pub_name,
        }
    }
}

impl Display for Participant {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let ParticipantType::System = self.participant_type {
            f.write_str(format!("{}", self.participant_type).as_str())
        } else {
            f.write_str(self.pub_name.to_string().as_str())
        }
    }
}

/// Struct for holding a strategy and its owner.
pub struct OwnedStrategy {
    pub owner: Participant,
    pub strategy: Rc<RefCell<Box<dyn Strategy>>>,
}

impl Debug for OwnedStrategy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("OwnedStrategy")
            .field("owner", &self.owner)
            .field("strategy", &self.strategy.borrow().name())
            .finish()
    }
}

impl Display for OwnedStrategy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(format!("{}: {}", self.owner.name, self.strategy.borrow().name()).as_str())
    }
}

impl PartialEq<Self> for OwnedStrategy {
    fn eq(&self, other: &Self) -> bool {
        self.owner.eq(&other.owner)
            && self
                .strategy
                .borrow()
                .name()
                .eq(other.strategy.borrow().name())
    }
}

impl Eq for OwnedStrategy {}
impl Hash for OwnedStrategy {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.owner.hash(state);
        self.strategy.borrow().name().hash(state);
    }
}

impl OwnedStrategy {
    pub fn new(owner: Participant, strategy: Rc<RefCell<Box<dyn Strategy>>>) -> Self {
        OwnedStrategy { owner, strategy }
    }

    /// Returns the an ID for the strategy
    pub fn id(&self) -> String {
        self.to_string()
    }
}

/// Something that has a name
pub trait Named {
    /// Return the name of `self`
    fn name(&self) -> &str;
}

