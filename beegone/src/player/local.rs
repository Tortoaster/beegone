use std::{ops::DerefMut, sync::Mutex, time::Duration};

use tokio::time;

use crate::Action;

static LOCAL_ACTION: Mutex<Option<Action>> = Mutex::new(None);

pub async fn submit_action(action: Action) {
    *LOCAL_ACTION.lock().unwrap() = Some(action);
}

pub async fn retrieve_action() -> Action {
    loop {
        let submitted = LOCAL_ACTION.lock().unwrap().deref_mut().take();
        match submitted {
            None => time::sleep(Duration::from_millis(200)).await,
            Some(action) => return action,
        }
    }
}
