use super::sqlite3types::*;
static mut API: *mut sqlite3_api_routines = std::ptr::null_mut();
pub unsafe fn init_api_routines(api: *mut sqlite3_api_routines) -> crate::types::Result<()> {
    API = api;
    Ok(())
}
pub unsafe fn sqlite3_aggregate_context(
    arg1: *mut sqlite3_context,
    nBytes: ::std::os::raw::c_int,
) -> *mut ::std::os::raw::c_void {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).aggregate_context.unwrap_unchecked())(arg1, nBytes)
}
pub unsafe fn sqlite3_aggregate_count(arg1: *mut sqlite3_context) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).aggregate_count.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_bind_blob(
    arg1: *mut sqlite3_stmt,
    arg2: ::std::os::raw::c_int,
    arg3: *const ::std::os::raw::c_void,
    n: ::std::os::raw::c_int,
    arg4: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).bind_blob.unwrap_unchecked())(arg1, arg2, arg3, n, arg4)
}
pub unsafe fn sqlite3_bind_double(
    arg1: *mut sqlite3_stmt,
    arg2: ::std::os::raw::c_int,
    arg3: f64,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).bind_double.unwrap_unchecked())(arg1, arg2, arg3)
}
pub unsafe fn sqlite3_bind_int(
    arg1: *mut sqlite3_stmt,
    arg2: ::std::os::raw::c_int,
    arg3: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).bind_int.unwrap_unchecked())(arg1, arg2, arg3)
}
pub unsafe fn sqlite3_bind_int64(
    arg1: *mut sqlite3_stmt,
    arg2: ::std::os::raw::c_int,
    arg3: sqlite_int64,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).bind_int64.unwrap_unchecked())(arg1, arg2, arg3)
}
pub unsafe fn sqlite3_bind_null(
    arg1: *mut sqlite3_stmt,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).bind_null.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_bind_parameter_count(arg1: *mut sqlite3_stmt) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).bind_parameter_count.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_bind_parameter_index(
    arg1: *mut sqlite3_stmt,
    zName: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).bind_parameter_index.unwrap_unchecked())(arg1, zName)
}
pub unsafe fn sqlite3_bind_parameter_name(
    arg1: *mut sqlite3_stmt,
    arg2: ::std::os::raw::c_int,
) -> *const ::std::os::raw::c_char {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).bind_parameter_name.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_bind_text(
    arg1: *mut sqlite3_stmt,
    arg2: ::std::os::raw::c_int,
    arg3: *const ::std::os::raw::c_char,
    n: ::std::os::raw::c_int,
    arg4: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).bind_text.unwrap_unchecked())(arg1, arg2, arg3, n, arg4)
}
pub unsafe fn sqlite3_bind_text16(
    arg1: *mut sqlite3_stmt,
    arg2: ::std::os::raw::c_int,
    arg3: *const ::std::os::raw::c_void,
    arg4: ::std::os::raw::c_int,
    arg5: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).bind_text16.unwrap_unchecked())(arg1, arg2, arg3, arg4, arg5)
}
pub unsafe fn sqlite3_bind_value(
    arg1: *mut sqlite3_stmt,
    arg2: ::std::os::raw::c_int,
    arg3: *const sqlite3_value,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).bind_value.unwrap_unchecked())(arg1, arg2, arg3)
}
pub unsafe fn sqlite3_busy_handler(
    arg1: *mut sqlite3,
    arg2: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    arg3: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).busy_handler.unwrap_unchecked())(arg1, arg2, arg3)
}
pub unsafe fn sqlite3_busy_timeout(
    arg1: *mut sqlite3,
    ms: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).busy_timeout.unwrap_unchecked())(arg1, ms)
}
pub unsafe fn sqlite3_changes(arg1: *mut sqlite3) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).changes.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_close(arg1: *mut sqlite3) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).close.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_collation_needed(
    arg1: *mut sqlite3,
    arg2: *mut ::std::os::raw::c_void,
    arg3: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *mut sqlite3,
            eTextRep: ::std::os::raw::c_int,
            arg3: *const ::std::os::raw::c_char,
        ),
    >,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).collation_needed.unwrap_unchecked())(arg1, arg2, arg3)
}
pub unsafe fn sqlite3_collation_needed16(
    arg1: *mut sqlite3,
    arg2: *mut ::std::os::raw::c_void,
    arg3: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *mut sqlite3,
            eTextRep: ::std::os::raw::c_int,
            arg3: *const ::std::os::raw::c_void,
        ),
    >,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).collation_needed16.unwrap_unchecked())(arg1, arg2, arg3)
}
pub unsafe fn sqlite3_column_blob(
    arg1: *mut sqlite3_stmt,
    iCol: ::std::os::raw::c_int,
) -> *const ::std::os::raw::c_void {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).column_blob.unwrap_unchecked())(arg1, iCol)
}
pub unsafe fn sqlite3_column_bytes(
    arg1: *mut sqlite3_stmt,
    iCol: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).column_bytes.unwrap_unchecked())(arg1, iCol)
}
pub unsafe fn sqlite3_column_bytes16(
    arg1: *mut sqlite3_stmt,
    iCol: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).column_bytes16.unwrap_unchecked())(arg1, iCol)
}
pub unsafe fn sqlite3_column_count(pStmt: *mut sqlite3_stmt) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).column_count.unwrap_unchecked())(pStmt)
}
pub unsafe fn sqlite3_column_database_name(
    arg1: *mut sqlite3_stmt,
    arg2: ::std::os::raw::c_int,
) -> *const ::std::os::raw::c_char {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).column_database_name.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_column_database_name16(
    arg1: *mut sqlite3_stmt,
    arg2: ::std::os::raw::c_int,
) -> *const ::std::os::raw::c_void {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).column_database_name16.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_column_decltype(
    arg1: *mut sqlite3_stmt,
    i: ::std::os::raw::c_int,
) -> *const ::std::os::raw::c_char {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).column_decltype.unwrap_unchecked())(arg1, i)
}
pub unsafe fn sqlite3_column_decltype16(
    arg1: *mut sqlite3_stmt,
    arg2: ::std::os::raw::c_int,
) -> *const ::std::os::raw::c_void {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).column_decltype16.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_column_double(arg1: *mut sqlite3_stmt, iCol: ::std::os::raw::c_int) -> f64 {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).column_double.unwrap_unchecked())(arg1, iCol)
}
pub unsafe fn sqlite3_column_int(
    arg1: *mut sqlite3_stmt,
    iCol: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).column_int.unwrap_unchecked())(arg1, iCol)
}
pub unsafe fn sqlite3_column_int64(
    arg1: *mut sqlite3_stmt,
    iCol: ::std::os::raw::c_int,
) -> sqlite_int64 {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).column_int64.unwrap_unchecked())(arg1, iCol)
}
pub unsafe fn sqlite3_column_name(
    arg1: *mut sqlite3_stmt,
    arg2: ::std::os::raw::c_int,
) -> *const ::std::os::raw::c_char {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).column_name.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_column_name16(
    arg1: *mut sqlite3_stmt,
    arg2: ::std::os::raw::c_int,
) -> *const ::std::os::raw::c_void {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).column_name16.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_column_origin_name(
    arg1: *mut sqlite3_stmt,
    arg2: ::std::os::raw::c_int,
) -> *const ::std::os::raw::c_char {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).column_origin_name.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_column_origin_name16(
    arg1: *mut sqlite3_stmt,
    arg2: ::std::os::raw::c_int,
) -> *const ::std::os::raw::c_void {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).column_origin_name16.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_column_table_name(
    arg1: *mut sqlite3_stmt,
    arg2: ::std::os::raw::c_int,
) -> *const ::std::os::raw::c_char {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).column_table_name.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_column_table_name16(
    arg1: *mut sqlite3_stmt,
    arg2: ::std::os::raw::c_int,
) -> *const ::std::os::raw::c_void {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).column_table_name16.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_column_text(
    arg1: *mut sqlite3_stmt,
    iCol: ::std::os::raw::c_int,
) -> *const ::std::os::raw::c_uchar {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).column_text.unwrap_unchecked())(arg1, iCol)
}
pub unsafe fn sqlite3_column_text16(
    arg1: *mut sqlite3_stmt,
    iCol: ::std::os::raw::c_int,
) -> *const ::std::os::raw::c_void {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).column_text16.unwrap_unchecked())(arg1, iCol)
}
pub unsafe fn sqlite3_column_type(
    arg1: *mut sqlite3_stmt,
    iCol: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).column_type.unwrap_unchecked())(arg1, iCol)
}
pub unsafe fn sqlite3_column_value(
    arg1: *mut sqlite3_stmt,
    iCol: ::std::os::raw::c_int,
) -> *mut sqlite3_value {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).column_value.unwrap_unchecked())(arg1, iCol)
}
pub unsafe fn sqlite3_commit_hook(
    arg1: *mut sqlite3,
    arg2: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    >,
    arg3: *mut ::std::os::raw::c_void,
) -> *mut ::std::os::raw::c_void {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).commit_hook.unwrap_unchecked())(arg1, arg2, arg3)
}
pub unsafe fn sqlite3_complete(sql: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).complete.unwrap_unchecked())(sql)
}
pub unsafe fn sqlite3_complete16(sql: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).complete16.unwrap_unchecked())(sql)
}
pub unsafe fn sqlite3_create_collation(
    arg1: *mut sqlite3,
    arg2: *const ::std::os::raw::c_char,
    arg3: ::std::os::raw::c_int,
    arg4: *mut ::std::os::raw::c_void,
    arg5: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: ::std::os::raw::c_int,
            arg3: *const ::std::os::raw::c_void,
            arg4: ::std::os::raw::c_int,
            arg5: *const ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).create_collation.unwrap_unchecked())(arg1, arg2, arg3, arg4, arg5)
}
pub unsafe fn sqlite3_create_collation16(
    arg1: *mut sqlite3,
    arg2: *const ::std::os::raw::c_void,
    arg3: ::std::os::raw::c_int,
    arg4: *mut ::std::os::raw::c_void,
    arg5: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: ::std::os::raw::c_int,
            arg3: *const ::std::os::raw::c_void,
            arg4: ::std::os::raw::c_int,
            arg5: *const ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).create_collation16.unwrap_unchecked())(arg1, arg2, arg3, arg4, arg5)
}
pub unsafe fn sqlite3_create_function(
    arg1: *mut sqlite3,
    arg2: *const ::std::os::raw::c_char,
    arg3: ::std::os::raw::c_int,
    arg4: ::std::os::raw::c_int,
    arg5: *mut ::std::os::raw::c_void,
    xFunc: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_context,
            arg2: ::std::os::raw::c_int,
            arg3: *mut *mut sqlite3_value,
        ),
    >,
    xStep: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_context,
            arg2: ::std::os::raw::c_int,
            arg3: *mut *mut sqlite3_value,
        ),
    >,
    xFinal: ::std::option::Option<unsafe extern "C" fn(arg1: *mut sqlite3_context)>,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).create_function.unwrap_unchecked())(arg1, arg2, arg3, arg4, arg5, xFunc, xStep, xFinal)
}
pub unsafe fn sqlite3_create_function16(
    arg1: *mut sqlite3,
    arg2: *const ::std::os::raw::c_void,
    arg3: ::std::os::raw::c_int,
    arg4: ::std::os::raw::c_int,
    arg5: *mut ::std::os::raw::c_void,
    xFunc: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_context,
            arg2: ::std::os::raw::c_int,
            arg3: *mut *mut sqlite3_value,
        ),
    >,
    xStep: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_context,
            arg2: ::std::os::raw::c_int,
            arg3: *mut *mut sqlite3_value,
        ),
    >,
    xFinal: ::std::option::Option<unsafe extern "C" fn(arg1: *mut sqlite3_context)>,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).create_function16.unwrap_unchecked())(
        arg1, arg2, arg3, arg4, arg5, xFunc, xStep, xFinal,
    )
}
pub unsafe fn sqlite3_create_module(
    arg1: *mut sqlite3,
    arg2: *const ::std::os::raw::c_char,
    arg3: *const sqlite3_module,
    arg4: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).create_module.unwrap_unchecked())(arg1, arg2, arg3, arg4)
}
pub unsafe fn sqlite3_data_count(pStmt: *mut sqlite3_stmt) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).data_count.unwrap_unchecked())(pStmt)
}
pub unsafe fn sqlite3_db_handle(arg1: *mut sqlite3_stmt) -> *mut sqlite3 {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).db_handle.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_declare_vtab(
    arg1: *mut sqlite3,
    arg2: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).declare_vtab.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_enable_shared_cache(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).enable_shared_cache.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_errcode(db: *mut sqlite3) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).errcode.unwrap_unchecked())(db)
}
pub unsafe fn sqlite3_errmsg(arg1: *mut sqlite3) -> *const ::std::os::raw::c_char {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).errmsg.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_errmsg16(arg1: *mut sqlite3) -> *const ::std::os::raw::c_void {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).errmsg16.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_exec(
    arg1: *mut sqlite3,
    arg2: *const ::std::os::raw::c_char,
    arg3: sqlite3_callback,
    arg4: *mut ::std::os::raw::c_void,
    arg5: *mut *mut ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).exec.unwrap_unchecked())(arg1, arg2, arg3, arg4, arg5)
}
pub unsafe fn sqlite3_expired(arg1: *mut sqlite3_stmt) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).expired.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).finalize.unwrap_unchecked())(pStmt)
}
pub unsafe fn sqlite3_free(arg1: *mut ::std::os::raw::c_void) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).free.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_free_table(result: *mut *mut ::std::os::raw::c_char) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).free_table.unwrap_unchecked())(result)
}
pub unsafe fn sqlite3_get_autocommit(arg1: *mut sqlite3) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).get_autocommit.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_get_auxdata(
    arg1: *mut sqlite3_context,
    arg2: ::std::os::raw::c_int,
) -> *mut ::std::os::raw::c_void {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).get_auxdata.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_get_table(
    arg1: *mut sqlite3,
    arg2: *const ::std::os::raw::c_char,
    arg3: *mut *mut *mut ::std::os::raw::c_char,
    arg4: *mut ::std::os::raw::c_int,
    arg5: *mut ::std::os::raw::c_int,
    arg6: *mut *mut ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).get_table.unwrap_unchecked())(arg1, arg2, arg3, arg4, arg5, arg6)
}
pub unsafe fn sqlite3_global_recover() -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).global_recover.unwrap_unchecked())()
}
pub unsafe fn sqlite3_interrupt(arg1: *mut sqlite3) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).interruptx.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_last_insert_rowid(arg1: *mut sqlite3) -> sqlite_int64 {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).last_insert_rowid.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_libversion() -> *const ::std::os::raw::c_char {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).libversion.unwrap_unchecked())()
}
pub unsafe fn sqlite3_libversion_number() -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).libversion_number.unwrap_unchecked())()
}
pub unsafe fn sqlite3_malloc(arg1: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_void {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).malloc.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_mprintf(
) -> unsafe extern "C" fn(arg1: *const ::std::os::raw::c_char, ...) -> *mut ::std::os::raw::c_char {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    (*API).mprintf.unwrap_unchecked()
}
pub unsafe fn sqlite3_open(
    arg1: *const ::std::os::raw::c_char,
    arg2: *mut *mut sqlite3,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).open.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_open16(
    arg1: *const ::std::os::raw::c_void,
    arg2: *mut *mut sqlite3,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).open16.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_prepare(
    arg1: *mut sqlite3,
    arg2: *const ::std::os::raw::c_char,
    arg3: ::std::os::raw::c_int,
    arg4: *mut *mut sqlite3_stmt,
    arg5: *mut *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).prepare.unwrap_unchecked())(arg1, arg2, arg3, arg4, arg5)
}
pub unsafe fn sqlite3_prepare16(
    arg1: *mut sqlite3,
    arg2: *const ::std::os::raw::c_void,
    arg3: ::std::os::raw::c_int,
    arg4: *mut *mut sqlite3_stmt,
    arg5: *mut *const ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).prepare16.unwrap_unchecked())(arg1, arg2, arg3, arg4, arg5)
}
pub unsafe fn sqlite3_profile(
    arg1: *mut sqlite3,
    arg2: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_char,
            arg3: sqlite_uint64,
        ),
    >,
    arg3: *mut ::std::os::raw::c_void,
) -> *mut ::std::os::raw::c_void {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).profile.unwrap_unchecked())(arg1, arg2, arg3)
}
pub unsafe fn sqlite3_progress_handler(
    arg1: *mut sqlite3,
    arg2: ::std::os::raw::c_int,
    arg3: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    >,
    arg4: *mut ::std::os::raw::c_void,
) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).progress_handler.unwrap_unchecked())(arg1, arg2, arg3, arg4)
}
pub unsafe fn sqlite3_realloc(
    arg1: *mut ::std::os::raw::c_void,
    arg2: ::std::os::raw::c_int,
) -> *mut ::std::os::raw::c_void {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).realloc.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_reset(pStmt: *mut sqlite3_stmt) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).reset.unwrap_unchecked())(pStmt)
}
pub unsafe fn sqlite3_result_blob(
    arg1: *mut sqlite3_context,
    arg2: *const ::std::os::raw::c_void,
    arg3: ::std::os::raw::c_int,
    arg4: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).result_blob.unwrap_unchecked())(arg1, arg2, arg3, arg4)
}
pub unsafe fn sqlite3_result_double(arg1: *mut sqlite3_context, arg2: f64) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).result_double.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_result_error(
    arg1: *mut sqlite3_context,
    arg2: *const ::std::os::raw::c_char,
    arg3: ::std::os::raw::c_int,
) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).result_error.unwrap_unchecked())(arg1, arg2, arg3)
}
pub unsafe fn sqlite3_result_error16(
    arg1: *mut sqlite3_context,
    arg2: *const ::std::os::raw::c_void,
    arg3: ::std::os::raw::c_int,
) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).result_error16.unwrap_unchecked())(arg1, arg2, arg3)
}
pub unsafe fn sqlite3_result_int(arg1: *mut sqlite3_context, arg2: ::std::os::raw::c_int) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).result_int.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_result_int64(arg1: *mut sqlite3_context, arg2: sqlite_int64) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).result_int64.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_result_null(arg1: *mut sqlite3_context) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).result_null.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_result_text(
    arg1: *mut sqlite3_context,
    arg2: *const ::std::os::raw::c_char,
    arg3: ::std::os::raw::c_int,
    arg4: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).result_text.unwrap_unchecked())(arg1, arg2, arg3, arg4)
}
pub unsafe fn sqlite3_result_text16(
    arg1: *mut sqlite3_context,
    arg2: *const ::std::os::raw::c_void,
    arg3: ::std::os::raw::c_int,
    arg4: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).result_text16.unwrap_unchecked())(arg1, arg2, arg3, arg4)
}
pub unsafe fn sqlite3_result_text16be(
    arg1: *mut sqlite3_context,
    arg2: *const ::std::os::raw::c_void,
    arg3: ::std::os::raw::c_int,
    arg4: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).result_text16be.unwrap_unchecked())(arg1, arg2, arg3, arg4)
}
pub unsafe fn sqlite3_result_text16le(
    arg1: *mut sqlite3_context,
    arg2: *const ::std::os::raw::c_void,
    arg3: ::std::os::raw::c_int,
    arg4: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).result_text16le.unwrap_unchecked())(arg1, arg2, arg3, arg4)
}
pub unsafe fn sqlite3_result_value(arg1: *mut sqlite3_context, arg2: *mut sqlite3_value) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).result_value.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_rollback_hook(
    arg1: *mut sqlite3,
    arg2: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    arg3: *mut ::std::os::raw::c_void,
) -> *mut ::std::os::raw::c_void {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).rollback_hook.unwrap_unchecked())(arg1, arg2, arg3)
}
pub unsafe fn sqlite3_set_authorizer(
    arg1: *mut sqlite3,
    arg2: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: ::std::os::raw::c_int,
            arg3: *const ::std::os::raw::c_char,
            arg4: *const ::std::os::raw::c_char,
            arg5: *const ::std::os::raw::c_char,
            arg6: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    arg3: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).set_authorizer.unwrap_unchecked())(arg1, arg2, arg3)
}
pub unsafe fn sqlite3_set_auxdata(
    arg1: *mut sqlite3_context,
    arg2: ::std::os::raw::c_int,
    arg3: *mut ::std::os::raw::c_void,
    arg4: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).set_auxdata.unwrap_unchecked())(arg1, arg2, arg3, arg4)
}
pub unsafe fn sqlite3_snprintf() -> unsafe extern "C" fn(
    arg1: ::std::os::raw::c_int,
    arg2: *mut ::std::os::raw::c_char,
    arg3: *const ::std::os::raw::c_char,
    ...
) -> *mut ::std::os::raw::c_char {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    (*API).xsnprintf.unwrap_unchecked()
}
pub unsafe fn sqlite3_step(arg1: *mut sqlite3_stmt) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).step.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_table_column_metadata(
    arg1: *mut sqlite3,
    arg2: *const ::std::os::raw::c_char,
    arg3: *const ::std::os::raw::c_char,
    arg4: *const ::std::os::raw::c_char,
    arg5: *mut *const ::std::os::raw::c_char,
    arg6: *mut *const ::std::os::raw::c_char,
    arg7: *mut ::std::os::raw::c_int,
    arg8: *mut ::std::os::raw::c_int,
    arg9: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).table_column_metadata.unwrap_unchecked())(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9,
    )
}
pub unsafe fn sqlite3_thread_cleanup() {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).thread_cleanup.unwrap_unchecked())()
}
pub unsafe fn sqlite3_total_changes(arg1: *mut sqlite3) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).total_changes.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_trace(
    arg1: *mut sqlite3,
    xTrace: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_char,
        ),
    >,
    arg2: *mut ::std::os::raw::c_void,
) -> *mut ::std::os::raw::c_void {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).trace.unwrap_unchecked())(arg1, xTrace, arg2)
}
pub unsafe fn sqlite3_transfer_bindings(
    arg1: *mut sqlite3_stmt,
    arg2: *mut sqlite3_stmt,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).transfer_bindings.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_update_hook(
    arg1: *mut sqlite3,
    arg2: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: ::std::os::raw::c_int,
            arg3: *const ::std::os::raw::c_char,
            arg4: *const ::std::os::raw::c_char,
            arg5: sqlite_int64,
        ),
    >,
    arg3: *mut ::std::os::raw::c_void,
) -> *mut ::std::os::raw::c_void {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).update_hook.unwrap_unchecked())(arg1, arg2, arg3)
}
pub unsafe fn sqlite3_user_data(arg1: *mut sqlite3_context) -> *mut ::std::os::raw::c_void {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).user_data.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_value_blob(arg1: *mut sqlite3_value) -> *const ::std::os::raw::c_void {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).value_blob.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_value_bytes(arg1: *mut sqlite3_value) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).value_bytes.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_value_bytes16(arg1: *mut sqlite3_value) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).value_bytes16.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_value_double(arg1: *mut sqlite3_value) -> f64 {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).value_double.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_value_int(arg1: *mut sqlite3_value) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).value_int.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_value_int64(arg1: *mut sqlite3_value) -> sqlite_int64 {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).value_int64.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_value_numeric_type(arg1: *mut sqlite3_value) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).value_numeric_type.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_value_text(arg1: *mut sqlite3_value) -> *const ::std::os::raw::c_uchar {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).value_text.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_value_text16(arg1: *mut sqlite3_value) -> *const ::std::os::raw::c_void {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).value_text16.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_value_text16be(arg1: *mut sqlite3_value) -> *const ::std::os::raw::c_void {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).value_text16be.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_value_text16le(arg1: *mut sqlite3_value) -> *const ::std::os::raw::c_void {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).value_text16le.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_value_type(arg1: *mut sqlite3_value) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).value_type.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_vmprintf(
    arg1: *const ::std::os::raw::c_char,
    arg2: va_list,
) -> *mut ::std::os::raw::c_char {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).vmprintf.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_overload_function(
    arg1: *mut sqlite3,
    zFuncName: *const ::std::os::raw::c_char,
    nArg: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).overload_function.unwrap_unchecked())(arg1, zFuncName, nArg)
}
pub unsafe fn sqlite3_prepare_v2(
    arg1: *mut sqlite3,
    arg2: *const ::std::os::raw::c_char,
    arg3: ::std::os::raw::c_int,
    arg4: *mut *mut sqlite3_stmt,
    arg5: *mut *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).prepare_v2.unwrap_unchecked())(arg1, arg2, arg3, arg4, arg5)
}
pub unsafe fn sqlite3_prepare16_v2(
    arg1: *mut sqlite3,
    arg2: *const ::std::os::raw::c_void,
    arg3: ::std::os::raw::c_int,
    arg4: *mut *mut sqlite3_stmt,
    arg5: *mut *const ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).prepare16_v2.unwrap_unchecked())(arg1, arg2, arg3, arg4, arg5)
}
pub unsafe fn sqlite3_clear_bindings(arg1: *mut sqlite3_stmt) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).clear_bindings.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_create_module_v2(
    arg1: *mut sqlite3,
    arg2: *const ::std::os::raw::c_char,
    arg3: *const sqlite3_module,
    arg4: *mut ::std::os::raw::c_void,
    xDestroy: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).create_module_v2.unwrap_unchecked())(arg1, arg2, arg3, arg4, xDestroy)
}
pub unsafe fn sqlite3_bind_zeroblob(
    arg1: *mut sqlite3_stmt,
    arg2: ::std::os::raw::c_int,
    arg3: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).bind_zeroblob.unwrap_unchecked())(arg1, arg2, arg3)
}
pub unsafe fn sqlite3_blob_bytes(arg1: *mut sqlite3_blob) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).blob_bytes.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_blob_close(arg1: *mut sqlite3_blob) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).blob_close.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_blob_open(
    arg1: *mut sqlite3,
    arg2: *const ::std::os::raw::c_char,
    arg3: *const ::std::os::raw::c_char,
    arg4: *const ::std::os::raw::c_char,
    arg5: sqlite3_int64,
    arg6: ::std::os::raw::c_int,
    arg7: *mut *mut sqlite3_blob,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).blob_open.unwrap_unchecked())(arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}
