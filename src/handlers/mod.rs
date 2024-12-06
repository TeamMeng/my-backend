mod urls;
mod users;

pub use urls::*;
pub use users::*;

#[cfg(test)]
mod tests {
    use crate::{get_router, AppState, MoreOutput, Output};
    use anyhow::Result;
    use reqwest::Client;
    use serde::Deserialize;
    use std::net::SocketAddr;
    use tokio::net::TcpListener;

    const ADDR: &str = "0.0.0.0:0";

    struct Server {
        addr: SocketAddr,
        token: String,
        client: Client,
        path: String,
    }

    #[derive(Deserialize)]
    struct AuthToken {
        token: String,
    }

    #[tokio::test]
    async fn server_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let mut server = Server::new(state).await?;
        server.path = server.short().await?;
        server.get_url().await?;
        server.get_urls().await?;
        server.token = server.change().await?;
        server.delete().await?;

        Ok(())
    }

    impl Server {
        async fn new(state: AppState) -> Result<Self> {
            let app = get_router(state)?;
            let listener = TcpListener::bind(ADDR).await?;
            let addr = listener.local_addr()?;

            tokio::spawn(async move {
                axum::serve(listener, app.into_make_service())
                    .await
                    .unwrap()
            });

            let client = Client::new();

            let mut ret = Self {
                addr,
                client,
                token: "".to_string(),
                path: "".to_string(),
            };

            ret.signup().await?;

            ret.token = ret.login().await?;

            Ok(ret)
        }

        async fn signup(&self) -> Result<()> {
            let res = self
                .client
                .post(format!("http://{}/signup", self.addr))
                .header("Content-Type", "application/json")
                .body(r#"{"email": "Meng@acme.org","name": "TeamMeng", "password":"123456"}"#)
                .send()
                .await?;

            assert_eq!(res.status(), 201);

            Ok(())
        }

        async fn login(&self) -> Result<String> {
            let res = self
                .client
                .post(format!("http://{}/login", self.addr))
                .header("Content-Type", "application/json")
                .body(r#"{"email": "Meng@acme.org", "password":"123456"}"#)
                .send()
                .await?;

            assert_eq!(res.status(), 200);

            let ret: AuthToken = res.json().await?;

            Ok(ret.token)
        }

        async fn change(&self) -> Result<String> {
            let res = self
                .client
                .post(format!("http://{}/change", self.addr))
                .header("Authorization", format!("Bearer {}", self.token))
                .header("Content-Type", "application/json")
                .body(r#"{"email": "Alice@acme.org", "name": "TeamAlice", "password":"hunter42"}"#)
                .send()
                .await?;

            assert_eq!(res.status(), 200);

            let ret: AuthToken = res.json().await?;
            Ok(ret.token)
        }

        async fn delete(&self) -> Result<()> {
            let res = self
                .client
                .delete(format!("http://{}/delete", self.addr))
                .header("Authorization", format!("Bearer {}", self.token))
                .send()
                .await?;

            assert_eq!(res.status(), 200);

            Ok(())
        }

        async fn short(&self) -> Result<String> {
            let res = self
                .client
                .post(format!("http://{}/short", self.addr))
                .header("Authorization", format!("Bearer {}", self.token))
                .header("Content-Type", "application/json")
                .body(r#"{"url": "www.360.com"}"#)
                .send()
                .await?;

            assert_eq!(res.status(), 200);

            let ret: Output = res.json().await?;

            Ok(ret.output)
        }

        async fn get_url(&self) -> Result<()> {
            let res = self
                .client
                .get(format!("http://{}/{}", self.addr, self.path))
                .header("Authorization", format!("Bearer {}", self.token))
                .send()
                .await?;

            assert_eq!(res.status(), 200);

            let ret: Output = res.json().await?;

            assert_eq!(ret.output, "www.360.com");

            Ok(())
        }

        async fn get_urls(&self) -> Result<()> {
            let res = self
                .client
                .get(format!("http://{}/urls", self.addr))
                .header("Authorization", format!("Bearer {}", self.token))
                .send()
                .await?;

            assert_eq!(res.status(), 200);

            let ret: MoreOutput = res.json().await?;

            assert_eq!(ret.output.len(), 1);

            Ok(())
        }
    }
}
