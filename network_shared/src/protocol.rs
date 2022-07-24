use naia_shared::Protocolize;

mod auth;
pub use auth::Auth;

#[derive(Protocolize)]
pub enum Protocol {
    Auth(Auth)
}