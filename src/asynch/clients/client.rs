use crate::models::requests::{Request, XRPLRequest};
use alloc::string::String;
use url::Url;

use super::exceptions::XRPLClientResult;

#[allow(async_fn_in_trait)]
pub trait XRPLClient {
    async fn request_impl(&self, request: XRPLRequest) -> XRPLClientResult<String>;

    fn get_host(&self) -> Url;

    fn set_request_id(&self, request: &mut XRPLRequest) {
        let common_fields = request.get_common_fields_mut();
        if common_fields.id.is_none() {
            #[cfg(feature = "std")]
            {
                common_fields.id = Some(self.get_random_id());
            }
            #[cfg(not(feature = "std"))]
            unimplemented!(
                "Random ID generation is not supported in no_std. Please provide an ID."
            );
        }
    }

    /// Generate a random id.
    #[cfg(feature = "std")]
    fn get_random_id(&self) -> String {
        use alloc::string::ToString;

        rand::random::<u32>().to_string()
    }
}
