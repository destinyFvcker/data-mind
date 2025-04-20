//! 刷新/过期 github OAuth 登陆发放的随机字符串state
use std::{collections::HashMap, sync::Arc, time::Duration};

use dashmap::DashMap;
use rand::{distr::Alphanumeric, Rng};

/// state将在生成2分钟后过期
const FRESH_TIME: i8 = 2;

/// 一个dashmap，用于在用户使用github进行鉴权登陆时生成并储存一个随机生成的字符串
/// 并在稍后用户重定向回服务器时取出进行校验
pub struct GithubStateCache {
    /// i8之中记录了剩余过期时间，2分钟过期
    pub state_map: DashMap<String, i8>,
}

impl GithubStateCache {
    /// 开始后台的dashmap花型行为，将超过期限的dashmap state对删除
    pub fn begin_processing() -> Arc<Self> {
        let github_state = Arc::new(Self {
            state_map: DashMap::new(),
        });

        let arc_ref = github_state.clone();
        actix_web::rt::spawn(async move {
            loop {
                // arc_ref.dump_states();
                actix_web::rt::time::sleep(Duration::from_secs(60)).await;
                arc_ref.state_map.alter_all(|_, v| v - 1);
                arc_ref.state_map.retain(|_, value| *value > 0);
            }
        });

        github_state
    }

    pub fn new_state(&self) -> String {
        let state = Self::gen_state();
        self.state_map.insert(state.clone(), FRESH_TIME);
        state
    }

    pub fn verify_state(&self, input_state: &str) -> bool {
        if let Some(_) = self.state_map.remove(input_state) {
            true
        } else {
            false
        }
    }

    fn gen_state() -> String {
        rand::rng()
            .sample_iter(&Alphanumeric)
            .take(16) // 这里是长度
            .map(char::from)
            .collect()
    }

    #[allow(unused)]
    fn dump_states(&self) {
        let current_states = self
            .state_map
            .iter()
            .map(|pair_ref| (pair_ref.key().to_owned(), *pair_ref.value()))
            .collect::<HashMap<String, i8>>();

        ftlog::debug!("dump current states = {:?}", current_states);
    }
}

#[cfg(test)]
mod test {
    use rand::{distr::Alphanumeric, Rng};

    #[test]
    fn test_random_string() {
        let random_string: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(16) // 这里是长度
            .map(char::from)
            .collect();

        println!("随机字符串: {}", random_string);
    }
}
