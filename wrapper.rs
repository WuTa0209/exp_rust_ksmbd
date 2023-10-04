use kernel::bindings::*;
// use kernel::prelude::*;
use kernel::fmt;
use core::ffi::*;

macro_rules! le32_to_cpu {
    ($x:expr) => {
        $x as u32
    };
}

macro_rules! le16_to_cpu {
    ($x:expr) => {
        $x as u16
    };
}

// #define __le32_to_cpu(x) ((__u32)(__le32)(x))
// #define __cpu_to_le16(x) ((__le16)(__u16)(x))
// #define __le16_to_cpu(x) ((__u16)(__le16)(x))

#[no_mangle]
extern "C" {
    pub fn rust_pr_info(
        val: ::core::ffi::c_uint);
}
//int cifs_arc4_setkey(struct arc4_ctx *ctx, const u8 *in_key, unsigned int key_len);
#[no_mangle]
extern "C" {
    pub fn cifs_arc4_setkey(ctx: *mut arc4_ctx, in_key: *const u8, key_len: core::ffi::c_uint) -> core::ffi::c_int;
}
// void cifs_arc4_crypt(struct arc4_ctx *ctx, u8 *out, const u8 *in, unsigned int len);
#[no_mangle]
extern "C" {
    pub fn cifs_arc4_crypt(ctx: *mut arc4_ctx, out: *mut u8, in_: *const u8, len: core::ffi::c_uint);
}

#[no_mangle]
extern "C" {
    #[link(name="rust_helper_ERR_PTR")]
    pub fn rust_helper_ERR_PTR(error: i64) -> *mut core::ffi::c_void;
}

// long rust_helper_PTR_ERR(__force const void *ptr)
// {
// 	return PTR_ERR(ptr);
// }
#[no_mangle]
#[link(name="rust_helper_PTR_ERR")]
extern "C" {
    pub fn rust_helper_PTR_ERR(ptr: *const core::ffi::c_void) -> i64;
}

pub const __LOG_PREFIX: &[u8] = b"wrapper\0";

// #[no_mangle]
// extern "C" {
//     pub fn rust_ksmbd_decode_ntlmssp_auth_blob(authblob: *mut authenticate_message,
//         blob_len:  core::ffi::c_int,
//         conn: *mut ksmbd_conn,
//         sess: *mut ksmbd_session,
//     ) ->  core::ffi::c_int
// }

