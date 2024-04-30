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

pub use core::fmt::Debug;
pub use std::collections::VecDeque;

use urandom::Random;
use urandom::rng::Xoshiro256;

use crate::Move;
use crate::Move::{X, Y, Z};

/// Something that has a limited memory based on a `VecDeque`
pub trait Memory<T: Copy + Debug> {
    fn get_memory(&mut self) -> &mut VecDeque<T>;

    /// Remember some `T`. If the current memory is equal to the capacity, the oldest entry will drop.
    fn remember(&mut self, data: T) {
        if self.get_memory().capacity() == self.get_memory().len() {
            self.get_memory().pop_front();
        }
        self.get_memory().push_back(data);
    }

    /// Returns the last remembered `T`
    fn last(&mut self) -> Option<T> {
        self.get_memory().back().copied()
    }

    /// Returns the `n` latest remembered `T`
    fn last_n(&mut self, n: usize) -> Option<T> {
        self.get_memory().get(n).copied()
    }
}

pub struct RandomBoolean {
    random: Random<Xoshiro256>,
    probability: f32,
}

impl RandomBoolean {
    pub fn new(probability: f32) -> RandomBoolean {
        assert!(
            (0.0..=1.0).contains(&probability),
            "Probability must be between 0.0 and 1.0"
        );
        RandomBoolean {
            random: Xoshiro256::new(),
            probability,
        }
    }

    pub fn get(&mut self) -> bool {
        let random_value: f32 = self.random.range(0f32..1f32);
        random_value < self.probability
    }
}

pub struct RandomMove {
    random: Random<Xoshiro256>,
    prob_x: f32,
    prob_y: f32,
}

impl RandomMove {
    /// Create a new `RandomMove` with the given probabilities for X and Y. Z would be inferred as the remainder probability.
    /// Combined probability of X and Y cannot exceed 1.0
    pub fn new(prob_x: f32, prob_y: f32) -> RandomMove {
        assert!(
            (0.0..=1.0).contains(&prob_x),
            "Probability of X must be between 0.0 and 1.0"
        );
        assert!(
            (0.0..=1.0).contains(&prob_y),
            "Probability of Y must be between 0.0 and 1.0"
        );
        assert!(
            prob_x + prob_y <= 1.0,
            "Combined probability of X and Y cannot exceed 1.0"
        );

        RandomMove {
            random: Xoshiro256::new(),
            prob_x,
            prob_y,
        }
    }

    pub fn get(&mut self) -> Move {
        let random_value: f32 = self.random.range(0f32..1f32);

        if random_value < self.prob_x {
            X
        } else if random_value < (self.prob_x + self.prob_y) {
            Y
        } else {
            Z
        }
    }
}

impl Default for RandomMove {
    fn default() -> Self {
        let third = 1f32 / 3f32;
        RandomMove::new(third, third)
    }
}