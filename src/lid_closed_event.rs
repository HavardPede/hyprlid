//! Monitor laptop lid state via systemd's login1 D-Bus interface.
//!
//! This module listens for changes to the `LidClosed` property and automatically
//! moves workspaces between monitors when the lid opens or closes.

use futures_util::stream::StreamExt;
use zbus::{Connection, proxy};

use crate::config::Config;
use crate::lid::Lid;

/// D-Bus proxy interface for systemd-logind's Manager object.
///
/// See: <https://www.freedesktop.org/software/systemd/man/latest/org.freedesktop.login1.html>
#[proxy(
    default_service = "org.freedesktop.login1",
    default_path = "/org/freedesktop/login1",
    interface = "org.freedesktop.login1.Manager"
)]
trait Login1Manager {
    #[zbus(property)]
    fn lid_closed(&self) -> zbus::Result<bool>;
}

/// Start watching for laptop lid state changes.
///
/// This establishes a D-Bus connection and listens for changes to the
/// `LidClosed` property. When the lid closes, workspaces from the laptop's internal
/// monitor are moved to external monitors. When the lid opens, those workspaces are
/// moved back.
pub async fn watch() -> zbus::Result<()> {
    let connection = Connection::system().await?;
    let proxy = Login1ManagerProxy::new(&connection).await?;
    let property_stream = proxy.receive_lid_closed_changed().await;
    let config = Config::get();

    handle_lid_state_changes(property_stream, config.lid_name).await
}

/// Handle lid state changes from the property stream.
///
/// Maintains state of which workspaces were moved when the lid closed,
/// so they can be restored to their original location when it opens.
async fn handle_lid_state_changes(
    mut property_stream: zbus::proxy::PropertyStream<'_, bool>,
    monitor_name: String,
) -> zbus::Result<()> {
    let mut lid = Lid::setup(&monitor_name);

    while let Some(property) = property_stream.next().await {
        let is_closed = property.get().await?;
        if is_closed {
            lid.disable();
        } else {
            lid.enable();
        }
    }

    Ok(())
}
