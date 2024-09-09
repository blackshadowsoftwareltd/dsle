use futures_util::StreamExt;
use zbus::dbus_proxy;
use zbus::zvariant;
use zbus::Connection;

#[dbus_proxy(interface = "org.freedesktop.DBus.Properties")]
trait Properties {
    #[dbus_proxy(signal)]
    fn properties_changed(
        &self,
        interface_name: &str,
        changed_properties: std::collections::HashMap<String, zvariant::OwnedValue>,
        invalidated_properties: Vec<String>,
    ) -> zbus::Result<()>;
}

pub async fn linux_lock_unlock() {
    let connection = Connection::system().await.unwrap();

    let proxy = PropertiesProxy::new(
        &connection,
        "org.freedesktop.login1",
        "/org/freedesktop/login1/session/_32", // Path to the session (you may need to adapt this path)
    )
    .await
    .unwrap();
    let mut signal_stream = proxy.receive_properties_changed().await.unwrap();
    println!("Listening for signals");
    while let Some(signal) = signal_stream.next().await {
        if let Ok(args) = signal.args() {
            if let Some(value) = args.changed_properties.get("IdleHint") {
                if let Ok(x) = value.downcast_ref::<bool>() {
                    println!("Signal received {:?}", x);
                }
            }
        }
    }
}
