//! `GET /_matrix/client/*/user/mutual_rooms/{user_id}`
//!
//! Get mutual rooms with another user.

pub mod unstable {
    //! `/unstable/` ([spec])
    //!
    //! [spec]: https://github.com/matrix-org/matrix-spec-proposals/blob/hs/shared-rooms/proposals/2666-get-rooms-in-common.md

    use ruma_common::{
        api::{request, response, Metadata},
        metadata, OwnedRoomId, OwnedUserId,
    };

    const METADATA: Metadata = metadata! {
        method: GET,
        rate_limited: true,
        authentication: AccessToken,
        history: {
            unstable => "/_matrix/client/unstable/uk.half-shot.msc2666/user/mutual_rooms/:user_id",
        }
    };

    /// Request type for the `mutual_rooms` endpoint.
    #[request(error = crate::Error)]
    pub struct Request {
        /// The user to search mutual rooms for.
        #[ruma_api(path)]
        pub user_id: OwnedUserId,
    }

    /// Response type for the `mutual_rooms` endpoint.
    #[response(error = crate::Error)]
    pub struct Response {
        /// A list of rooms the user is in together with the authenticated user.
        pub joined: Vec<OwnedRoomId>,
    }

    impl Request {
        /// Creates a new `Request` with the given user id.
        pub fn new(user_id: OwnedUserId) -> Self {
            Self { user_id }
        }
    }

    impl Response {
        /// Creates a `Response` with the given room ids.
        pub fn new(joined: Vec<OwnedRoomId>) -> Self {
            Self { joined }
        }
    }
}
