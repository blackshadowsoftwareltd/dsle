use futures_util::StreamExt;
use tokio;
use zbus::dbus_proxy;
use zbus::Connection;

#[dbus_proxy(interface = "org.freedesktop.login1.Manager")]
trait Manager {
    #[dbus_proxy(signal)]
    fn prepare_for_sleep(&self, going_down: bool) -> zbus::Result<()>;
}
#[dbus_proxy(interface = "org.freedesktop.ScreenSaver")]
trait ScreenSaver {
    #[dbus_proxy(signal)]
    fn active_changed(&self, is_active: bool) -> zbus::Result<()>;
}

pub async fn detect_lock_unlock() {
    println!("1");
    let connection = Connection::system().await.unwrap();
    println!("2");
    let proxy = ManagerProxy::new(
        &connection,               // The DBus connection
        "org.freedesktop.login1",  // The DBus destination
        "/org/freedesktop/login1", // The object path
    )
    .await
    .unwrap();
    println!("3");
    let mut signal_stream = proxy.receive_prepare_for_sleep().await.unwrap();
    println!("4");

    while let Some(signal) = signal_stream.next().await {
        println!("Signal received {:?}", signal);
    }
}

#[tokio::main]
async fn main() {
    println!("0");
    detect_lock_unlock().await;
    println!("5");
}
