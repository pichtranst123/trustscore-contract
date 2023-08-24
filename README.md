page structure

- /
  - /spaces
    - /spaces/forex_trading **#forex_trading**
      - /thread/thread_001 **thread_001**
      - /thread/thread_002 **thread_002**
    - /spaces/crypto_trading **#crypto_trading**
      - /thread/thread_003 **thread_003**
      - /thread/thread_004 **thread_004**
  - /me
    - /me/create

### Step to run local

- clean up after update code

  ```
  cm clean
  ```

- build lại contract

  ```
  cm build
  ```

- deploy lại contract

  ```
  cm dev-deploy
  ```

- init contract (bắt buộc)

  ```
  cm call-self init
  ```

- re check lại init ok không
  ```
  cm call --account_id gmfam.testnet get_me
  ```
  ```
  {
    spec: 'thread_score-0.0.1',
    name: 'thread_score',
    symbol: 'TS',
    icon: null,
    base_uri: null,
    reference: null
  }
  ```
- get owner id
  ```
  cm call --account_id gmfam.testnet get_owner_id
  ```
  ```
  dev-1691914103143-32544821462759
  ```
- get array danh sách id all user (sau khi đã đăng ký vào TS)

  ```
  cm call --account_id gmfam.testnet get_subscriber_users
  ```

  ```
  []
  ```

- tạo 1 user trên hệ thống TS
  ```
  cm call create_user '{"nickname":"longn","first_name":"Ân","last_name":"Nguyễn Văn", "bio":"this is my bio" ,"avatar":"bafkreibnaelo4monu6jpndqtb3pmza22j7k77gcak3xrux6mrkdq5fakuu"}' --account_id gmfam.testnet
  ```
  ```
  ' '
  ```
- get info của 1 user by user_id

  ```
  cm call get_user_metadata_by_user_id '{"user_id":"gmfam.testnet"}' --account_id gmfam.testnet
  ```

  ```
  {
    user_id: 'gmfam.testnet',
    metadata: {
      user_id: 'gmfam.testnet',
      nickname: 'longn',
      role: 'Verified',
      first_name: 'Ân',
      last_name: 'Nguyễn Văn',
      bio: 'this is my bio',
      avatar: 'bafkreibnaelo4monu6jpndqtb3pmza22j7k77gcak3xrux6mrkdq5fakuu',
      created_at: 1692823540083,
      updated_at: 1692823540083
    },
    threads: [],
    total_point: 1000,
    threads_owned: 0
  }
  ```

- active role cho 1 user by user_id

  ```
  cm call active_user_role '{"user_id":"gmfam.testnet"}' --account_id gmfam.testnet
  ```

  ```
  {
    user_id: 'gmfam.testnet',
    metadata: {
      user_id: 'gmfam.testnet',
      nickname: 'longn',
      role: 'Verified',
      first_name: 'Ân',
      last_name: 'Nguyễn Văn',
      bio: 'this is my bio',
      avatar: 'bafkreibnaelo4monu6jpndqtb3pmza22j7k77gcak3xrux6mrkdq5fakuu',
      created_at: 1692823540083,
      updated_at: 1692823540083
    },
    threads: [],
    total_point: 1000,
    threads_owned: 0
  }
  ```

- tạo 1 thread mới

  ```
  cm call create_thread '{"title": "thread title 01", "description": "thread desc 01", "media_link":"bafkreifko42xz73mizlglr235icoexdicld5xqutbsymwph4fvnoktvnym", "init_point": 90, "space_name": "crypto trading", "start_time": "1692824790000", "end_time": "1695477352000", "options": ["No", "Yes"]}' --account_id gmfam.testnet
  ```

  ```
  {
    thread_id: 'gmfam.testnet_thread_title_02',
    title: 'thread title 02',
    media_link: 'bafkreifko42xz73mizlglr235icoexdicld5xqutbsymwph4fvnoktvnym',
    creator_id: 'gmfam.testnet',
    content: null,
    created_at: 1692874267501,
    init_point: 190,
    space_name: 'future trading',
    last_id: 0,
    choices_count: 2,
    choices_map: { '0': 'No', '1': 'Yes' },
    user_votes_map: {},
    choices_rating: {},
    start_time: 1692824790000,
    end_time: 1695477352000
  }
  ```

