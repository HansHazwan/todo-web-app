use crate::prelude::*;
use crate::utils::*;
use crate::api::*;
use tokio::net::TcpListener;

pub struct Application {
    listener: TcpListener,
}

impl Application {
    pub fn new(listener: TcpListener) -> Application {
        Application {
            listener,
        }
    }

    pub async fn run(mut self) -> Result<()> {
        loop {
            let (mut socket, addr) = self.listener.accept().await?;
            log::info!("New Connection: {:?}", addr);

            tokio::spawn(async move {
                let request = read_socket(&mut socket).await.unwrap();

                let response = generate_response(request);

                write_socket(&mut socket, &response).await.unwrap();
            });
        }

        Ok(())
    }
}

