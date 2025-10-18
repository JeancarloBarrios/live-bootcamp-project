use std::collections::HashMap;

use crate::domain::data_stores::{UserStore, UserStoreError};
use crate::domain::user::User;

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    async fn get_user(&self, email: &str) -> Result<&User, UserStoreError> {
        self.users.get(email).ok_or(UserStoreError::UserNotFound)
    }

    async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let user = self.users.get(email).ok_or(UserStoreError::UserNotFound)?;

        if user.password == password {
            Ok(())
        } else {
            Err(UserStoreError::InvalidCredentials)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore::default();
        let user = User::new(
            "test@example.com".to_string(),
            "password123".to_string(),
            false,
        );

        // Should successfully add a new user
        assert_eq!(store.add_user(user.clone()).await, Ok(()));
        // Should return error when adding duplicate user
        let duplicate_user = User::new(
            "test@example.com".to_string(),
            "different_password".to_string(),
            true,
        );
        assert_eq!(
            store.add_user(duplicate_user).await,
            Err(UserStoreError::UserAlreadyExists)
        );
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore::default();
        let user = User::new(
            "test@example.com".to_string(),
            "password123".to_string(),
            false,
        );

        // Should return UserNotFound for non-existent user
        assert_eq!(
            store.get_user("test@example.com").await,
            Err(UserStoreError::UserNotFound)
        );

        // Add user and verify retrieval
        store.add_user(user).await.unwrap();
        let retrieved_user = store.get_user("test@example.com").await.unwrap();
        assert_eq!(retrieved_user.email, "test@example.com");
        assert_eq!(retrieved_user.password, "password123");
        assert_eq!(retrieved_user.requires_2fa, false);
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();
        let user = User::new(
            "test@example.com".to_string(),
            "password123".to_string(),
            false,
        );

        // Should return UserNotFound for non-existent user
        assert_eq!(
            store.validate_user("test@example.com", "password123").await,
            Err(UserStoreError::UserNotFound)
        );

        // Add user
        store.add_user(user).await.unwrap();
        let retrieved_user = store.get_user("test@example.com").await.unwrap();

        // Should return Ok for correct credentials
        assert_eq!(
            store.validate_user("test@example.com", "password123").await,
            Ok(())
        );

        // Should return InvalidCredentials for wrong password
        assert_eq!(
            store
                .validate_user("test@example.com", "wrong_password")
                .await,
            Err(UserStoreError::InvalidCredentials)
        );
    }
}
