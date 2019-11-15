/*
* Copyright 2018-2019 TON DEV SOLUTIONS LTD.
*
* Licensed under the SOFTWARE EVALUATION License (the "License"); you may not use
* this file except in compliance with the License.  You may obtain a copy of the
* License at: https://ton.dev/licenses
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific TON DEV software governing permissions and
* limitations under the License.
*/

use dispatch::DispatchTable;

pub(crate) mod query;

pub(crate) fn register(handlers: &mut DispatchTable) {
    handlers.spawn("queries.query",
        query::query);
    handlers.spawn("queries.wait.for",
        query::wait_for);
    handlers.spawn("queries.subscribe",
        query::subscribe);
    handlers.spawn("queries.get.next",
        query::get_next);
    handlers.spawn("queries.unsubscribe",
        query::unsubscribe);
}
