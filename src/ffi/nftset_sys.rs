/* automatically generated by rust-bindgen 0.71.1 */

unsafe extern "C" {
    pub fn nftset_add(
        familyname: *const ::std::os::raw::c_char,
        tablename: *const ::std::os::raw::c_char,
        setname: *const ::std::os::raw::c_char,
        addr: *const ::std::os::raw::c_uchar,
        addr_len: ::std::os::raw::c_int,
        timeout: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn nftset_del(
        familyname: *const ::std::os::raw::c_char,
        tablename: *const ::std::os::raw::c_char,
        setname: *const ::std::os::raw::c_char,
        addr: *const ::std::os::raw::c_uchar,
        addr_len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
