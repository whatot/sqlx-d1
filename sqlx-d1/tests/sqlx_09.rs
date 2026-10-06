use sqlx_d1::sqlx_core::{decode::Decode, encode::Encode, executor::Execute, types::Type};
use sqlx_d1::{AssertSqlSafe, D1, SqlStr};
use std::borrow::Cow;

#[test]
fn smart_pointer_types_keep_their_d1_contract() {
    fn owned<T: for<'q> Encode<'q, D1> + for<'r> Decode<'r, D1> + Type<D1>>() {}
    owned::<String>();
    owned::<Box<str>>();
    owned::<Box<[u8]>>();
    fn cow<'q, 'r>()
    where
        Cow<'q, str>: Encode<'q, D1> + Decode<'r, D1> + Type<D1>,
    {
    }
    cow();
    assert_eq!(
        <str as Type<D1>>::type_info(),
        <String as Type<D1>>::type_info()
    );
    assert_eq!(
        <Box<str> as Type<D1>>::type_info(),
        <String as Type<D1>>::type_info()
    );
    assert_eq!(
        <Cow<'_, str> as Type<D1>>::type_info(),
        <String as Type<D1>>::type_info()
    );
    assert_eq!(
        <Box<[u8]> as Type<D1>>::type_info(),
        <Vec<u8> as Type<D1>>::type_info()
    );
}

#[test]
fn queries_own_dynamic_sql_and_accept_static_sql() {
    let query = {
        let sql = String::from("SELECT 1");
        sqlx_d1::query(AssertSqlSafe(sql))
    };
    assert_eq!(query.sql().as_str(), "SELECT 1");
    assert_eq!(sqlx_d1::query("SELECT 2").sql().as_str(), "SELECT 2");
    assert_eq!(
        sqlx_d1::query_as::<(i64,)>(SqlStr::from_static("SELECT 3"))
            .sql()
            .as_str(),
        "SELECT 3"
    );
    assert_eq!(
        sqlx_d1::query_scalar::<i64>("SELECT 4").sql().as_str(),
        "SELECT 4"
    );
}
