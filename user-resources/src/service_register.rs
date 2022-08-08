use std::sync::Arc;

use tracing::info;
use x_core::{traits::{users::service::DynUsersService, oauth2::security::DynSecurity}, connection_pool::PgConnPool, config::AppConfig};
use x_core::service::defualt_sucurity_service::DefualtSecurityService;
use crate::{service::user_service::UserService, repository::user_repository::PostgresUsersRepository};


#[derive(Clone)]
pub struct ServiceRegister {
    pub user_service: DynUsersService,
    pub security_utils: DynSecurity,
}

impl ServiceRegister {
    pub fn new(pool: PgConnPool, config: Arc<AppConfig>) ->Self {
        info!("initializing utility services...");
        let security_utils =  Arc::new(DefualtSecurityService::new(config));

        info!("utility user_services initialized, building feature services...");
        let user_repository = Arc::new(PostgresUsersRepository::new(pool));
        let user_service = Arc::new(UserService::new(user_repository, security_utils.clone()));
        Self {
            user_service,
            security_utils,
        }
    }
    
}
