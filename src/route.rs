use futures_util::TryStreamExt;
use ipnet::IpNet;
use netlink_packet_route::AddressFamily;
use netlink_packet_route::route::{
    RouteAddress, RouteAttribute, RouteProtocol, RouteScope, RouteType,
};
use rtnetlink::{Error, Handle, IpVersion, new_connection};

const LOCAL_TABLE_ID: u8 = 255;

pub async fn ip_route_add_cidr(cidr: IpNet) {
    let (connection, handle, _) = new_connection().unwrap();
    tokio::spawn(connection);

    if let Err(e) = add_route(handle.clone(), cidr).await {
        tracing::trace!("Failed to apply route: {}", e);
    }
}

async fn add_route(handle: Handle, cidr: IpNet) -> Result<(), Error> {
    let route = handle.route();
    let lo = handle
        .link()
        .get()
        .match_name("lo".to_string())
        .execute()
        .try_next()
        .await?
        .unwrap()
        .header
        .index;

    let route_exists = |ip_version: IpVersion,
                        address_family: AddressFamily,
                        destination_prefix_length: u8,
                        route_address: RouteAddress| async move {
        let mut routes = handle.route().get(ip_version).execute();
        while let Some(route) = routes.try_next().await? {
            let header = route.header;
            tracing::trace!(
                "route attributes: {:?}\nroute header: {:?}",
                route.attributes,
                header
            );

            if header.address_family == address_family
                && header.destination_prefix_length == destination_prefix_length
                && header.table == LOCAL_TABLE_ID
            {
                for attr in route.attributes.iter() {
                    if let RouteAttribute::Destination(dest) = attr {
                        if dest == &route_address {
                            tracing::info!("IP route {} already exists", cidr);
                            return Ok(true);
                        }
                    }
                }
            }
        }
        Ok(false)
    };

    match cidr {
        IpNet::V4(v4) => {
            if !route_exists(
                IpVersion::V4,
                AddressFamily::Inet,
                v4.prefix_len(),
                RouteAddress::Inet(v4.network()),
            )
            .await?
            {
                route
                    .add()
                    .v4()
                    .destination_prefix(v4.network(), v4.prefix_len())
                    .output_interface(lo)
                    .kind(RouteType::Local)
                    .protocol(RouteProtocol::Boot)
                    .scope(RouteScope::Universe)
                    .priority(1024)
                    .table_id(LOCAL_TABLE_ID.into())
                    .execute()
                    .await?;
                tracing::info!("Added IPv4 route {}", cidr);
            }
        }
        IpNet::V6(v6) => {
            if !route_exists(
                IpVersion::V6,
                AddressFamily::Inet6,
                v6.prefix_len(),
                RouteAddress::Inet6(v6.network()),
            )
            .await?
            {
                route
                    .add()
                    .v6()
                    .destination_prefix(v6.network(), v6.prefix_len())
                    .output_interface(lo)
                    .kind(RouteType::Local)
                    .protocol(RouteProtocol::Boot)
                    .scope(RouteScope::Universe)
                    .priority(1024)
                    .table_id(LOCAL_TABLE_ID.into())
                    .execute()
                    .await?;
                tracing::info!("Added IPv6 route {}", cidr);
            }
        }
    }

    Ok(())
}