#[no_mangle]
pub fn rust_ksmbd_decode_ntlmssp_auth_blob(authblob: *mut authenticate_message,
    blob_len:  core::ffi::c_int,
    conn: *mut ksmbd_conn,
    sess: *mut ksmbd_session,
) ->  core::ffi::c_int{
    let domain_name: *mut core::ffi::c_char;
    let nt_off: u32;
    let dn_off: u32;
    let nt_len: u16;
    let dn_len: u16;
    let ret: core::ffi::c_int;
    // unsafe{_printk("%x", blob_len)};
    // if (blob_len < sizeof(struct authenticate_message))
    // generate rust code following above c code
    // unsafe{rust_pr_info(core::mem::size_of::<authenticate_message>() as u32)};
    
    if blob_len < core::mem::size_of::<authenticate_message>() as core::ffi::c_int {
        // unsafe{ksmbd_debug(AUTH, "negotiate blob len %d too small\n",
        //     blob_len)};
        return -EINVAL;
    }
    
    //BUG 
    // if unsafe{memcmp((*authblob).Signature.as_ptr() as *const c_void, "NTLMSSP", 8)} != 0 {
    if unsafe{(*authblob).Signature} != unsafe{*(b"NTLMSSP\0")} {
        // unsafe{rust_pr_info((*authblob).Signature.as_ptr() as u32)};
        // unsafe{ksmbd_debug(AUTH, "blob signature incorrect %s\n",
        //     (*authblob).Signature.as_ptr())};
        return -EINVAL;
    }
    // kernel::prelude::pr_info!("{:?}\n", unsafe{(*authblob).NtChallengeResponse.BufferOffset});
    // kernel::prelude::pr_info!("{:?}\n", unsafe{(*authblob).NtChallengeResponse.Length});
    // kernel::prelude::pr_info!("{:?}\n", unsafe{(*authblob).DomainName.BufferOffset});
    // kernel::prelude::pr_info!("{:?}\n", unsafe{(*authblob).DomainName.Length});

    nt_off = unsafe{le32_to_cpu!((*authblob).NtChallengeResponse.BufferOffset)};
    nt_len = unsafe{le16_to_cpu!((*authblob).NtChallengeResponse.Length)};
    dn_off = unsafe{le32_to_cpu!((*authblob).DomainName.BufferOffset)};
    dn_len = unsafe{le16_to_cpu!((*authblob).DomainName.Length)};
    // nt_off = le32_to_cpu(authblob->NtChallengeResponse.BufferOffset);
    // nt_len = le16_to_cpu(authblob->NtChallengeResponse.Length);
    // dn_off = le32_to_cpu(authblob->DomainName.BufferOffset);
    // dn_len = le16_to_cpu(authblob->DomainName.Length);
    // kernel::prelude::pr_info!("{:?}\n", nt_off);
    // kernel::prelude::pr_info!("{:?}\n", nt_len);
    // kernel::prelude::pr_info!("{:?}\n", dn_off);
    // kernel::prelude::pr_info!("{:?}\n", dn_len);
    // unsafe{dump_stack()};
    if (blob_len < ((dn_off as core::ffi::c_ulonglong + dn_len as core::ffi::c_ulonglong) as core::ffi::c_int)) || (blob_len < ((nt_off as core::ffi::c_ulonglong + nt_len as core::ffi::c_ulonglong) as core::ffi::c_int)) {
        return -EINVAL;
    }
    // if (blob_len < (u64)dn_off + dn_len || blob_len < (u64)nt_off + nt_len ||
// nt_len < CIFS_ENCPWD_SIZE)
    /* TODO : use domain name that imported from configuration file */
    /*
        這段程式碼是在C語言中進行指標操作，其中 authblob 是一個指向結構或數組的指針，
        dn_off 則是一個無號整數。當你使用 authblob + dn_off 時，實際上是在計算一
        個新的指針位置，它指向了 authblob 指向的記憶體中的特定位置，並且這個位置在原
        始指標的基礎上偏移了 dn_off 個元素或字節（視情況而定）。
     */
    // unsafe{rust_pr_info((authblob as *const c_char).add(dn_off as usize) as u32)};
    // kernel::prelude::pr_info!("{:?}\n", authblob as *const c_char);
    // kernel::prelude::pr_info!("{:?}\n", dn_off);
    // kernel::prelude::pr_info!("{:?}\n", dn_off as usize);
    // kernel::prelude::pr_info!("{:?}\n", unsafe{(authblob as *const c_char).add(dn_off as usize)});
    // unsafe{rust_pr_info(1)};
    domain_name = unsafe{smb_strndup_from_utf16((authblob as *const c_char).add(dn_off as usize),
            dn_len as i32, true, (*conn).local_nls)};
    // domain_name = smb_strndup_from_utf16((const char *)authblob + dn_off,
    //           dn_len, true, conn->local_nls);
    if domain_name.is_null() {
        return unsafe{rust_helper_PTR_ERR(domain_name as *const core::ffi::c_void) as i32};
        // return 0;
    }

    /* process NTLMv2 authentication */
    // unsafe{ksmbd_debug(AUTH, "decode_ntlmssp_authenticate_blob dname%s\n",
    //     domain_name)};

    //There is BUG Happen
    ret = unsafe{ksmbd_auth_ntlmv2(conn, sess,
        (authblob as *mut c_char).add(nt_off as usize) as *mut ntlmv2_resp,
        (nt_len - CIFS_ENCPWD_SIZE as u16) as core::ffi::c_int,
        domain_name, (*conn).ntlmssp.cryptkey.as_mut_ptr())};
// ret = ksmbd_auth_ntlmv2(conn, sess,
    //  (struct ntlmv2_resp *)((char *)authblob + nt_off),
    //  nt_len - CIFS_ENCPWD_SIZE,
    //  domain_name, conn->ntlmssp.cryptkey);

    
    unsafe{kfree(domain_name as *const core::ffi::c_void)};
    if ret < 0 {
        return ret;
    }

    /* The recovered secondary session key */
    if unsafe{(*conn).ntlmssp.client_flags & NTLMSSP_NEGOTIATE_KEY_XCH} != 0 {
        let mut ctx_arc4: *mut arc4_ctx;

        let sess_key_off: u32;
        let sess_key_len: u16;

        sess_key_off = unsafe{le32_to_cpu!((*authblob).SessionKey.BufferOffset)};
        sess_key_len = unsafe{le16_to_cpu!((*authblob).SessionKey.Length)};

        if blob_len < (sess_key_off + u32::from(sess_key_len)) as core::ffi::c_int {
            return -EINVAL;
        }

        ctx_arc4 = unsafe{__kmalloc(core::mem::size_of::<arc4_ctx>() as usize, GFP_KERNEL) as *mut kernel::bindings::arc4_ctx};
        if ctx_arc4.is_null() {
            return -ENOMEM;
        }

        unsafe{cifs_arc4_setkey(ctx_arc4, (*sess).sess_key.as_mut_ptr() as *mut u8, 
            SMB2_NTLMV2_SESSKEY_SIZE as core::ffi::c_uint)};
        unsafe{cifs_arc4_crypt(ctx_arc4, (*sess).sess_key.as_mut_ptr() as *mut u8,
            (authblob as *mut c_char).add(sess_key_off as usize) as *const u8, sess_key_len as core::ffi::c_uint)};
        // cifs_arc4_crypt(ctx_arc4, sess->sess_key,
        //  (char *)authblob + sess_key_off, sess_key_len);
        unsafe{kfree_sensitive(ctx_arc4 as *const core::ffi::c_void)};
    }

    return ret;
}