- lấy info của 1 thread by thread_id

  ```
  cm call get_thread_metadata_by_thread_id '{"thread_id":"gmfam.testnet_thread_title_01"}' --account_id gmfam.testnet
  ```

  ```
  {
    thread_id: 'gmfam.testnet_thread_title_02',
    title: 'thread title 02',
    media_link: 'bafkreifko42xz73mizlglr235icoexdicld5xqutbsymwph4fvnoktvnym',
    creator_id: 'gmfam.testnet',
    content: null,
    created_at: 1692874267501,
    init_point: 190,
    space_name: 'future trading',
    last_id: 0,
    choices_count: 2,
    choices_map: { '0': 'No', '1': 'Yes' },
    user_votes_map: {},
    choices_rating: {},
    start_time: 1692824790000,
    end_time: 1695477352000
  }
  ```

- lấy status của 1 thread by thread_id

  ```
  cm call get_thread_status '{"thread_id":"gmfam.testnet_thread_title_01"}' --account_id gmfam.testnet
  ```

  ```
  'Closed' // 'Open', 'Upcoming'

  ```

- lấy info các thread của 1 user own by user_id

  ```
  cm call get_all_threads_per_user_own '{"user_id":"gmfam.testnet"}' --account_id gmfam.testnet
  ```

  ```
  [
    {
      thread_id: 'gmfam.testnet_thread_title_01',
      title: 'thread title 01',
      media_link: 'bafkreifko42xz73mizlglr235icoexdicld5xqutbsymwph4fvnoktvnym',
      creator_id: 'gmfam.testnet',
      content: null,
      created_at: 1692874190923,
      init_point: 90,
      space_name: 'crypto trading',
      last_id: 0,
      choices_count: 2,
      choices_map: { '0': 'No', '1': 'Yes' },
      user_votes_map: {},
      choices_rating: {},
      start_time: 1692824790000,
      end_time: 1695477352000
    },
    {
      thread_id: 'gmfam.testnet_thread_title_02',
      title: 'thread title 02',
      media_link: 'bafkreifko42xz73mizlglr235icoexdicld5xqutbsymwph4fvnoktvnym',
      creator_id: 'gmfam.testnet',
      content: null,
      created_at: 1692874267501,
      init_point: 190,
      space_name: 'future trading',
      last_id: 0,
      choices_count: 2,
      choices_map: { '0': 'No', '1': 'Yes' },
      user_votes_map: {},
      choices_rating: {},
      start_time: 1692824790000,
      end_time: 1695477352000
    }
  ]
  ```

- get info của 1 space not existed
  ```
  cm call get_space_metadata_by_space_id '{"space_id":"crypto"}' --account_id gmfam.testnet
  ```
  ```
  null
  ```
- get info của 1 space
  ```
  cm call get_space_metadata_by_space_id '{"space_id":"crypto_trading"}' --account_id gmfam.testnet
  ```
  ```
  {
    space_id: 'crypto_trading',
    space_name: 'crypto trading',
    creator_id: 'gmfam.testnet',
    created_at: 1692737457246
  }
  ```
- get_all_threads_of_space_by_space_id info của 1 space

  ```
  cm call get_all_threads_of_space_by_space_id '{"space_id": "crypto_trading"}' --account_id gmfam.testnet
  ```

  ```
  [ 'gmfam.testnet_thread_title_01', 'gmfam.testnet_thread_title_02' ]

  ```

- get_all_threads_of_space_by_space_id info của 1 space

  ```
  cm call get_all_threads_of_space_by_space_id '{"space_id":"trading"}' --account_id gmfam.testnet
  ```

  ```
  ['gmfam.testnet_thread_title_03' ]

  ```

- get all spaces info
  ```
  cm call get_all_spaces --account_id gmfam.testnet
  ```
  ```
  [
    {
      space_id: 'crypto_trading',
      space_name: 'crypto trading',
      creator_id: 'gmfam.testnet',
      created_at: 1692737457246
    },
    {
      space_id: 'trading',
      space_name: 'trading',
      creator_id: 'gmfam.testnet',
      created_at: 1692739794022
    }
  ]
  ```
- vote thread by thread_id

  ```
  cm call vote_thread '{"thread_id":"gmfam.testnet_thread_title_01", "choice_number": 0, "point": 50}' --account_id gmfam.testnet
  ```

  ```
  OK
  ```
- end thread by thread_id

  ```
  cm call end_thread '{"thread_id":"gmfam.testnet_thread_title_01"}' --account_id gmfam.testnet
  ```

  ```
  OK
  ```

### Requirements

- Install Cargo Make
  `cargo install cargo-make`
  `cargo install cargo-generate`

* Install Near Cli
  `npm install -g near-cli`

* Prepare
  `cargo make prepare`

* Build Contract
  `cargo make build`

* deploy Contract
  `cargo make dev-deploy`

near deploy --accountId trust-score-project.gmfam.testnet --wasmFile target/wasm32-unknown-unknown/release/thread_score.wasm
