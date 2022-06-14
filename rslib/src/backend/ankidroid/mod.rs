// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

pub(crate) mod db;
pub(crate) mod error;

use self::{db::active_sequences, error::debug_produce_error};
use super::{
    dbproxy::{db_command_bytes, db_command_proto},
    Backend,
};
pub(super) use crate::backend_proto::ankidroid_service::Service as AnkidroidService;
use crate::{
    backend::ankidroid::db::{execute_for_row_count, insert_for_id},
    backend_proto::{
        self as pb,
        ankidroid::{DbResponse, GetActiveSequenceNumbersResponse, GetNextResultPageRequest},
        generic::{self, Empty, Int32, Json},
        Int64, StringList,
    },
    prelude::*,
    scheduler::timing::{self, fixed_offset_from_minutes},
};

impl AnkidroidService for Backend {
    fn sched_timing_today_legacy(
        &self,
        input: pb::SchedTimingTodayLegacyRequest,
    ) -> Result<pb::SchedTimingTodayResponse> {
        let result = timing::sched_timing_today(
            TimestampSecs::from(input.created_secs),
            TimestampSecs::from(input.now_secs),
            Some(fixed_offset_from_minutes(input.created_mins_west)),
            fixed_offset_from_minutes(input.now_mins_west),
            Some(input.rollover_hour as u8),
        );
        Ok(pb::SchedTimingTodayResponse::from(result))
    }

    fn local_minutes_west_legacy(&self, input: pb::Int64) -> Result<pb::Int32> {
        Ok(pb::Int32 {
            val: timing::local_minutes_west_for_stamp(input.val),
        })
    }

    fn run_db_command(&self, input: Json) -> Result<Json> {
        self.with_col(|col| db_command_bytes(col, &input.json))
            .map(|json| Json { json })
    }

    fn run_db_command_proto(&self, input: Json) -> Result<DbResponse> {
        self.with_col(|col| db_command_proto(col, &input.json))
    }

    fn run_db_command_for_row_count(&self, input: Json) -> Result<Int64> {
        self.with_col(|col| execute_for_row_count(col, &input.json))
            .map(|val| Int64 { val })
    }

    fn flush_all_queries(&self, _input: Empty) -> Result<Empty> {
        self.with_col(|col| {
            db::flush_all(backend_id(col));
            Ok(Empty {})
        })
    }

    fn flush_query(&self, input: Int32) -> Result<Empty> {
        self.with_col(|col| {
            db::flush_cache(backend_id(col), input.val);
            Ok(Empty {})
        })
    }

    fn get_next_result_page(&self, input: GetNextResultPageRequest) -> Result<DbResponse> {
        self.with_col(|col| {
            let id = backend_id(col);
            db::get_next(id, input.sequence, input.index).ok_or(AnkiError::NotFound)
        })
    }

    fn insert_for_id(&self, input: Json) -> Result<Int64> {
        self.with_col(|col| insert_for_id(col, &input.json).map(Into::into))
    }

    fn set_page_size(&self, input: Int64) -> Result<Empty> {
        // we don't require an open collection, but should avoid modifying this
        // concurrently
        let _guard = self.col.lock();
        db::set_max_page_size(input.val as usize);
        Ok(().into())
    }

    fn get_column_names_from_query(&self, input: generic::String) -> Result<StringList> {
        self.with_col(|col| {
            let stmt = col.storage.db.prepare(&input.val)?;
            let names = stmt.column_names();
            let names: Vec<_> = names.iter().map(ToString::to_string).collect();
            Ok(names.into())
        })
    }

    fn get_active_sequence_numbers(
        &self,
        _input: Empty,
    ) -> Result<GetActiveSequenceNumbersResponse> {
        self.with_col(|col| {
            Ok(GetActiveSequenceNumbersResponse {
                numbers: active_sequences(backend_id(col)),
            })
        })
    }

    fn debug_produce_error(&self, input: generic::String) -> Result<Empty> {
        Err(debug_produce_error(&input.val))
    }
}

/// The old AnkiDroid code used the pointer to the backend as a cache index;
/// Now we use a pointer to SqliteStorage instead.
pub(crate) fn backend_id(col: &Collection) -> i64 {
    (&col.storage as *const _) as i64
}
