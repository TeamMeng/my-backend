use crate::{AppError, AppState};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Url {
    pub id: String,
    pub user_id: i64,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUrl {
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Output {
    pub output: String,
}

#[derive(Serialize, Deserialize)]
pub struct MoreOutput {
    pub output: Vec<Output>,
}

impl AppState {
    pub async fn shorten(&self, user_id: i64, input: CreateUrl) -> Result<Output, AppError> {
        let id = nanoid::nanoid!(6);
        let url: Url = sqlx::query_as(
            "
            INSERT INTO urls (id, user_id, url) VALUES ($1, $2, $3) ON CONFLICT(url) DO UPDATE SET url = EXCLUDED.url RETURNING *
            "
        )
        .bind(id)
        .bind(user_id)
        .bind(input.url)
        .fetch_one(&self.pool)
        .await?;

        Ok(Output::new(url.id))
    }

    pub async fn get_url(&self, user_id: i64, id: String) -> Result<Output, AppError> {
        let url: Url = sqlx::query_as(
            "
            SELECT * FROM urls WHERE user_id = $1 AND id = $2
            ",
        )
        .bind(user_id)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(Output::new(url.url))
    }

    pub async fn get_all_url(&self, user_id: i64) -> Result<MoreOutput, AppError> {
        let url: Vec<Url> = sqlx::query_as(
            "
            SELECT * FROM urls WHERE user_id = $1
            ",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        let output = url.iter().map(|v| Output::new(v.url.clone())).collect();

        Ok(MoreOutput { output })
    }
}

impl Output {
    pub fn new(id: impl Into<String>) -> Self {
        Self { output: id.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn shorten_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        let email = "Meng@123.com";

        let user = state
            .find_user_by_email(email)
            .await?
            .expect("user should exists");

        let url = "www.baidu.com";
        let input = CreateUrl::new(url);

        let url = state.shorten(user.id, input).await?;

        assert_eq!(url.output.len(), 6);

        Ok(())
    }

    #[tokio::test]
    async fn get_url_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        let email = "Meng@123.com";

        let user = state
            .find_user_by_email(email)
            .await?
            .expect("user should exists");

        let url = "www.baidu.com";
        let input = CreateUrl::new(url);

        let id = state.shorten(user.id, input).await?;

        assert_eq!(id.output.len(), 6);

        let ret = state.get_url(user.id, id.output).await?;

        assert_eq!(ret.output, url);

        Ok(())
    }

    #[tokio::test]
    async fn get_all_url() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        let email = "Meng@123.com";

        let user = state
            .find_user_by_email(email)
            .await?
            .expect("user should exists");

        let mut url = "www.baidu.com";
        let mut input = CreateUrl::new(url);

        state.shorten(user.id, input).await?;

        url = "www.360.com";
        input = CreateUrl::new(url);

        state.shorten(user.id, input).await?;

        let ret = state.get_all_url(user.id).await?;

        assert_eq!(ret.output.len(), 2);

        Ok(())
    }

    impl CreateUrl {
        fn new(url: impl Into<String>) -> Self {
            Self { url: url.into() }
        }
    }
}
