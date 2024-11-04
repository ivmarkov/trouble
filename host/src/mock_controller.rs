use core::convert::Infallible;
use core::future::Future;

use bt_hci::cmd::{self, AsyncCmd, SyncCmd};
use bt_hci::controller::{ControllerCmdAsync, ControllerCmdSync};
use embedded_io::ErrorType;

pub struct MockController {}

impl MockController {
    pub fn new() -> Self {
        Self {}
    }
}

impl ErrorType for MockController {
    type Error = Infallible;
}

impl bt_hci::controller::blocking::Controller for MockController {
    fn write_acl_data(&self, packet: &bt_hci::data::AclPacket) -> Result<(), Self::Error> {
        todo!()
    }

    fn write_sync_data(&self, packet: &bt_hci::data::SyncPacket) -> Result<(), Self::Error> {
        todo!()
    }

    fn write_iso_data(&self, packet: &bt_hci::data::IsoPacket) -> Result<(), Self::Error> {
        todo!()
    }

    fn try_write_acl_data(
        &self,
        packet: &bt_hci::data::AclPacket,
    ) -> Result<(), bt_hci::controller::blocking::TryError<Self::Error>> {
        todo!()
    }

    fn try_write_sync_data(
        &self,
        packet: &bt_hci::data::SyncPacket,
    ) -> Result<(), bt_hci::controller::blocking::TryError<Self::Error>> {
        todo!()
    }

    fn try_write_iso_data(
        &self,
        packet: &bt_hci::data::IsoPacket,
    ) -> Result<(), bt_hci::controller::blocking::TryError<Self::Error>> {
        todo!()
    }

    fn read<'a>(&self, buf: &'a mut [u8]) -> Result<bt_hci::ControllerToHostPacket<'a>, Self::Error> {
        todo!()
    }

    fn try_read<'a>(
        &self,
        buf: &'a mut [u8],
    ) -> Result<bt_hci::ControllerToHostPacket<'a>, bt_hci::controller::blocking::TryError<Self::Error>> {
        todo!()
    }
}

impl bt_hci::controller::Controller for MockController {
    fn write_acl_data(&self, packet: &bt_hci::data::AclPacket) -> impl Future<Output = Result<(), Self::Error>> {
        async { todo!() }
    }

    fn write_sync_data(&self, packet: &bt_hci::data::SyncPacket) -> impl Future<Output = Result<(), Self::Error>> {
        async { todo!() }
    }

    fn write_iso_data(&self, packet: &bt_hci::data::IsoPacket) -> impl Future<Output = Result<(), Self::Error>> {
        async { todo!() }
    }

    fn read<'a>(
        &self,
        buf: &'a mut [u8],
    ) -> impl Future<Output = Result<bt_hci::ControllerToHostPacket<'a>, Self::Error>> {
        async { todo!() }
    }
}

impl<C: SyncCmd> ControllerCmdSync<C> for MockController {
    fn exec(&self, cmd: &C) -> impl Future<Output = Result<C::Return, cmd::Error<Self::Error>>> {
        async { todo!() }
    }
}

impl<C: AsyncCmd> ControllerCmdAsync<C> for MockController {
    fn exec(&self, cmd: &C) -> impl Future<Output = Result<(), cmd::Error<Self::Error>>> {
        async { todo!() }
    }
}
