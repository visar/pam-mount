use std::libc::{c_int, size_t, c_char};
use std::ptr;
#[allow(non_camel_case_types)]
pub type pam_handle_t = *uint;
#[allow(non_camel_case_types)]
type c_str = *c_char;

#[allow(non_camel_case_types)]
pub enum PamError {
	PAM_SUCCESS = 0
}

#[repr(uint)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
enum PamItemType {
	PAM_SERVICE	    = 1,	/* The service name */
	PAM_USER        = 2,	/* The user name */
	PAM_TTY         = 3,	/* The tty name */
	PAM_RHOST       = 4,	/* The remote host name */
	PAM_CONV        = 5,	/* The pam_conv structure */
	PAM_AUTHTOK     = 6,	/* The authentication token (password) */
	PAM_OLDAUTHTOK  = 7,	/* The old authentication token */
	PAM_RUSER       = 8,	/* The remote user name */
	PAM_USER_PROMPT = 9,    /* the prompt for getting a username Linux-PAM extensions */
	PAM_FAIL_DELAY  = 10,   /* app supplied function to override failure delays */
	PAM_XDISPLAY    = 11,   /* X display name */
	PAM_XAUTHDATA   = 12,   /* X server authentication data */
	PAM_AUTHTOK_TYPE= 13   /* The type for pam_get_authtok */
}

#[link(name = "pam")]
extern "C" {
	// int pam_get_item(const pam_handle_t *pamh, int item_type, const void **item);
	fn pam_get_item(pamh: pam_handle_t, item_type: c_int, item: *mut *i8) -> c_int;
}	


// PAM_EXTERN int pam_sm_setcred(pam_handle_t *pamh, int flags, int argc, const char **argv);
#[no_mangle]
#[allow(unused_variable)]
#[allow(dead_code)]
pub fn pam_sm_setcred(pamh: pam_handle_t, flags: c_int, argc: size_t, argv: *u8) -> c_int {
	// println!("pam_sm_setcred: hello from rust!!! {}", argc);
	PAM_SUCCESS as c_int
}

pub fn getPassword(pamh: pam_handle_t) -> ~str {
	getItem(pamh, PAM_AUTHTOK)
}

fn getItem(pamh: pam_handle_t, item_type: PamItemType) -> ~str {
	unsafe {
		let mut info: c_str = ptr::null();
		pam_get_item(pamh, item_type as c_int, &mut info);
		let z = ::std::c_str::CString::new(info, false);
		z.as_str().unwrap_or("").to_owned()
	}
}
