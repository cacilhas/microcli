use std::{
    ops::Deref,
    sync::Arc,
};

use ::users::{
    self,
    Users,
};


#[derive(Debug)]
pub struct User(Arc<users::User>);


impl User {
    pub fn is_power_user(&self) -> bool {
        match self.groups() {
            Some(groups) =>
                groups
                    .iter()
                    .any(|group| group.name() == "power"),
            None => false,
        }
    }
}


impl Default for User {
    fn default() -> Self {
        let cache = users::UsersCache::new();
        let uid = cache.get_current_uid();
        let user = match cache.get_user_by_uid(uid) {
            Some(user) => user,
            None => panic!("current user not found"),
        };
        User { 0: user }
    }
}


impl Deref for User {
    type Target = users::User;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}
