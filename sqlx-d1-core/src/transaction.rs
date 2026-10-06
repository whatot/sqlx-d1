use sqlx_core::sql_str::SqlStr;

use crate::D1Connection;

pub struct D1TransactionManager;

impl sqlx_core::transaction::TransactionManager for D1TransactionManager {
    type Database = crate::D1;

    fn begin<'conn>(
        #[allow(unused)]
        conn: &'conn mut <Self::Database as sqlx_core::database::Database>::Connection,
        #[allow(unused)] statement: Option<SqlStr>,
    ) -> impl Future<Output = Result<(), sqlx_core::Error>> + Send + 'conn {
        #[cfg(not(target_arch = "wasm32"))]
        {
            async { unreachable!("Native `TransactionManager` impl") }
        }
        #[cfg(target_arch = "wasm32")]
        {
            async { Ok(()) }
        }
    }

    fn commit(
        #[allow(unused)] conn: &mut <Self::Database as sqlx_core::database::Database>::Connection,
    ) -> impl Future<Output = Result<(), sqlx_core::Error>> + Send + '_ {
        #[cfg(not(target_arch = "wasm32"))]
        {
            async { unreachable!("Native `TransactionManager` impl") }
        }
        #[cfg(target_arch = "wasm32")]
        {
            async { Ok(()) }
        }
    }

    fn rollback(
        #[allow(unused)] conn: &mut <Self::Database as sqlx_core::database::Database>::Connection,
    ) -> impl Future<Output = Result<(), sqlx_core::Error>> + Send + '_ {
        #[cfg(not(target_arch = "wasm32"))]
        {
            async { unreachable!("Native `TransactionManager` impl") }
        }
        #[cfg(target_arch = "wasm32")]
        {
            async { Ok(()) }
        }
    }

    fn start_rollback(_: &mut <Self::Database as sqlx_core::database::Database>::Connection) {
        /* do nothing */
    }

    fn get_transaction_depth(#[allow(unused)] conn: &D1Connection) -> usize {
        1 // dummy since this will never be used because d1 does not support transactions
    }
}