// extern "C" {
//     #[link(name = "smb_utf16_bytes")]
//     pub fn rust_smb_utf16_bytes(from: *const u16, maxbytes: core::ffi::c_int,
//         codepage: *const nls_table) -> core::ffi::c_int;    
// }

// extern "C" {
//     #[link(name = "smb_from_utf16")]
//     pub fn rust_smb_from_utf16(to: *mut core::ffi::c_char, from: *const u16, tolen: core::ffi::c_int,
//         fromlen: core::ffi::c_int, codepage: *const nls_table, mapchar: bool_) -> core::ffi::c_int;
// }




//static inline void * __must_check ERR_PTR(long error)
// {
// 	return (void *) error;
// }

// static inline int
// nls_nullsize(const struct nls_table *codepage)
// {
// 	int charlen;
// 	char tmp[NLS_MAX_CHARSET_SIZE];

// 	charlen = codepage->uni2char(0, tmp, NLS_MAX_CHARSET_SIZE);

// 	return charlen > 0 ? charlen : 1;
// }
pub fn rust_nls_nullsize(codepage: *const nls_table) -> core::ffi::c_int {
    let mut charlen: core::ffi::c_int;
    let mut tmp: [core::ffi::c_char; NLS_MAX_CHARSET_SIZE as usize] = [0; NLS_MAX_CHARSET_SIZE as usize];
    let uni2char_func = unsafe{(*codepage).uni2char.unwrap()};
    charlen = unsafe{uni2char_func(0, tmp.as_mut_ptr() as *mut u8, NLS_MAX_CHARSET_SIZE as i32)};
    if charlen > 0 {
        return charlen;
    } else {
        return 1;
    }
}

#[feature(vec_into_raw_parts)]
#[no_mangle]
pub extern "C" fn rust_ksmbd_alloc_user(resp: *mut ksmbd_login_response) -> *mut ksmbd_user {
    let mut _passkey = kernel::prelude::Box::try_new(unsafe{(*resp).hash_sz} as usize).unwrap();
    let mut user: kernel::prelude::Box<ksmbd_user> = kernel::prelude::Box::try_new(ksmbd_user{
        name: unsafe{kstrdup((*resp).account.as_ptr(), GFP_KERNEL)},
        passkey: kernel::prelude::Box::into_raw(_passkey) as *mut i8,
        passkey_sz: unsafe{(*resp).hash_sz as usize},
        flags: unsafe{(*resp).status},
        uid: unsafe{(*resp).uid},
        gid: unsafe{(*resp).gid},
        failed_login_count: 0,
    }).unwrap();
    
    if user.name.is_null() || user.passkey.is_null(){
        drop(user);
        return core::ptr::null_mut();
    }
    
    unsafe{core::ptr::copy_nonoverlapping( unsafe{(*resp).hash.as_ptr()}, user.passkey, user.passkey_sz as usize)};
    kernel::prelude::Box::into_raw(user)
}


// struct ksmbd_user *ksmbd_alloc_user(struct ksmbd_login_response *resp)
// {
// 	struct ksmbd_user *user = NULL;

// 	user = kmalloc(sizeof(struct ksmbd_user), GFP_KERNEL);
// 	if (!user)
// 		return NULL;

// 	user->name = kstrdup(resp->account, GFP_KERNEL);
// 	user->flags = resp->status;
// 	user->gid = resp->gid;
// 	user->uid = resp->uid;
// 	user->passkey_sz = resp->hash_sz;
// 	user->passkey = kmalloc(resp->hash_sz, GFP_KERNEL);
// 	if (user->passkey)
// 		memcpy(user->passkey, resp->hash, resp->hash_sz);

// 	if (!user->name || !user->passkey) {
// 		kfree(user->name);
// 		kfree(user->passkey);
// 		kfree(user);
// 		user = NULL;
// 	}
// 	return user;
// }