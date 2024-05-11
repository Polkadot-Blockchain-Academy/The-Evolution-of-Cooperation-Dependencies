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

#[macro_export]
macro_rules! submit_strategy {
    ($strategy:expr, $participant_type:ident, $participant_name:literal, $participant_pub_name:literal) => {
            pub fn provide_strategy() -> OwnedStrategy {
                if $participant_type == ParticipantType::System {
                    compile_error!("ParticipantType::System isn't acceptable. Use Remote or Onsite.");
                }
                OwnedStrategy::new(
                    Participant::new($participant_type, $participant_name, $participant_pub_name),
                    Rc::new(RefCell::new(Box::new($strategy))),
                )
            }
        }
}
