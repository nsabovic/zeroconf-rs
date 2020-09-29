//! Trait definition for cross-platform browser

use crate::{EventLoop, NetworkInterface, Result, ServiceDiscoveredCallback};
use std::any::Any;

pub trait TMdnsBrowser {
    /// Creates a new `MdnsBrowser` that browses for the specified `kind` (e.g. `_http._tcp`)
    fn new(kind: &str) -> Self;

    /// Sets the network interface on which to browse for services on.
    ///
    /// Most applications will want to use the default value `NetworkInterface::Unspec` to browse
    /// on all available interfaces.
    fn set_network_interface(&mut self, interface: NetworkInterface);

    /// Sets the [`ServiceDiscoveredCallback`] that is invoked when the browser has discovered and
    /// resolved a service.
    ///
    /// [`ServiceDiscoveredCallback`]: ../type.ServiceDiscoveredCallback.html
    fn set_service_discovered_callback(
        &mut self,
        service_discovered_callback: Box<ServiceDiscoveredCallback>,
    );

    /// Sets the optional user context to pass through to the callback. This is useful if you need
    /// to share state between pre and post-callback. The context type must implement `Any`.
    fn set_context(&mut self, context: Box<dyn Any>);

    /// Starts the browser. Returns an `EventLoop` which can be called to keep the browser alive.
    fn browse_services(&mut self) -> Result<EventLoop>;
}
