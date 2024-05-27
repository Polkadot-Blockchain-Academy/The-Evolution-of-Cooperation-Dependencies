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

use crate::{Move, Participant, Round, Strategy};
#[macro_export]
macro_rules! submit_strategy {
    ($strategy:expr, $participant_type:ident, $participant_name:literal, $participant_pub_name:literal) => {
        pub fn provide_strategy() -> (Participant, impl Fn() -> Box<dyn Strategy>) {
            (
                Participant::new($participant_type, $participant_name, $participant_pub_name),
                move || Box::new($strategy),
            )
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            use std::time::{Duration, Instant};

            #[test]
            fn test_participant_type() {
                assert_ne!(
                    ParticipantType::System,
                    $participant_type,
                    "participant type should not be System"
                );
            }

            #[test]
            fn test_strategy_time() {
                let max_move_time = Duration::from_millis(100);
                let max_handle_round_time = Duration::from_millis(100);
                let strategy = OwnedStrategy::new(
                    Participant::new($participant_type, $participant_name, $participant_pub_name),
                    Rc::new(RefCell::new(Box::new($strategy))),
                );

                let start_time = Instant::now();
                strategy.strategy.borrow_mut().play_for_favoured_move(X);
                let elapsed = start_time.elapsed();
                assert!(
                    elapsed < max_move_time,
                    "play_for_favoured_move exceeded timeout. elapsed:{:?}, max:{:?}",
                    elapsed,
                    max_move_time
                );

                let mut rounds = vec![];
                for m1 in vec![X, Y, Z] {
                    for m2 in vec![X, Y, Z] {
                        rounds.push(Round::of(m1, m2));
                    }
                }
                for round in rounds {
                    let start_time = Instant::now();
                    strategy.strategy.borrow_mut().handle_last_round(round, X);
                    let elapsed = start_time.elapsed();
                    assert!(
                        elapsed < max_handle_round_time,
                        "handle_last_round exceeded timeout. elapsed:{:?}, max:{:?}",
                        elapsed,
                        max_handle_round_time
                    );
                }
            }
        }
    };
}
