use std::{collections::HashMap, u8};

use crate::{
  application::repository::{convert_title_to_id, convert_title_to_id_no_account, hash_account_id, hash_space_id},
  models::{
    contract::{StorageKey, ThreadScoreContract, ThreadScoreContractExt},
    space::SpaceFeatures,
    thread::{ThreadFeatures, ThreadId, ThreadMetadata, ThreadState},
    user::{UserId, UserRoles},
  },
};
use near_sdk::{borsh::BorshSerialize, json_types::U64};
use near_sdk::{collections::UnorderedSet, env, near_bindgen};

#[near_bindgen]
impl ThreadFeatures for ThreadScoreContract {
  fn create_thread(
    &mut self,
    title: String,
    content: Option<String>,
    media_link: Option<String>,
    init_point: u32,
    space_name: String,
    start_time: U64,
    end_time: U64,
    options: Vec<String>,
  ) -> ThreadMetadata {
    let creator_id = env::signer_account_id();

    // check option have at least 2
    assert!(options.len() > 1, "Vote option must be greater than 2!");
    assert!(options.len() < 4, "Vote option must be less than 4!");

    let mut choices_map = HashMap::<u8, String>::new();

    options.iter().enumerate().for_each(|(idx, option)| {
      choices_map.insert(idx as u8, option.to_owned());
    });

    let thread_id = convert_title_to_id(&title, creator_id.to_string());

    match self.user_metadata_by_id.get(&creator_id) {
      Some(creator_json) => {
        assert!(creator_json.metadata.role == UserRoles::Verified, "Your account is not verified!");

        assert!(creator_json.total_point > init_point, "Your trust point is not enough to create new thread!");
      },
      None => assert!(false, "Your account is not created!"),
    }

    assert!(self.thread_metadata_by_id.get(&thread_id).is_none(), "This thread already created!");

    let thread_meta = ThreadMetadata {
      thread_id: thread_id.clone(),
      title,
      media_link,
      creator_id: creator_id.clone(),
      content,
      init_point,
      space_name: space_name.clone(),
      start_time: start_time.into(),
      end_time: end_time.into(),
      created_at: env::block_timestamp_ms(),
      choices_count: options.len() as u8,
      choices_map,
      user_votes_map: HashMap::new(),
      choices_rating: HashMap::new(),
      last_id: 0_u32,
    };

    let init_new_user_threads_list: UnorderedSet<String> = UnorderedSet::new(
      StorageKey::ThreadsPerUserInner { account_id_hash: hash_account_id(&creator_id) }.try_to_vec().unwrap(),
    );

    let mut new_user_threads_list = if let Some(user_threads_list) = self.threads_per_user.get(&creator_id) {
      user_threads_list
    } else {
      init_new_user_threads_list
    };

    new_user_threads_list.insert(&thread_id);

    self.threads_per_user.insert(&creator_id, &new_user_threads_list);

    self.thread_metadata_by_id.insert(&thread_id, &thread_meta);

    let space_id = convert_title_to_id_no_account(&space_name);
    let is_space_id_exists = self.space_metadata_by_id.contains_key(&space_id);

    if !is_space_id_exists {
      self.create_space(space_name);
    }

    let init_new_space_threads_list: UnorderedSet<String> = UnorderedSet::new(
      StorageKey::ThreadsPerSpaceInner { space_id_hash: hash_space_id(&space_id) }.try_to_vec().unwrap(),
    );

    let mut new_space_threads_list = if let Some(space_threads_list) = self.threads_per_space.get(&space_id) {
      space_threads_list
    } else {
      init_new_space_threads_list
    };

    new_space_threads_list.insert(&thread_id);

    self.threads_per_space.insert(&space_id, &new_space_threads_list);

    // update Total number of threads owned by the user.
    self.user_metadata_by_id.get(&creator_id);

    let mut new_json_creator = self.user_metadata_by_id.get(&creator_id).unwrap();
    new_json_creator.threads_owned += 1;
    new_json_creator.total_point -= init_point;
    new_json_creator.threads_list.push(thread_id);

    self.user_metadata_by_id.insert(&creator_id, &new_json_creator);

    thread_meta
  }

  fn get_thread_metadata_by_thread_id(&self, thread_id: ThreadId) -> Option<ThreadMetadata> {
    let found_thread = self.thread_metadata_by_id.get(&thread_id);
    found_thread
  }

  /// Get all the thread per user have. Current and complete thread
  fn get_all_threads_per_user_own(
    &self,
    user_id: UserId,
    start: Option<u32>,
    limit: Option<u32>,
  ) -> Vec<ThreadMetadata> {
    let mut result: Vec<ThreadMetadata> = Vec::new();

    let thread_array = self.threads_per_user.get(&user_id).unwrap();

    for thread_id in thread_array.iter().skip(start.unwrap_or(0_u32) as usize).take(limit.unwrap_or(5) as usize) {
      let thread_found = self.thread_metadata_by_id.get(&thread_id);
      result.push(thread_found.unwrap());
    }

    result
  }

  // Check thread status
  fn get_thread_status(&self, thread_id: &ThreadId) -> ThreadState {
    let thread_found = self.thread_metadata_by_id.get(&thread_id);

    assert!(thread_found.is_some(), "Thread not existed!");

    let current_time = env::block_timestamp_ms();
    let start_time = thread_found.clone().unwrap().start_time;
    let end_time = thread_found.unwrap().end_time;

    if current_time >= end_time {
      return ThreadState::Closed;
    }

    if current_time > start_time {
      return ThreadState::Open;
    }

    return ThreadState::Upcoming;
  }

  fn vote_thread(&mut self, thread_id: ThreadId, choice_number: u8, point: u32) -> Option<String> {
    let voter = env::signer_account_id();

    assert!(point > 10, "Your point must be greater than 10!");

    // check point of user > initial point
    let found_voter = self.user_metadata_by_id.get(&voter);
    assert!(found_voter.is_some(), "This user is not existed!");

    if let Some(json_user) = &found_voter {
      assert!(json_user.total_point > point, "You don't have enough point!");
    }

    // check thread id valid
    let thread_found = self.thread_metadata_by_id.get(&thread_id);
    assert!(thread_found.is_some(), "Thread is not existed!");

    // check time is valid

    let cur_thread_state = self.get_thread_status(&thread_id);
    assert!(cur_thread_state != ThreadState::Upcoming, "This thread is not live yet!");
    assert!(cur_thread_state != ThreadState::Closed, "This thread is ended!");

    // check choice is valid
    if let Some(mut thread_metadata) = thread_found {
      assert!(thread_metadata.choices_map.get(&choice_number).is_some(), "Your choice is not valid!");

      // update user_votes_map
      let new_user_votes_map = thread_metadata.user_votes_map.get_key_value(&voter);

      assert!(new_user_votes_map.is_none(), "This user already voted!");

      thread_metadata.user_votes_map.insert(voter.clone(), (choice_number, point));

      // update choices_rating
      if let Some(cur_point) = thread_metadata.choices_rating.get_mut(&choice_number) {
        *cur_point += point;
      }

      self.thread_metadata_by_id.insert(&thread_id, &thread_metadata);
    }

    // update new point for user
    let mut new_json_user = found_voter.unwrap();

    new_json_user.total_point -= point;

    self.user_metadata_by_id.insert(&voter, &new_json_user);

    Some("OK".to_string())
  }

  fn end_thread(&mut self, thread_id: ThreadId) -> Option<String> {
    // check thread status

    // check is admin

    // calculate which win

    // calc total point
    


    None
  }
}
