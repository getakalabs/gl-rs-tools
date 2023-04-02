pub mod authentication;
pub mod guard;
pub mod middleware;
pub mod options;

pub use authentication::AuthenticationFuture;
pub use guard::Guard;
pub use middleware::GuardMiddleware;
pub use options::GuardOptions;

use mongodb::Database;
use crate::Errors;
use crate::Paseto;
pub type GuardParams<R, T> = Option<fn(Database, GuardOptions<R>, Paseto) -> Result<T, Errors>>;

