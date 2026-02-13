use napi_derive::napi;

#[napi]
pub mod friends {
    use napi::bindgen_prelude::BigInt;
    use steamworks::SteamId;

    #[napi(object)]
    pub struct Friend {
        pub steam_id: BigInt,
        pub name: String,
    }

    #[napi]
    pub enum FriendFlags {
        None = 0x00,
        Blocked = 0x01,
        FriendshipRequested = 0x02,
        Immediate = 0x04,
        ClanMember = 0x08,
        OnGameServer = 0x10,
        RequestingFriendship = 0x80,
        RequestingInfo = 0x100,
        All = 0xFFFF,
    }

    /// Get an array of friends matching the provided flags.
    ///
    /// @param flags - The flags to filter friends by.
    /// @returns An array of friend objects containing steamId and name.
    ///
    /// @example
    /// ```js
    /// const friends = client.friends.getFriends(client.friends.FriendFlags.Immediate)
    /// console.log(friends)
    /// // Output:
    /// // [
    /// //   { steamId: 76561197985341433n, name: 'XXX' },
    /// //   { steamId: 76561198034399293n, name: 'YYY' },
    /// // ]
    /// ```
    #[napi]
    pub fn get_friends(flags: i32) -> Vec<Friend> {
        let client = crate::client::get_client();
        let flags = steamworks::FriendFlags::from_bits_truncate(flags as u16);
        client
            .friends()
            .get_friends(flags)
            .into_iter()
            .map(|f| Friend {
                steam_id: BigInt::from(f.id().raw()),
                name: f.name(),
            })
            .collect()
    }

    /// Get the persona name of a friend.
    ///
    /// @param steam_id64 - The Steam ID of the friend.
    /// @returns The name of the friend.
    ///
    /// @example
    /// ```js
    /// const name = client.friends.getFriendName(76561197985341433n)
    /// console.log(name)
    /// ```
    #[napi]
    pub fn get_friend_name(steam_id64: BigInt) -> String {
        let client = crate::client::get_client();
        client
            .friends()
            .get_friend(SteamId::from_raw(steam_id64.get_u64().1))
            .name()
    }
}
