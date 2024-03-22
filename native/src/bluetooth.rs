

use tonic::{async_trait, Status};

struct Bluetooth {
    sender: UnboundedSender<Vec<u8>>,
    receiver: UnboundedReceiver<Vec<u8>>,
}
use crate::bluetooth;
use bluetooth::{
    bluetooth_api_server::{BluetoothApi, BluetoothApiServer},
    BluetoothData, Null,
};

#[async_trait]
impl BluetoothApi for Bluetooth {
    async fn send_to_bt(
        &self,
        request: tonic::Request<BluetoothData>,
    ) -> Result<tonic::Response<Null>, Status> {
        todo!()
    }

    async fn receive_from_bt(
        &self,
        request: tonic::Request<Null>,
    ) -> Result<tonic::Response<BluetoothData>, Status> {
        todo!()
    }
}
