/*
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */
use crate::models::*;

pub fn analyze_stats(stats: &MailingListStats) -> AnalyzedStats {
    let total_emails: i32 = stats.total_emails;
    let total_participants: i32 = stats.total_participants as i32;
    let total_threads: i32 = stats.total_threads;
    let days = stats.active_months.len() as f64 * 30.44;

    AnalyzedStats {
        total_emails,
        total_participants,
        total_threads,
        avg_emails: total_emails as f64 / days,
        avg_participants: total_participants as f64 / days,
        avg_threads: total_threads as f64 / days,
        daily_stats: stats.active_months.clone(),
        list_info: ListInfo {
            list_name: stats.list_name.clone(),
            domain: stats.domain.clone(),
            period_from: stats.period_start.clone(),
            period_to: stats.period_end.clone(),
        },
    }
}
