use tokio::{
    net::{lookup_host, ToSocketAddrs, UdpSocket},
    select,
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
};

pub async fn udp_redirect(
    recv_address: impl ToSocketAddrs,
    connect_address: impl ToSocketAddrs,
) -> std::io::Result<(UnboundedSender<Vec<u8>>, UnboundedReceiver<Vec<u8>>)> {
    let s = lookup_host(recv_address)
        .await?
        .next()
        .ok_or(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "unknown address",
        ))?;

    let connect_address = lookup_host(connect_address)
        .await?
        .next()
        .ok_or(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "unknown remote",
        ))?;
    let (tx_to_receiver, rx) = tokio::sync::mpsc::unbounded_channel::<Vec<u8>>();
    let (tx, mut rx_from_receiver) = tokio::sync::mpsc::unbounded_channel::<Vec<u8>>();
    let socket = UdpSocket::bind(s).await?;
    socket.connect(connect_address).await?;
    tokio::spawn(async move {
        loop {
            let mut v = Vec::with_capacity(u16::MAX as usize);
            select! {
                rx_res = socket.recv_buf(&mut v) => {
                    let Ok(_) = rx_res else {
                        break;
                    };
                    let Ok(_) = tx_to_receiver.send(v) else {
                        break;
                };
                },
                msg_to_send = rx_from_receiver.recv() => {
                    let Some(vec_to_send) = msg_to_send else { break;
                    };
                    let Ok (_) = socket.send(&vec_to_send).await else {
                        break;
                    };
                }
            }
        }
    });

    return Ok((tx, rx));
}

async fn foo() {
    let walla = udp_redirect("localhost:0", "127.0.0.1:1234").await.unwrap();
}