pub unsafe fn sqlite3_blob_read(
    arg1: *mut sqlite3_blob,
    arg2: *mut ::std::os::raw::c_void,
    arg3: ::std::os::raw::c_int,
    arg4: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).blob_read.unwrap_unchecked())(arg1, arg2, arg3, arg4)
}
pub unsafe fn sqlite3_blob_write(
    arg1: *mut sqlite3_blob,
    arg2: *const ::std::os::raw::c_void,
    arg3: ::std::os::raw::c_int,
    arg4: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).blob_write.unwrap_unchecked())(arg1, arg2, arg3, arg4)
}
pub unsafe fn sqlite3_create_collation_v2(
    arg1: *mut sqlite3,
    arg2: *const ::std::os::raw::c_char,
    arg3: ::std::os::raw::c_int,
    arg4: *mut ::std::os::raw::c_void,
    arg5: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: ::std::os::raw::c_int,
            arg3: *const ::std::os::raw::c_void,
            arg4: ::std::os::raw::c_int,
            arg5: *const ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    arg6: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).create_collation_v2.unwrap_unchecked())(arg1, arg2, arg3, arg4, arg5, arg6)
}
pub unsafe fn sqlite3_file_control(
    arg1: *mut sqlite3,
    arg2: *const ::std::os::raw::c_char,
    arg3: ::std::os::raw::c_int,
    arg4: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).file_control.unwrap_unchecked())(arg1, arg2, arg3, arg4)
}
pub unsafe fn sqlite3_memory_highwater(arg1: ::std::os::raw::c_int) -> sqlite3_int64 {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).memory_highwater.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_memory_used() -> sqlite3_int64 {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).memory_used.unwrap_unchecked())()
}
pub unsafe fn sqlite3_mutex_alloc(arg1: ::std::os::raw::c_int) -> *mut sqlite3_mutex {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).mutex_alloc.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_mutex_enter(arg1: *mut sqlite3_mutex) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).mutex_enter.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_mutex_free(arg1: *mut sqlite3_mutex) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).mutex_free.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_mutex_leave(arg1: *mut sqlite3_mutex) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).mutex_leave.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_mutex_try(arg1: *mut sqlite3_mutex) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).mutex_try.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_open_v2(
    arg1: *const ::std::os::raw::c_char,
    arg2: *mut *mut sqlite3,
    arg3: ::std::os::raw::c_int,
    arg4: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).open_v2.unwrap_unchecked())(arg1, arg2, arg3, arg4)
}
pub unsafe fn sqlite3_release_memory(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).release_memory.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_result_error_nomem(arg1: *mut sqlite3_context) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).result_error_nomem.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_result_error_toobig(arg1: *mut sqlite3_context) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).result_error_toobig.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_sleep(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).sleep.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_soft_heap_limit(arg1: ::std::os::raw::c_int) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).soft_heap_limit.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_vfs_find(arg1: *const ::std::os::raw::c_char) -> *mut sqlite3_vfs {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).vfs_find.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_vfs_register(
    arg1: *mut sqlite3_vfs,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).vfs_register.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_vfs_unregister(arg1: *mut sqlite3_vfs) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).vfs_unregister.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_threadsafe() -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).xthreadsafe.unwrap_unchecked())()
}
pub unsafe fn sqlite3_result_zeroblob(arg1: *mut sqlite3_context, arg2: ::std::os::raw::c_int) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).result_zeroblob.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_result_error_code(arg1: *mut sqlite3_context, arg2: ::std::os::raw::c_int) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).result_error_code.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_test_control(
) -> unsafe extern "C" fn(arg1: ::std::os::raw::c_int, ...) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    (*API).test_control.unwrap_unchecked()
}
pub unsafe fn sqlite3_randomness(arg1: ::std::os::raw::c_int, arg2: *mut ::std::os::raw::c_void) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).randomness.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_context_db_handle(arg1: *mut sqlite3_context) -> *mut sqlite3 {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).context_db_handle.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_extended_result_codes(
    arg1: *mut sqlite3,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).extended_result_codes.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_limit(
    arg1: *mut sqlite3,
    arg2: ::std::os::raw::c_int,
    arg3: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).limit.unwrap_unchecked())(arg1, arg2, arg3)
}
pub unsafe fn sqlite3_next_stmt(arg1: *mut sqlite3, arg2: *mut sqlite3_stmt) -> *mut sqlite3_stmt {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).next_stmt.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_sql(arg1: *mut sqlite3_stmt) -> *const ::std::os::raw::c_char {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).sql.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_status(
    arg1: ::std::os::raw::c_int,
    arg2: *mut ::std::os::raw::c_int,
    arg3: *mut ::std::os::raw::c_int,
    arg4: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).status.unwrap_unchecked())(arg1, arg2, arg3, arg4)
}
pub unsafe fn sqlite3_backup_finish(arg1: *mut sqlite3_backup) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).backup_finish.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_backup_init(
    arg1: *mut sqlite3,
    arg2: *const ::std::os::raw::c_char,
    arg3: *mut sqlite3,
    arg4: *const ::std::os::raw::c_char,
) -> *mut sqlite3_backup {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).backup_init.unwrap_unchecked())(arg1, arg2, arg3, arg4)
}
pub unsafe fn sqlite3_backup_pagecount(arg1: *mut sqlite3_backup) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).backup_pagecount.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_backup_remaining(arg1: *mut sqlite3_backup) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).backup_remaining.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_backup_step(
    arg1: *mut sqlite3_backup,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).backup_step.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_compileoption_get(
    arg1: ::std::os::raw::c_int,
) -> *const ::std::os::raw::c_char {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).compileoption_get.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_compileoption_used(
    arg1: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).compileoption_used.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_create_function_v2(
    arg1: *mut sqlite3,
    arg2: *const ::std::os::raw::c_char,
    arg3: ::std::os::raw::c_int,
    arg4: ::std::os::raw::c_int,
    arg5: *mut ::std::os::raw::c_void,
    xFunc: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_context,
            arg2: ::std::os::raw::c_int,
            arg3: *mut *mut sqlite3_value,
        ),
    >,
    xStep: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_context,
            arg2: ::std::os::raw::c_int,
            arg3: *mut *mut sqlite3_value,
        ),
    >,
    xFinal: ::std::option::Option<unsafe extern "C" fn(arg1: *mut sqlite3_context)>,
    xDestroy: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).create_function_v2.unwrap_unchecked())(
        arg1, arg2, arg3, arg4, arg5, xFunc, xStep, xFinal, xDestroy,
    )
}
pub unsafe fn sqlite3_db_config() -> unsafe extern "C" fn(
    arg1: *mut sqlite3,
    arg2: ::std::os::raw::c_int,
    ...
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    (*API).db_config.unwrap_unchecked()
}
pub unsafe fn sqlite3_db_mutex(arg1: *mut sqlite3) -> *mut sqlite3_mutex {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).db_mutex.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_db_status(
    arg1: *mut sqlite3,
    arg2: ::std::os::raw::c_int,
    arg3: *mut ::std::os::raw::c_int,
    arg4: *mut ::std::os::raw::c_int,
    arg5: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).db_status.unwrap_unchecked())(arg1, arg2, arg3, arg4, arg5)
}
pub unsafe fn sqlite3_extended_errcode(arg1: *mut sqlite3) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).extended_errcode.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_log(
) -> unsafe extern "C" fn(arg1: ::std::os::raw::c_int, arg2: *const ::std::os::raw::c_char, ...) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    (*API).log.unwrap_unchecked()
}
pub unsafe fn sqlite3_soft_heap_limit64(arg1: sqlite3_int64) -> sqlite3_int64 {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).soft_heap_limit64.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_sourceid() -> *const ::std::os::raw::c_char {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).sourceid.unwrap_unchecked())()
}
pub unsafe fn sqlite3_stmt_status(
    arg1: *mut sqlite3_stmt,
    arg2: ::std::os::raw::c_int,
    arg3: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).stmt_status.unwrap_unchecked())(arg1, arg2, arg3)
}
pub unsafe fn sqlite3_strnicmp(
    arg1: *const ::std::os::raw::c_char,
    arg2: *const ::std::os::raw::c_char,
    arg3: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).strnicmp.unwrap_unchecked())(arg1, arg2, arg3)
}
pub unsafe fn sqlite3_unlock_notify(
    arg1: *mut sqlite3,
    arg2: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut *mut ::std::os::raw::c_void, arg2: ::std::os::raw::c_int),
    >,
    arg3: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).unlock_notify.unwrap_unchecked())(arg1, arg2, arg3)
}
pub unsafe fn sqlite3_wal_autocheckpoint(
    arg1: *mut sqlite3,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).wal_autocheckpoint.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_wal_checkpoint(
    arg1: *mut sqlite3,
    arg2: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).wal_checkpoint.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_wal_hook(
    arg1: *mut sqlite3,
    arg2: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *mut sqlite3,
            arg3: *const ::std::os::raw::c_char,
            arg4: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    arg3: *mut ::std::os::raw::c_void,
) -> *mut ::std::os::raw::c_void {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).wal_hook.unwrap_unchecked())(arg1, arg2, arg3)
}
pub unsafe fn sqlite3_blob_reopen(
    arg1: *mut sqlite3_blob,
    arg2: sqlite3_int64,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).blob_reopen.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_vtab_config(
) -> unsafe extern "C" fn(arg1: *mut sqlite3, op: ::std::os::raw::c_int, ...) -> ::std::os::raw::c_int
{
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    (*API).vtab_config.unwrap_unchecked()
}
pub unsafe fn sqlite3_vtab_on_conflict(arg1: *mut sqlite3) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).vtab_on_conflict.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_close_v2(arg1: *mut sqlite3) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).close_v2.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_db_filename(
    arg1: *mut sqlite3,
    arg2: *const ::std::os::raw::c_char,
) -> *const ::std::os::raw::c_char {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).db_filename.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_db_readonly(
    arg1: *mut sqlite3,
    arg2: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).db_readonly.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_db_release_memory(arg1: *mut sqlite3) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).db_release_memory.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_errstr(arg1: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).errstr.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_stmt_busy(arg1: *mut sqlite3_stmt) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).stmt_busy.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_stmt_readonly(arg1: *mut sqlite3_stmt) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).stmt_readonly.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_stricmp(
    arg1: *const ::std::os::raw::c_char,
    arg2: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).stricmp.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_uri_boolean(
    arg1: *const ::std::os::raw::c_char,
    arg2: *const ::std::os::raw::c_char,
    arg3: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).uri_boolean.unwrap_unchecked())(arg1, arg2, arg3)
}
pub unsafe fn sqlite3_uri_int64(
    arg1: *const ::std::os::raw::c_char,
    arg2: *const ::std::os::raw::c_char,
    arg3: sqlite3_int64,
) -> sqlite3_int64 {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).uri_int64.unwrap_unchecked())(arg1, arg2, arg3)
}
pub unsafe fn sqlite3_uri_parameter(
    arg1: *const ::std::os::raw::c_char,
    arg2: *const ::std::os::raw::c_char,
) -> *const ::std::os::raw::c_char {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).uri_parameter.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_vsnprintf(
    arg1: ::std::os::raw::c_int,
    arg2: *mut ::std::os::raw::c_char,
    arg3: *const ::std::os::raw::c_char,
    arg4: va_list,
) -> *mut ::std::os::raw::c_char {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).xvsnprintf.unwrap_unchecked())(arg1, arg2, arg3, arg4)
}
pub unsafe fn sqlite3_wal_checkpoint_v2(
    arg1: *mut sqlite3,
    arg2: *const ::std::os::raw::c_char,
    arg3: ::std::os::raw::c_int,
    arg4: *mut ::std::os::raw::c_int,
    arg5: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).wal_checkpoint_v2.unwrap_unchecked())(arg1, arg2, arg3, arg4, arg5)
}
pub unsafe fn sqlite3_auto_extension(
    arg1: ::std::option::Option<unsafe extern "C" fn()>,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).auto_extension.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_bind_blob64(
    arg1: *mut sqlite3_stmt,
    arg2: ::std::os::raw::c_int,
    arg3: *const ::std::os::raw::c_void,
    arg4: sqlite3_uint64,
    arg5: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).bind_blob64.unwrap_unchecked())(arg1, arg2, arg3, arg4, arg5)
}
pub unsafe fn sqlite3_bind_text64(
    arg1: *mut sqlite3_stmt,
    arg2: ::std::os::raw::c_int,
    arg3: *const ::std::os::raw::c_char,
    arg4: sqlite3_uint64,
    arg5: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    arg6: ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).bind_text64.unwrap_unchecked())(arg1, arg2, arg3, arg4, arg5, arg6)
}
pub unsafe fn sqlite3_cancel_auto_extension(
    arg1: ::std::option::Option<unsafe extern "C" fn()>,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).cancel_auto_extension.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_load_extension(
    arg1: *mut sqlite3,
    arg2: *const ::std::os::raw::c_char,
    arg3: *const ::std::os::raw::c_char,
    arg4: *mut *mut ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).load_extension.unwrap_unchecked())(arg1, arg2, arg3, arg4)
}
pub unsafe fn sqlite3_malloc64(arg1: sqlite3_uint64) -> *mut ::std::os::raw::c_void {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).malloc64.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_msize(arg1: *mut ::std::os::raw::c_void) -> sqlite3_uint64 {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).msize.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_realloc64(
    arg1: *mut ::std::os::raw::c_void,
    arg2: sqlite3_uint64,
) -> *mut ::std::os::raw::c_void {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).realloc64.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_reset_auto_extension() {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).reset_auto_extension.unwrap_unchecked())()
}
pub unsafe fn sqlite3_result_blob64(
    arg1: *mut sqlite3_context,
    arg2: *const ::std::os::raw::c_void,
    arg3: sqlite3_uint64,
    arg4: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).result_blob64.unwrap_unchecked())(arg1, arg2, arg3, arg4)
}
pub unsafe fn sqlite3_result_text64(
    arg1: *mut sqlite3_context,
    arg2: *const ::std::os::raw::c_char,
    arg3: sqlite3_uint64,
    arg4: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    arg5: ::std::os::raw::c_uchar,
) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).result_text64.unwrap_unchecked())(arg1, arg2, arg3, arg4, arg5)
}
pub unsafe fn sqlite3_strglob(
    arg1: *const ::std::os::raw::c_char,
    arg2: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).strglob.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_value_dup(arg1: *const sqlite3_value) -> *mut sqlite3_value {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).value_dup.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_value_free(arg1: *mut sqlite3_value) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).value_free.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_result_zeroblob64(
    arg1: *mut sqlite3_context,
    arg2: sqlite3_uint64,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).result_zeroblob64.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_bind_zeroblob64(
    arg1: *mut sqlite3_stmt,
    arg2: ::std::os::raw::c_int,
    arg3: sqlite3_uint64,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).bind_zeroblob64.unwrap_unchecked())(arg1, arg2, arg3)
}
pub unsafe fn sqlite3_value_subtype(arg1: *mut sqlite3_value) -> ::std::os::raw::c_uint {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).value_subtype.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_result_subtype(arg1: *mut sqlite3_context, arg2: ::std::os::raw::c_uint) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).result_subtype.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_status64(
    arg1: ::std::os::raw::c_int,
    arg2: *mut sqlite3_int64,
    arg3: *mut sqlite3_int64,
    arg4: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).status64.unwrap_unchecked())(arg1, arg2, arg3, arg4)
}
pub unsafe fn sqlite3_strlike(
    arg1: *const ::std::os::raw::c_char,
    arg2: *const ::std::os::raw::c_char,
    arg3: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).strlike.unwrap_unchecked())(arg1, arg2, arg3)
}
pub unsafe fn sqlite3_db_cacheflush(arg1: *mut sqlite3) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).db_cacheflush.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_system_errno(arg1: *mut sqlite3) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).system_errno.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_trace_v2(
    arg1: *mut sqlite3,
    arg2: ::std::os::raw::c_uint,
    arg3: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: ::std::os::raw::c_uint,
            arg2: *mut ::std::os::raw::c_void,
            arg3: *mut ::std::os::raw::c_void,
            arg4: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    arg4: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).trace_v2.unwrap_unchecked())(arg1, arg2, arg3, arg4)
}
pub unsafe fn sqlite3_expanded_sql(arg1: *mut sqlite3_stmt) -> *mut ::std::os::raw::c_char {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).expanded_sql.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_set_last_insert_rowid(arg1: *mut sqlite3, arg2: sqlite3_int64) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).set_last_insert_rowid.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_prepare_v3(
    arg1: *mut sqlite3,
    arg2: *const ::std::os::raw::c_char,
    arg3: ::std::os::raw::c_int,
    arg4: ::std::os::raw::c_uint,
    arg5: *mut *mut sqlite3_stmt,
    arg6: *mut *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).prepare_v3.unwrap_unchecked())(arg1, arg2, arg3, arg4, arg5, arg6)
}
pub unsafe fn sqlite3_prepare16_v3(
    arg1: *mut sqlite3,
    arg2: *const ::std::os::raw::c_void,
    arg3: ::std::os::raw::c_int,
    arg4: ::std::os::raw::c_uint,
    arg5: *mut *mut sqlite3_stmt,
    arg6: *mut *const ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).prepare16_v3.unwrap_unchecked())(arg1, arg2, arg3, arg4, arg5, arg6)
}
pub unsafe fn sqlite3_bind_pointer(
    arg1: *mut sqlite3_stmt,
    arg2: ::std::os::raw::c_int,
    arg3: *mut ::std::os::raw::c_void,
    arg4: *const ::std::os::raw::c_char,
    arg5: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).bind_pointer.unwrap_unchecked())(arg1, arg2, arg3, arg4, arg5)
}
pub unsafe fn sqlite3_result_pointer(
    arg1: *mut sqlite3_context,
    arg2: *mut ::std::os::raw::c_void,
    arg3: *const ::std::os::raw::c_char,
    arg4: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).result_pointer.unwrap_unchecked())(arg1, arg2, arg3, arg4)
}
pub unsafe fn sqlite3_value_pointer(
    arg1: *mut sqlite3_value,
    arg2: *const ::std::os::raw::c_char,
) -> *mut ::std::os::raw::c_void {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).value_pointer.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_vtab_nochange(arg1: *mut sqlite3_context) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).vtab_nochange.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_value_nochange(arg1: *mut sqlite3_value) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).value_nochange.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_vtab_collation(
    arg1: *mut sqlite3_index_info,
    arg2: ::std::os::raw::c_int,
) -> *const ::std::os::raw::c_char {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).vtab_collation.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_keyword_count() -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).keyword_count.unwrap_unchecked())()
}
pub unsafe fn sqlite3_keyword_name(
    arg1: ::std::os::raw::c_int,
    arg2: *mut *const ::std::os::raw::c_char,
    arg3: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).keyword_name.unwrap_unchecked())(arg1, arg2, arg3)
}
pub unsafe fn sqlite3_keyword_check(
    arg1: *const ::std::os::raw::c_char,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).keyword_check.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_str_new(arg1: *mut sqlite3) -> *mut sqlite3_str {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).str_new.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_str_finish(arg1: *mut sqlite3_str) -> *mut ::std::os::raw::c_char {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).str_finish.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_str_appendf(
) -> unsafe extern "C" fn(arg1: *mut sqlite3_str, zFormat: *const ::std::os::raw::c_char, ...) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    (*API).str_appendf.unwrap_unchecked()
}
pub unsafe fn sqlite3_str_vappendf(
    arg1: *mut sqlite3_str,
    zFormat: *const ::std::os::raw::c_char,
    arg2: va_list,
) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).str_vappendf.unwrap_unchecked())(arg1, zFormat, arg2)
}
pub unsafe fn sqlite3_str_append(
    arg1: *mut sqlite3_str,
    zIn: *const ::std::os::raw::c_char,
    N: ::std::os::raw::c_int,
) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).str_append.unwrap_unchecked())(arg1, zIn, N)
}
pub unsafe fn sqlite3_str_appendall(arg1: *mut sqlite3_str, zIn: *const ::std::os::raw::c_char) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).str_appendall.unwrap_unchecked())(arg1, zIn)
}
pub unsafe fn sqlite3_str_appendchar(
    arg1: *mut sqlite3_str,
    N: ::std::os::raw::c_int,
    C: ::std::os::raw::c_char,
) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).str_appendchar.unwrap_unchecked())(arg1, N, C)
}
pub unsafe fn sqlite3_str_reset(arg1: *mut sqlite3_str) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).str_reset.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_str_errcode(arg1: *mut sqlite3_str) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).str_errcode.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_str_length(arg1: *mut sqlite3_str) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).str_length.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_str_value(arg1: *mut sqlite3_str) -> *mut ::std::os::raw::c_char {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).str_value.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_create_window_function(
    arg1: *mut sqlite3,
    arg2: *const ::std::os::raw::c_char,
    arg3: ::std::os::raw::c_int,
    arg4: ::std::os::raw::c_int,
    arg5: *mut ::std::os::raw::c_void,
    xStep: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_context,
            arg2: ::std::os::raw::c_int,
            arg3: *mut *mut sqlite3_value,
        ),
    >,
    xFinal: ::std::option::Option<unsafe extern "C" fn(arg1: *mut sqlite3_context)>,
    xValue: ::std::option::Option<unsafe extern "C" fn(arg1: *mut sqlite3_context)>,
    xInv: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut sqlite3_context,
            arg2: ::std::os::raw::c_int,
            arg3: *mut *mut sqlite3_value,
        ),
    >,
    xDestroy: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).create_window_function.unwrap_unchecked())(
        arg1, arg2, arg3, arg4, arg5, xStep, xFinal, xValue, xInv, xDestroy,
    )
}
pub unsafe fn sqlite3_normalized_sql(arg1: *mut sqlite3_stmt) -> *const ::std::os::raw::c_char {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).normalized_sql.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_stmt_isexplain(arg1: *mut sqlite3_stmt) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).stmt_isexplain.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_value_frombind(arg1: *mut sqlite3_value) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).value_frombind.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_drop_modules(
    arg1: *mut sqlite3,
    arg2: *mut *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).drop_modules.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_hard_heap_limit64(arg1: sqlite3_int64) -> sqlite3_int64 {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).hard_heap_limit64.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_uri_key(
    arg1: *const ::std::os::raw::c_char,
    arg2: ::std::os::raw::c_int,
) -> *const ::std::os::raw::c_char {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).uri_key.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_filename_database(
    arg1: *const ::std::os::raw::c_char,
) -> *const ::std::os::raw::c_char {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).filename_database.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_filename_journal(
    arg1: *const ::std::os::raw::c_char,
) -> *const ::std::os::raw::c_char {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).filename_journal.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_filename_wal(
    arg1: *const ::std::os::raw::c_char,
) -> *const ::std::os::raw::c_char {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).filename_wal.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_create_filename(
    arg1: *const ::std::os::raw::c_char,
    arg2: *const ::std::os::raw::c_char,
    arg3: *const ::std::os::raw::c_char,
    arg4: ::std::os::raw::c_int,
    arg5: *mut *const ::std::os::raw::c_char,
) -> *const ::std::os::raw::c_char {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).create_filename.unwrap_unchecked())(arg1, arg2, arg3, arg4, arg5)
}
pub unsafe fn sqlite3_free_filename(arg1: *const ::std::os::raw::c_char) {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).free_filename.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_database_file_object(
    arg1: *const ::std::os::raw::c_char,
) -> *mut sqlite3_file {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).database_file_object.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_txn_state(
    arg1: *mut sqlite3,
    arg2: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).txn_state.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_changes64(arg1: *mut sqlite3) -> sqlite3_int64 {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).changes64.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_total_changes64(arg1: *mut sqlite3) -> sqlite3_int64 {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).total_changes64.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_autovacuum_pages(
    arg1: *mut sqlite3,
    arg2: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_uint,
            arg4: ::std::os::raw::c_uint,
            arg5: ::std::os::raw::c_uint,
        ) -> ::std::os::raw::c_uint,
    >,
    arg3: *mut ::std::os::raw::c_void,
    arg4: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).autovacuum_pages.unwrap_unchecked())(arg1, arg2, arg3, arg4)
}
pub unsafe fn sqlite3_error_offset(arg1: *mut sqlite3) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).error_offset.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_vtab_rhs_value(
    arg1: *mut sqlite3_index_info,
    arg2: ::std::os::raw::c_int,
    arg3: *mut *mut sqlite3_value,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).vtab_rhs_value.unwrap_unchecked())(arg1, arg2, arg3)
}
pub unsafe fn sqlite3_vtab_distinct(arg1: *mut sqlite3_index_info) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).vtab_distinct.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_vtab_in(
    arg1: *mut sqlite3_index_info,
    arg2: ::std::os::raw::c_int,
    arg3: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).vtab_in.unwrap_unchecked())(arg1, arg2, arg3)
}
pub unsafe fn sqlite3_vtab_in_first(
    arg1: *mut sqlite3_value,
    arg2: *mut *mut sqlite3_value,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).vtab_in_first.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_vtab_in_next(
    arg1: *mut sqlite3_value,
    arg2: *mut *mut sqlite3_value,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).vtab_in_next.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_deserialize(
    arg1: *mut sqlite3,
    arg2: *const ::std::os::raw::c_char,
    arg3: *mut ::std::os::raw::c_uchar,
    arg4: sqlite3_int64,
    arg5: sqlite3_int64,
    arg6: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).deserialize.unwrap_unchecked())(arg1, arg2, arg3, arg4, arg5, arg6)
}
pub unsafe fn sqlite3_serialize(
    arg1: *mut sqlite3,
    arg2: *const ::std::os::raw::c_char,
    arg3: *mut sqlite3_int64,
    arg4: ::std::os::raw::c_uint,
) -> *mut ::std::os::raw::c_uchar {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).serialize.unwrap_unchecked())(arg1, arg2, arg3, arg4)
}
pub unsafe fn sqlite3_db_name(
    arg1: *mut sqlite3,
    arg2: ::std::os::raw::c_int,
) -> *const ::std::os::raw::c_char {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).db_name.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_value_encoding(arg1: *mut sqlite3_value) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).value_encoding.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_is_interrupted(arg1: *mut sqlite3) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).is_interrupted.unwrap_unchecked())(arg1)
}
pub unsafe fn sqlite3_stmt_explain(
    arg1: *mut sqlite3_stmt,
    arg2: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).stmt_explain.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_get_clientdata(
    arg1: *mut sqlite3,
    arg2: *const ::std::os::raw::c_char,
) -> *mut ::std::os::raw::c_void {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).get_clientdata.unwrap_unchecked())(arg1, arg2)
}
pub unsafe fn sqlite3_set_clientdata(
    arg1: *mut sqlite3,
    arg2: *const ::std::os::raw::c_char,
    arg3: *mut ::std::os::raw::c_void,
    arg4: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).set_clientdata.unwrap_unchecked())(arg1, arg2, arg3, arg4)
}
pub unsafe fn sqlite3_setlk_timeout(
    arg1: *mut sqlite3,
    arg2: ::std::os::raw::c_int,
    arg3: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).setlk_timeout.unwrap_unchecked())(arg1, arg2, arg3)
}
pub unsafe fn sqlite3_set_errmsg(
    arg1: *mut sqlite3,
    arg2: ::std::os::raw::c_int,
    arg3: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).set_errmsg.unwrap_unchecked())(arg1, arg2, arg3)
}
pub unsafe fn sqlite3_db_status64(
    arg1: *mut sqlite3,
    arg2: ::std::os::raw::c_int,
    arg3: *mut sqlite3_int64,
    arg4: *mut sqlite3_int64,
    arg5: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    debug_assert!(!API.is_null(), "SQLite API not initialized");
    ((*API).db_status64.unwrap_unchecked())(arg1, arg2, arg3, arg4, arg5)
}
