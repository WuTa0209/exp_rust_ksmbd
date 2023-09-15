use kernel::bindings::*;
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
pub const __LOG_PREFIX: &[u8] = b"wrapper\0";
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
    kernel::prelude::pr_info!("{:?}\n", authblob as *const c_char);
    kernel::prelude::pr_info!("{:?}\n", dn_off);
    kernel::prelude::pr_info!("{:?}\n", dn_off as usize);
    kernel::prelude::pr_info!("{:?}\n", unsafe{(authblob as *const c_char).add(dn_off as usize)});
    domain_name = unsafe{smb_strndup_from_utf16((authblob as *const c_char).add(dn_off as usize),
            dn_len as i32, true, (*conn).local_nls)};
    // domain_name = smb_strndup_from_utf16((const char *)authblob + dn_off,
    //           dn_len, true, conn->local_nls);
    if domain_name.is_null() {
        return unsafe{PTR_ERR(domain_name as *const core::ffi::c_void) as i32};
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

// pub fn rust_smb_strndup_from_utf16(
//     src: *const core::ffi::c_char,
//     maxlen: core::ffi::c_int,
//     is_unicode: bool_,
//     codepage: *const nls_table,
// ) -> *mut core::ffi::c_char {
//     let len: core::ffi::c_int;
//     let ret: core::ffi::c_int;
//     let mut dst: *mut core::ffi::c_char;

//     if is_unicode {
//         len = unsafe{smb_utf16_bytes(src as *const u16, maxlen, codepage)};
//         len += unsafe{nls_nullsize(codepage)};
//         dst = unsafe{kmalloc(len as usize, GFP_KERNEL) as *mut core::ffi::c_char};
//         if dst.is_null() {
//             return unsafe{ERR_PTR(-ENOMEM)};
//         }
//         ret = unsafe{smb_from_utf16(dst, src as *const u16, len as i32, maxlen, codepage,
//                 false)};
//         if ret < 0 {
//             unsafe{kfree(dst as *const core::ffi::c_void)};
//             return unsafe{ERR_PTR(-EINVAL)};
//         }
//     } else {
//         len = unsafe{strnlen(src, maxlen as usize)};
//         len += 1;
//         dst = unsafe{kmalloc(len as usize, GFP_KERNEL) as *mut core::ffi::c_char};
//         if dst.is_null() {
//             return unsafe{ERR_PTR(-ENOMEM)};
//         }
//         unsafe{strscpy(dst, src, len as usize)};
//     }

//     return dst;
// }
// char *smb_strndup_from_utf16(const char *src, const int maxlen,
//     const bool is_unicode,
//     const struct nls_table *codepage)
// {
// int len, ret;
// char *dst;

// if (is_unicode) {
// len = smb_utf16_bytes((__le16 *)src, maxlen, codepage);
// len += nls_nullsize(codepage);
// dst = kmalloc(len, GFP_KERNEL);
// if (!dst)
// return ERR_PTR(-ENOMEM);
// ret = smb_from_utf16(dst, (__le16 *)src, len, maxlen, codepage,
//         false);
// if (ret < 0) {
// kfree(dst);
// return ERR_PTR(-EINVAL);
// }
// } else {
// len = strnlen(src, maxlen);
// len++;
// dst = kmalloc(len, GFP_KERNEL);
// if (!dst)
// return ERR_PTR(-ENOMEM);
// strscpy(dst, src, len);
// }

// return dst;
// }
