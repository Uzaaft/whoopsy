use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cycle {
    pub id: i64,
    pub user_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub start: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<DateTime<Utc>>,
    pub timezone_offset: String,
    pub score_state: ScoreState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<CycleScore>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ScoreState {
    Scored,
    PendingScore,
    Unscorable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CycleScore {
    pub strain: f32,
    pub kilojoule: f32,
    pub average_heart_rate: i32,
    pub max_heart_rate: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedCycleResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<Cycle>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sleep {
    pub id: Uuid,
    pub cycle_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v1_id: Option<i64>,
    pub user_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub timezone_offset: String,
    pub nap: bool,
    pub score_state: ScoreState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<SleepScore>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepScore {
    pub stage_summary: SleepStageSummary,
    pub sleep_needed: SleepNeeded,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub respiratory_rate: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sleep_performance_percentage: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sleep_consistency_percentage: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sleep_efficiency_percentage: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepStageSummary {
    pub total_in_bed_time_milli: i32,
    pub total_awake_time_milli: i32,
    pub total_no_data_time_milli: i32,
    pub total_light_sleep_time_milli: i32,
    pub total_slow_wave_sleep_time_milli: i32,
    pub total_rem_sleep_time_milli: i32,
    pub sleep_cycle_count: i32,
    pub disturbance_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepNeeded {
    pub baseline_milli: i64,
    pub need_from_sleep_debt_milli: i64,
    pub need_from_recent_strain_milli: i64,
    pub need_from_recent_nap_milli: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedSleepResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<Sleep>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recovery {
    pub cycle_id: i64,
    pub sleep_id: Uuid,
    pub user_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub score_state: ScoreState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<RecoveryScore>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryScore {
    pub user_calibrating: bool,
    pub recovery_score: f32,
    pub resting_heart_rate: f32,
    pub hrv_rmssd_milli: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spo2_percentage: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin_temp_celsius: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryCollection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<Recovery>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBodyMeasurement {
    pub height_meter: f32,
    pub weight_kilogram: f32,
    pub max_heart_rate: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBasicProfile {
    pub user_id: i64,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutV2 {
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v1_id: Option<i64>,
    pub user_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub timezone_offset: String,
    pub sport_name: String,
    pub score_state: ScoreState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<WorkoutScore>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sport_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutScore {
    pub strain: f32,
    pub average_heart_rate: i32,
    pub max_heart_rate: i32,
    pub kilojoule: f32,
    pub percent_recorded: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_meter: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub altitude_gain_meter: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub altitude_change_meter: Option<f32>,
    pub zone_durations: ZoneDurations,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneDurations {
    pub zone_zero_milli: i64,
    pub zone_one_milli: i64,
    pub zone_two_milli: i64,
    pub zone_three_milli: i64,
    pub zone_four_milli: i64,
    pub zone_five_milli: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutCollection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<WorkoutV2>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CycleQueryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryQueryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepQueryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutQueryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}
