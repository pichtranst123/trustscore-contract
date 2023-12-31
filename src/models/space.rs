use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  serde::{Deserialize, Serialize},
};
use schemars::JsonSchema;

use super::{thread::ThreadId, user::UserId};

/// `SpaceId` is a type alias for `String`, typically representing a unique identifier for a thread
/// in the system.
pub type SpaceId = String;

/// The `SpaceMetadata` struct represents metadata for a Space in the system.
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
pub struct SpaceMetadata {
  /// Unique identifier for the Space, of type `SpaceId`.
  pub space_id: SpaceId,

  /// Name of the thread.
  pub space_name: String,

  /// Creator's account ID.
  pub creator_id: UserId,

  /// Date when the thread was created, represented as a timestamp.
  pub created_at: u64,

  // List of user follow this Space
  pub followed_users: Vec<UserId>,

  // Total point of this space
  pub total_point : u64
}

pub trait SpaceFeatures {
  fn create_space(&mut self, space_name: String) -> SpaceMetadata;

  // TODO: add readme.md
  fn get_space_metadata_by_space_id(&self, space_id: SpaceId) -> Option<SpaceMetadata>;

  fn get_all_threads_of_space_by_space_id(&self, space_id: SpaceId) -> Vec<ThreadId>;

  /// Get all the space. Current and complete space
  fn get_all_spaces(&self, from_index: Option<u32>, limit: Option<u32>) -> Vec<SpaceMetadata>;

  fn follow_space(&mut self, space_id: String) -> Option<String>;

  fn get_followed_user_of_space_by_space_id(&self, space_id: SpaceId) -> Vec<UserId>;
}
